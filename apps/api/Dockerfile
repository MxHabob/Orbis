# Base image with Bun
FROM oven/bun:1.2.22 AS base

# Stage 1: Builder stage to prune the monorepo
FROM base AS builder
RUN bun add -g turbo
WORKDIR /app
COPY . .
RUN turbo prune @orbis/api --docker

# Stage 2: Installer stage to compile native dependencies
FROM base AS installer
WORKDIR /app

# Install build-time dependencies needed for compiling canvas/native modules
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    build-essential \
    libcairo2-dev \
    libpango1.0-dev \
    libjpeg-dev \
    libgif-dev \
    librsvg2-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy pruned json description for caching dependency installation
COPY --from=builder /app/out/json/ .

# Install production dependencies and clean Bun cache
RUN rm -f pnpm-lock.yaml bun.lock && bun install --production && rm -rf ~/.bun/install/cache

# Copy pruned source files from builder
COPY --from=builder /app/out/full/ .

# Stage 3: Runner stage (clean, minimal runtime environment)
FROM base AS runner
WORKDIR /app

# Install only runtime dependencies for native modules
RUN apt-get update && apt-get install -y \
    libcairo2 \
    libpango-1.0-0 \
    libjpeg62-turbo \
    libgif7 \
    librsvg2-2 \
    && rm -rf /var/lib/apt/lists/*

# Copy production node_modules and built code from installer stage
COPY --from=installer /app/node_modules ./node_modules
COPY --from=installer /app .
COPY --from=builder /app/packages/db/migrations ./packages/db/migrations

# Set working directory to the API sub-app
WORKDIR /app/apps/api

ENV NODE_ENV=production
EXPOSE 3000

CMD ["bun", "run", "src/index.ts"]
