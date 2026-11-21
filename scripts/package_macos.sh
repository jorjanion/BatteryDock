#!/bin/bash
set -e

echo "=== Building BatteryDock for macOS (Universal Binary) ==="

# 1. Build targets
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# 2. Create Universal Binary
mkdir -p target/universal
lipo -create -output target/universal/battery-dock \
    target/aarch64-apple-darwin/release/battery-dock \
    target/x86_64-apple-darwin/release/battery-dock

echo "=== Universal binary created successfully ==="
lipo -info target/universal/battery-dock

# 3. Create .app bundle structure
APP_DIR="target/BatteryDock.app"
rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

cp target/universal/battery-dock "$APP_DIR/Contents/MacOS/battery-dock"
chmod +x "$APP_DIR/Contents/MacOS/battery-dock"

# Copy Icon if available
ICON_SRC=""
if [ -f "assets/icon.icns" ]; then
    cp assets/icon.icns "$APP_DIR/Contents/Resources/AppIcon.icns"
elif [ -f "assets/icon.png" ]; then
    ICON_SRC="assets/icon.png"
elif [ -f "assets/logo.png" ]; then
    ICON_SRC="assets/logo.png"
fi

if [ -n "$ICON_SRC" ]; then
    # Convert PNG to ICNS if sips/iconutil is available
    mkdir -p /tmp/battery_dock.iconset
    sips -z 16 16     "$ICON_SRC" --out /tmp/battery_dock.iconset/icon_16x16.png 2>/dev/null || true
    sips -z 32 32     "$ICON_SRC" --out /tmp/battery_dock.iconset/icon_16x16@2x.png 2>/dev/null || true
    sips -z 32 32     "$ICON_SRC" --out /tmp/battery_dock.iconset/icon_32x32.png 2>/dev/null || true
    sips -z 64 64     "$ICON_SRC" --out /tmp/battery_dock.iconset/icon_32x32@2x.png 2>/dev/null || true
    sips -z 128 128   "$ICON_SRC" --out /tmp/battery_dock.iconset/icon_128x128.png 2>/dev/null || true
    sips -z 256 256   "$ICON_SRC" --out /tmp/battery_dock.iconset/icon_128x128@2x.png 2>/dev/null || true
    sips -z 256 256   "$ICON_SRC" --out /tmp/battery_dock.iconset/icon_256x256.png 2>/dev/null || true
    sips -z 512 512   "$ICON_SRC" --out /tmp/battery_dock.iconset/icon_256x256@2x.png 2>/dev/null || true
    sips -z 512 512   "$ICON_SRC" --out /tmp/battery_dock.iconset/icon_512x512.png 2>/dev/null || true
    sips -z 1024 1024 "$ICON_SRC" --out /tmp/battery_dock.iconset/icon_512x512@2x.png 2>/dev/null || true
    iconutil -c icns /tmp/battery_dock.iconset -o "$APP_DIR/Contents/Resources/AppIcon.icns" 2>/dev/null || true
    rm -rf /tmp/battery_dock.iconset
fi

# Create Info.plist
cat <<EOF > "$APP_DIR/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>battery-dock</string>
    <key>CFBundleIdentifier</key>
    <string>com.jorjanion.BatteryDock</string>
    <key>CFBundleName</key>
    <string>BatteryDock</string>
    <key>CFBundleDisplayName</key>
    <string>BatteryDock</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>LSUIElement</key>
    <true/>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSRequiresAquaSystemAppearance</key>
    <false/>
</dict>
</plist>
EOF

echo "=== BatteryDock.app bundle created ==="

# 4. Create DMG
DMG_SRC="target/dmg_src"
rm -rf "$DMG_SRC"
mkdir -p "$DMG_SRC"
cp -R "$APP_DIR" "$DMG_SRC/"
ln -s /Applications "$DMG_SRC/Applications"

mkdir -p output
hdiutil create -volname "BatteryDock" -srcfolder "$DMG_SRC" -ov -format UDZO output/BatteryDock-macOS.dmg

echo "=== BatteryDock-macOS.dmg successfully built at output/BatteryDock-macOS.dmg ==="
