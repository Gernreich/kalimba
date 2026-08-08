# Kalimba

A seven-sided kalimba body whose front carries a seven-fold knot rosette. Output is
millimetre-true — `1 user unit = 1 mm` with a physical `width`/`height` — so it prints
and cuts at real size.

<table>
<tr>
<td align="center"><a href="KalimbaSeptaBox.svg"><img src="previews/KalimbaSeptaBox.svg" alt="The cut sheet: seven finger-jointed side panels, and two heptagonal faces — one carrying a septafoil knot rosette in red with blue engrave lines, the other a single small round hole" width="620"></a></td>
</tr>
<tr>
<td align="center"><sub>KalimbaSeptaBox.svg · 495 × 279mm sheet</sub></td>
</tr>
</table>

*Click it to download the cut file. This is a display rendering — the cut file draws a
hairline on no background, which a browser shows almost invisibly.*

**[Read the writeup](https://gernreich.github.io/kalimba/)**

Built for **[LaserMadeMusic](https://www.youtube.com/@LaserMadeMusic)**, where the cutting
and assembly are shown.

**[Download everything as a ZIP](https://github.com/Gernreich/kalimba/archive/refs/heads/main.zip)**

## What is on the sheet

Measured out of the file, not copied from whatever drew it:

Run the colours **blue, red, purple, black** — engrave first, then cut from the detail
outward to the outlines:

| | Colour | Part | Count | Size |
|---|---|---|---|---|
| 1 | blue `#0000ff` | rosette interlace and rim lines | 1 | 50.1mm band |
| 2 | red `#ff0000` | the rosette | 1 | 48.6mm across |
| 2 | red `#ff0000` | round hole in the plain face | 1 | 15mm |
| 3 | purple `#ff00ff` | reinforcing trapezoids | 3 | 133.3 × 37.3mm each |
| 4 | black `#000000` | two faces and seven sides | 9 | faces 175.8 × 171mm, sides 79.3 × 65.3mm |

Seven sides, two faces, and a rosette that is a septafoil — one ribbon crossing itself
seven times, so the hole echoes the plan of the box.

**The blue is an engrave, not a cut.** Those lines run across the ribbon; cutting them
severs it and the rosette comes apart as it leaves the machine. Give them a score or
engrave operation, or delete the layer. Outlines go last so nothing moves once a cut
frees it.

**The three purple trapezoids are optional.** They glue under the top sheet of the
heptagon to stiffen it — worth having, since that face has the rosette cut out of it —
but the build stands up without them. Skip the purple stage if you do not want them.

## The rosette

A cut-out: the removed material is the open area and the ribbon is what stays. Its peaks
overrun the rim so the rosette fuses into the face instead of dropping out when the last
cut closes — there is deliberately no continuous rim circle, and adding one drops it on
the floor. **[knotwork-soundholes](https://gernreich.github.io/knotwork-soundholes/)**
documents the family and generates any coprime leads × bights, if you want a different
fold count for a differently sided box.

## Before you cut

**Cut it in 3mm Baltic birch plywood** — that is what this is built in, and what the
finger joints are sized for. Too thin and they rattle; too thick and they will not go
together, so substituting a different stock means regenerating the joints for it.

**No acoustic claim is made.** The tines, bridge and tuning are not in this file — it is
the body only.

## Files

| | |
|---|---|
| `KalimbaSeptaBox.svg` | the whole body — seven sides, two faces, rosette |
| `previews/` | display rendering — **not** a cut file |
| `index.md` · `index.html` | the published page; the markdown is the source |

Released under [CC0 1.0](LICENSE).
