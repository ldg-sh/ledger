import { Resend } from "resend";
import { config } from "../config";
import { loggers } from "./logger";

const resend = new Resend(config.resendApiKey);
const log = loggers.mail;

export async function sendEmail(to: string, subject: string, html: string) {
    const { error } = await resend.emails.send({
        from: config.emailFrom,
        to,
        subject,
        html,
    });

    if (error) {
        log.error("Failed to send email", { to, subject, error: error.message });
        throw new Error(`Failed to send email: ${error.message}`);
    }

    log.debug("Email sent", { to, subject });
}

export async function sendVerificationEmail(email: string, url: string) {
    await sendEmail(
        email,
        "Verify your Ledger account",
        `
        <h2>Welcome to Ledger!</h2>
        <p>Click the link below to verify your email address:</p>
        <p><a href="${url}">Verify Email</a></p>
        <p>Or copy this link: ${url}</p>
        `
    );
}

export async function sendPasswordResetEmail(email: string, url: string) {
    await sendEmail(
        email,
        "Reset your Ledger password",
        `
        <h2>Password Reset</h2>
        <p>Click the link below to reset your password:</p>
        <p><a href="${url}">Reset Password</a></p>
        <p>Or copy this link: ${url}</p>
        <p>If you didn't request this, you can ignore this email.</p>
        `
    );
}
