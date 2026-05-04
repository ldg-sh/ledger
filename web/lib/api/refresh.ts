const EDGE_URL = process.env.NEXT_PUBLIC_EDGE_URL || "http://localhost:8787";

export async function handleRefresh(originalRequest: Request) {
  const refreshRes = await fetch(`${EDGE_URL}/user/refresh`, {
    credentials: "include",
    method: "POST",
  });

  if (refreshRes.ok) {
    return fetch(originalRequest);
  } else {
    return refreshRes;
  }
}
