import { trpcServer } from "@hono/trpc-server";
import { OpenAPIHono } from "@hono/zod-openapi";
import { Scalar } from "@scalar/hono-api-reference";
import { cors } from "hono/cors";
import { secureHeaders } from "hono/secure-headers";
import { createTRPCContext } from "./trpc/init";
import { appRouter } from "./trpc/routers/_app";
import { httpLogger } from "./utils/logger";
import { auth } from "@midday/auth";
import type { HonoEnv } from "./utils/auth";
import { handleUpload } from "./routes/upload";

const app = new OpenAPIHono<HonoEnv>();

app.use(httpLogger());

app.use(
  secureHeaders({
    crossOriginResourcePolicy: "cross-origin",
  }),
);

app.use(
  "*",
  cors({
    origin: [
      "http://localhost:3001",
      ...(process.env.ALLOWED_API_ORIGINS?.split(",") || []),
      ...(process.env.NEXT_PUBLIC_URL ? [process.env.NEXT_PUBLIC_URL] : []),
    ],
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
  const session = await auth.api.getSession({
    headers: c.req.raw.headers,
  });

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

app.doc("/openapi", {
  openapi: "3.1.0",
  info: {
    version: "0.0.1",
    title: "Mommy School API",
    description: "School Management System API Platform",
    contact: {
      name: "Mommy School Support",
      email: "engineer@mommyschool.com",
      url: "https://mommyschool.com",
    },
    license: {
      name: "Proprietary",
      url: "https://mommyschool.com/terms",
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
  Scalar({ url: "/openapi", pageTitle: "Mommy School API Docs", theme: "saturn" }),
);

export default {
  port: process.env.PORT ? Number.parseInt(process.env.PORT) : 3000,
  fetch: app.fetch,
  host: "0.0.0.0",
  idleTimeout: 60,
};
