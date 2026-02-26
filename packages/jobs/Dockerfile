# Start from a clean Debian image for maximum stability
FROM debian:bookworm-slim AS base

# Set shell
SHELL ["/bin/bash", "-c"]

# Install basic tools and Bun
RUN apt-get update && apt-get install -y \
    curl \
    unzip \
    ca-certificates \
    && curl -fsSL https://bun.sh/install | bash \
    && ln -s /root/.bun/bin/bun /usr/local/bin/bun \
    && rm -rf /var/lib/apt/lists/*

# Add Bun to PATH
ENV PATH="/root/.bun/bin:${PATH}"

# Install Chromium and Arabic Fonts
# Using --no-install-recommends and minimal libs to avoid Exit 100
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

# Final stage
FROM base AS runner
WORKDIR /app
COPY --from=builder /app/out/full/ .

# Environment
ENV NODE_ENV=production
ENV PUPPETEER_SKIP_CHROMIUM_DOWNLOAD=true
ENV PUPPETEER_EXECUTABLE_PATH=/usr/bin/chromium

# Run worker
CMD ["bun", "run", "packages/jobs/src/worker.ts"]
