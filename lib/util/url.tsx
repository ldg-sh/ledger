export function extractPathFromUrl(endPath: string): string {
  try {
    if (endPath.startsWith("/dashboard")) {
      return endPath.slice("/dashboard".length) || ""
    }

    return endPath;
  } catch (error) {
    console.error("Invalid URL:", endPath);
    return "";
  }
}
