import { withSentryConfig } from "@sentry/nextjs";

/** @type {import("next").NextConfig} */
const isWindows = process.platform === "win32";

const config = {
  output: isWindows ? undefined : "standalone",
  poweredByHeader: false,
  reactStrictMode: true,
  experimental: {
    serverActions: {
      bodySizeLimit: "50mb",
    },
    optimizePackageImports: [
      "lucide-react",
      "react-icons",
      "date-fns",
      "framer-motion",
      "recharts",
      "@dnd-kit/core",
      "@dnd-kit/sortable",
      "usehooks-ts",
    ],
  },
  images: {
    loader: "custom",
    loaderFile: "./image-loader.ts",
    qualities: [80, 100],
    remotePatterns: [
      {
        protocol: "https",
        hostname: "**",
      },
      {
        protocol: "http",
        hostname: "localhost",
      },
      {
        protocol: "http",
        hostname: "127.0.0.1",
      },
      {
        protocol: "http",
        hostname: "*.*.*.* ",
      },
    ],
  },
  transpilePackages: [
    "@orbis/ui",
    "@orbis/tailwind",
    "@orbis/api",
  ],
  serverExternalPackages: ["@react-pdf/renderer", "pino"],
  typescript: {
    ignoreBuildErrors: true,
  },
  devIndicators: false,
  async rewrites() {
    return [
      {
        source: "/files/proxy",
        destination: `${process.env.NEXT_PUBLIC_API_URL || "http://localhost:3002"}/files/proxy`,
      },
      {
        source: "/api/proxy",
        destination: `${process.env.NEXT_PUBLIC_API_URL || "http://localhost:3002"}/files/proxy`,
      },
    ];
  },
  async headers() {
    return [
      {
        source: "/((?!api/proxy).*)",
        headers: [
          {
            key: "X-Frame-Options",
            value: "DENY",
          },
        ],
      },
    ];
  },
};

// Only apply Sentry configuration in production
const isProduction = process.env.NODE_ENV === "production";

export default isProduction
  ? withSentryConfig(config, {
      org: process.env.SENTRY_ORG,
      project: process.env.SENTRY_PROJECT,
      authToken: process.env.SENTRY_AUTH_TOKEN,
      telemetry: false,

      // Only print logs for uploading source maps in CI
      silent: !process.env.CI,

      // Upload source maps for better stack traces
      widenClientFileUpload: true,
    })
  : config;
