<div align="center">

# Solitaire

**Klondike for jailbroken Kindle e-readers.**

[**Install guide**](INSTALL.md) ·
[**Device notes**](docs/device-notes.md) ·
[**Changelog**](CHANGELOG.md)

</div>

---

Tap an open card or run, then tap the receiving column or suit. Large ranks and suit symbols make the cards easy
to distinguish on grayscale e-ink.

## Screenshots

<table>
  <tr>
    <td align="center"><img src="docs/images/solitaire.png" alt="Draw-one Klondike deal with seven columns and an ace in the waste pile" width="260"><br><sub>Draw-one Klondike</sub></td>
    <td align="center"><img src="docs/images/solitaire-dark.png" alt="Klondike deal with light card ranks and suit symbols on black" width="260"><br><sub>The deal in dark mode</sub></td>
  </tr>
</table>

<sub>Screenshots from the desktop simulator at 1264 × 1680, using a sample deal.</sub>

## Highlights

- **Draw one or three.** Choose how many cards to turn over from the stock.
- **Undo and resume.** Reopen your saved deal with its recent undo history.
- **Auto-finish.** Send the remaining cards to their suit piles once they can all be moved there.

## Quick start

For a local USB test, extract the ARMHF test package and copy the `solitaire` and `documents` folders to the
Kindle's user storage. Eject the Kindle and open Solitaire from the Primitive Launcher grid.
The app uses fonts installed with Primitive Launcher. [Install guide](INSTALL.md)

## Status

Desktop tests and previews passed on macOS, and the ARMHF package passed the local Store fixture.
On an ARMHF Kindle Oasis 3 running firmware 5.18.2, Solitaire showed the new-deal confirmation, preserved the
original deal after Cancel, and returned to Launcher. [Device notes](docs/device-notes.md)

## Develop locally

Keep the Primitive Tools and Launcher repositories beside this checkout. The desktop command plays a sample
draw and writes PNG previews to `build/previews`.

```sh
cargo test --offline --locked --workspace
cargo run --offline --locked -p solitaire-desktop
```

## Licence

Copyright (C) 2026 Doan Nguyen.

- Original code is under the [PolyForm Noncommercial License 1.0.0](LICENSE).
- Original documentation and pictures, including these screenshots, are under [CC BY-NC-SA 4.0](LICENSE-DOCS).
- The Noto Sans fonts supplied by Primitive Launcher retain their SIL Open Font License.
