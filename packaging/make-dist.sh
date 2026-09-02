#!/bin/bash
# Wraps the Tauri .app in a shareable disk image with the installer and a
# short read-me. Output lands in release/, which git ignores.
#
# Usage: packaging/make-dist.sh [path-to-GTA VI Countdown.app]

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
APP_NAME="GTA VI Countdown.app"
VERSION="$(python3 -c "import json; print(json.load(open('${ROOT}/src-tauri/tauri.conf.json'))['version'])")"
STAGE="$(mktemp -d "${TMPDIR:-/tmp}/gta6countdown-dist.XXXXXX")"
OUT_DIR="${ROOT}/release"
OUT_DMG="${OUT_DIR}/GTA-VI-Countdown-${VERSION}-macos-arm64.dmg"

cleanup() {
  rm -rf "${STAGE}"
}
trap cleanup EXIT

candidates=()
if [[ $# -ge 1 ]]; then
  candidates+=("$1")
fi
if [[ -n "${CARGO_TARGET_DIR:-}" ]]; then
  candidates+=("${CARGO_TARGET_DIR}/release/bundle/macos/${APP_NAME}")
fi
candidates+=(
  "${ROOT}/src-tauri/target/release/bundle/macos/${APP_NAME}"
  "${ROOT}/src-tauri/target/universal-apple-darwin/release/bundle/macos/${APP_NAME}"
)

APP=""
for candidate in "${candidates[@]}"; do
  if [[ -d "${candidate}" ]]; then
    APP="${candidate}"
    break
  fi
done

if [[ -z "${APP}" ]]; then
  printf 'could not find %s; pass its path or run npm run widget:build first\n' "${APP_NAME}" >&2
  exit 1
fi

printf 'staging %s\n' "${APP}"
mkdir -p "${STAGE}"
ditto "${APP}" "${STAGE}/${APP_NAME}"
cp "${ROOT}/packaging/Nainstalovat.command" "${STAGE}/Nainstalovat.command"
cp "${ROOT}/packaging/Přečti mě.txt" "${STAGE}/Přečti mě.txt"
cp "${ROOT}/packaging/com.realroyalcrown.gta6countdown.plist" \
  "${STAGE}/com.realroyalcrown.gta6countdown.plist"
chmod +x "${STAGE}/Nainstalovat.command"
ln -s /Applications "${STAGE}/Applications"

mkdir -p "${OUT_DIR}"
rm -f "${OUT_DMG}"

# UDZO is the compressed read-only image Finder users expect.
hdiutil create \
  -volname "GTA VI Countdown" \
  -srcfolder "${STAGE}" \
  -ov \
  -format UDZO \
  "${OUT_DMG}" >/dev/null

printf 'wrote %s (%s)\n' "${OUT_DMG}" "$(du -h "${OUT_DMG}" | awk '{print $1}')"
