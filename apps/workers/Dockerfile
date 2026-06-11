# Stage 1: Builder — prune the monorepo (no Chromium needed here)
FROM oven/bun:1.2.22-slim AS builder
RUN bun add -g turbo
WORKDIR /app
COPY . .
RUN turbo prune @orbis/workers --docker

# Stage 2: Installer — install production dependencies only (no Chromium needed here)
FROM oven/bun:1.2.22-slim AS installer
WORKDIR /app

# Copy pruned package.json files for caching dependency installation
COPY --from=builder /app/out/json/ .

# Install production dependencies and clean Bun cache
RUN rm -f pnpm-lock.yaml bun.lock && bun install --production && rm -rf ~/.bun/install/cache

# Copy pruned source files from builder
COPY --from=builder /app/out/full/ .

# Stage 3: Runner — Chromium is only installed here (saves ~900MB from builder/installer)
FROM oven/bun:1.2.22-slim AS runner
WORKDIR /app

# Install Chromium and runtime dependencies ONLY in the final stage
RUN apt-get update && apt-get install -y --no-install-recommends \
    chromium \
    fonts-noto-core \
    libnss3 \
    libxss1 \
    libasound2 \
    libatk-bridge2.0-0 \
    libgtk-3-0 \
    && rm -rf /var/lib/apt/lists/*

# Copy everything from installer in a single layer (no duplicate node_modules)
COPY --from=installer /app .

ENV NODE_ENV=production
ENV PUPPETEER_SKIP_CHROMIUM_DOWNLOAD=true
ENV PUPPETEER_EXECUTABLE_PATH=/usr/bin/chromium

CMD ["bun", "run", "apps/workers/src/worker.ts"]
