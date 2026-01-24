// Import Sentry instrumentation first, before any other modules
import "./instrument";

import { trpcServer } from "@hono/trpc-server";
import { OpenAPIHono } from "@hono/zod-openapi";
import { Scalar } from "@scalar/hono-api-reference";
import { cors } from "hono/cors";
import { secureHeaders } from "hono/secure-headers";
import { createTRPCContext } from "./trpc/init";
import { appRouter } from "./trpc/routers/_app";
import { httpLogger } from "./utils/logger";

const app = new OpenAPIHono();

app.use(httpLogger());
app.use(
  secureHeaders({
    crossOriginResourcePolicy: "cross-origin",
  }),
);

app.use(
  "*",
  cors({
    origin: process.env.ALLOWED_API_ORIGINS?.split(",") ?? [],
    allowMethods: ["GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH"],
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
      // School system specific headers
      "x-school-id",
    ],
    exposeHeaders: [
      "Content-Length",
      "Content-Type",
      "Cache-Control",
      "Cross-Origin-Resource-Policy",
    ],
    maxAge: 86400,
  }),
);

app.use(
  "/trpc/*",
  trpcServer({
    router: appRouter,
    createContext: createTRPCContext,
  }),
);

app.get("/health", (c) => {
  return c.json({ status: "ok" }, 200);
});

app.doc("/openapi", {
  openapi: "3.1.0",
  info: {
    version: "0.0.1",
    title: "Entity API",
    description: "School Management System API Platform",
    contact: {
      name: "Entity Support",
      email: "engineer@entity.com",
      url: "https://entity.com",
    },
    license: {
      name: "Proprietary",
      url: "https://entity.com/terms",
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
  Scalar({ url: "/openapi", pageTitle: "Entity API Docs", theme: "saturn" }),
);

export default {
  port: process.env.PORT ? Number.parseInt(process.env.PORT) : 3000,
  fetch: app.fetch,
  host: "::", // Listen on all interfaces
  idleTimeout: 60,
};
