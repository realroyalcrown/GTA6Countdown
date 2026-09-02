#!/bin/bash
# Copies the widget into /Applications, strips the Gatekeeper quarantine
# that macOS stamps on downloads, registers the login item, and launches it.
#
# Apple will not bless this app without a paid Developer ID, so a downloaded
# copy is blocked until that quarantine flag is removed. Running this script
# from Terminal is the reliable way to do that on macOS 15 and 26.

set -euo pipefail

APP_NAME="GTA VI Countdown.app"
DEST="/Applications/${APP_NAME}"
LABEL="com.realroyalcrown.gta6countdown"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

say() {
  printf '%s\n' "$*"
}

die() {
  say "Chyba: $*" >&2
  exit 1
}

if [[ -d "${SCRIPT_DIR}/${APP_NAME}" ]]; then
  SRC="${SCRIPT_DIR}/${APP_NAME}"
elif [[ -d "${DEST}" ]]; then
  SRC="${DEST}"
else
  die "Nenašel jsem ${APP_NAME}. Nech tento soubor vedle aplikace v otevřeném disku, nebo ji nejdřív přetáhni do složky Aplikace."
fi

# Replacing a running binary fails on some macOS versions.
osascript -e 'quit app "GTA VI Countdown"' >/dev/null 2>&1 || true
sleep 1

if [[ "${SRC}" != "${DEST}" ]]; then
  say "Kopíruji do /Applications…"
  rm -rf "${DEST}"
  ditto "${SRC}" "${DEST}"
fi

[[ -d "${DEST}" ]] || die "Kopírování do ${DEST} selhalo."

say "Odstraňuji zámek Gatekeeperu (karanténa po stažení)…"
xattr -cr "${DEST}"

PLIST_DEST="${HOME}/Library/LaunchAgents/${LABEL}.plist"
mkdir -p "${HOME}/Library/LaunchAgents"

if [[ -f "${SCRIPT_DIR}/${LABEL}.plist" ]]; then
  PLIST_SRC="${SCRIPT_DIR}/${LABEL}.plist"
elif [[ -f "${DEST}/Contents/Resources/${LABEL}.plist" ]]; then
  PLIST_SRC="${DEST}/Contents/Resources/${LABEL}.plist"
else
  PLIST_SRC=""
fi

if [[ -n "${PLIST_SRC}" ]]; then
  say "Zapínám spuštění po přihlášení…"
  cp "${PLIST_SRC}" "${PLIST_DEST}"
  launchctl bootout "gui/$(id -u)/${LABEL}" >/dev/null 2>&1 || true
  launchctl bootstrap "gui/$(id -u)" "${PLIST_DEST}" >/dev/null 2>&1 || true
fi

say "Spouštím widget…"
open "${DEST}"

say ""
say "Hotovo. Widget je na ploše, odpočet v horní liště."
say "Ukončení z lišty ho nechá zavřený do příštího přihlášení."

osascript -e 'display dialog "GTA VI Countdown je nainstalovaný." & return & return & "Widget je na ploše, odpočet v horní liště." buttons {"OK"} default button 1 with title "GTA VI Countdown"' >/dev/null 2>&1 || true
