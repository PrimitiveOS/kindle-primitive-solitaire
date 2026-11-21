<div align="center">

# Solitaire

**Play Klondike on Kindle**

[![CI](https://img.shields.io/github/actions/workflow/status/PrimitiveOS/kindle-primitive-solitaire/ci.yml?branch=main&label=CI)](https://github.com/PrimitiveOS/kindle-primitive-solitaire/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/PrimitiveOS/kindle-primitive-solitaire?label=release)](https://github.com/PrimitiveOS/kindle-primitive-solitaire/releases)
[![Tag](https://img.shields.io/github/v/tag/PrimitiveOS/kindle-primitive-solitaire?label=tag)](https://github.com/PrimitiveOS/kindle-primitive-solitaire/tags)
[![Licence: PolyForm-Noncommercial-1.0.0](https://img.shields.io/badge/licence-PolyForm--Noncommercial--1.0.0-blue)](LICENSE)
[![Stars](https://img.shields.io/github/stars/PrimitiveOS/kindle-primitive-solitaire?style=social)](https://github.com/PrimitiveOS/kindle-primitive-solitaire/stargazers)
[![Store downloads](https://img.shields.io/endpoint?url=https%3A%2F%2Fprimitiveos.github.io%2Fkindle-primitive-catalog%2Fv1%2Fbadges%2Fprimitive-solitaire.json)](https://primitiveos.github.io/kindle-primitive-catalog/v1/index.json)

[**Download**](https://github.com/PrimitiveOS/kindle-primitive-solitaire/releases/latest) · [**Install guide**](INSTALL.md) · [**Changelog**](CHANGELOG.md)

</div>

---

Play draw-one or draw-three Klondike with tap controls and undo.

## Quick start

After setting up PrimitiveOS, open **Store** from Primitive Launcher. Choose **Solitaire** under **Apps**,
tap **Get**, and open it from the Launcher grid. The [install guide](INSTALL.md) also covers the standalone USB package.

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

## Develop locally

Keep the Primitive Tools and Launcher repositories beside this checkout. The desktop command plays a sample
draw and writes PNG previews to `build/previews`.

```sh
cargo test --offline --locked --workspace
cargo run --offline --locked -p solitaire-desktop
```

## Status

| Device | Status |
|---|---|
| Kindle Oasis 3 (1264 × 1680, firmware 5.18.2) | Tested on hardware |

**Current version: 0.1.1.** Changes are in [CHANGELOG.md](CHANGELOG.md).

## Downloads

The badge at the top shows the number of Store downloads. It counts GitHub downloads of this app's release packages, across all versions and architectures, and is refreshed daily from the [Store catalog](https://primitiveos.github.io/kindle-primitive-catalog/v1/index.json).

## Documentation

[Install guide](INSTALL.md) · [Changelog](CHANGELOG.md) · [Device notes](docs/device-notes.md)

## Licence

Copyright (C) 2026 Doan Nguyen.

- Original code is under the [PolyForm Noncommercial License 1.0.0](LICENSE).
- Original documentation and pictures, including these screenshots, are under [CC BY-NC-SA 4.0](LICENSE-DOCS).
- The Noto Sans fonts supplied by Primitive Launcher retain their SIL Open Font License.
