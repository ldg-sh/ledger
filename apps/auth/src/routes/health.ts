import { Hono } from "hono";
import { handle } from "../lib/handler";
import { Api } from "../types/mod";

const router = new Hono();

router.get("/", handle(async () => {
    return Api.ok({
        status: "healthy",
        timestamp: new Date().toISOString(),
    });
}));

export default { prefix: "/health", router };
