#!/usr/bin/env python3
"""The kalimba body in 3D, assembled from KalimbaSeptaBox.svg.

    python3 viewer.py [--out=kalimba-3d.html]

The parts are the cut file's own outlines, read by lasermade-tools/turn3d.py.
What the cut file does not say is where each one goes, and that is this file:
the plain face on the bench, the seven walls standing on its edges, the
rosette face on top.

The box is Boxes.py's RegularBox at radius 90, seven sides, h=65 outside, so
each wall stands on one edge of the face's heptagon with its outer skin flush
to the face's edge, and is 65mm tall overall with the faces inside that.
The walls' fingers protrude the same at both ends, so centring a wall on its
edge puts its fingers in the face's slots.

The three orange stiffeners are left out. They are optional, and where they go
is drawn nowhere -- the README says only that they glue under the top face, and
their shape fits no obvious place there. Placing them would be a guess shown as
a build, so the page says they are missing instead.
"""
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..', 'lasermade-tools'))
import turn3d as t   # noqa: E402

SVG = os.path.join(HERE, 'KalimbaSeptaBox.svg')
N, H, TH = 7, 65.0, 3.0


def hull(pts):
    pts = sorted(set(map(tuple, pts)))
    cross = lambda o, a, b: (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lo, hi = [], []
    for p in pts:
        while len(lo) >= 2 and cross(lo[-2], lo[-1], p) <= 0:
            lo.pop()
        lo.append(p)
    for p in reversed(pts):
        while len(hi) >= 2 and cross(hi[-2], hi[-1], p) <= 0:
            hi.pop()
        hi.append(p)
    return lo[:-1] + hi[:-1]            # counter-clockwise


def heptagon(face):
    """Centre, and the seven edges as (midpoint, direction angle), of a face:
    the seven longest runs of its convex hull, which the finger notches only
    shorten, never turn."""
    h = hull(face['o'])
    edges = []
    for i in range(len(h)):
        a, b = h[i], h[(i + 1) % len(h)]
        edges.append((math.dist(a, b), a, b))
    edges.sort(key=lambda e: -e[0])
    sides = []
    for _, a, b in edges:
        ang = math.atan2(b[1] - a[1], b[0] - a[0])
        if all(abs(math.remainder(ang - s[1], 2 * math.pi)) > 0.2 for s in sides):
            sides.append((((a[0] + b[0]) / 2, (a[1] + b[1]) / 2), ang, a, b))
        if len(sides) == N:
            break
    sides.sort(key=lambda s: s[1])
    # the true corners: where neighbouring edge lines cross
    corners = []
    for i in range(N):
        (_, a1, p1, q1), (_, a2, p2, q2) = sides[i - 1][0:4], sides[i][0:4]
        d1 = (q1[0] - p1[0], q1[1] - p1[1]); d2 = (q2[0] - p2[0], q2[1] - p2[1])
        den = d1[0] * d2[1] - d1[1] * d2[0]
        s = ((p2[0] - p1[0]) * d2[1] - (p2[1] - p1[1]) * d2[0]) / den
        corners.append((p1[0] + s * d1[0], p1[1] + s * d1[1]))
    cx = sum(c[0] for c in corners) / N
    cy = sum(c[1] for c in corners) / N
    out = []
    for i in range(N):
        a, b = corners[i], corners[(i + 1) % N]
        out.append((((a[0] + b[0]) / 2 - cx, (a[1] + b[1]) / 2 - cy), math.atan2(b[1] - a[1], b[0] - a[0])))
    return (cx, cy), out


def build():
    parts = t.parts(SVG)
    faces = [p for p in parts if p['ink'] == '#000000' and min(p['box'][2] - p['box'][0], p['box'][3] - p['box'][1]) > 150]
    walls = [p for p in parts if p['ink'] == '#000000' and p not in faces]
    stiff = [p for p in parts if p['ink'] == '#ff8000']
    assert len(faces) == 2 and len(walls) == N and len(stiff) == 3, (len(faces), len(walls), len(stiff))
    top = max(faces, key=lambda f: len(f['h']))       # the rosette's fifteen openings
    bottom = min(faces, key=lambda f: len(f['h']))    # the one round hole
    out = []
    (cx, cy), edges = heptagon(bottom)
    out.append(t.placed(t.centred(bottom, cx, cy), t.T(0, 0, 0), (0, 0, -1)))
    for (m, ang), w in zip(edges, walls):
        w = t.centred(w)
        x0, y0, x1, y1 = w['box']
        turn = t.Rz(math.pi / 2) if (y1 - y0) > (x1 - x0) else t.T(0, 0, 0)   # stood upright on the sheet
        hh = min(x1 - x0, y1 - y0)
        n = (math.sin(ang), -math.cos(ang))                                     # outward
        M = t.chain(t.T(m[0] - TH * n[0], m[1] - TH * n[1], hh / 2), t.Rz(ang), t.Rx(math.pi / 2), turn)
        out.append(t.placed(w, M, (n[0] * 0.8, n[1] * 0.8, 0)))
    (tx, ty), _ = heptagon(top)
    # the top face goes on rosette side up; its heptagon is turned to match the bottom's
    (_, e_top) = heptagon(top)
    rot = math.remainder(edges[0][1] - e_top[0][1], 2 * math.pi / N)
    out.append(t.placed(t.centred(top, tx, ty), t.chain(t.T(0, 0, H - TH), t.Rz(rot)), (0, 0, 1.6)))
    return {'septa box': {'parts': out, 'ex': 28}}


def main(argv):
    o = [a.split('=', 1)[1] for a in argv if a.startswith('--out=')]
    path = o[0] if o else os.path.join(HERE, 'kalimba-3d.html')
    t.write(path, 'Kalimba body', build(),
            'Drag to turn, scroll or pinch to zoom; draw Explode to take it apart. Every part is the '
            'cut file’s own outline at 3mm: the plain face with its round hole underneath, seven '
            'finger-jointed walls, and the septafoil rosette face on top. The sheet’s three '
            'optional stiffeners are left out, because nothing says where under the rosette face they go. '
            'The tines and bridge are not part of this build.', 'kalimba/viewer.py')
    print(f'  wrote {os.path.relpath(path)}')
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
