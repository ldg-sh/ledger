export async function handleRefresh(originalRequest: Request) {
  const refreshRes = await fetch(`/auth/refresh`, {
    credentials: "include",
    method: "POST",
  });

  if (refreshRes.ok) {
    return fetch(originalRequest);
  } else {
    window.location.href = "/login";
    throw new Error("Session expired");
  }
}