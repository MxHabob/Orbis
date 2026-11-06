#!/usr/bin/env bash
set -euo pipefail

if [[ "$(id -u)" -ne 0 ]]; then
  echo "Run this installer as root, for example: sudo ./install.sh" >&2
  exit 1
fi

if ! command -v systemctl >/dev/null 2>&1; then
  echo "systemd is required to install the Orbis API service." >&2
  exit 1
fi

if systemctl is-active --quiet orbis-api.service; then
  systemctl stop orbis-api.service
fi

package_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
binary="$package_dir/orbis-server"
unit="$package_dir/orbis-api.service"
version_file="$package_dir/VERSION"

for required_file in "$binary" "$unit" "$version_file"; do
  if [[ ! -f "$required_file" ]]; then
    echo "Required package file is missing: $required_file" >&2
    exit 1
  fi
done

if ! id orbis >/dev/null 2>&1; then
  useradd --system --home-dir /nonexistent --no-create-home --shell /usr/sbin/nologin orbis
fi

install -d -o root -g root -m 0755 /opt/orbis/bin
install -d -o root -g orbis -m 0770 /etc/orbis
install -d -o orbis -g orbis -m 0750 /var/lib/orbis/license
install -o root -g root -m 0755 "$binary" /opt/orbis/bin/orbis-server
install -o root -g root -m 0644 "$version_file" /opt/orbis/VERSION
install -o root -g root -m 0644 "$unit" /etc/systemd/system/orbis-api.service

systemctl daemon-reload

printf 'Orbis API %s installed.\n' "$(cat /opt/orbis/VERSION)"
echo "The API creates /etc/orbis/server.json on first start if it is missing."
echo "Set database_url in that file before starting the API again."
echo "After the database schema and migrations are ready, start the service with:"
echo "  sudo systemctl enable --now orbis-api.service"
