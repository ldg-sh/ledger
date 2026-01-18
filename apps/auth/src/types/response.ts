import type { Result } from "./result";
import { AppError } from "./error";

type OkResponse<T> = { type: "ok"; data: T };
type CreatedResponse<T> = { type: "created"; data: T };
type NoContentResponse = { type: "no_content" };
type ErrResponse = { type: "error"; error: AppError };

export type ApiResponse<T = unknown> =
    | OkResponse<T>
    | CreatedResponse<T>
    | NoContentResponse
    | ErrResponse;

export const Api = {
    ok: <T>(data: T): ApiResponse<T> => ({ type: "ok", data }),
    created: <T>(data: T): ApiResponse<T> => ({ type: "created", data }),
    noContent: (): ApiResponse<never> => ({ type: "no_content" }),
    err: (error: AppError): ApiResponse<never> => ({ type: "error", error }),
};

export function toResponse<T>(res: ApiResponse<T>): Response {
    switch (res.type) {
        case "ok":
            return Response.json(res.data, { status: 200 });
        case "created":
            return Response.json(res.data, { status: 201 });
        case "no_content":
            return new Response(null, { status: 204 });
        case "error":
            return res.error.toResponse();
    }
}

export function fromResult<T>(result: Result<T, AppError>): ApiResponse<T> {
    return result.ok ? Api.ok(result.value) : Api.err(result.error);
}

export type ApiResult<T> = ApiResponse<T>;
