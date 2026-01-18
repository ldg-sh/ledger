/**
 * Application error types with automatic HTTP status codes.
 */

export type AppErrorKind =
    | { type: "ALREADY_EXISTS" }
    | { type: "NOT_FOUND" }
    | { type: "CONFLICT"; message: string }
    | { type: "VALIDATION"; message: string }
    | { type: "BAD_REQUEST"; message: string }
    | { type: "UNAUTHORIZED" }
    | { type: "FORBIDDEN" }
    | { type: "INTERNAL"; message: string };

export class AppError extends Error {
    constructor(public readonly kind: AppErrorKind) {
        super(AppError.getMessage(kind));
        this.name = "AppError";
    }

    get code(): string {
        return this.kind.type;
    }

    get status(): number {
        switch (this.kind.type) {
            case "ALREADY_EXISTS":
            case "CONFLICT":
                return 409;
            case "NOT_FOUND":
                return 404;
            case "VALIDATION":
            case "BAD_REQUEST":
                return 400;
            case "UNAUTHORIZED":
                return 401;
            case "FORBIDDEN":
                return 403;
            case "INTERNAL":
                return 500;
        }
    }

    toJSON() {
        return { error: this.code, message: this.message };
    }

    toResponse(): Response {
        return Response.json(this.toJSON(), { status: this.status });
    }

    private static getMessage(kind: AppErrorKind): string {
        switch (kind.type) {
            case "ALREADY_EXISTS":
                return "already exists";
            case "NOT_FOUND":
                return "not found";
            case "CONFLICT":
                return `conflict: ${kind.message}`;
            case "VALIDATION":
                return `validation error: ${kind.message}`;
            case "BAD_REQUEST":
                return `bad request: ${kind.message}`;
            case "UNAUTHORIZED":
                return "unauthorized";
            case "FORBIDDEN":
                return "forbidden";
            case "INTERNAL":
                return `internal error: ${kind.message}`;
        }
    }

    static alreadyExists = () => new AppError({ type: "ALREADY_EXISTS" });
    static notFound = () => new AppError({ type: "NOT_FOUND" });
    static conflict = (message: string) => new AppError({ type: "CONFLICT", message });
    static validation = (message: string) => new AppError({ type: "VALIDATION", message });
    static badRequest = (message: string) => new AppError({ type: "BAD_REQUEST", message });
    static unauthorized = () => new AppError({ type: "UNAUTHORIZED" });
    static forbidden = () => new AppError({ type: "FORBIDDEN" });
    static internal = (message: string) => new AppError({ type: "INTERNAL", message });
}
