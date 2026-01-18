import { Hono } from "hono";
import { handle } from "../lib/handler";
import { Api, AppError } from "../types/mod";
import { config } from "../config";
import { sessions } from "../db/mod";

const router = new Hono();

/**
 * Server-to-server endpoint for validating session tokens.
 *
 * Your Rust backend calls this with:
 * - Header `X-Session-Token`: The user's session token
 * - Header `X-Server-Secret`: Shared secret to authenticate the server
 *
 * Returns user info if valid, 401 if invalid.
 */
router.get("/session", handle(async (c) => {
    const serverSecret = c.req.header("X-Server-Secret");
    if (serverSecret !== config.serverKey) {
        throw AppError.unauthorized();
    }

    const sessionToken = c.req.header("X-Session-Token");
    if (!sessionToken) {
        throw AppError.badRequest("Missing X-Session-Token header");
    }

    const session = await sessions.findByTokenWithUser(sessionToken);

    if (!session || session.expiresAt < new Date()) {
        throw AppError.unauthorized();
    }

    const user = session.user;

    return Api.ok({
        id: user.id,
        email: user.email,
        name: user.name,
        isAdmin: user.isAdmin || false,
    });
}));

export default { prefix: "/validate", router };
