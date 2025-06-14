#!/bin/bash

cd "$(dirname "$0")"

TEMP_DIR="../temp"
OUT_DIR="../out"
APP_NAME="Sandy Fact'ry"
RUST_CRATE_NAME="sandy-factry"
APP_ICON="./icon1024.png"

rm -rf "${TEMP_DIR}"

mkdir "${TEMP_DIR}"
mkdir "${OUT_DIR}"

mkdir -p "${TEMP_DIR}/AppIcon.iconset"
sips -z 16 16 "${APP_ICON}" --out "${TEMP_DIR}/AppIcon.iconset/icon_16x16.png"
sips -z 32 32 "${APP_ICON}" --out "${TEMP_DIR}/AppIcon.iconset/icon_16x16@2x.png"
sips -z 32 32 "${APP_ICON}" --out "${TEMP_DIR}/AppIcon.iconset/icon_32x32.png"
sips -z 64 64 "${APP_ICON}" --out "${TEMP_DIR}/AppIcon.iconset/icon_32x32@2x.png"
sips -z 128 128 "${APP_ICON}" --out "${TEMP_DIR}/AppIcon.iconset/icon_128x128.png"
sips -z 256 256 "${APP_ICON}" --out "${TEMP_DIR}/AppIcon.iconset/icon_128x128@2x.png"
sips -z 256 256 "${APP_ICON}" --out "${TEMP_DIR}/AppIcon.iconset/icon_256x256.png"
sips -z 512 512 "${APP_ICON}" --out "${TEMP_DIR}/AppIcon.iconset/icon_256x256@2x.png"
sips -z 512 512 "${APP_ICON}" --out "${TEMP_DIR}/AppIcon.iconset/icon_512x512.png"
cp "${APP_ICON}" "${TEMP_DIR}/AppIcon.iconset/icon_512x512@2x.png"
iconutil -c icns "${TEMP_DIR}/AppIcon.iconset"

rm "${OUT_DIR}/${APP_NAME}.dmg"

# create the folder structure
mkdir -p "${TEMP_DIR}/${APP_NAME}.app/Contents/MacOS"
mkdir -p "${TEMP_DIR}/${APP_NAME}.app/Contents/Resources"
# copy Info.plist
cp macos/Info.plist "${TEMP_DIR}/${APP_NAME}.app/Contents/Info.plist"
# copy the icon (assuming you already have it in Apple ICNS format)
cp "${TEMP_DIR}/AppIcon.icns" "${TEMP_DIR}/${APP_NAME}.app/Contents/Resources/AppIcon.icns"
# copy your Bevy game assets
cp -a ../assets "${TEMP_DIR}/${APP_NAME}.app/Contents/MacOS/"
# compile the executables for each architecture
cargo build --release --target x86_64-apple-darwin  # build for Intel
cargo build --release --target aarch64-apple-darwin # build for Apple Silicon
# combine the executables into a single file and put it in the bundle
lipo "../target/x86_64-apple-darwin/release/${RUST_CRATE_NAME}" \
     "../target/aarch64-apple-darwin/release/${RUST_CRATE_NAME}" \
     -create -output "${TEMP_DIR}/${APP_NAME}.app/Contents/MacOS/${RUST_CRATE_NAME}"

create-dmg \
     --volname "Sandy Fact'ry Installer" \
     --window-pos 200 120 \
     --window-size 500 300 \
     --icon-size 100 \
     --icon "${APP_NAME}.app" 100 100 \
     --app-drop-link 300 100 \
     "${OUT_DIR}/${APP_NAME}.dmg" \
     "${TEMP_DIR}/${APP_NAME}.app"

rm -rf "${TEMP_DIR}"
