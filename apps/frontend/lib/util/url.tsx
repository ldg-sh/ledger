export function extractPathFromUrl(endPath: string): string {
  try {
    console.log("Extracting path from URL:", endPath);
    return endPath;
  } catch (error) {
    console.error("Invalid URL:", endPath);
    return "";
  }
}
