//! The kalimba body in 3D, assembled from KalimbaSeptaBox.svg.
//!
//!     target/release/viewer [--out=kalimba-3d.html]
//!
//! The parts are the cut file's own outlines, read by the turn3d module in lasermade-tools.
//! What the cut file does not say is where each one goes, and that is this file:
//! the plain face on the bench, the seven walls standing on its edges, the
//! rosette face on top.
//!
//! The box is Boxes.py's RegularBox at radius 90, seven sides, h=65 outside, so
//! each wall stands on one edge of the face's heptagon with its outer skin flush
//! to the face's edge, and is 65mm tall overall with the faces inside that.
//! The walls' fingers protrude the same at both ends, so centring a wall on its
//! edge puts its fingers in the face's slots.
//!
//! The three orange stiffeners are left out. They are optional, and where they go
//! is drawn nowhere -- the README says only that they glue under the top face, and
//! their shape fits no obvious place there. Placing them would be a guess shown as
//! a build, so the page says they are missing instead.
use lasermade_tools::py::{dist, matan2, max, mcos, min, msin, remainder};
use lasermade_tools::turn3d::*;
use std::f64::consts::PI;

const HERE: &str = env!("CARGO_MANIFEST_DIR");
const SVG: &str = "KalimbaSeptaBox.svg";
const N: usize = 7;
const H: f64 = 65.0;
const TH: f64 = 3.0;

type P = (f64, f64);

fn cross(o: P, a: P, b: P) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

/// counter-clockwise
fn hull(pts: &[P]) -> Vec<P> {
    let mut pts = pts.to_vec();
    pts.sort_by(|a, b| a.partial_cmp(b).unwrap());
    pts.dedup();
    let (mut lo, mut hi): (Vec<P>, Vec<P>) = (vec![], vec![]);
    for &p in &pts {
        while lo.len() >= 2 && cross(lo[lo.len() - 2], lo[lo.len() - 1], p) <= 0.0 {
            lo.pop();
        }
        lo.push(p);
    }
    for &p in pts.iter().rev() {
        while hi.len() >= 2 && cross(hi[hi.len() - 2], hi[hi.len() - 1], p) <= 0.0 {
            hi.pop();
        }
        hi.push(p);
    }
    lo.pop();
    hi.pop();
    lo.extend(hi);
    lo
}

