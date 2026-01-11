#!/bin/bash
set -euo pipefail

# Directory to store artifacts
DIST_DIR="dist"
mkdir -p "$DIST_DIR"

# Get version from lib/Cargo.toml
VERSION=$(grep '^version =' lib/Cargo.toml | head -n1 | cut -d '"' -f 2)
echo "Building Xvc version $VERSION"

# Targets to build
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-gnu"
)

for TARGET in "${TARGETS[@]}"; do
    echo "--------------------------------------------------"
    echo "Building for target: $TARGET"
    echo "--------------------------------------------------"

    # Define base features
    FEATURES="bundled-sqlite"
    
    # Use bundled OpenSSL for Linux and Windows to ensure static linking/availability
    # macOS typically uses the system OpenSSL or the one provided by the environment,
    # but for cross-compilation consistency and portability on Linux/Windows, we bundle it.
    if [[ "$TARGET" == *"linux"* ]] || [[ "$TARGET" == *"windows"* ]]; then
        FEATURES="$FEATURES,bundled-openssl"
    fi

    # Build using cross
    # Note: For macOS targets on a macOS host, cross usually falls back to cargo.
    cross build --release --target "$TARGET" -p xvc --features "$FEATURES"

    # Prepare artifact
    BINARY_NAME="xvc"
    if [[ "$TARGET" == *"windows"* ]]; then
        BINARY_NAME="xvc.exe"
    fi

    SOURCE_PATH="target/$TARGET/release/$BINARY_NAME"
    
    if [[ -f "$SOURCE_PATH" ]]; then
        # Create archive folder name
        ARCHIVE_NAME="xvc-$VERSION-$TARGET"
        
        # Staging directory for archive
        STAGING_DIR="$DIST_DIR/$ARCHIVE_NAME"
        mkdir -p "$STAGING_DIR"
        
        # Copy binary and docs
        cp "$SOURCE_PATH" "$STAGING_DIR/"
        if [[ -f "README.md" ]]; then cp README.md "$STAGING_DIR/"; fi
        if [[ -f "LICENSE" ]]; then cp LICENSE "$STAGING_DIR/"; fi

        # Compress
        pushd "$DIST_DIR" > /dev/null
        if [[ "$TARGET" == *"windows"* ]]; then
            zip -r "$ARCHIVE_NAME.zip" "$ARCHIVE_NAME"
            echo "Created $DIST_DIR/$ARCHIVE_NAME.zip"
        else
            tar -czf "$ARCHIVE_NAME.tar.gz" "$ARCHIVE_NAME"
            echo "Created $DIST_DIR/$ARCHIVE_NAME.tar.gz"
        fi
        popd > /dev/null
        
        # Clean up staging
        rm -rf "$STAGING_DIR"
    else
        echo "Error: Binary not found at $SOURCE_PATH"
        exit 1
    fi
done

echo "--------------------------------------------------"
echo "Build complete. Artifacts are in $DIST_DIR/"
echo "--------------------------------------------------"
