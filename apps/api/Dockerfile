# Base image with Bun (slim variant for smaller footprint)
FROM oven/bun:1.2.22-slim AS base

# Stage 1: Builder — prune the monorepo to only API-related packages
FROM base AS builder
RUN bun add -g turbo
WORKDIR /app
COPY . .
RUN turbo prune @orbis/api --docker

# Stage 2: Installer — compile native dependencies and install production deps
FROM base AS installer
WORKDIR /app

# Install build-time dependencies needed for compiling canvas/native modules
# These are only needed for compilation and won't be in the final image
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    python3-pip \
    build-essential \
    libcairo2-dev \
    libpango1.0-dev \
    libjpeg-dev \
    libgif-dev \
    librsvg2-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy pruned package.json files for caching dependency installation
COPY --from=builder /app/out/json/ .

# Install production dependencies only and clean Bun cache
RUN rm -f pnpm-lock.yaml bun.lock && bun install --production && rm -rf ~/.bun/install/cache

# Copy pruned source files from builder
COPY --from=builder /app/out/full/ .

# Stage 3: Runner — clean, minimal runtime environment
FROM oven/bun:1.2.22-slim AS runner
WORKDIR /app

# Install only lightweight runtime libraries for native modules (no dev headers)
RUN apt-get update && apt-get install -y --no-install-recommends \
    libcairo2 \
    libpango-1.0-0 \
    libjpeg62-turbo \
    libgif7 \
    librsvg2-2 \
    && rm -rf /var/lib/apt/lists/*

# Copy everything from installer in a single layer (no duplicate node_modules)
COPY --from=installer /app .
COPY --from=builder /app/packages/db/migrations ./packages/db/migrations

# Set working directory to the API sub-app
WORKDIR /app/apps/api

ENV NODE_ENV=production
EXPOSE 3000

CMD ["bun", "run", "src/index.ts"]