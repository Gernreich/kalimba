//! The kalimba body from a few numbers: its cut file and its SolidCraft
//! design (2026-11-08; until then the box came off Boxes.py and the rest
//! was placed by hand in Inkscape).
//!
//!     target/release/kalimba [--ply=MM --radius=MM ...] [--out=DIR] [--solidcraft=DIR] [--no-write]
//!
//! THE BOX is Boxes.py's RegularBox as the sheet was made with it (radius
//! 90 outside, seven sides, h=65 outside, top and bottom closed, finger and
//! space twice the ply, burn 0.07), worked out the way RegularBox works it
//! out: the faces are the outside heptagon with a slot under every wall
//! finger, and each wall stands on one edge, its bottom and top fingers
//! counted along the inside edge, one end with fingers and the other cut
//! back with slots for the next wall's, angled for the 360/sides corner. At
//! the defaults every outline lies on Boxes' own, kerf-free, to the
//! micrometre, and with the kerf on to within 0.05 mm² of Boxes' own. The
//! count of sides is odd, as the rosette's must be, so every wall has one
//! fingered end and one slotted.
//!
//! THE KERF. Boxes grows each outline by half the kerf with round outside
//! corners and sharp inside ones; so does this, on the box's parts only.
//! The holes, the rosette and the stiffeners are drawn true: none of them
//! is a fit. The SolidCraft design is the parts as they come off the laser,
//! so the kerf is not in it.
//!
//! THE ROSETTE is knotwork-soundholes' 2-lead knot with as many bights as
//! the box has sides, read from that repository's 30 mm cut file (so it
//! expects the checkout beside this one) and scaled to `rosette_d`. The
//! drawn one was the same file scaled by 0.80 and turned by hand, a little
//! skewed and 1.7° off the corners; here it is regular: scaled evenly,
//! a bight pointing at every corner, its centre `rosette_offset` from the
//! face's towards one corner, as the drawn one was.
//!
//! THE STIFFENERS are three slices of the box's inside heptagon, each cut
//! off `stiffener_depth` in from one edge, with both corners of that edge
//! cut back `stiffener_corner` along each side; they glue under the rosette
//! face. The drawn ones were this shape to the tenth of a millimetre, turned
//! half a degree. Where they go is drawn nowhere, so they are on the sheet
//! and not in the 3D view or the design.
use lasermade_tools::flat::{f as fm, outs, write, Checks, Params, Sheet, Unit::*};
use lasermade_tools::solidcraft::{n as nn, Script};
use std::f64::consts::PI;

const HERE: &str = env!("CARGO_MANIFEST_DIR");
const BED: (f64, f64) = (600.0, 305.0);
const STEM: &str = "KalimbaSeptaBox";
const GAP: f64 = 3.0; // between parts on the sheet
const CLEAR: f64 = 3.0; // a hole's least ply to the walls

fn table() -> Params {
    Params::new(&[
        ("ply", 3.0, Len, "the ply"),
        ("radius", 90.0, Len, "the box's outside corner radius"),
        ("height", 65.0, Len, "the box's outside height, face to face"),
        ("sides", 7.0, Num, "sides of the box, and bights of the rosette; read only in the design"),
        ("finger", 2.0, Ratio, "a finger's width, in plies"),
        ("space", 2.0, Ratio, "the space between fingers, in plies"),
        ("kerf", 0.14, Len, "the laser's kerf: the box's outlines grow by half of it; not in the design"),
        ("rosette_d", 48.0, Len, "the rosette's rim, across"),
        ("rosette_offset", 5.5, Len, "the rosette's centre from the face's, towards a corner"),
        ("hole_d", 15.0, Len, "the plain face's round hole"),
        ("hole_offset", 7.3, Len, "the hole's centre from the face's, towards an edge"),
        ("stiffeners", 3.0, Num, "stiffeners on the sheet"),
        ("stiffener_depth", 36.4, Len, "a stiffener, from the wall it sits against to its long edge"),
        ("stiffener_corner", 4.7, Len, "how far each of its two wall-side corners is cut back, along each side"),
    ])
}

type P = (f64, f64);

// --- lengths as sums of named lengths ------------------------------------------------

