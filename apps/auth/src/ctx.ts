/**
 * Application Context
 *
 * Single source of truth for all application dependencies.
 * Import `ctx` anywhere to access db, logger, config, etc.
 */

import { config } from "./config";
import { prisma, users, sessions } from "./db/mod";
import { log, loggers, createLogger, type Logger } from "./lib/logger";

export const ctx = {
    config,

    db: {
        /** Raw Prisma client for complex queries */
        prisma,
        users,
        sessions,
    },

    log,

    loggers,

    /** Create a child logger for a specific namespace */
    createLogger: (namespace: string): Logger => createLogger(namespace),
} as const;

export type AppContext = typeof ctx;
