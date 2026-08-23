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
      minify: "esbuild",
      rollupOptions: {
        output: {
          // Split heavy vendor libs into separate chunks for better caching
          manualChunks(id: string) {
            if (id.includes("node_modules")) {
              if (id.includes("@lucide/svelte")) return "vendor-icons";
              if (id.includes("@tauri-apps")) return "vendor-tauri";
              if (id.includes("@tanstack")) return "vendor-query";
              if (id.includes("svelte-sonner") || id.includes("zod"))
                return "vendor-utils";
            }
          },
        },
      },
      chunkSizeWarningLimit: 350,
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
