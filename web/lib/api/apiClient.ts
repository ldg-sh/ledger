"use client";

export async function authenticatedFetch(
  endpoint: string,
  options: RequestInit = {},
) {
  const response = await fetch(`/api${endpoint}`, {
    ...options,
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      ...options.headers,
    },
  });

  return response;
}