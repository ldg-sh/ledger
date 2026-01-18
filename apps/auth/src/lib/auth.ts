import { betterAuth } from "better-auth";
import { prismaAdapter } from "better-auth/adapters/prisma";
import { config } from "../config";
import { prisma } from "../db/client";
import { sendVerificationEmail, sendPasswordResetEmail } from "./mail";
import { invalidateSession, invalidateUserSessions } from "./cache";

export const auth = betterAuth({
    database: prismaAdapter(prisma, {
        provider: "postgresql",
    }),

    user: {
        additionalFields: {
            isAdmin: {
                type: "boolean",
                defaultValue: false,
            },
        },
    },

    // No plugins payment integration can be added here later
    plugins: [],

    socialProviders: {
        google: {
            clientId: config.googleConfig.clientId,
            clientSecret: config.googleConfig.clientSecret,
        },
        // Future OAuth providers can be added here:
        // twitter: {
        //     clientId: config.twitterConfig.clientId,
        //     clientSecret: config.twitterConfig.clientSecret,
        // },
        // apple: {
        //     clientId: config.appleConfig.clientId,
        //     clientSecret: config.appleConfig.clientSecret,
        // },
    },

    emailAndPassword: {
        enabled: true,
        requireEmailVerification: true,
        sendResetPassword: async ({ user, url }) => {
            await sendPasswordResetEmail(user.email, url);
        },
    },

    emailVerification: {
        sendVerificationEmail: async ({ user, url }) => {
            await sendVerificationEmail(user.email, url);
        },
        sendOnSignUp: true,
    },

    session: {
        expiresIn: 60 * 60 * 24 * 7, // 7 days
        updateAge: 60 * 60 * 24, // 24 hours
        cookieCache: {
            enabled: true,
            maxAge: 60 * 5, // 5 minutes
        },
    },

    advanced: {
        cookiePrefix: "ledger",
        crossSubDomainCookies: {
            enabled: false,
        },
        defaultCookieAttributes: {
            sameSite: "none",
            secure: true,
            httpOnly: false, // Allow JS to read session token for cross-domain auth
        },
    },

    basePath: "/api/auth",
    baseURL: config.authUrl,
    secret: config.authSecret,

    trustedOrigins: config.trustedOrigins,

    databaseHooks: {
        session: {
            delete: {
                after: async (session) => {
                    // Invalidate specific session cache when user logs out
                    await invalidateSession(session.userId, session.token);
                },
            },
        },
        user: {
            update: {
                after: async (user) => {
                    // Invalidate all sessions when user is updated (password change, etc.)
                    // This ensures cached user data is refreshed
                    await invalidateUserSessions(user.id);
                },
            },
        },
    },
});

export type Auth = typeof auth;
