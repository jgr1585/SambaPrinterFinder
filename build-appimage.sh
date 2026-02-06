#!/bin/bash
# Script to build AppImage with proper AppStream metadata

set -e

echo "Building with cargo appimage..."
cargo appimage

# Clean up any existing AppImage to avoid confusion
rm -rf target/appimage/samba_printer_finder.AppImage

echo "Preparing AppStream metadata..."
# Create required directories
mkdir -p target/samba_printer_finder.AppDir/usr/share/metainfo
mkdir -p target/samba_printer_finder.AppDir/usr/share/applications

# Copy metadata file with proper name
cp flatpak/io.github.jgr1585.SambaPrinterFinder.metainfo.xml \
   target/samba_printer_finder.AppDir/usr/share/metainfo/io.github.jgr1585.SambaPrinterFinder.appdata.xml

# Also copy as metainfo.xml for compatibility
cp flatpak/io.github.jgr1585.SambaPrinterFinder.metainfo.xml \
   target/samba_printer_finder.AppDir/usr/share/metainfo/io.github.jgr1585.SambaPrinterFinder.metainfo.xml

# Copy proper desktop file
cp flatpak/io.github.jgr1585.SambaPrinterFinder.desktop \
   target/samba_printer_finder.AppDir/usr/share/applications/

# Copy desktop file to root (required by appimagetool)
cp flatpak/io.github.jgr1585.SambaPrinterFinder.desktop \
   target/samba_printer_finder.AppDir/

# Create icon symlink
ln -sf samba_printer_finder.svg \
   target/samba_printer_finder.AppDir/io.github.jgr1585.SambaPrinterFinder.svg

# Remove auto-generated desktop file if it exists
rm -f target/samba_printer_finder.AppDir/cargo-appimage.desktop

echo "Packaging AppImage with appimagetool..."
appimagetool target/samba_printer_finder.AppDir \
   target/appimage/samba_printer_finder-$(grep '^version' Cargo.toml | cut -d'"' -f2)-$(uname -m).AppImage

echo "✓ AppImage created successfully!"
