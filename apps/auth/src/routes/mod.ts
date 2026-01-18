import { Hono } from "hono";
import health from "./health";
import validate from "./validate";

export type RouteModule = {
    prefix: string;
    router: Hono;
};

const modules: RouteModule[] = [
    health,
    validate,
];

export function createRouter(): Hono {
    const app = new Hono();

    for (const mod of modules) {
        app.route(mod.prefix, mod.router);
    }

    return app;
}
