import { paraglideVitePlugin } from "@inlang/paraglide-js";
import tailwindcss from "@tailwindcss/vite";
import adapter from "@sveltejs/adapter-static";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig, loadEnv } from "vite";

const host = process.env.TAURI_DEV_HOST;
const rootDir = import.meta.dirname;


export default defineConfig(({ mode }) => {
  return {
    plugins: [
      tailwindcss(),
      sveltekit({
        compilerOptions: {
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
    // ── Build Optimization ─────────────────────────────────────────────
    build: {
      target: "esnext",
      minify: true,
      manifest: true,
      rolldownOptions: {
        output: {
          codeSplitting: {
            groups: [
              {
                name: "vendor-icons",
                test: (id: string) => id.includes("node_modules") && id.includes("@orbis/icons"),
                priority: 10,
              },
              {
                name: "vendor-tauri",
                test: (id: string) => id.includes("node_modules") && id.includes("@tauri-apps"),
                priority: 10,
              },
              {
                name: "vendor-query",
                test: (id: string) => id.includes("node_modules") && id.includes("@tanstack"),
                priority: 10,
              },
              {
                name: "vendor-utils",
                test: (id: string) => id.includes("node_modules") && id.includes("zod"),
                priority: 10,
              },
              {
                name: "settings",
                test: (id: string) => /[\\/]features[\\/]settings[\\/]/.test(id),
                entriesAware: true,
                maxSize: 300 * 1024,
              },
            ],
          },
        },
      },
      // XLSX is lazy-loaded and compresses to ~142 kB; route chunks remain below 350 kB.
      chunkSizeWarningLimit: 450,
    },
    // Tauri expects a fixed port; fail if that port is not available
    server: {
      port: 1520,
      strictPort: true,
      shortcuts: false,
      host: host || undefined,
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
