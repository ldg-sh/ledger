"use client";

let EDGE_URL = process.env.NEXT_PUBLIC_EDGE_URL || "http://localhost:8787";

export async function authenticatedFetch(
  endpoint: string,
  options: RequestInit = {},
) {
  const response = await fetch(`${EDGE_URL}${endpoint}`, {
    ...options,
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      ...options.headers,
    },
  });

  return response;
}