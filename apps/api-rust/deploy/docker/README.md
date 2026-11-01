# Orbis API Docker deployment

## Requirements

- Docker Engine with the Compose v2 plugin.
- A PostgreSQL database reachable from the Docker host. The database is customer-managed and is not included in this bundle.
- A TLS-terminating reverse proxy for public access. Keep the container port bound to loopback unless access is restricted by a trusted private network.
- The `ghcr.io/mxhabob/orbis-api` container package must be public for anonymous pulls, or the customer must configure private-registry credentials.

## First start

1. Copy `.env.example` to `.env`. The release bundle pins `ORBIS_API_IMAGE` to the published multi-architecture image digest. Keep that value unchanged unless intentionally choosing another verified image. Keep the UID, GID, and bind address restricted to the service account and intended network.
2. Copy `server.example.json` to `server.json` and set `database_url` to the customer-managed PostgreSQL connection string. Use TLS and a database account with only the permissions required by Orbis.
3. Restrict access to `.env` and `server.json` to the service administrator. Do not commit credentials or expose them through logs.
4. Start and inspect the service:

   ```sh
   docker compose up -d
   docker compose ps
   docker compose logs --tail=100 api
   ```

The container runs as a non-root user, drops Linux capabilities, uses a read-only root filesystem, and persists only its license data in a named volume. It does not create, expose, or manage a database.

## Upgrade and rollback

Back up the database and retain the current `server.json` and `.env` before changing the version. Replace `ORBIS_API_IMAGE` with the digest-pinned image reference from the desired release bundle, then run `docker compose pull` and `docker compose up -d`. To roll back, restore the previous image reference and start the service again. Confirm that the selected API version supports the existing database schema before upgrading or rolling back.

Release CI validates the Compose bundle and publishes a multi-architecture manifest for Linux AMD64 and ARM64. Container startup and API health require a customer-supplied, migrated PostgreSQL database; automated release checks do not modify a customer database.
