import { cookies } from "next/headers";

const API_URL = "http://localhost:8080";

export async function authenticatedFetch(endpoint: string, options: RequestInit = {}) {
  const cookieStore = await cookies();
  const token = cookieStore.get("auth_token")?.value;

  const headers = {
    ...options.headers,
    "Authorization": `Bearer ${token}`,
    "Content-Type": "application/json",
  };

  const response = await fetch(`${API_URL}${endpoint}`, { ...options, headers });
  
  if (response.status === 401) {
    // TODO
  }

  return response;
}

export async function authenticatedMultipartFetch(endpoint: string, formData: FormData, options: RequestInit = {}) {
  const cookieStore = await cookies();
  const token = cookieStore.get("auth_token")?.value;

  const headers = {
    ...options.headers,
    "Authorization": `Bearer ${token}`,
  };

  const response = await fetch(`${API_URL}${endpoint}`, { 
    ...options, 
    method: "POST",
    headers,
    body: formData 
  });
  
  if (response.status === 401) {
    // TODO
  }

  return response;
}