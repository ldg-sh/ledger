import { PasskeyAuthCompleteRequest } from "../types/generated/PasskeyAuthCompleteRequest";
import { PasskeyCompleteRequest } from "../types/generated/PasskeyCompleteRequest";
import { PasskeyInitRequest } from "../types/generated/PasskeyInitRequest";

export async function beginRegistration(
  username: string,
  email: string,
  user_id: string | null,
) {
  let request: PasskeyInitRequest = {
    username,
    existing_id: user_id,
    email
  };

  const res = await fetch(`/auth/passkey/register/init`, {
    body: JSON.stringify(request),
    headers: {
      "Content-Type": "application/json",
    },
    method: "POST",
  });

  return res;
}

export async function completeRegistration(user_id: string, username: string, email: string, avatar_url: string, data: any) {
  let request: PasskeyCompleteRequest = {
    user_id,
    username,
    email,
    avatar_url,
    data,
  };

  const res = await fetch(`/auth/passkey/register/complete`, {
    body: JSON.stringify(request),
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
    method: "POST",
  });

  return res;
}

export async function beginAuthentication() {
  const res = await fetch(`/auth/passkey/auth/init`, {
    method: "POST",
  });

  return res;
}

export async function completeAuthentication(ticket: string, data: any) {
  let request: PasskeyAuthCompleteRequest = {
    ticket,
    data,
  };
  
  const res = await fetch(`/auth/passkey/auth/complete`, {
    body: JSON.stringify(request),
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
    method: "POST",
  });

  return res;
}

export async function decode(input: string) {
  const res = await fetch(`/auth/decode`, {
    body: input,
    method: "POST",
  });

  return await res.text();
}
