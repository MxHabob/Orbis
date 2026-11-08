#!/usr/bin/env bash
set -euo pipefail

if [[ "$(id -u)" -ne 0 ]]; then
  echo "Run this uninstaller as root, for example: sudo ./uninstall.sh" >&2
  exit 1
fi

if ! command -v systemctl >/dev/null 2>&1; then
  echo "systemd is required to remove the Orbis API service." >&2
  exit 1
fi

if systemctl is-active --quiet orbis-api.service; then
  systemctl stop orbis-api.service
fi

if systemctl is-enabled --quiet orbis-api.service; then
  systemctl disable orbis-api.service
fi

rm -f /etc/systemd/system/orbis-api.service
rm -f /opt/orbis/bin/orbis-server /opt/orbis/VERSION
systemctl daemon-reload

echo "Orbis API service and executable removed."
echo "Configuration and license data were preserved at /etc/orbis and /var/lib/orbis."