/// Centre, and the seven edges as (midpoint, direction angle), of a face:
/// the seven longest runs of its convex hull, which the finger notches only
/// shorten, never turn.
fn heptagon(face: &Part) -> (P, Vec<(P, f64)>) {
    let h = hull(&face.of());
    let mut edges: Vec<(f64, P, P)> = (0..h.len()).map(|i| (dist(h[i], h[(i + 1) % h.len()]), h[i], h[(i + 1) % h.len()])).collect();
    edges.sort_by(|a, b| (-a.0).partial_cmp(&-b.0).unwrap());
    let mut sides: Vec<(P, f64, P, P)> = vec![];
    for &(_, a, b) in &edges {
        let ang = matan2(b.1 - a.1, b.0 - a.0);
        if sides.iter().all(|s| remainder(ang - s.1, 2.0 * PI).abs() > 0.2) {
            sides.push((((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0), ang, a, b));
        }
        if sides.len() == N {
            break;
        }
    }
    sides.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    // the true corners: where neighbouring edge lines cross
    let mut corners: Vec<P> = vec![];
    for i in 0..N {
        let (_, _, p1, q1) = sides[(i + N - 1) % N];
        let (_, _, p2, q2) = sides[i];
        let d1 = (q1.0 - p1.0, q1.1 - p1.1);
        let d2 = (q2.0 - p2.0, q2.1 - p2.1);
        let den = d1.0 * d2.1 - d1.1 * d2.0;
        let s = ((p2.0 - p1.0) * d2.1 - (p2.1 - p1.1) * d2.0) / den;
        corners.push((p1.0 + s * d1.0, p1.1 + s * d1.1));
    }
    let cx = corners.iter().fold(0.0, |s, c| s + c.0) / N as f64;
    let cy = corners.iter().fold(0.0, |s, c| s + c.1) / N as f64;
    let out = (0..N)
        .map(|i| {
            let (a, b) = (corners[i], corners[(i + 1) % N]);
            (((a.0 + b.0) / 2.0 - cx, (a.1 + b.1) / 2.0 - cy), matan2(b.1 - a.1, b.0 - a.0))
        })
        .collect();
    ((cx, cy), out)
}

fn build() -> Vec<(String, J)> {
    let parts = parts(&format!("{HERE}/{SVG}"));
    let wide = |p: &Part| min(p.bx[2].f() - p.bx[0].f(), p.bx[3].f() - p.bx[1].f()) > 150.0;
    let faces: Vec<&Part> = parts.iter().filter(|p| p.ink == "#000000" && wide(p)).collect();
    let walls: Vec<&Part> = parts.iter().filter(|p| p.ink == "#000000" && !wide(p)).collect();
    let stiff = parts.iter().filter(|p| p.ink == "#ff8000").count();
    assert!(faces.len() == 2 && walls.len() == N && stiff == 3, "{:?}", (faces.len(), walls.len(), stiff));
    // the rosette's fifteen openings; the one round hole
    let top = if faces[1].h.len() > faces[0].h.len() { faces[1] } else { faces[0] };
    let bottom = if faces[1].h.len() < faces[0].h.len() { faces[1] } else { faces[0] };
    let mut out = vec![];
    let ((cx, cy), edges) = heptagon(bottom);
    out.push(placed(&centred(bottom, Some((F(cx), F(cy)))), &T(0, 0, 0), [I(0), I(0), I(-1)], "ply"));
    for (&(m, ang), w) in edges.iter().zip(&walls) {
        let w = centred(w, None);
        let [x0, y0, x1, y1] = w.bx.map(Num::f);
        // stood upright on the sheet
        let turn = if (y1 - y0) > (x1 - x0) { Rz(PI / 2.0) } else { T(0, 0, 0) };
        let hh = min(x1 - x0, y1 - y0);
        let n = (msin(ang), -mcos(ang)); // outward
        let mm = chain(&[T(m.0 - TH * n.0, m.1 - TH * n.1, hh / 2.0), Rz(ang), Rx(PI / 2.0), turn]);
        out.push(placed(&w, &mm, [F(n.0 * 0.8), F(n.1 * 0.8), I(0)], "ply"));
    }
    // the top face goes on rosette side up; its heptagon is turned to match the bottom's
    let ((tx, ty), e_top) = heptagon(top);
    let rot = remainder(edges[0].1 - e_top[0].1, 2.0 * PI / N as f64);
    out.push(placed(&centred(top, Some((F(tx), F(ty)))), &chain(&[T(0, 0, H - TH), Rz(rot)]), [I(0), I(0), F(1.6)], "ply"));
    let _ = max;
    vec![("septa box".into(), design(vec![("parts", J::A(out)), ("ex", J::N(I(28)))]))]
}

fn main() {
    let path = out_arg(&format!("{HERE}/kalimba-3d.html"));
    write(&path, "Kalimba body", &mut build(), "Drag to turn; right-drag, shift-drag or move two fingers to slide the view; scroll or pinch to zoom in on whatever is under the pointer; double-click a part to turn round it, and Reset view to go back. Draw Explode to take it apart. Every part is the cut file’s own outline at 3 mm: the plain face with its round hole underneath, seven finger-jointed walls, and the septafoil rosette face on top. The sheet’s three optional stiffeners are left out, because nothing says where under the rosette face they go. The tines and bridge are not part of this build.", "kalimba/viewer.rs", false);
    println!("  wrote {}", relpath(&path));
}
