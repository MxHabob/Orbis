import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import path from "node:path";
import { defineConfig, loadEnv } from "vite";

const host = process.env.TAURI_DEV_HOST;
const rootDir = __dirname;

export default defineConfig(({ mode }) => {
  // Load env file from the project root
  const env = loadEnv(mode, path.resolve(rootDir, "../.."), "");

  return {
    plugins: [tailwindcss(), react()],
    define: {
      "process.env": {
        NEXT_PUBLIC_API_URL: JSON.stringify(env.NEXT_PUBLIC_API_URL || "http://localhost:3002"),
        NEXT_PUBLIC_URL: JSON.stringify(env.NEXT_PUBLIC_URL || "http://localhost:3001"),
        NEXT_PUBLIC_GOOGLE_API_KEY: JSON.stringify(env.NEXT_PUBLIC_GOOGLE_API_KEY || ""),
        VERCEL_TARGET_ENV: JSON.stringify(env.VERCEL_TARGET_ENV || ""),
        VERCEL_URL: JSON.stringify(env.VERCEL_URL || ""),
      },
    },
    resolve: {
      alias: {
        "@": path.resolve(rootDir, "./src"),
        "public/appicon.png": path.resolve(rootDir, "./public/appicon.svg"),
      },
    },
    clearScreen: false,
    server: {
      port: 1420,
      strictPort: true,
      host: host || false,
      hmr: host
        ? {
            protocol: "ws",
            host,
            port: 1421,
          }
        : undefined,
      watch: {
        ignored: ["**/src-tauri/**", "!../../packages/ui/**"],
      },
    },
  };
});
