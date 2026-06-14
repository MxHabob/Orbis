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
  transpilePackages: ["@orbis/ui", "@orbis/tailwind", "@orbis/sdk"],
  serverExternalPackages: ["@react-pdf/renderer", "pino"],
  typescript: {
    ignoreBuildErrors: true,
  },
  devIndicators: false,

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

export default config;
