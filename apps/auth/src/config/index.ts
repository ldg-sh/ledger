import type { AppConfig } from "./types";

type EnvVarType = string | number | boolean;

function from_env<T extends EnvVarType = string>(
    key: string,
    fallback?: T
): T {
    const value = process.env[key];
    if (value !== undefined) {
        if (typeof fallback === "number") return Number(value) as T;
        if (typeof fallback === "boolean") return (value === "true" || value === "1") as T;
        return value as T;
    }
    if (fallback !== undefined) {
        return fallback;
    }
    throw new Error(`Environment variable ${key} is required but was not provided.`);
}

function parseTrustedOrigins(): string[] {
    const envOrigins = process.env.TRUSTED_ORIGINS;
    if (envOrigins) {
        return envOrigins.split(",").map(o => o.trim());
    }
    // Default development origins
    return [
        "http://localhost:3000",
        "http://localhost:5173",
    ];
}

export const config: AppConfig = {
    port: from_env<number>("PORT", 3001),
    isDev: from_env<string>("NODE_ENV", "production") !== "production",
    authUrl: from_env<string>("BETTER_AUTH_URL", "http://localhost:3001"),
    authSecret: from_env<string>("BETTER_AUTH_SECRET"),
    serverKey: from_env<string>("SERVER_KEY"),
    trustedOrigins: parseTrustedOrigins(),
    resendApiKey: from_env<string>("RESEND_API_KEY"),
    emailFrom: from_env<string>("EMAIL_FROM", "noreply@ledger.app"),
    googleConfig: {
        clientId: from_env<string>("GOOGLE_CLIENT_ID"),
        clientSecret: from_env<string>("GOOGLE_CLIENT_SECRET"),
    },
    backendInternalUrl: from_env<string>("BACKEND_INTERNAL_URL"),
};

export type { AppConfig } from "./types";
