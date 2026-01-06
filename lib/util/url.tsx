export function extractPathFromUrl(url: string): string {
    try {
        const parsedUrl = new URL(url);
        const pathName = parsedUrl.pathname.endsWith('/')
            ? parsedUrl.pathname.slice(0, -1)
            : parsedUrl.pathname;

        if (pathName.startsWith('/dashboard')) {
            return pathName.slice('/dashboard'.length) || '/';
        }

        return pathName;
    } catch (error) {
        console.error("Invalid URL:", url);
        return "";
    }
}