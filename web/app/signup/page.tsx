"use client";

import TextInput from "@/components/browser/popups/TextInput";
import styles from "./page.module.scss";
import { useUser } from "@/context/UserContext";
import { useState } from "react";
import { beginRegistration, completeRegistration } from "@/lib/api/passkey";
import { PasskeyInitResponse } from "@/lib/types/generated/PasskeyInitResponse";
import LoginButton from "@/components/login/LoginButton";
import { useRouter } from "next/navigation";
import Link from "next/link";

export default function SignupPage() {
  const [isLoading, setIsLoading] = useState(false);
  const router = useRouter();

  const [email, setEmail] = useState("");
  const [username, setUsername] = useState("");
  const [avatarUrl, setAvatarUrl] = useState("");

  const [emailError, setEmailError] = useState("");
  const [usernameError, setUsernameError] = useState("");
  const [avatarUrlError, setAvatarUrlError] = useState("");

  const user = useUser();

  if (user.user) {
    router.push("/");
    return null;
  }

  const validateEmail = (email: string) => {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email) && email.length <= 254;
  };

  const validateUsername = (username: string) => {
    return username.length >= 3 && username.length <= 16;
  };

  const validateAvatarUrl = (url: string) => {
    if (url.length == 0) {
      return true;
    }

    try {
      new URL(url);
      return true;
    } catch (e) {
      return false;
    }
  };

  return (
    <div className={styles.loginPage}>
      <div className={styles.content}>
        <div className={styles.top}>
          <h1 className={styles.title}>Welcome to Ledger</h1>
          <p className={styles.subtitle}>
            More information is required to sign up with a passkey.
          </p>
        </div>
        <div className={styles.signupForm}>
          <TextInput
            onChange={(email) => {
              setEmail(email);
            }}
            formType="email"
            onSubmit={() => {}}
            placeholder="sam@example.com"
            hint={
              emailError ? (
                <p className={styles.error}>{emailError}</p>
              ) : (
                "A unique email address to associate with your account."
              )
            }
            title="Email"
            disabled={isLoading}
          />
          <TextInput
            onChange={(username) => {
              setUsername(username);
            }}
            onSubmit={() => {}}
            placeholder="Sam Gordon"
            hint={
              usernameError ? (
                <p className={styles.error}>{usernameError}</p>
              ) : (
                "A non-unique username to identify you, between 3 and 16 characters."
              )
            }
            title="Username"
            disabled={isLoading}
          />
          <TextInput
            onChange={(avatarUrl) => {
              setAvatarUrl(avatarUrl);
            }}
            onSubmit={() => {}}
            placeholder="https://example.com/avatar.jpg"
            hint={
              avatarUrlError ? (
                <p className={styles.error}>{avatarUrlError}</p>
              ) : (
                "Optional. A URL to your avatar image."
              )
            }
            title="Avatar URL"
            disabled={isLoading}
          />
        </div>
        <div className={styles.buttons}>
          <LoginButton
            procedure={async () => {
              if (!validateEmail(email)) {
                setEmailError("Please enter a valid email address.");
              } else {
                setEmailError("");
              }

              if (!validateUsername(username)) {
                setUsernameError(
                  "Username must be between 3 and 16 characters long.",
                );
              } else {
                setUsernameError("");
              }

              if (!validateAvatarUrl(avatarUrl)) {
                setAvatarUrlError("Please enter a valid avatar URL.");
              } else {
                setAvatarUrlError("");
              }

              if (
                !validateEmail(email) ||
                !validateUsername(username) ||
                !validateAvatarUrl(avatarUrl)
              ) {
                return;
              }

              setIsLoading(true);
              let existing_id = null;

              let res = await beginRegistration(username, email, existing_id);

              if (!res.ok) {
                if (res.status === 409) {
                  setEmailError("An account with this email already exists.");
                }

                setIsLoading(false);
                return;
              }

              let object = (await res.json()) as PasskeyInitResponse;
              let creds = object.response as CredentialCreationOptions;
              let user_id = object.user_id;

              if (creds.publicKey == undefined) {
                return;
              }

              if (typeof creds.publicKey.challenge === "string") {
                creds.publicKey.challenge = Buffer.from(
                  creds.publicKey.challenge,
                  "base64",
                );
              }

              if (typeof creds.publicKey.user.id === "string") {
                creds.publicKey.user.id = Buffer.from(
                  creds.publicKey.user.id,
                  "base64",
                );
              }

              creds.publicKey.excludeCredentials?.forEach(function (listItem) {
                listItem.id = Uint8Array.from(listItem.id as any, (c: string) =>
                  c.charCodeAt(0),
                );
              });

              let assertion: any = await window.navigator.credentials
                .create({
                  publicKey: creds.publicKey,
                })
                .catch((err) => {
                  console.error("Error creating credentials:", err);
                  setIsLoading(false);
                  return null;
                });

              if (assertion == null) {
                return;
              }

              let response = {
                id: assertion.id,
                rawId: Buffer.from(assertion.rawId).toString("base64"),
                type: assertion.type,
                response: {
                  attestationObject: Buffer.from(
                    (assertion.response as AuthenticatorAttestationResponse)
                      .attestationObject,
                  ).toString("base64"),
                  clientDataJSON: Buffer.from(
                    (assertion.response as AuthenticatorAttestationResponse)
                      .clientDataJSON,
                  ).toString("base64"),
                },
              };

              let finishRes = await completeRegistration(
                user_id,
                username,
                email,
                avatarUrl,
                response,
              );
              if (finishRes.ok) {
                document.dispatchEvent(new CustomEvent("reloadUser"));
              } else {
                alert("Passkey registration failed");
                setIsLoading(false);
              }
            }}
            title="Sign up with a Passkey"
            isLoading={isLoading}
            bold
            svg={
              <svg
                version="1.1"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 26.5625 25.0855"
                className={styles.logo}
              >
                <g>
                  <rect
                    height="25.0855"
                    opacity="0"
                    width="26.5625"
                    x="0"
                    y="0"
                  />
                  <path
                    d="M17.6783 15.1277C17.7761 16.537 18.5147 17.7709 19.6582 18.5242L19.6582 21.3169C19.6551 21.3171 19.6518 21.3171 19.6484 21.3171L6.55273 21.3171C5.50781 21.3171 4.88281 20.8289 4.88281 20.0183C4.88281 17.4988 8.03711 14.0222 13.0957 14.0222C14.8809 14.0222 16.4286 14.4535 17.6783 15.1277ZM17.0117 7.95777C17.0117 10.3992 15.1953 12.2742 13.1055 12.2742C11.0059 12.2742 9.19922 10.3992 9.19922 7.9773C9.19922 5.58472 11.0156 3.75855 13.1055 3.75855C15.1953 3.75855 17.0117 5.54566 17.0117 7.95777Z"
                    fill="var(--color-background)"
                  />
                  <path
                    d="M22.1191 11.698C20.3809 11.698 19.0039 13.0945 19.0039 14.8035C19.0039 16.1316 19.7852 17.2546 20.9863 17.7234L20.9863 22.5574C20.9863 22.6746 21.0449 22.7625 21.123 22.8601L21.9434 23.6804C22.041 23.7781 22.1777 23.7878 22.2852 23.6804L23.8379 22.1375C23.9355 22.03 23.9355 21.8933 23.8379 21.7957L22.8711 20.8289L24.209 19.5203C24.3066 19.4324 24.3066 19.2859 24.1895 19.1687L22.8809 17.8699C24.3848 17.2546 25.2246 16.1609 25.2246 14.8035C25.2246 13.0945 23.8379 11.698 22.1191 11.698ZM22.1094 12.9675C22.6367 12.9675 23.0566 13.3972 23.0566 13.9148C23.0566 14.4519 22.6367 14.8816 22.1094 14.8816C21.5918 14.8816 21.1523 14.4519 21.1523 13.9148C21.1523 13.3972 21.5723 12.9675 22.1094 12.9675Z"
                    fill="var(--color-background)"
                  />
                </g>
              </svg>
            }
          />
          <div className={styles.backText}>
            <Link href="/login" className={styles.backLink}>
              Already have an account? Log in.
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
}
