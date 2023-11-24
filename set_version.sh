#!/bin/bash

# Specify the new version
NEW_VERSION=$(cat VERSION)

# List of directories containing Cargo.toml files
CRATES=("types" "actix-api" "yew-ui")

# Loop through each crate and update the version
for CRATE in "${CRATES[@]}"; do
    echo "Updating version for $CRATE to $NEW_VERSION"
    sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$CRATE/Cargo.toml"
done

echo "Version update complete."
