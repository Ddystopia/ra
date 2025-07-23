#!/bin/bash
set -euo pipefail

cargo_toml="Cargo.toml"

# 1. Extract current version
current_version=$(grep -E '^package\.version\s*=' "$cargo_toml" | sed -E 's/package\.version\s*=\s*"([^"]+)"/\1/')
echo "Current version: $current_version"

# 2. Prompt for new version
read -p "Enter new version: " new_version
new_version=$(echo "$new_version" | xargs) # trim
if [[ -z "$new_version" ]]; then
  echo "No version entered. Aborting."
  exit 1
fi

# 3. Update version in workspace and dependencies
echo "Updating version to $new_version..."

# Update workspace.package.version
sed -i -E 's/^package\.version\s*=.*$/package.version = "'"$new_version"'"/' "$cargo_toml"

# 4. Check ra-fsp-sys for ra6m3
echo "Checking ra-fsp-sys for ra6m3..."
cargo check -p ra-fsp-sys -Fra6m3 --target thumbv7em-none-eabihf

# 5. Check one PAC
for pac_path in "${PWD}/pac/ra6m3"; do
  pac_name=$(basename "$pac_path")-fsp-pac
  echo "Checking $pac_name..."
  cargo check -p "$pac_name" --target thumbv7em-none-eabihf
done

# 6. Publish ra-fsp-sys
echo "Publishing ra-fsp-sys..."
cargo publish -p ra-fsp-sys -Fra6m3 --target thumbv7em-none-eabihf --allow-dirty

# 7. Publish PACs
for pac_path in "${PWD}/pac/"*; do
  pac=$(basename "$pac_path")
  pac_name="${pac}-fsp-pac"
  echo "Publishing $pac_name..."
  cargo publish -p "$pac_name" --target thumbv7em-none-eabihf --allow-dirty
done

# Update all versions in [workspace.dependencies]
sed -i -E 's/ version = "[^"]*"/ version = "'"$new_version"'"/' "$cargo_toml"

# 8. Finally, publish ra-fsp-rs
echo "Publishing ra-fsp-rs..."
cargo publish -p ra-fsp-rs -Fra6m3 --target thumbv7em-none-eabihf --allow-dirty

echo "All done!"
