"use client";

import { handleRefresh } from "./refresh";

let EDGE_URL = process.env.NEXT_PUBLIC_EDGE_URL || "http://localhost:8787";

export async function authenticatedFetch(
  endpoint: string,
  options: RequestInit = {},
) {
  const request = new Request(`${EDGE_URL}${endpoint}`, {
    ...options,
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      ...options.headers,
    },
  });

  const response = await fetch(request.clone());

  if (response.status === 401) {
    return await handleRefresh(request);
  }

  return response;
}
