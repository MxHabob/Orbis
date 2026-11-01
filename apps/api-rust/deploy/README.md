# Orbis API server packages

The native release archives contain the API executable, a version file, an example database configuration, and a service installer for the target operating system. The Docker Compose bundle is available separately. Downloading a package does not configure, install, or start a server.

## Requirements and database setup

The customer supplies and operates PostgreSQL; no package creates a database or opens a database port. Before starting Orbis API, create the database, configure a restricted database account, and configure the connection string in the OS-specific server configuration. Use TLS for database connections and protect the configuration file because it contains credentials.

The database must already contain the Orbis schema and required migrations. The API intentionally refuses to start against an uninitialized or incompatible schema. Back up the database before applying migrations or upgrading the API.

## Supported packages

- Linux x64 and ARM64: install with `sudo ./install.sh`, inspect `/etc/systemd/system/orbis-api.service`, then explicitly start with `sudo systemctl enable --now orbis-api.service`. Use `sudo systemctl stop orbis-api.service` to stop it.
- Windows x64: the package includes the WinSW v2.12.0 license. Run `install.ps1` from an elevated PowerShell session to register the service. Configure `%ProgramData%\Orbis\server.json`, then explicitly start with `Start-Service -Name OrbisApi`; use `Stop-Service -Name OrbisApi` to stop it.
- macOS Apple Silicon: install with `sudo ./install.sh` to register the launchd service. Configure `/Library/Application Support/Orbis/server.json`, then explicitly start with `sudo launchctl bootstrap system /Library/LaunchDaemons/com.orbis.api.plist`; stop with `sudo launchctl bootout system/com.orbis.api`. The executable is signed and notarized.
- Docker: use the included Compose bundle and follow `docker/README.md`.

Installers create a configuration template but do not start an API until the customer supplies a valid database URL. Keep the API behind a TLS reverse proxy for public access, restrict firewall rules to intended clients, and do not expose PostgreSQL to the public internet.

## Upgrade and rollback

Back up PostgreSQL and retain the existing configuration before upgrading. On Linux and macOS, the installer stops an active service before replacing the executable and leaves it stopped; on Windows, the installer stops and re-registers the service. Start it only after confirming the database schema is compatible. To roll back, run the installer from the previous package; the installer preserves the current customer configuration. Database migrations may not be reversible; restore a matching database backup if a rollback requires the previous schema.

To remove the service and executable without deleting customer configuration or license data, run `sudo ./uninstall.sh` on Linux/macOS or `uninstall.ps1` from an elevated PowerShell session on Windows.

## Release signing configuration

Release publication requires the `WINDOWS_API_SIGNING_PFX_BASE64` and `WINDOWS_API_SIGNING_PFX_PASSWORD` secrets plus `WINDOWS_API_SIGNING_SUBJECT`; Apple signing/notarization secrets `APPLE_API_CERTIFICATE_P12_BASE64`, `APPLE_API_CERTIFICATE_PASSWORD`, `APPLE_API_KEYCHAIN_PASSWORD`, `APPLE_API_ID`, and `APPLE_API_APP_PASSWORD` plus `APPLE_API_SIGNING_IDENTITY` and `APPLE_API_TEAM_ID`; and the `TAURI_SIGNING_PRIVATE_KEY` and optional `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` secrets plus the `TAURI_UPDATER_PUBLIC_KEY` repository variable. Store private material only as protected GitHub Actions secrets. The API manifest uses the same updater public key pinned by the desktop application. For key rotation, first ship a desktop application that trusts the replacement public key; only then use its matching private key to sign API manifests. Make the `orbis-api` GHCR package public before distribution so customer servers can pull the image without private registry credentials.

## CI smoke tests

The Windows release runner registers and removes the packaged service while checking that it stays stopped and preserves the existing database configuration. The macOS runner validates the notarized binary and archive, then bootstraps and removes the launchd service while preserving a pre-existing customer configuration. Linux release packaging performs the corresponding systemd install/remove smoke test. These smoke tests do not connect to a real customer database; production setup must use the customer's own PostgreSQL configuration and a verified compatible schema.
