# RA

This repository contains PACs, code to generate them, as well as Safe rust bindings to FSP (Flexible Software Package).

For further information, read READMEs for `ra-fsp-sys`, `ra-fsp-rs`, PACs and `update-pacs`.

### Publishing

Order should be:

1. `ra-fsp-sys`
2. PACs
3. `ra-fsp-rs`

When generating PACs and bumping versions, `ra-fsp-rs` should be temporary removed from the workspace, because it depends on the PACs which are not yet publised.

When publishing, remove all workspace deps and member of the workspace until they needed.

```bash

export FSP_PATH="$HOME/job/fw-micrortu/dep/fsp"; export CMSIS_PATH="$HOME/job/fw-micrortu/dep/cmsis5"; export FSP_CFG="$HOME/code/ra6m3-rtic/fsp_cfg";

cargo check -p ra-fsp-sys -Fra6m3 --target thumbv7em-none-eabihf && cargo publish -p ra-fsp-sys -Fra6m3 --no-verify --allow-dirty --target thumbv7em-none-eabihf
cargo check -p ra4m1-fsp-pac --target thumbv7em-none-eabihf && cargo publish -p ra4m1-fsp-pac --no-verify --allow-dirty --target thumbv7em-none-eabihf
cargo check -p ra6m3-fsp-pac --target thumbv7em-none-eabihf && cargo publish -p ra6m3-fsp-pac --no-verify --allow-dirty --target thumbv7em-none-eabihf
cargo check -p ra-fsp-rs -Fra6m3 --target thumbv7em-none-eabihf && cargo publish -p ra-fsp-rs -Fra6m3 --no-verify --allow-dirty --target thumbv7em-none-eabihf

```
