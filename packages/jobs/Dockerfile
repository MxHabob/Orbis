# Base image with Bun
FROM oven/bun:1.2.22 AS base

# Install Puppeteer dependencies and Chromium
RUN apt-get update && apt-get install -y \
    chromium \
    fonts-noto-arabic \
    fonts-liberation \
    libnss3 \
    libatk1.0-0 \
    libatk-bridge2.0-0 \
    libcups2 \
    libdrm2 \
    libxcomposite1 \
    libxdamage1 \
    libxrandr2 \
    libgbm1 \
    libasound2 \
    libpangocairo-1.0-0 \
    libxshmfence1 \
    && rm -rf /var/lib/apt/lists/*

# Install turbo CLI globally
RUN bun add -g turbo

# Builder stage
FROM base AS builder
WORKDIR /app
COPY . .
RUN turbo prune @midday/jobs --docker

# Installer stage
FROM base AS installer
WORKDIR /app
COPY --from=builder /app/out/json/ .
RUN bun install

# Final source
COPY --from=builder /app/out/full/ .

# Environment
ENV NODE_ENV=production
ENV PUPPETEER_SKIP_CHROMIUM_DOWNLOAD=true
ENV PUPPETEER_EXECUTABLE_PATH=/usr/bin/chromium

# Run worker
CMD ["bun", "run", "packages/jobs/src/worker.ts"]
