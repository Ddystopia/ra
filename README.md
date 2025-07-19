# RA

This repository contains PACs, code to generate them, as well as Safe rust bindings to FSP (Flexible Software Package).

For further information, read READMEs for `ra-fsp-sys`, `ra-fsp-rs`, PACs and `update-pacs`.

### Publishing

Order should be:

1. `ra-fsp-sys`
2. PACs
3. `ra-fsp-rs`

When generating PACs and bumping versions, `ra-fsp-rs` should be temporary removed from the workspace, because it depends on the PACs which are not yet publised.
