# INTA brand kit

| Artwork | Where to use it | Preview |
| --- | --- | --- |
| **Application** | Default mark: navigation, tool/source links, Desktop and Mobile headers, Windows ICO/MSIX, web and mobile launchers, Rescue launcher and workspace. Use the supplied transparent artwork with no decorative backing. | <img src="application-64.png" width="36" height="36" alt="INTA Application"> |
| **Heritage** | Front-page heroes, such as Hub Home, the repository cover, and Wiki Home; also appropriate for a dedicated splash screen. Never use it in shared navigation, in-line links, or ordinary app chrome. | <img src="heritage-192.png" width="48" height="48" alt="INTA Heritage"> |

## Source pack

[Download the original complete icon pack](source/INTA-Complete-Icon-Pack.zip) · [SHA-256](source/SHA256SUMS)

The archive preserves the supplied bytes and original folder layout. Each family has a native PNG, a multi-size ICO, 19 PNG sizes from 16 to 1024 px, and its own notes. The archive also includes the original icon-size proof. The four working PNGs here and the [Windows Application ICO](../../native/workbench/desktop/inta.ico) are the build inputs. The stable inline icon URL remains a compatibility alias.

## Replace the kit

1. Make a replacement ZIP with the same Application and Heritage layout. Provide transparent Application PNGs at 64, 192 and 512 px, a transparent multi-size Application ICO, and Heritage at 192 px. Keep the supplied originals and proof together.
2. Install Pillow if needed, then run `python3 scripts/refresh-inta-brand.py --pack path/to/new-pack.zip` from the repository root. The command also saves the original ZIP and its SHA-256 in `source/`. Use `--sync` to regenerate from the current committed working inputs without replacing the pack. The importer reads only five known files, rejects opaque canvases, and regenerates the site, offline cache, inline mark, covers and MSIX resources.
3. Run `python3 scripts/refresh-inta-brand.py --check` and `python3 scripts/build-hub.py --check`. Review the Hub, Windows and Android builds. Rescue requires a separately verified, security-cleared build; existing release packages are not changed.

Do not edit the generated graphics or published site HTML by hand.
