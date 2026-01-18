import type { Context } from "hono";
import { AppError, Api, toResponse, type ApiResponse } from "../types/mod";
import { ctx } from "../ctx";

export function handle<T>(
    fn: (c: Context) => Promise<ApiResponse<T>> | ApiResponse<T>
): (c: Context) => Promise<Response> {
    return async (c) => {
        try {
            const result = await fn(c);
            return toResponse(result);
        } catch (e) {
            if (e instanceof AppError) {
                return e.toResponse();
            }
            ctx.log.error("Unhandled error in handler", { error: String(e) });
            return AppError.internal("unexpected error").toResponse();
        }
    };
}
