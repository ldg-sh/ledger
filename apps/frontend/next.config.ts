/** @type {import('next').NextConfig} */
const nextConfig = {
  productionBrowserSourceMaps: false,
  experimental: {
    serverSourceMaps: false, 
    serverActions: {
      bodySizeLimit: '10mb',
    }
  },
  async rewrites() {
    return [
      {
        source: '/auth/:path*',
        destination: 'http://localhost:8080/auth/:path*', 
      },
    ];
  },
}

module.exports = nextConfig