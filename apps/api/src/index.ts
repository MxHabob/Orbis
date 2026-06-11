import { trpcServer } from "@hono/trpc-server";
import { OpenAPIHono } from "@hono/zod-openapi";
import { Scalar } from "@scalar/hono-api-reference";
import { cors } from "hono/cors";
import { secureHeaders } from "hono/secure-headers";
import { createTRPCContext } from "./trpc/init";
import { appRouter } from "./trpc/routers/_app";
import { httpLogger } from "./utils/logger";
import { auth } from "@orbis/auth";
import type { HonoEnv } from "./utils/auth";
import { handleUpload } from "./routes/upload";
import { handleFileProxy } from "./routes/files";

const app = new OpenAPIHono<HonoEnv>();

app.use(httpLogger());

app.use(
  "*",
  async (c, next) => {
    if (c.req.path.includes("/files/proxy")) {
      return next();
    }
    return secureHeaders({
      crossOriginResourcePolicy: "cross-origin",
    })(c, next);
  },
);

app.use(
  "*",
  cors({
    origin: (origin) => {
      // Always allow configured origins
      const allowedOrigins = [
        "http://localhost:3001",
        ...(process.env.ALLOWED_API_ORIGINS?.split(",") || []),
        ...(process.env.NEXT_PUBLIC_URL ? [process.env.NEXT_PUBLIC_URL] : []),
      ];

      if (!origin || allowedOrigins.includes(origin)) {
        return origin || "*";
      }

      // Allow any local network origin (school LAN)
      try {
        const url = new URL(origin);
        const hostname = url.hostname;
        const isLocalNetwork =
          hostname === "localhost" ||
          hostname === "127.0.0.1" ||
          hostname.startsWith("192.168.") ||
          hostname.startsWith("10.") ||
          /^172\.(1[6-9]|2\d|3[01])\./.test(hostname);

        if (isLocalNetwork) {
          return origin;
        }
      } catch {}

      return allowedOrigins[0] || "*";
    },
    allowMethods: ["POST", "GET", "OPTIONS"],
    allowHeaders: [
      "Authorization",
      "Content-Type",
      "User-Agent",
      "accept-language",
      "x-trpc-source",
      "x-user-locale",
      "x-user-timezone",
      "x-user-country",
      "x-force-primary",
      "x-school-id",
      "x-better-auth-session",
      "x-better-auth-id",
    ],
    exposeHeaders: [
      "Content-Length",
      "Content-Type",
      "Cache-Control",
      "Cross-Origin-Resource-Policy",
      "Set-Cookie",
    ],
    credentials: true,
    maxAge: 86400,
  }),
);

app.use("*", async (c, next) => {
  let session = await auth.api.getSession({
    headers: c.req.raw.headers,
  });

  // Fallback for background jobs/Puppeteer using Authorization header or custom header
  if (!session) {
    const authHeader = c.req.header("Authorization");
    const sessionToken = authHeader?.startsWith("Bearer ") 
      ? authHeader.substring(7) 
      : c.req.header("x-better-auth-session");

    if (sessionToken) {
      const { getSession: getDbSession } = await import("@orbis/db/queries");
      const dbResult = await getDbSession(sessionToken);
      if (dbResult) {
        session = {
          user: dbResult.user,
          session: dbResult.session,
        };
      }
    }
  }

  if (!session) {
    c.set("user", null);
    c.set("session", null);
    await next();
    return;
  }

  c.set("user", session.user);
  c.set("session", session.session);
  await next();
});

// app.get("/realtime", (c) => realtimeHandler(c));

app.use(
  "/trpc/*",
  trpcServer({
    router: appRouter,
    createContext: createTRPCContext,
  }),
);

app.on(["POST", "GET"], "/api/auth/*", (c) => {
  return auth.handler(c.req.raw);
});

app.get("/health", (c) => {
  return c.json({ status: "ok" }, 200);
});

app.post("/api/upload", (c) => handleUpload(c));
app.get("/files/proxy", (c) => handleFileProxy(c));

