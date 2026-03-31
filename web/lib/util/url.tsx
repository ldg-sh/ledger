export function extractPathFromUrl(endPath: string): string {
  try {
    let urlEncodedPath = decodeURIComponent(endPath);
    
    return urlEncodedPath;
  } catch (error) {
    console.error("Invalid URL:", endPath);
    return "";
  }
}
