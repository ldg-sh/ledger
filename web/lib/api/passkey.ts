import { PasskeyAuthCompleteRequest } from "../types/generated/PasskeyAuthCompleteRequest";
import { PasskeyCompleteRequest } from "../types/generated/PasskeyCompleteRequest";
import { PasskeyInitRequest } from "../types/generated/PasskeyInitRequest";
import { JsonValue } from "../types/generated/serde_json/JsonValue";

export async function beginRegistration(
  username: string,
  email: string,
  user_id: string | null,
) {
  const request: PasskeyInitRequest = {
    username,
    existing_id: user_id,
    email,
  };

  return await fetch(`/auth/passkey/register/init`, {
    body: JSON.stringify(request),
    headers: {
      "Content-Type": "application/json",
    },
    method: "POST",
  });
}

export async function completeRegistration(
  user_id: string,
  username: string,
  email: string,
  avatar_url: string,
  data: JsonValue,
) {
  const request: PasskeyCompleteRequest = {
    user_id,
    username,
    email,
    avatar_url,
    data,
  };

  return await fetch(`/auth/passkey/register/complete`, {
    body: JSON.stringify(request),
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
    method: "POST",
  });
}

export async function beginAuthentication() {
  return await fetch(`/auth/passkey/auth/init`, {
    method: "POST",
  });
}

export async function completeAuthentication(ticket: string, data: JsonValue) {
  const request: PasskeyAuthCompleteRequest = {
    ticket,
    data,
  };

  return await fetch(`/auth/passkey/auth/complete`, {
    body: JSON.stringify(request),
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
    method: "POST",
  });
}
