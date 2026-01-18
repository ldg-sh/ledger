export interface AppConfig {
    port: number;
    isDev: boolean;
    authUrl: string;
    authSecret: string;
    serverKey: string;
    trustedOrigins: string[];
    resendApiKey: string;
    emailFrom: string;
    googleConfig: GoogleConfig;
    backendInternalUrl: string;
}

export interface GoogleConfig {
    clientId: string;
    clientSecret: string;
}

// Placeholder interfaces for future OAuth providers
// Uncomment and configure when adding new providers
// export interface TwitterConfig {
//     clientId: string;
//     clientSecret: string;
// }

// export interface AppleConfig {
//     clientId: string;
//     clientSecret: string;
// }

// Placeholder interface for future payment integration
// export interface PaymentConfig {
//     // Add payment provider fields here when needed
// }
