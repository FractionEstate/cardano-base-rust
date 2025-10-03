#!/bin/bash

# Add workspace lints to all package Cargo.toml files

for pkg in base-deriving-via cardano-base cardano-binary cardano-crypto-class cardano-git-rev cardano-slotting cardano-strict-containers deepseq heapwords measures nothunks orphans-deriving-via; do
  toml_file="$pkg/Cargo.toml"
  if [ -f "$toml_file" ]; then
    if ! grep -q "^\[lints\]" "$toml_file"; then
      echo "Adding lints to $pkg"
      # Find the line number after [package] section ends (first empty line or next section)
      awk '
        /^\[package\]/ { in_package=1; print; next }
        in_package && /^$/ { print "\n[lints]\nworkspace = true"; in_package=0; next }
        in_package && /^\[/ { print "\n[lints]\nworkspace = true\n"; in_package=0 }
        { print }
      ' "$toml_file" > "$toml_file.tmp" && mv "$toml_file.tmp" "$toml_file"
    else
      echo "Lints already present in $pkg"
    fi
  fi
done

echo "Done!"
