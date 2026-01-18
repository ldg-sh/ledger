import { config } from "../config";
import { loggers } from "./logger";

const log = loggers.cache;

interface InvalidateSessionRequest {
    userId: string;
    sessionToken?: string;
}

/**
 * Invalidate cached sessions in the backend service.
 *
 * @param userId - The user ID whose sessions should be invalidated
 * @param sessionToken - Optional specific session token to invalidate. If not provided, all sessions for the user are invalidated.
 */
export async function invalidateSession(userId: string, sessionToken?: string): Promise<void> {
    const url = `${config.backendInternalUrl}/internal/invalidate-session`;

    const body: InvalidateSessionRequest = { userId };
    if (sessionToken) {
        body.sessionToken = sessionToken;
    }

    try {
        const response = await fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "X-Server-Secret": config.serverKey,
            },
            body: JSON.stringify(body),
        });

        if (!response.ok) {
            const text = await response.text();
            log.error("Failed to invalidate session cache", {
                userId,
                hasToken: !!sessionToken,
                status: response.status,
                response: text,
            });
            return;
        }

        log.info("Session cache invalidated", {
            userId,
            scope: sessionToken ? "single" : "all",
        });
    } catch (error) {
        log.error("Error calling cache invalidation endpoint", {
            userId,
            hasToken: !!sessionToken,
            error: error instanceof Error ? error.message : String(error),
        });
    }
}

/**
 * Invalidate all cached sessions for a user.
 */
export async function invalidateUserSessions(userId: string): Promise<void> {
    return invalidateSession(userId);
}
