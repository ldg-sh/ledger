import { Hono } from "hono";
import { logger as honoLogger } from "hono/logger";
import { cors } from "hono/cors";

import { ctx } from "./ctx";
import { createRouter } from "./routes/mod";
import { auth } from "./lib/auth";

const app = new Hono();
const log = ctx.loggers.http;

app.use("*", honoLogger());
app.use("*", cors({
    origin: ctx.config.trustedOrigins,
    credentials: true,
}));

app.use("/api/auth/*", async (c) => {
    try {
        return await auth.handler(c.req.raw);
    } catch (err) {
        ctx.loggers.auth.error("Auth error", { error: String(err) });
        return c.json({ error: "AUTH_ERROR", message: String(err) }, 500);
    }
});
app.route("/api", createRouter());

app.get("/", (c) => {
    return c.json({ message: "Email verified! You can close this tab." });
});

app.get("/test", async (c) => {
    const file = Bun.file("./test/test.html");
    return new Response(await file.text(), {
        headers: { "Content-Type": "text/html" },
    });
});

app.notFound((c) => {
    return c.json({ error: "NOT_FOUND", message: "not found" }, 404);
});

app.onError((err, c) => {
    log.error("Unhandled error", { error: err.message });
    return c.json({ error: "INTERNAL", message: "internal server error" }, 500);
});

const server = Bun.serve({
    port: ctx.config.port,
    fetch: app.fetch,
});

log.info("Ledger Auth server started", { port: server.port, url: `http://localhost:${server.port}` });
