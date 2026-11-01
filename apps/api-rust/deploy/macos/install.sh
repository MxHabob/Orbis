#!/usr/bin/env bash
set -euo pipefail

if [[ "$(id -u)" -ne 0 ]]; then
  echo "Run this installer as root, for example: sudo ./install.sh" >&2
  exit 1
fi

package_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
binary="$package_dir/orbis-server"
plist="$package_dir/com.orbis.api.plist"
version_file="$package_dir/VERSION"
config_template="$package_dir/server.example.json"

for required_file in "$binary" "$plist" "$version_file" "$config_template"; do
  if [[ ! -f "$required_file" ]]; then
    echo "Required package file is missing: $required_file" >&2
    exit 1
  fi
done

if ! command -v launchctl >/dev/null 2>&1; then
  echo "launchd is required to install the Orbis API service." >&2
  exit 1
fi

if launchctl print system/com.orbis.api >/dev/null 2>&1; then
  launchctl bootout system/com.orbis.api
fi

install -d -o root -g wheel -m 0755 /Library/Orbis/bin
install -d -o root -g nobody -m 0750 "/Library/Application Support/Orbis"
install -d -o nobody -g nobody -m 0750 \
  "/Library/Application Support/Orbis/license" /Library/Logs/Orbis
install -o root -g wheel -m 0755 "$binary" /Library/Orbis/bin/orbis-server
install -o root -g wheel -m 0644 "$version_file" /Library/Orbis/VERSION
install -o root -g wheel -m 0644 "$plist" /Library/LaunchDaemons/com.orbis.api.plist

if [[ ! -e "/Library/Application Support/Orbis/server.json" ]]; then
  install -o root -g nobody -m 0640 \
    "$config_template" "/Library/Application Support/Orbis/server.json"
fi

printf 'Orbis API %s installed.\n' "$(cat /Library/Orbis/VERSION)"
echo "Configure /Library/Application Support/Orbis/server.json with the existing database URL."
echo "After the database schema and migrations are ready, start the service with:"
echo "  sudo launchctl bootstrap system /Library/LaunchDaemons/com.orbis.api.plist"
