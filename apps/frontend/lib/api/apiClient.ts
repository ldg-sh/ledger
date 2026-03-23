import { cookies } from "next/headers";

const API_URL = process.env.API_URL || "http://localhost:8080";

export async function authenticatedFetch(
  endpoint: string,
  options: RequestInit = {},
) {
  const cookieStore = await cookies();

  const headers = {
    ...options.headers,
    "Content-Type": "application/json",
    Cookie: cookieStore.get("session")
      ? `session=${cookieStore.get("session")?.value}`
      : "",
  };

  const response = await fetch(`${API_URL}${endpoint}`, {
    ...options,
    headers,
  });

  if (response.status === 401) {
    // TODO
  }

  return response;
}

export async function authenticatedMultipartFetch(
  endpoint: string,
  formData: FormData,
  options: RequestInit = {},
) {
  const cookieStore = await cookies();

  const headers = {
    Cookie: cookieStore.get("session")
      ? `session=${cookieStore.get("session")?.value}`
      : "",
    ...options.headers,
  };

  const response = await fetch(`${API_URL}${endpoint}`, {
    ...options,
    method: "POST",
    headers,
    body: formData,
  });

  if (response.status === 401) {
    // TODO
  }

  return response;
}
