# Orbis API server packages

The native release archives contain the API executable, a version file, and a service installer for the target operating system. The Docker Compose bundle is available separately. Downloading a package does not configure, install, or start a server.

## Requirements and database setup

The customer supplies and operates PostgreSQL; no package creates a database or opens a database port. Before starting Orbis API, create the database and configure a restricted database account. If the server configuration is missing, the API creates `server.json` with an empty `database_url`, reports its location, and exits. Set that value to the customer-managed PostgreSQL connection string, then start the API again. Use TLS for database connections and protect the configuration file because it contains credentials.

Run `orbis-server migrate` with the new release before starting the API against
a new or older schema. The command uses the server configuration and exits
after applying migrations. Use a protected configuration via
`ORBIS_SERVER_CONFIG_FILE` for a separate database role with schema modification
privileges. Back up an existing database first, stop all API instances that
share it, and run the migration once.

On first startup with a configured database, the API generates independent cryptographically random JWT and MFA-encryption keys and stores them in the protected `server.json`. No secret environment variables are needed for a single server installation, and these keys are never sent to Desktop clients. Keep the configuration file private and include it in protected backups. Existing valid `JWT_SECRET` and `MFA_ENCRYPTION_KEY` environment variables override the stored values; clustered deployments must provide the same stable overrides to every instance. Rotating the JWT key invalidates existing sessions. Rotating the MFA key makes previously encrypted integration credentials unreadable unless they are re-encrypted with the replacement key.

## Supported packages

- Linux x64 and ARM64: install with `sudo ./install.sh`, then start with `sudo systemctl enable --now orbis-api.service`. If `server.json` is missing, the API creates it and exits; set `database_url`, then restart the service. Stop with `sudo systemctl stop orbis-api.service`.
- Windows x64: the package includes the WinSW v2.12.0 license. The API executable is not Authenticode-signed, so Windows may show an unknown-publisher or SmartScreen warning. Run `install.ps1` from an elevated PowerShell session to register the service only if your organization's policy permits it. Start `OrbisApi` once to create a missing `%ProgramData%\Orbis\server.json`, set `database_url`, then restart the service. Stop it with `Stop-Service -Name OrbisApi`.
- macOS Apple Silicon: the API executable is not signed or notarized, so Gatekeeper may block it. Install and run it only if your administrator's security policy permits unsigned software. Install with `sudo ./install.sh` to register the launchd service. Start it with `sudo launchctl bootstrap system /Library/LaunchDaemons/com.orbis.api.plist`; if the config is missing, the API creates it and exits. Set `database_url` and restart the service. Stop it with `sudo launchctl bootout system /Library/LaunchDaemons/com.orbis.api.plist`.
- Docker: use the included Compose bundle and follow `docker/README.md`.

Installers register the service but do not configure the database or start the API. The API creates the protected server configuration when first started. Keep the API behind a TLS reverse proxy for public access, restrict firewall rules to intended clients, and do not expose PostgreSQL to the public internet.

## Upgrade and rollback

### Backup and recovery

Schedule backups on the PostgreSQL host or through the institution's database
backup service. For self-managed PostgreSQL, [pgBackRest](https://pgbackrest.org/user-guide.html)
can keep database backups and WAL archives for point-in-time recovery. Monitor
the age of the latest successful backup and WAL archive, and alert when either
exceeds the institution's recovery point objective.

Keep protected copies of the effective server configuration and secrets, license
data, and files from every configured document storage provider. Record the Orbis
version and the corresponding database recovery point and storage snapshot.
Use a coordinated snapshot or pause application writes during capture when the
storage provider cannot recover to the same point as PostgreSQL.

Rehearse recovery before upgrading and after changes to storage or database
configuration. Restore a copy into an isolated environment using the matching
Orbis release and a database and storage configuration that point to the restored
resources. Keep outbound integrations disabled during the rehearsal. Verify
the API readiness endpoint, sign-in, document downloads and uploads, and a
representative academic and finance record. Record the time taken and the latest
recovered transaction to check the institution's recovery objectives.

### Service probes

Use `GET /api/v1/live` to check that the API process can serve requests.
Use `GET /api/v1/health` for traffic readiness. It returns HTTP 503 if the
database cannot answer within the probe deadline.

### Client connections

Configure Desktop with the externally visible HTTPS URL of the API or its
reverse proxy. HTTP is supported by Desktop for localhost development. Browser
clients hosted on a different origin need their exact origins in the comma-separated
`ORBIS_WEB_ORIGINS` setting, such as `https://orbis.school.example`. The packaged
Tauri origins and local Desktop development origins are included by the API.

The API uses up to 10 database connections per replica by default. Set
`ORBIS_DB_MAX_CONNECTIONS` to a positive integer and size it against the total
number of replicas and PostgreSQL's connection budget. Leave room for migrations,
backups, and other applications. Connection and pool acquisition have bounded
wait times.

### Applying an upgrade

On Linux and macOS, the installer stops an active service before replacing the
executable and leaves it stopped. On Windows, the installer stops and
re-registers the service. Apply migrations before restarting the service. To
roll back, run the installer from the previous package and restore a matching
database backup if the previous version requires the previous schema. The
installer preserves the current customer configuration.

To remove the service and executable without deleting customer configuration or license data, run `sudo ./uninstall.sh` on Linux/macOS or `uninstall.ps1` from an elevated PowerShell session on Windows.

## Release integrity and signing

The API manifest is signed with the existing desktop updater key: `TAURI_SIGNING_PRIVATE_KEY` and optional `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` are GitHub Actions secrets, and `TAURI_UPDATER_PUBLIC_KEY` is a repository variable pinned by the desktop application. No Windows or Apple code-signing certificates are required for API packages. The signed manifest authenticates release metadata and includes artifact SHA-256 checksums, but the desktop app does not verify the bytes downloaded by the browser. Windows and macOS binaries remain unsigned and may trigger operating-system security warnings. For key rotation, first ship a desktop application that trusts the replacement public key; only then use its matching private key to sign API manifests. Make the `orbis-api` GHCR package public before distribution so customer servers can pull the image without private registry credentials.

## CI smoke tests

The Windows release runner registers and removes the packaged service while checking that it stays stopped and preserves the existing database configuration. The macOS runner validates the archive, then bootstraps and removes the launchd service while preserving a pre-existing customer configuration. Linux release packaging performs the corresponding systemd install/remove smoke test. These smoke tests do not connect to a real customer database; production setup must use the customer's own PostgreSQL configuration and a verified compatible schema.
