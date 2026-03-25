export function extractPathFromUrl(endPath: string): string {
  try {
    return endPath;
  } catch (error) {
    console.error("Invalid URL:", endPath);
    return "";
  }
}
