#!/bin/bash

# Check if ImageMagick is installed
if ! command -v convert &> /dev/null; then
    echo "Error: ImageMagick is not installed. Please install it first."
    exit 1
fi

# Source icon path
SOURCE_ICON="src/img/sleet_code_icon_trans.png"
ICONS_DIR="icons"

# Create icons directory if it doesn't exist
mkdir -p "$ICONS_DIR"

# Function to create PNG icons
create_png_icon() {
    local size=$1
    local output=$2
    magick "$SOURCE_ICON" -resize ${size}x${size} -alpha on -define png:format=png32 "$ICONS_DIR/$output"
    echo "Created $output"
}

# Create various PNG sizes
create_png_icon 32 "32x32.png"
create_png_icon 128 "128x128.png"
create_png_icon 256 "128x128@2x.png"

# Create Square logos for Windows
create_png_icon 30 "Square30x30Logo.png"
create_png_icon 44 "Square44x44Logo.png"
create_png_icon 71 "Square71x71Logo.png"
create_png_icon 89 "Square89x89Logo.png"
create_png_icon 107 "Square107x107Logo.png"
create_png_icon 142 "Square142x142Logo.png"
create_png_icon 150 "Square150x150Logo.png"
create_png_icon 284 "Square284x284Logo.png"
create_png_icon 310 "Square310x310Logo.png"
create_png_icon 50 "StoreLogo.png"

# Create icon.png (1024x1024 for high resolution)
magick "$SOURCE_ICON" -resize 1024x1024 "$ICONS_DIR/icon.png"
echo "Created icon.png"

# Create .ico file for Windows
magick "$SOURCE_ICON" -define icon:auto-resize=256,128,96,64,48,32,16 "$ICONS_DIR/icon.ico"
echo "Created icon.ico"

# Create .icns file for macOS
if command -v iconutil &> /dev/null; then
    # Create temporary iconset directory
    ICONSET="$ICONS_DIR/icon.iconset"
    mkdir -p "$ICONSET"

    # Generate iconset
    magick "$SOURCE_ICON" -resize 16x16 "$ICONSET/icon_16x16.png"
    magick "$SOURCE_ICON" -resize 32x32 "$ICONSET/icon_16x16@2x.png"
    magick "$SOURCE_ICON" -resize 32x32 "$ICONSET/icon_32x32.png"
    magick "$SOURCE_ICON" -resize 64x64 "$ICONSET/icon_32x32@2x.png"
    magick "$SOURCE_ICON" -resize 128x128 "$ICONSET/icon_128x128.png"
    magick "$SOURCE_ICON" -resize 256x256 "$ICONSET/icon_128x128@2x.png"
    magick "$SOURCE_ICON" -resize 256x256 "$ICONSET/icon_256x256.png"
    magick "$SOURCE_ICON" -resize 512x512 "$ICONSET/icon_256x256@2x.png"
    magick "$SOURCE_ICON" -resize 512x512 "$ICONSET/icon_512x512.png"
    magick "$SOURCE_ICON" -resize 1024x1024 "$ICONSET/icon_512x512@2x.png"

    # Convert iconset to .icns
    iconutil -c icns -o "$ICONS_DIR/icon.icns" "$ICONSET"
    
    # Clean up
    rm -rf "$ICONSET"
    echo "Created icon.icns"
else
    echo "Warning: iconutil not found. Skipping .icns creation."
fi

echo "Icon conversion complete!"