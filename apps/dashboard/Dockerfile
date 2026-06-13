# Base image with Bun (slim variant)
FROM oven/bun:1.2.22-slim AS base

# Install turbo CLI globally
FROM base AS turbo-cli
RUN bun add -g turbo

# Builder stage — prune monorepo to dashboard-related packages
FROM turbo-cli AS builder
WORKDIR /app
COPY . .
RUN turbo prune @orbis/dashboard --docker

# Installer stage — install deps and build Next.js
FROM base AS installer
WORKDIR /app

# Copy pruned workspace
COPY --from=builder /app/out/json/ .
# Install ALL dependencies (dev deps needed for build: tailwindcss, postcss, etc.)
RUN bun install

# Copy full source and build
COPY --from=builder /app/out/full/ .

# Build-time environment variables (default to placeholders for runtime substitution)
ARG NEXT_PUBLIC_API_URL=http://__NEXT_PUBLIC_API_URL_PLACEHOLDER__
ARG NEXT_PUBLIC_URL=__NEXT_PUBLIC_URL_PLACEHOLDER__
ARG NEXT_PUBLIC_S3_PUBLIC_URL=__NEXT_PUBLIC_S3_PUBLIC_URL_PLACEHOLDER__
ENV NEXT_PUBLIC_API_URL=$NEXT_PUBLIC_API_URL
ENV NEXT_PUBLIC_URL=$NEXT_PUBLIC_URL
ENV NEXT_PUBLIC_S3_PUBLIC_URL=$NEXT_PUBLIC_S3_PUBLIC_URL
ENV NODE_ENV=production
ENV NEXT_PRIVATE_WORKERS=2

RUN bun run build --filter=@orbis/dashboard


# Runner stage — uses slim image, only copies standalone output (no node_modules bloat)
FROM oven/bun:1.2.22-slim AS runner
WORKDIR /app

ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1

# Don't run production as root
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs
USER nextjs

# Copy standalone output from installer (Next.js standalone includes only needed deps)
COPY --from=installer --chown=nextjs:nodejs /app/apps/dashboard/.next/standalone ./
COPY --from=installer --chown=nextjs:nodejs /app/apps/dashboard/.next/static ./apps/dashboard/.next/static
COPY --from=installer --chown=nextjs:nodejs /app/apps/dashboard/public ./apps/dashboard/public

EXPOSE 3001
ENV PORT=3001
ENV HOSTNAME="0.0.0.0"

# At startup: substitute placeholders in compiled JS/HTML/JSON files with actual runtime values.
# When NEXT_PUBLIC_API_URL is empty, the placeholder is replaced with empty string
# so client-side dynamic host detection activates automatically.
CMD ["/bin/sh", "-c", "find /app -type f \\( -name '*.js' -o -name '*.html' -o -name '*.json' \\) | xargs sed -i \"s|http://__NEXT_PUBLIC_API_URL_PLACEHOLDER__|${NEXT_PUBLIC_API_URL}|g; s|__NEXT_PUBLIC_API_URL_PLACEHOLDER__|${NEXT_PUBLIC_API_URL}|g; s|__NEXT_PUBLIC_URL_PLACEHOLDER__|${NEXT_PUBLIC_URL}|g; s|__NEXT_PUBLIC_S3_PUBLIC_URL_PLACEHOLDER__|${NEXT_PUBLIC_S3_PUBLIC_URL}|g\" 2>/dev/null; bun run apps/dashboard/server.js"]

