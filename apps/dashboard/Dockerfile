# Base image with Bun
FROM oven/bun:1.2.22 AS base

# Install turbo CLI globally
FROM base AS turbo-cli
RUN bun add -g turbo

# Builder stage
FROM turbo-cli AS builder
WORKDIR /app
COPY . .
RUN turbo prune @midday/dashboard --docker

# Installer stage
FROM base AS installer
WORKDIR /app

# Copy pruned workspace
COPY --from=builder /app/out/json/ .
# Install dependencies
RUN bun install

# Copy full source and build
COPY --from=builder /app/out/full/ .

# Build-time environment variables
ARG NEXT_PUBLIC_API_URL
ARG NEXT_PUBLIC_URL
ARG NEXT_PUBLIC_S3_PUBLIC_URL
ENV NEXT_PUBLIC_API_URL=$NEXT_PUBLIC_API_URL
ENV NEXT_PUBLIC_URL=$NEXT_PUBLIC_URL
ENV NEXT_PUBLIC_S3_PUBLIC_URL=$NEXT_PUBLIC_S3_PUBLIC_URL
ENV NODE_ENV=production

RUN bun run build --filter=@midday/dashboard

# Runner stage - using Bun for compatibility with Bun-specific APIs (e.g. RedisClient)
FROM oven/bun:1.2.22-slim AS runner
WORKDIR /app

ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1

# Don't run production as root
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs
USER nextjs

# Copy standalone output from installer
# Note: turbo prune puts things in out/full/apps/dashboard
COPY --from=installer --chown=nextjs:nodejs /app/apps/dashboard/.next/standalone ./
COPY --from=installer --chown=nextjs:nodejs /app/apps/dashboard/.next/static ./apps/dashboard/.next/static
COPY --from=installer --chown=nextjs:nodejs /app/apps/dashboard/public ./apps/dashboard/public

EXPOSE 3001
ENV PORT=3001
ENV HOSTNAME="0.0.0.0"

CMD ["bun", "run", "apps/dashboard/server.js"]
