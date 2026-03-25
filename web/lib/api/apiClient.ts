"use client";

const API_URL = process.env.NEXT_PUBLIC_EDGE_URL || "http://localhost:8787";

export async function authenticatedFetch(
  endpoint: string,
  options: RequestInit = {},
) {
  const response = await fetch(`${API_URL}${endpoint}`, {
    ...options,
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      ...options.headers,
    },
  });

  return response;
}