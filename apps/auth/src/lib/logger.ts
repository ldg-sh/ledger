type LogLevel = "debug" | "info" | "warn" | "error";

interface LogEntry {
    level: LogLevel;
    message: string;
    data?: Record<string, unknown>;
    timestamp: string;
    caller?: string;
}

const LEVEL_PRIORITY: Record<LogLevel, number> = {
    debug: 0,
    info: 1,
    warn: 2,
    error: 3,
};

const LEVEL_COLORS: Record<LogLevel, string> = {
    debug: "\x1b[90m",
    info: "\x1b[36m",
    warn: "\x1b[33m",
    error: "\x1b[31m",
};

const LEVEL_ICONS: Record<LogLevel, string> = {
    debug: "·",
    info: "→",
    warn: "⚠",
    error: "✕",
};

const RESET = "\x1b[0m";
const DIM = "\x1b[90m";
const BOLD = "\x1b[1m";

export interface Logger {
    debug(message: string, data?: Record<string, unknown>): void;
    info(message: string, data?: Record<string, unknown>): void;
    warn(message: string, data?: Record<string, unknown>): void;
    error(message: string, data?: Record<string, unknown>): void;
    child(namespace: string): Logger;
}

function formatData(data: Record<string, unknown>): string {
    const parts: string[] = [];
    for (const [key, value] of Object.entries(data)) {
        if (value === undefined) continue;
        const formatted = typeof value === "object"
            ? JSON.stringify(value)
            : String(value);
        parts.push(`${DIM}${key}=${RESET}${formatted}`);
    }
    return parts.join(" ");
}

export function createLogger(namespace?: string, minLevel: LogLevel = "debug"): Logger {
    const log = (level: LogLevel, message: string, data?: Record<string, unknown>) => {
        if (LEVEL_PRIORITY[level] < LEVEL_PRIORITY[minLevel]) return;

        const timestamp = new Date().toISOString().slice(11, 23);
        const color = LEVEL_COLORS[level];
        const icon = LEVEL_ICONS[level];
        const ns = namespace ? `${DIM}[${namespace}]${RESET} ` : "";
        const dataStr = data ? ` ${formatData(data)}` : "";

        const output = `${DIM}${timestamp}${RESET} ${color}${icon}${RESET} ${ns}${message}${dataStr}`;

        if (level === "error") {
            console.error(output);
        } else if (level === "warn") {
            console.warn(output);
        } else {
            console.log(output);
        }
    };

    return {
        debug: (msg, data) => log("debug", msg, data),
        info: (msg, data) => log("info", msg, data),
        warn: (msg, data) => log("warn", msg, data),
        error: (msg, data) => log("error", msg, data),
        child: (childNamespace: string) => {
            const fullNamespace = namespace
                ? `${namespace}:${childNamespace}`
                : childNamespace;
            return createLogger(fullNamespace, minLevel);
        },
    };
}

// Root logger instance
export const log = createLogger();

// Pre-configured child loggers for common use cases
export const loggers = {
    http: createLogger("http"),
    db: createLogger("db"),
    auth: createLogger("auth"),
    mail: createLogger("mail"),
    cache: createLogger("cache"),
};
