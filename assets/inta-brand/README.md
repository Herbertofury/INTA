# INTA branding

Use **Heritage** where the logo has room to be seen: the site wordmark, Windows and Android app headers, the Rescue desktop backdrop, and the repository and wiki banners. Use **Application** at icon size: Windows executables and shortcuts, Android launcher, web tab and home-screen icons, Rescue launcher, and inline catalog marks. Existing stable URLs are preserved.

## Change the logo

1. In an approved INTA icon pack, provide the Application ICO and transparent PNGs (64, 192 and 512 px) and the Heritage 192 px PNG. Preserve the current pack's file layout. The importer reads only the five named image entries, never extracts or runs anything.
2. Run `python3 -m pip install Pillow` if your development environment does not already have it, then `python3 scripts/refresh-inta-brand.py --pack path/to/INTA-Complete-Icon-Pack.zip`. To rebuild after changing the canonical images directly, use `--sync` instead.
3. The swap regenerates the checked-in Hub and offline icons in the same command. Verify with `python3 scripts/refresh-inta-brand.py --check` and `python3 scripts/build-hub.py --check`.
4. Review generated changes and run the Hub, Windows and Android workflows. Publish Rescue only from a new security-verified build with BIOS and UEFI evidence. Files already released are immutable; the new branding appears in separately built releases.

The PNGs beside this README and `native/workbench/desktop/inta.ico` are the source of truth. Site assets are generated; Android packages the PNGs directly; Windows reads the ICO and embeds Heritage; the MSIX package uses derived fixed-size logos whose hashes are sealed. The old hash-shaped inline icon URL is retained as a compatibility alias. Do not edit a generated surface by hand.