app.get("/s/:code", async (c) => {
  const code = c.req.param("code");
  const { db } = await import("@orbis/db/client");
  const { shortLinks } = await import("@orbis/db/schema");
  const { eq, and, gt } = await import("drizzle-orm");

  const [link] = await db
    .select()
    .from(shortLinks)
    .where(
      and(
        eq(shortLinks.code, code),
        gt(shortLinks.expiresAt, new Date()),
      ),
    );

  if (!link) {
    return c.redirect(`${process.env.NEXT_PUBLIC_URL}/404`);
  }

  // If the link points to our internal vault, generate a public signed URL
  if (link.url.includes("/files/proxy?filePath=")) {
    try {
      const url = new URL(link.url);
      const filePath = url.searchParams.get("filePath");

      if (filePath?.startsWith("vault/")) {
        const key = filePath.replace("vault/", "");
        const { getDownloadUrl } = await import("@orbis/storage");

        const signedUrl = await getDownloadUrl({
          bucket: "vault",
          key,
          expiresIn: 3600, // Link itself expires based on short_links table, but S3 gets 1h
        });

        return c.redirect(signedUrl);
      }
    } catch (e) {
      console.error("Failed to parse link URL", e);
    }
  }

  return c.redirect(link.url);
});

app.doc("/openapi", {
  openapi: "3.1.0",
  info: {
    version: "0.0.1",
    title: "Orbis School API",
    description: "School Management System API Platform",
    contact: {
      name: "Orbis School Support",
      email: "engineer@orbisschool.com",
      url: "https://orbisschool.com",
    },
    license: {
      name: "Proprietary",
      url: "https://orbisschool.com/terms",
    },
  },
  servers: [
    {
      url: process.env.API_URL || "http://localhost:3000",
      description: "API Server",
    },
  ],
  security: [{ token: [] }],
});

// Register security scheme
app.openAPIRegistry.registerComponent("securitySchemes", "token", {
  type: "http",
  scheme: "bearer",
  description: "Session Token",
});

app.get(
  "/",
  Scalar({ url: "/openapi", pageTitle: "Orbis School API Docs", theme: "saturn" }),
);

// Automatic Database Migrations and Seeding on Startup
async function initializeDatabase() {
  if (process.env.RUN_MIGRATIONS_ON_STARTUP === "true") {
    try {
      const { db } = await import("@orbis/db/client");
      const { migrate } = await import("drizzle-orm/node-postgres/migrator");
      const { sql } = await import("drizzle-orm");
      const path = await import("path");
      const fs = await import("fs");

      let migrationsFolder = "";
      const pathsToTry = [
        path.resolve(process.cwd(), "packages/db/migrations"),
        path.resolve(process.cwd(), "../../packages/db/migrations"),
      ];

      try {
        const fileDir = path.dirname(new URL(import.meta.url).pathname);
        // On Windows, URL pathname might look like /C:/path..., we strip leading slash
        const cleanFileDir = process.platform === "win32" && fileDir.startsWith("/") ? fileDir.substring(1) : fileDir;
        pathsToTry.push(path.resolve(cleanFileDir, "../../../packages/db/migrations"));
      } catch {}

      for (const p of pathsToTry) {
        if (fs.existsSync(p)) {
          migrationsFolder = p;
          break;
        }
      }

      if (!migrationsFolder) {
        console.error("❌ Could not find database migrations folder in any of the search paths:", pathsToTry);
        return;
      }

      let retries = 5;
      while (retries > 0) {
        try {
          console.log(`⏳ Connecting to database to apply migrations from: ${migrationsFolder}...`);
          await migrate(db, { migrationsFolder });
          console.log("✅ Database migrations applied successfully!");

          // Check if database needs seeding
          const { academicYears } = await import("@orbis/db/schema");
          const result = await db.select({ count: sql`count(*)` }).from(academicYears);
          const count = Number(result[0]?.count || 0);

          if (count === 0) {
            console.log("🌱 Database is empty. Running initial seed data...");
            process.env.BYPASS_SEED_PROD_CHECK = "true";
            const { seedDatabase } = await import("@orbis/db/seed");
            await seedDatabase();
            console.log("✅ Initial seed data applied successfully!");
          } else {
            console.log("ℹ️ Database already contains data. Skipping initial seeding.");
          }
          break;
        } catch (err) {
          retries--;
          console.error(`❌ Migration/Seed attempt failed. Retries left: ${retries}`, err);
          if (retries === 0) {
            console.error("🚨 Failed to initialize database after all retries. Continuing startup...");
          } else {
            await new Promise((resolve) => setTimeout(resolve, 3000));
          }
        }
      }
    } catch (e) {
      console.error("🚨 Failed to setup automatic migrations/seeding:", e);
    }
  }
}

// Run async initialization
initializeDatabase();

export default {
  port: process.env.PORT ? Number.parseInt(process.env.PORT) : 3000,
  fetch: app.fetch,
  host: "0.0.0.0",
  idleTimeout: 60,
};
