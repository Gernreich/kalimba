# Kalimba — a seven-sided body with a septafoil rosette

A kalimba is a lamellophone: tuned metal tines fixed over a bridge on a resonating
body, plucked with the thumbs. This repository holds the **body** — a seven-sided box
whose front carries a seven-fold knot rosette.

Seven and seven is the whole idea. The box has seven sides, and the rosette is a
septafoil: one continuous ribbon crossing itself seven times, so the sound hole echoes
the plan of the thing it is cut into.

**The tines, bridge and tuning are not here.** Nothing in this file is an instrument on
its own.

## Get the files

- **[Download the cut file](KalimbaSeptaBox.svg)** — the whole body on one sheet.
- **[Everything as a ZIP](https://github.com/Gernreich/kalimba/archive/refs/heads/main.zip)**
  — the cut file and this writeup.
- **[Repository](https://github.com/Gernreich/kalimba)** — if you want to change the box
  or the rosette.

Released under CC0 1.0 — do what you like with them, no attribution needed. Built for
**[LaserMadeMusic](https://www.youtube.com/@LaserMadeMusic)**.

## The sheet

Click the picture to download the cut file. It is a display rendering — the cut file
draws a hairline on no background at all, which a browser shows almost invisibly, so
this is thickened and painted onto a light ground. Colours, geometry and sheet position
are untouched.

<div class="tw">
<table>
<tr>
<td align="center"><a href="KalimbaSeptaBox.svg"><img src="previews/KalimbaSeptaBox.svg" alt="The cut sheet: seven finger-jointed side panels, and two heptagonal faces — one carrying a septafoil knot rosette in red with blue engrave lines, the other a single small round hole" width="620"></a></td>
</tr>
<tr>
<td align="center"><sub>KalimbaSeptaBox.svg · 495 × 279mm sheet</sub></td>
</tr>
</table>
</div>

## What is on it

Measured out of the file, not copied from whatever drew it:

| Part | Count | Size | Stroke |
|---|---|---|---|
| Side panels, finger-jointed | 7 | 79.3 × 65.3mm each | black |
| Face, with the rosette | 1 | 175.8 × 171.7mm | black |
| Face, with a round hole | 1 | 175.8 × 171.4mm | black |
| Rosette | — | 48.6mm across | **red `#ff0000`** |
| Round hole | — | 15mm | **red `#ff0000`** |
| Rosette interlace and rim lines | — | 50.1mm band | **blue `#0000ff`** |

Seven sides, two faces. Everything black or red is cut; the blue is not.

The sheet is 495 × 279mm and millimetre-true — `1 user unit = 1 mm` with a physical
`width`/`height` — so it prints and cuts at real size.

### One thing to set before you send it

**The blue is an engrave, not a cut.** Those lines are the rosette's over/under interlace
hints and the short continuations that carry the ribbon's edges across the rim. They run
across the ribbon, so cutting them severs it and the rosette comes apart as it leaves the
machine. Give them a score or engrave operation, or delete the layer.

Give every colour you keep an explicit operation. A per-colour job silently skips any
colour you leave unmapped — leave the black unmapped and you will engrave a picture of a
kalimba and cut no parts.

## The rosette

One self-crossing ribbon with seven crossings, 48.6mm across. It is a **cut-out**: the
removed material is the open area and the ribbon is what stays, and the ribbon's peaks
deliberately overrun the rim so the rosette fuses into the face rather than dropping out
when the last cut closes. There is no continuous rim circle in the cut layer, and that is
correct — adding one drops the rosette on the floor.

**[The knotwork sound holes repository](https://gernreich.github.io/knotwork-soundholes/)**
documents this family in full and generates any coprime leads × bights, if you want a
different fold count for a differently sided box.

## Before you cut

**Material and thickness are yours, and the finger joints are not.** The joints were
generated for one specific thickness, and finger joints do not tolerate being cut in stock
they were not sized for — too thin and they rattle, too thick and they will not go
together. The sheet does not record which thickness it assumed, so cut a test joint before
committing the whole sheet.

**Nothing here has been validated against cut stock**, and no acoustic claim is made. A
kalimba body is a resonator; how it sounds depends on the material, the thickness, how
well the box closes, and the tines — none of which these files decide.

**The rosette removes a good half of its disc** from a face that also has to hold the box
square. Consider that before choosing a thin material.

## Files

| | |
|---|---|
| `KalimbaSeptaBox.svg` | the whole body — seven sides, two faces, rosette |
| `previews/` | display rendering — **not** a cut file |
| `index.md` · `index.html` | this page; the markdown is the source |
