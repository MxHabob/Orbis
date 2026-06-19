import { paraglideVitePlugin } from "@inlang/paraglide-js";
import tailwindcss from "@tailwindcss/vite";
import adapter from "@sveltejs/adapter-static";
import { sveltekit } from "@sveltejs/kit/vite";
import path from "node:path";
import { defineConfig, loadEnv } from "vite";

const host = process.env.TAURI_DEV_HOST;
const rootDir = import.meta.dirname;

export default defineConfig(({ mode }) => {
  // Load env file from the monorepo root
  const env = loadEnv(mode, path.resolve(rootDir, "../.."), "");

  return {
    plugins: [
      tailwindcss(),
      sveltekit({
        compilerOptions: {
          // Force runes mode for the project, except for libraries. Can be removed in svelte 6.
          runes: ({ filename }) =>
            filename.split(/[/\\]/).includes("node_modules") ? undefined : true,
        },
        adapter: adapter({
          fallback: "index.html",
        }),
      }),
      paraglideVitePlugin({
        project: "./project.inlang",
        outdir: "./src/lib/paraglide",
      }),
    ],
    define: {
      "process.env": {
        NEXT_PUBLIC_API_URL: JSON.stringify(
          env.NEXT_PUBLIC_API_URL || "http://localhost:3002",
        ),
        NEXT_PUBLIC_URL: JSON.stringify(
          env.NEXT_PUBLIC_URL || "http://localhost:3001",
        ),
      },
    },
    // Tauri expects a fixed port; fail if that port is not available
    server: {
      port: 1520,
      strictPort: true,
      host: host || false,
      hmr: host
        ? {
            protocol: "ws",
            host,
            port: 1521,
          }
        : undefined,
      watch: {
        ignored: ["**/src-tauri/**"],
      },
    },
  };
});
