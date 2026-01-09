/** @type {import('next').NextConfig} */
const nextConfig = {
  productionBrowserSourceMaps: false,
  experimental: {
    serverSourceMaps: false, 
    serverActions: {
      bodySizeLimit: '10mb',
    }
  },
}

module.exports = nextConfig