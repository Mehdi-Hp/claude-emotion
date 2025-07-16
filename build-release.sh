#!/bin/bash
# Build release binaries for multiple platforms

set -e

VERSION="0.1.0"
NAME="claude-emotion"
TARGETS=(
    "aarch64-apple-darwin"    # macOS ARM64
    "x86_64-apple-darwin"     # macOS Intel
    "x86_64-unknown-linux-gnu" # Linux x64
)

echo "🔨 Building release binaries for $NAME v$VERSION"
echo "================================================="

# Create release directory
mkdir -p release

# Build for each target
for target in "${TARGETS[@]}"; do
    echo
    echo "Building for $target..."
    
    # Check if target is installed
    if ! rustup target list --installed | grep -q "$target"; then
        echo "Installing target $target..."
        rustup target add "$target"
    fi
    
    # Build
    cargo build --release --target "$target"
    
    # Create archive
    archive_name="${NAME}-v${VERSION}-${target}"
    
    if [[ "$target" == *"windows"* ]]; then
        binary_name="${NAME}.exe"
    else
        binary_name="${NAME}"
    fi
    
    # Create temporary directory for archive
    temp_dir="release/temp_$target"
    mkdir -p "$temp_dir"
    
    # Copy binary and supporting files
    cp "target/$target/release/$binary_name" "$temp_dir/"
    cp README.md "$temp_dir/"
    cp LICENSE "$temp_dir/"
    cp -r shell "$temp_dir/"
    
    # Create archive
    cd release
    if [[ "$target" == *"windows"* ]]; then
        zip -r "${archive_name}.zip" "temp_$target/"
    else
        tar -czf "${archive_name}.tar.gz" "temp_$target/"
    fi
    cd ..
    
    # Cleanup
    rm -rf "$temp_dir"
    
    echo "✅ Created release/$(ls release/*$target* | xargs basename)"
done

echo
echo "🎉 Release binaries created:"
ls -la release/
echo
echo "📦 Ready for GitHub release!"
echo "Next steps:"
echo "1. Create GitHub repository"
echo "2. Push code to GitHub"
echo "3. Create release with these binaries"
echo "4. Update Homebrew formula with SHA256"