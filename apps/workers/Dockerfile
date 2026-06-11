# Base image with Bun and Chromium runtime dependencies
FROM oven/bun:1.2.22 AS base
RUN apt-get update && apt-get install -y \
    chromium \
    fonts-noto-core \
    libnss3 \
    libxss1 \
    libasound2 \
    libatk-bridge2.0-0 \
    libgtk-3-0 \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

# Stage 1: Builder stage to prune the monorepo
FROM base AS builder
RUN bun add -g turbo
WORKDIR /app
COPY . .
RUN turbo prune @orbis/workers --docker

# Stage 2: Installer stage to install production dependencies
FROM base AS installer
WORKDIR /app

# Copy pruned json description for caching dependency installation
COPY --from=builder /app/out/json/ .

# Install production dependencies and clean Bun cache
RUN rm -f pnpm-lock.yaml bun.lock && bun install --production && rm -rf ~/.bun/install/cache

# Copy pruned source files from builder
COPY --from=builder /app/out/full/ .

# Stage 3: Runner stage (clean, minimal runtime environment)
FROM base AS runner
WORKDIR /app

# Copy production node_modules and built code from installer stage
COPY --from=installer /app/node_modules ./node_modules
COPY --from=installer /app .

ENV NODE_ENV=production
ENV PUPPETEER_SKIP_CHROMIUM_DOWNLOAD=true
ENV PUPPETEER_EXECUTABLE_PATH=/usr/bin/chromium

CMD ["bun", "run", "apps/workers/src/worker.ts"]


