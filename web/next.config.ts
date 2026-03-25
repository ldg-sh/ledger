/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  productionBrowserSourceMaps: false,
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "avatars.githubusercontent.com",
        pathname: "**",
      },
      {
        protocol: "https",
        hostname: "*.googleusercontent.com",
        pathname: "**",
      },
    ],
  },
  experimental: {
    serverSourceMaps: false,
    serverActions: {
      bodySizeLimit: "10mb",
    },
  },
  async rewrites() {
    const isLocal = process.env.NODE_ENV === "development";
    const apiBase = isLocal ? "http://localhost:8080" : process.env.API_URL;

    const edgeBase = isLocal ? "http://localhost:8787" : process.env.EDGE_URL;

    return [
      {
        source: "/auth/:path*",
        destination: `${apiBase}/auth/:path*`,
      },
      {
        source: "/api/:path*",
        destination: `${edgeBase}/:path*`,
      },
    ];
  },
};

module.exports = nextConfig;
