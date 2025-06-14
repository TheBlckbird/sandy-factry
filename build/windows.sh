#!/bin/bash

cd "$(dirname "$0")"

TEMP_DIR="../temp"
OUT_DIR="../out"
APP_NAME="Sandy Fact'ry"
RUST_CRATE_NAME="sandy-factry"

rm -rf "${TEMP_DIR}"

mkdir "${TEMP_DIR}"
mkdir "${OUT_DIR}"

rm -rf "${TEMP_DIR}/windows"
rm -rf "${OUT_DIR}/windows.zip"

mkdir "${TEMP_DIR}/windows"
cp -R ../assets "${TEMP_DIR}/windows"

cargo build --release --target x86_64-pc-windows-gnu
cp "../target/x86_64-pc-windows-gnu/release/${RUST_CRATE_NAME}.exe" "${TEMP_DIR}/windows"

cd "${TEMP_DIR}"
zip -r "${OUT_DIR}/windows.zip" "windows/"

rm -rf "${TEMP_DIR}"
