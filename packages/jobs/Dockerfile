# Base image with Bun
FROM oven/bun:1.2.22 AS base

# Install Chromium and required fonts/libs for Puppeteer
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

# Turbo CLI stage
FROM base AS turbo-cli
RUN bun add -g turbo

# Builder stage
FROM turbo-cli AS builder
WORKDIR /app
# Copy all files
COPY . .
# Use turbo CLI from Bun global install to prune workspaces
RUN turbo prune @midday/jobs --docker

# Installer stage
FROM base AS installer
WORKDIR /app

# First install the dependencies (as they change less often)
COPY --from=builder /app/out/json/ .
# Install dependencies with Bun
RUN bun install

# Copy the full source code
COPY --from=builder /app/out/full/ .

# Runner stage (same as installer to avoid another copy)
FROM installer AS runner

# Set working directory to the app root
WORKDIR /app

# Set environment variables for Production and Puppeteer optimizations
ENV NODE_ENV=production
ENV PUPPETEER_SKIP_CHROMIUM_DOWNLOAD=true
ENV PUPPETEER_EXECUTABLE_PATH=/usr/bin/chromium

# Run the worker directly with Bun
CMD ["bun", "run", "packages/jobs/src/worker.ts"]
