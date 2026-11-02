#!/usr/bin/env bash
set -euo pipefail

if [[ "$(id -u)" -ne 0 ]]; then
  echo "Run this uninstaller as root, for example: sudo ./uninstall.sh" >&2
  exit 1
fi

if ! command -v launchctl >/dev/null 2>&1; then
  echo "launchd is required to remove the Orbis API service." >&2
  exit 1
fi

plist="/Library/LaunchDaemons/com.orbis.api.plist"
if launchctl list | awk '$NF == "com.orbis.api" { found = 1 } END { exit !found }'; then
  launchctl bootout system "$plist"
fi

rm -f "$plist"
rm -f /Library/Orbis/bin/orbis-server /Library/Orbis/VERSION

echo "Orbis API service and executable removed."
echo "Configuration, logs, and license data were preserved."
