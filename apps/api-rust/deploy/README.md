# Orbis API server packages

The native release archives contain the API executable, a version file, and a service installer for the target operating system. The Docker Compose bundle is available separately. Downloading a package does not configure, install, or start a server.

## Requirements and database setup

The customer supplies and operates PostgreSQL; no package creates a database or opens a database port. Before starting Orbis API, create the database and configure a restricted database account. If the server configuration is missing, the API creates `server.json` with an empty `database_url`, reports its location, and exits. Set that value to the customer-managed PostgreSQL connection string, then start the API again. Use TLS for database connections and protect the configuration file because it contains credentials.

The database must already contain the Orbis schema and required migrations. The API intentionally refuses to start against an uninitialized or incompatible schema. Back up the database before applying migrations or upgrading the API.

## Supported packages

- Linux x64 and ARM64: install with `sudo ./install.sh`, then start with `sudo systemctl enable --now orbis-api.service`. If `server.json` is missing, the API creates it and exits; set `database_url`, then restart the service. Stop with `sudo systemctl stop orbis-api.service`.
- Windows x64: the package includes the WinSW v2.12.0 license. The API executable is not Authenticode-signed, so Windows may show an unknown-publisher or SmartScreen warning. Run `install.ps1` from an elevated PowerShell session to register the service only if your organization's policy permits it. Start `OrbisApi` once to create a missing `%ProgramData%\Orbis\server.json`, set `database_url`, then restart the service. Stop it with `Stop-Service -Name OrbisApi`.
- macOS Apple Silicon: the API executable is not signed or notarized, so Gatekeeper may block it. Install and run it only if your administrator's security policy permits unsigned software. Install with `sudo ./install.sh` to register the launchd service. Start it with `sudo launchctl bootstrap system /Library/LaunchDaemons/com.orbis.api.plist`; if the config is missing, the API creates it and exits. Set `database_url` and restart the service. Stop it with `sudo launchctl bootout system/com.orbis.api`.
- Docker: use the included Compose bundle and follow `docker/README.md`.

Installers register the service but do not create the server configuration or start the API. Keep the API behind a TLS reverse proxy for public access, restrict firewall rules to intended clients, and do not expose PostgreSQL to the public internet.

## Upgrade and rollback

Back up PostgreSQL and retain the existing configuration before upgrading. On Linux and macOS, the installer stops an active service before replacing the executable and leaves it stopped; on Windows, the installer stops and re-registers the service. Start it only after confirming the database schema is compatible. To roll back, run the installer from the previous package; the installer preserves the current customer configuration. Database migrations may not be reversible; restore a matching database backup if a rollback requires the previous schema.

To remove the service and executable without deleting customer configuration or license data, run `sudo ./uninstall.sh` on Linux/macOS or `uninstall.ps1` from an elevated PowerShell session on Windows.

## Release integrity and signing

The API manifest is signed with the existing desktop updater key: `TAURI_SIGNING_PRIVATE_KEY` and optional `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` are GitHub Actions secrets, and `TAURI_UPDATER_PUBLIC_KEY` is a repository variable pinned by the desktop application. No Windows or Apple code-signing certificates are required for API packages. The signed manifest authenticates release metadata and includes artifact SHA-256 checksums, but the desktop app does not verify the bytes downloaded by the browser. Windows and macOS binaries remain unsigned and may trigger operating-system security warnings. For key rotation, first ship a desktop application that trusts the replacement public key; only then use its matching private key to sign API manifests. Make the `orbis-api` GHCR package public before distribution so customer servers can pull the image without private registry credentials.

## CI smoke tests

The Windows release runner registers and removes the packaged service while checking that it stays stopped and preserves the existing database configuration. The macOS runner validates the archive, then bootstraps and removes the launchd service while preserving a pre-existing customer configuration. Linux release packaging performs the corresponding systemd install/remove smoke test. These smoke tests do not connect to a real customer database; production setup must use the customer's own PostgreSQL configuration and a verified compatible schema.