/// A length as a sum of named parameters times fixed numbers: every point
/// of the box is one, because the corner directions are fixed by `sides`
/// and the finger counts by the defaults' sizes.
#[derive(Clone, Debug, Default)]
struct Lin(Vec<(&'static str, f64)>);

impl Lin {
    fn of(name: &'static str, k: f64) -> Lin {
        Lin(vec![(name, k)])
    }
    fn add(&self, o: &Lin) -> Lin {
        let mut v = self.0.clone();
        for &(n, k) in &o.0 {
            match v.iter_mut().find(|(m, _)| *m == n) {
                Some(t) => t.1 += k,
                None => v.push((n, k)),
            }
        }
        Lin(v)
    }
    fn mul(&self, k: f64) -> Lin {
        Lin(self.0.iter().map(|&(n, c)| (n, c * k)).collect())
    }
    fn val(&self, env: &[(&'static str, f64)]) -> f64 {
        self.0.iter().map(|&(n, k)| k * env.iter().find(|(m, _)| *m == n).unwrap_or_else(|| panic!("no {n}")).1).sum()
    }
    fn e(&self) -> String {
        let t: Vec<String> = self.0.iter().filter(|(_, k)| k.abs() > 1e-12).map(|&(n, k)| format!("{}*{n}", nn(k))).collect();
        if t.is_empty() {
            "0 mm".into()
        } else {
            format!("({})", t.join(" + "))
        }
    }
}

#[derive(Clone, Debug)]
struct Pt {
    x: Lin,
    y: Lin,
}

impl Pt {
    fn new(x: Lin, y: Lin) -> Pt {
        Pt { x, y }
    }
    fn plus(&self, o: &Pt) -> Pt {
        Pt::new(self.x.add(&o.x), self.y.add(&o.y))
    }
    /// a length along a fixed direction
    fn along(l: &Lin, d: P) -> Pt {
        Pt::new(l.mul(d.0), l.mul(d.1))
    }
}

// --- the box ---------------------------------------------------------------------------

/// RegularBox's numbers for this box.
struct Bx {
    n: usize,
    /// the named lengths every point is a sum of, at their values
    env: Vec<(&'static str, f64)>,
    nb: usize,
    nl: usize,
}

/// Boxes.py's finger count and leftover for an edge (surrounding spaces 1).
fn fingers(len: f64, f: f64, s: f64) -> (usize, f64) {
    let k = (len / (s + f)).floor().max(0.0) as usize;
    (k, len - k as f64 * (s + f) + s)
}

impl Bx {
    fn of(p: &Params) -> Result<Bx, String> {
        let n = p.get("sides") as usize;
        if n < 3 || n % 2 == 0 {
            return Err("error: --sides: an odd number, 3 at least: the rosette is a 2-lead knot with a bight for every side, and 2 leads need an odd number of bights to be one strand.".into());
        }
        let (t, r) = (p.get("ply"), p.get("radius"));
        let pn = PI / n as f64;
        let (fw, sw) = (p.get("finger") * t, p.get("space") * t);
        let r0 = r - t / pn.cos(); // the inside corner radius
        let side0 = 2.0 * pn.sin() * r0;
        let l = p.get("height") - 2.0 * t;
        let (nb, lob) = fingers(side0, fw, sw);
        let (nl, lol) = fingers(l, fw, sw);
        // the corner joint: RegularBox's angle is 360/sides; the slotted end
        // stands out tan(180/sides) ply past the inside corner, the fingers
        // that plus the space's recess
        let tp = t * pn.tan();
        let hg = tp * (1.0 + (PI / 2.0 - 2.0 * pn).sin());
        let env = vec![
            ("ply", t),
            ("radius", r),
            ("fw", fw),
            ("sw", sw),
            ("side0", side0),
            ("wall_l", l),
            ("lob", lob),
            ("lol", lol),
            ("tp", tp),
            ("hg", hg),
            ("a_out", r * pn.cos()),
            ("rosette_d", p.get("rosette_d")),
            ("rosette_offset", p.get("rosette_offset")),
        ];
        Ok(Bx { n, env, nb, nl })
    }
    fn v(&self, name: &str) -> f64 {
        self.env.iter().find(|(m, _)| *m == name).unwrap().1
    }
    fn val(&self, p: &Pt) -> P {
        (p.x.val(&self.env), p.y.val(&self.env))
    }

    /// A wall, in its own plane: x along the face edge from the inside
    /// corner, y up from the bottom face's underside. Counter-clockwise.
    /// `g` at an end: fingers there; otherwise cut back with slots.
    fn wall(&self, left_g: bool, right_g: bool) -> Vec<Pt> {
        let k = |n: &'static str| Lin::of(n, 1.0);
        let z = Lin::default();
        let (t, side0, l) = (k("ply"), k("side0"), k("wall_l"));
        let tl = t.add(&l); // the top face's underside
        let top = tl.add(&t);
        let mut o: Vec<Pt> = vec![Pt::new(z.clone(), t.clone())];
        // the bottom: fingers down through the bottom face
        let mut x = Lin::of("lob", 0.5);
        for i in 0..self.nb {
            if i > 0 {
                x = x.add(&k("sw"));
            }
            let x1 = x.add(&k("fw"));
            o.extend([Pt::new(x.clone(), t.clone()), Pt::new(x.clone(), z.clone()), Pt::new(x1.clone(), z.clone()), Pt::new(x1.clone(), t.clone())]);
            x = x1;
        }
        o.push(Pt::new(side0.clone(), t.clone()));
        // the far end, going up
        let ups = |o: &mut Vec<Pt>, at: &Lin, out: &Lin| {
            let mut y = t.add(&Lin::of("lol", 0.5));
            for i in 0..self.nl {
                if i > 0 {
                    y = y.add(&k("sw"));
                }
                let y1 = y.add(&k("fw"));
                o.extend([Pt::new(at.clone(), y.clone()), Pt::new(out.clone(), y.clone()), Pt::new(out.clone(), y1.clone()), Pt::new(at.clone(), y1.clone())]);
                y = y1;
            }
        };
        if right_g {
            ups(&mut o, &side0, &side0.add(&k("hg")));
        } else {
            let edge = side0.add(&k("tp"));
            o.push(Pt::new(edge.clone(), t.clone()));
            ups(&mut o, &edge, &edge.add(&k("hg").mul(-1.0)));
            o.push(Pt::new(edge, tl.clone()));
        }
        o.push(Pt::new(side0.clone(), tl.clone()));
        // the top: fingers up through the top face
        let mut x = side0.add(&Lin::of("lob", -0.5));
        for i in 0..self.nb {
            if i > 0 {
                x = x.add(&k("sw").mul(-1.0));
            }
            let x1 = x.add(&k("fw").mul(-1.0));
            o.extend([Pt::new(x.clone(), tl.clone()), Pt::new(x.clone(), top.clone()), Pt::new(x1.clone(), top.clone()), Pt::new(x1.clone(), tl.clone())]);
            x = x1;
        }
        o.push(Pt::new(z.clone(), tl.clone()));
        // the near end, going down
        let downs = |o: &mut Vec<Pt>, at: &Lin, out: &Lin| {
            let mut y = tl.add(&Lin::of("lol", -0.5));
            for i in 0..self.nl {
                if i > 0 {
                    y = y.add(&k("sw").mul(-1.0));
                }
                let y1 = y.add(&k("fw").mul(-1.0));
                o.extend([Pt::new(at.clone(), y.clone()), Pt::new(out.clone(), y.clone()), Pt::new(out.clone(), y1.clone()), Pt::new(at.clone(), y1.clone())]);
                y = y1;
            }
        };
        if left_g {
            downs(&mut o, &z, &k("hg").mul(-1.0));
        } else {
            let edge = k("tp").mul(-1.0);
            o.push(Pt::new(edge.clone(), tl.clone()));
            downs(&mut o, &edge, &edge.add(&k("hg")));
            o.push(Pt::new(edge, t.clone()));
        }
        self.tidy(o)
    }

    /// A corner of the outside heptagon, and an edge's direction: RegularBox
    /// draws the first edge along the bottom.
    fn corner_dir(&self, k: usize) -> (P, P) {
        let a = |k: usize| (-90.0 - 180.0 / self.n as f64 + 360.0 * k as f64 / self.n as f64).to_radians();
        let c = (a(k).cos(), a(k).sin());
        let e = 2.0 * PI * k as f64 / self.n as f64;
        (c, (e.cos(), e.sin()))
    }

    /// A face: the outside heptagon, a slot under every wall finger.
    fn face(&self) -> Vec<Pt> {
        let mut o = vec![];
        for k in 0..self.n {
            let (c, d) = self.corner_dir(k);
            let m = (-d.1, d.0); // inward
            let a = Pt::new(Lin::of("radius", c.0), Lin::of("radius", c.1));
            o.push(a.clone());
            let mut s = Lin::of("tp", 1.0).add(&Lin::of("lob", 0.5));
            for i in 0..self.nb {
                if i > 0 {
                    s = s.add(&Lin::of("sw", 1.0));
                }
                let s1 = s.add(&Lin::of("fw", 1.0));
                let inn = Pt::along(&Lin::of("ply", 1.0), m);
                o.push(a.plus(&Pt::along(&s, d)));
                o.push(a.plus(&Pt::along(&s, d)).plus(&inn));
                o.push(a.plus(&Pt::along(&s1, d)).plus(&inn));
                o.push(a.plus(&Pt::along(&s1, d)));
                s = s1;
            }
        }
        self.tidy(o)
    }

    /// Drop repeated points and points on a straight run.
    fn tidy(&self, o: Vec<Pt>) -> Vec<Pt> {
        let mut v: Vec<Pt> = vec![];
        for p in o {
            if v.last().map_or(true, |q| dist(self.val(q), self.val(&p)) > 1e-9) {
                v.push(p);
            }
        }
        while v.len() > 1 && dist(self.val(&v[0]), self.val(v.last().unwrap())) < 1e-9 {
            v.pop();
        }
        loop {
            let n = v.len();
            let k = (0..n).find(|&i| {
                let (a, b, c) = (self.val(&v[(i + n - 1) % n]), self.val(&v[i]), self.val(&v[(i + 1) % n]));
                ((b.0 - a.0) * (c.1 - b.1) - (b.1 - a.1) * (c.0 - b.0)).abs() < 1e-9
            });
            match k {
                Some(i) => {
                    v.remove(i);
                }
                None => return v,
            }
        }
    }

    /// The inside heptagon's apothem: the walls' inner faces.
    fn a_in(&self) -> f64 {
        self.v("a_out") - self.v("ply")
    }
}

fn dist(a: P, b: P) -> f64 {
    (a.0 - b.0).hypot(a.1 - b.1)
}

fn shoe(p: &[P]) -> f64 {
    let n = p.len();
    (0..n).map(|i| p[i].0 * p[(i + 1) % n].1 - p[(i + 1) % n].0 * p[i].1).sum::<f64>() / 2.0
}

// --- the rosette ---------------------------------------------------------------------

struct Knot {
    /// the cut file's radius
    r: f64,
    cut: Vec<Vec<P>>,
    engrave: Vec<Vec<P>>,
    file: String,
}

/// knotwork-soundholes' 2-lead knot, in its own frame turned right way up
/// (y up), from its cut file: every path there is M x y L x y ... [Z].
fn knot(bights: usize) -> Result<Knot, String> {
    let file = format!("../knotwork-soundholes/2-lead_{bights}-bight_knot_radius30mm.svg");
    let path = format!("{HERE}/{file}");
    let t = std::fs::read_to_string(&path).map_err(|e| format!("error: {file}: {e}. The rosette is knotwork-soundholes' 2-lead knot with as many bights as the box has sides; that checkout goes beside this one, and it ships 3, 5 and 7."))?;
    let group = |id: &str| -> Result<String, String> {
        let at = t.find(&format!("id=\"{id}\"")).ok_or(format!("error: {file} has no {id} group"))?;
        let d = t[at..].find(" d=\"").ok_or(format!("error: {file}: no path in {id}"))? + at + 4;
        let e = t[d..].find('"').unwrap() + d;
        Ok(t[d..e].to_string())
    };
    let loops = |d: &str| -> Vec<Vec<P>> {
        d.split('M')
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                let nums: Vec<f64> = s.replace(['L', 'Z'], " ").split_whitespace().map(|v| v.parse().unwrap()).collect();
                let mut v: Vec<P> = vec![];
                for c in nums.chunks(2) {
                    // a closed loop repeats its first point at the end
                    if v.last() != Some(&(c[0], -c[1])) && !(v.len() > 1 && v[0] == (c[0], -c[1])) {
                        v.push((c[0], -c[1]));
                    }
                }
                v
            })
            .collect()
    };
    Ok(Knot { r: 30.0, cut: loops(&group("cut")?), engrave: loops(&group("engrave")?), file })
}

/// Where the rosette goes in a face's frame: a point of the knot as a sum
/// of `rosette_d` and `rosette_offset`. A bight points at every corner.
fn rosette_pt(b: &Bx, k: &Knot, q: P) -> Pt {
    let rho = (-90.0 + 180.0 / b.n as f64).to_radians();
    let (c, s) = (rho.cos(), rho.sin());
    let (x, y) = ((q.0 * c - q.1 * s) / (2.0 * k.r), (q.0 * s + q.1 * c) / (2.0 * k.r));
    Pt::new(Lin::of("rosette_d", x).add(&Lin::of("rosette_offset", c)), Lin::of("rosette_d", y).add(&Lin::of("rosette_offset", s)))
}

// --- the stiffener -----------------------------------------------------------------

/// A slice of the inside heptagon against its bottom edge, both of that
/// edge's corners cut back. Counter-clockwise, in the face's frame.
fn stiffener(b: &Bx, depth: f64, cut: f64) -> Vec<P> {
    let (a, half) = (b.a_in(), b.v("side0") / 2.0);
    let e = 2.0 * PI / b.n as f64; // the next edges turn this far
    let (l0, r0) = ((-half, -a), (half, -a));
    let (dr, dl) = ((e.cos(), e.sin()), (-e.cos(), e.sin()));
    let up = depth / e.sin(); // along a side to the long edge
    vec![
        (l0.0 + cut, l0.1),
        (r0.0 - cut, r0.1),
        (r0.0 + cut * dr.0, r0.1 + cut * dr.1),
        (r0.0 + up * dr.0, r0.1 + up * dr.1),
        (l0.0 + up * dl.0, l0.1 + up * dl.1),
        (l0.0 + cut * dl.0, l0.1 + cut * dl.1),
    ]
}

// --- the cut sheet -------------------------------------------------------------------

/// An outline grown by `g` the way Boxes.py's burn grows it: round outside
/// corners, sharp inside ones. Counter-clockwise points in; path data out,
/// through `at` onto the sheet; and the grown area.
fn grown(p: &[P], g: f64, at: &dyn Fn(P) -> P) -> (String, f64) {
    let n = p.len();
    let pt = |q: P| {
        let s = at(q);
        format!("{},{}", fm(s.0), fm(s.1))
    };
    let unit = |a: P, b: P| {
        let l = dist(a, b);
        ((b.0 - a.0) / l, (b.1 - a.1) / l)
    };
    let mut d = String::new();
    let (mut per, mut turn) = (0.0, 0.0);
    for i in 0..n {
        let (a, v, c) = (p[(i + n - 1) % n], p[i], p[(i + 1) % n]);
        per += dist(v, c);
        let (u1, u2) = (unit(a, v), unit(v, c));
        let (n1, n2) = ((u1.1, -u1.0), (u2.1, -u2.0)); // outward
        let cr = u1.0 * u2.1 - u1.1 * u2.0;
        let th = cr.atan2(u1.0 * u2.0 + u1.1 * u2.1);
        let lead = if i == 0 { "M" } else { "L" };
        if g == 0.0 {
            d.push_str(&format!("{lead} {} ", pt(v)));
        } else if cr > 0.0 {
            // outside corner: an arc round it
            turn += g * g * th / 2.0;
            d.push_str(&format!("{lead} {} A {},{} 0 0 1 {} ", pt((v.0 + g * n1.0, v.1 + g * n1.1)), fm(g), fm(g), pt((v.0 + g * n2.0, v.1 + g * n2.1))));
        } else {
            turn -= g * g * (th.abs() / 2.0).tan();
            let k = g / (1.0 + n1.0 * n2.0 + n1.1 * n2.1);
            d.push_str(&format!("{lead} {} ", pt((v.0 + k * (n1.0 + n2.0), v.1 + k * (n1.1 + n2.1)))));
        }
    }
    d.push('Z');
    (d, shoe(p) + per * g + turn)
}

fn bbox(p: &[P]) -> (f64, f64, f64, f64) {
    let xs = p.iter().map(|q| q.0);
    let ys = p.iter().map(|q| q.1);
    (xs.clone().fold(f64::INFINITY, f64::min), ys.clone().fold(f64::INFINITY, f64::min), xs.fold(f64::NEG_INFINITY, f64::max), ys.fold(f64::NEG_INFINITY, f64::max))
}

/// Walls in order round the box, and whether each has fingers at its near
/// and far ends: slots at the near end, fingers at the far.
fn kinds(n: usize) -> Vec<(bool, bool)> {
    vec![(false, true); n]
}

// --- the SolidCraft design -------------------------------------------------------

fn design(p: &Params, b: &Bx, k: &Knot) -> (String, Vec<(String, f64)>) {
    let mut s = Script::new();
    for q in &p.0 {
        if ["kerf", "stiffeners", "stiffener_depth", "stiffener_corner"].contains(&q.name) {
            continue; // not in the design
        }
        match q.unit {
            Len => s.len(q.name, format!("{} mm", nn(q.value)), q.comment),
            Ang => s.angle(q.name, format!("{} deg", nn(q.value)), q.comment),
            Num | Ratio => s.num(q.name, nn(q.value), q.comment),
        }
    }
    let n = b.n;
    let deg = format!("180 deg/{n}");
    s.len("fw", "finger*ply", "read only: a finger's width");
    s.len("sw", "space*ply", "read only: a space's width");
    s.len("side0", format!("2*sin({deg})*(radius - ply/cos({deg}))"), "read only: a wall's inside length");
    s.len("wall_l", "height - 2*ply", "read only: a wall between the faces");
    s.num("nb", "floor(side0/(fw + sw))", "read only: fingers along a wall's top and bottom, as many as fit");
    s.num("nl", "floor(wall_l/(fw + sw))", "read only: fingers up a wall's end, as many as fit");
    s.len("lob", "side0 - nb*(fw + sw) + sw", "read only: what is left along a wall's bottom, half at each end");
    s.len("lol", "wall_l - nl*(fw + sw) + sw", "read only: what is left up a wall's end, half at each end");
    s.len("tp", format!("ply*tan({deg})"), "read only: how far a slotted end stands past the inside corner");
    s.len("hg", format!("tp*(1 + sin(90 deg - 2*{deg}))"), "read only: a corner finger's length, and a corner slot's depth");
    s.len("a_out", format!("radius*cos({deg})"), "read only: the outside heptagon's apothem, where the walls' outer faces are");
    let mut vols = vec![];
    let pts = |v: &[Pt]| -> Vec<[String; 2]> { v.iter().map(|q| [q.x.e(), q.y.e()]).collect() };
    let face = b.face();
    let fa = shoe(&face.iter().map(|q| b.val(q)).collect::<Vec<_>>());
    let t = b.v("ply");
    // a rectangle of expressions, x0..x1 by y0..y1
    let rect = |x0: &str, x1: &str, y0: &str, y1: &str| -> Vec<[String; 2]> {
        [(x0, y0), (x1, y0), (x1, y1), (x0, y1)].iter().map(|(x, y)| [x.to_string(), y.to_string()]).collect()
    };
    // a face: the outside heptagon, and one slot under a wall's first finger
    // on each edge, patterned along it as many times as the wall has fingers
    let heptagon: Vec<[String; 2]> = (0..n)
        .map(|k| {
            let (c, _) = b.corner_dir(k);
            [format!("{}*radius", nn(c.0)), format!("{}*radius", nn(c.1))]
        })
        .collect();
    let slots = |s: &mut Script, face: &str, offset: Option<&str>| {
        for k in 0..n {
            let (c, d) = b.corner_dir(k);
            let m = (-d.1, d.0); // inward
            // from the corner: along d by `along`, in by `inn`
            let at = |along: &str, inn: &str| -> [String; 2] {
                [format!("{}*radius + {}*({along}) + {}*({inn})", nn(c.0), nn(d.0), nn(m.0)), format!("{}*radius + {}*({along}) + {}*({inn})", nn(c.1), nn(d.1), nn(m.1))]
            };
            let (s0, s1) = ("tp + lob/2", "tp + lob/2 + fw");
            // out past the edge too, so the cut leaves no skin on it
            let v = vec![at(s0, "-ply"), at(s1, "-ply"), at(s1, "ply"), at(s0, "ply")];
            let name = format!("{face} slot {}", k + 1);
            let sk = match offset {
                Some(o) => s.sketch("XY", Some(o), &format!("{name} sketch")),
                None => s.sketch("XY", None, &format!("{name} sketch")),
            };
            s.poly(&v);
            s.finish();
            s.extrude(&sk, "ply", false, "cut", &[face], &name);
            s.pattern(&name, [d.0, d.1, 0.0], "nb", "fw + sw");
        }
    };
    // the plain face, on the bench
    let sk = s.sketch("XY", None, "bottom face sketch");
    s.poly(&heptagon);
    s.cmd("sketch.circle.center", r#"{"center":["0 mm","-hole_offset"],"diameter":"hole_d"}"#.into());
    s.finish();
    s.extrude(&sk, "ply", false, "new", &[], "bottom face");
    slots(&mut s, "bottom face", None);
    vols.push(("bottom face".into(), (fa - PI * p.get("hole_d").powi(2) / 4.0) * t));
    // the walls, each on its edge: the body between the faces, then one
    // finger top and bottom and one at each end, patterned as many times as
    // fit; a slotted end has its slots cut the same way
    for (i, &(lg, rg)) in kinds(n).iter().enumerate() {
        let w = b.wall(lg, rg);
        let a = 2.0 * PI * i as f64 / n as f64;
        let dir = [a.cos(), a.sin(), 0.0];
        let name = format!("wall {}", i + 1);
        let plane = |s: &mut Script, what: &str| s.sketch_on([0.0, 0.0, 0.0], dir, [0.0, 0.0, 1.0], Some("a_out"), &format!("{name} {what} sketch"));
        // x in the sketch is from the wall's middle
        let x = |e: &str| format!("{e} - side0/2");
        let left = if lg { "0 mm" } else { "-tp" };
        let right = if rg { "side0" } else { "side0 + tp" };
        let sk = plane(&mut s, "body");
        s.poly(&rect(&x(left), &x(right), "ply", "height - ply"));
        s.finish();
        s.extrude(&sk, "ply", true, "new", &[], &name);
        // a finger down through the bottom face and one up through the top
        let f = format!("{name} fingers");
        let sk = plane(&mut s, "fingers");
        s.poly(&rect(&x("lob/2"), &x("lob/2 + fw"), "0 mm", "ply"));
        s.poly(&rect(&x("lob/2"), &x("lob/2 + fw"), "height - ply", "height"));
        s.finish();
        s.extrude(&sk, "ply", true, "join", &[&name], &f);
        s.pattern(&f, dir, "nb", "fw + sw");
        // the two ends: fingers out past the corner, or slots cut back
        for (end, g, at) in [("left", lg, left), ("right", rg, right)] {
            let e = format!("{name} {end} end");
            let sk = plane(&mut s, &format!("{end} end"));
            let (x0, x1, op) = match (end, g) {
                ("left", true) => ("-hg".to_string(), "0 mm".to_string(), "join"),
                ("right", true) => ("side0".to_string(), "side0 + hg".to_string(), "join"),
                ("left", false) => (format!("{at} - ply"), format!("{at} + hg"), "cut"),
                _ => (format!("{at} - hg"), format!("{at} + ply"), "cut"),
            };
            s.poly(&rect(&x(&x0), &x(&x1), "ply + lol/2", "ply + lol/2 + fw"));
            s.finish();
            s.extrude(&sk, "ply", true, op, &[&name], &e);
            s.pattern(&e, [0.0, 0.0, 1.0], "nl", "fw + sw");
        }
        vols.push((name, shoe(&w.iter().map(|q| b.val(q)).collect::<Vec<_>>()) * t));
    }
    // the rosette face, on top
    let sk = s.sketch("XY", Some("height - ply"), "top face sketch");
    s.poly(&heptagon);
    let mut open = 0.0;
    for l in &k.cut {
        let v: Vec<Pt> = l.iter().map(|&q| rosette_pt(b, k, q)).collect();
        open += shoe(&v.iter().map(|q| b.val(q)).collect::<Vec<_>>()).abs();
        s.poly(&pts(&v));
    }
    s.finish();
    s.extrude(&sk, "ply", false, "new", &[], "top face");
    slots(&mut s, "top face", Some("height - ply"));
    vols.push(("top face".into(), (fa - open) * t));
    s.measure();
    (s.text(), vols)
}

// --- the run ---------------------------------------------------------------------------

fn run() -> Result<i32, String> {
    let defaults = table();
    let mut p = table();
    let rest = p.parse(std::env::args().skip(1).collect())?;
    let o = outs(rest, &p.flags())?;
    let b = Bx::of(&p)?;
    let k = knot(b.n)?;
    if let Some(dir) = &o.solidcraft {
        let (text, vols) = design(&p, &b, &k);
        write(&format!("{dir}/kalimba-septa-box.solidcraft.json"), &text)?;
        if std::env::var("SC_VOLUMES").is_ok() {
            for (name, v) in vols {
                eprintln!("{name}\t{v:.4}");
            }
        }
        return Ok(0);
    }
    let g = p.get("kerf") / 2.0;
    let face: Vec<P> = b.face().iter().map(|q| b.val(q)).collect();
    let walls: Vec<Vec<P>> = kinds(b.n).iter().map(|&(l, r)| b.wall(l, r).iter().map(|q| b.val(q)).collect()).collect();
    let stiff = stiffener(&b, p.get("stiffener_depth"), p.get("stiffener_corner"));
    let ns = p.get("stiffeners") as usize;
    let (hole_d, hole_off) = (p.get("hole_d"), p.get("hole_offset"));

    // the sheet: the faces and the stiffeners along the top, the walls below
    let fb = bbox(&face);
    let wb: Vec<_> = walls.iter().map(|w| bbox(w)).collect();
    let sb = bbox(&stiff);
    let (fw_, fh) = (fb.2 - fb.0 + 2.0 * g, fb.3 - fb.1 + 2.0 * g);
    let (sw_, sh) = (sb.2 - sb.0, sb.3 - sb.1);
    let stack = ns as f64 * sh + ns.saturating_sub(1) as f64 * GAP;
    let row1 = 2.0 * fw_ + GAP + if ns > 0 { GAP + sw_ } else { 0.0 };
    let row2: f64 = wb.iter().map(|w| w.2 - w.0 + 2.0 * g).sum::<f64>() + GAP * (b.n as f64 - 1.0);
    let wh = wb.iter().map(|w| w.3 - w.1 + 2.0 * g).fold(0.0, f64::max);
    let top_h = fh.max(stack);
    let (sw, sh_) = (row1.max(row2) + 2.0 * GAP, top_h + GAP + wh + 2.0 * GAP);
    let mut sheet = Sheet::new(sw, sh_);
    // a part's frame onto the sheet: its bbox's top left at (ox, oy)
    let place = |bx: (f64, f64, f64, f64), ox: f64, oy: f64, g: f64| move |q: P| (ox + g + q.0 - bx.0, oy + g + bx.3 - q.1);
    let mut cut_area = 0.0;
    let mut faces_at = vec![];
    for i in 0..2 {
        let at = place(fb, GAP + i as f64 * (fw_ + GAP), GAP, g);
        let (d, a) = grown(&face, g, &at);
        cut_area += a;
        sheet.path("#000000", d);
        faces_at.push(at);
    }
    // the plain face's hole
    {
        let c = faces_at[0]((0.0, -hole_off));
        sheet.circle("#00ff00", c.0, c.1, hole_d);
    }
    // the rosette on the other
    {
        let at = &faces_at[1];
        let tr = |l: &Vec<P>| -> Vec<P> { l.iter().map(|&q| at(b.val(&rosette_pt(&b, &k, q)))).collect() };
        for l in &k.cut {
            let v = tr(l);
            let s: Vec<String> = v.iter().map(|q| format!("{},{}", fm(q.0), fm(q.1))).collect();
            sheet.path("#00ff00", format!("M {} Z", s.join(" L ")));
        }
        for l in &k.engrave {
            let v = tr(l);
            let s: Vec<String> = v.iter().map(|q| format!("{},{}", fm(q.0), fm(q.1))).collect();
            sheet.path("#0000ff", format!("M {}", s.join(" L ")));
        }
    }
    // the stiffeners, stacked beside the faces
    for i in 0..ns {
        let at = place(sb, GAP + 2.0 * (fw_ + GAP), GAP + i as f64 * (sh + GAP), 0.0);
        let v: Vec<P> = stiff.iter().map(|&q| at(q)).collect();
        sheet.poly("#ff8000", &v);
    }
    // the walls
    let mut x = GAP;
    let mut wall_area = vec![];
    for (w, bx) in walls.iter().zip(&wb) {
        let at = place(*bx, x, GAP + top_h + GAP, g);
        let (d, a) = grown(w, g, &at);
        cut_area += a;
        wall_area.push((shoe(w), a));
        sheet.path("#000000", d);
        x += bx.2 - bx.0 + 2.0 * g + GAP;
    }

    // the report
    let (wx, wy) = (wb[0].2 - wb[0].0 + 2.0 * g, wb[0].3 - wb[0].1 + 2.0 * g);
    println!(
        "kalimba: {} sides, {} mm outside radius, {} mm high; faces {:.1} × {:.1} mm, walls {:.1} × {:.1} mm; rosette {} mm, hole {} mm; kerf {}; sheet {:.1} × {:.1} mm",
        b.n,
        p.get("radius"),
        p.get("height"),
        fw_,
        fh,
        wx,
        wy,
        p.get("rosette_d"),
        hole_d,
        p.get("kerf"),
        sheet.w,
        sheet.h
    );
    let mut c = Checks::new();
    c.note(b.nb >= 1 && b.nl >= 1, "fingers on every edge", format!("{} along a wall's top and bottom, {} up each end", b.nb, b.nl));
    let lo = (b.v("lob") / 2.0).min(b.v("lol") / 2.0);
    c.note(lo >= b.v("ply"), "a ply at least before the first finger", format!("{:.2} mm along, {:.2} mm up", b.v("lob") / 2.0, b.v("lol") / 2.0));
    let a_in = b.a_in();
    let reach = k.cut.iter().chain(&k.engrave).flatten().map(|&q| dist(b.val(&rosette_pt(&b, &k, q)), (0.0, 0.0))).fold(0.0, f64::max);
    // the inside heptagon's inscribed circle is the walls' nearest
    c.note(reach + CLEAR <= a_in, "the rosette inside the walls", format!("reaches {reach:.1} mm from the face's centre, the walls are {a_in:.1} (>= {CLEAR} mm between)"));
    c.note(hole_off + hole_d / 2.0 + CLEAR <= a_in, "the hole inside the walls", format!("reaches {:.1} mm, the walls are {a_in:.1}", hole_off + hole_d / 2.0));
    c.note(k.cut.len() == 2 * b.n + 1, "the rosette's openings", format!("{} from {} (2 × {} bights + the middle)", k.cut.len(), k.file, b.n));
    let (dep, cut) = (p.get("stiffener_depth"), p.get("stiffener_corner"));
    let e = 2.0 * PI / b.n as f64;
    c.note(dep < a_in && cut > 0.0 && cut < b.v("side0") / 2.0 && cut < dep / e.sin(), "the stiffener's shape", format!("{:.1} mm long at the wall, {:.1} mm long edge, {dep} mm deep", b.v("side0"), dist(stiff[3], stiff[4])));
    c.note(sheet.w <= BED.0 && sheet.h <= BED.1, "fits the bed", format!("{:.1} × {:.1} mm in {} × {} mm", sheet.w, sheet.h, BED.0, BED.1));
    let rc = c.done();
    if std::env::var("SC_AREAS").is_ok() {
        eprintln!("face\t{:.4}\t{:.4}", shoe(&face), grown(&face, g, &|q| q).1);
        for (i, (a, ga)) in wall_area.iter().enumerate() {
            eprintln!("wall {}\t{a:.4}\t{ga:.4}", i + 1);
        }
        eprintln!("stiffener\t{:.4}", shoe(&stiff));
        eprintln!("sheet cut area\t{cut_area:.4}");
    }
    if !o.no_write {
        let dir = o.out.clone().unwrap_or_else(|| HERE.to_string());
        let made = p.changed(&defaults);
        let with = if made.is_empty() { String::new() } else { format!(" Made with {made}.") };
        let desc = format!(
            "1 user unit = 1mm. A {}-sided kalimba body, drawn by kalimba.rs.{with} Cut it in {}mm Baltic birch; the box's outlines carry a {}mm kerf. Blue #0000ff engraves the rosette's interlace and is never cut; green #00ff00 cuts the rosette and the round hole, orange #ff8000 the optional stiffeners, then black #000000 frees the faces and walls.",
            b.n,
            nn(p.get("ply")),
            nn(p.get("kerf"))
        );
        write(&format!("{dir}/{STEM}.svg"), &sheet.svg("Kalimba body", &desc))?;
    }
    Ok(rc)
}

fn main() {
    std::process::exit(match run() {
        Ok(rc) => rc,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    });
}
