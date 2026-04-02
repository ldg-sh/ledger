"use client";

import LoginButton from "@/components/login/LoginButton";
import styles from "./page.module.scss";
import { useUser } from "@/context/UserContext";
import {
  beginAuthentication,
  beginRegistration,
  completeAuthentication,
  completeRegistration,
} from "@/lib/api/passkey";
import { PasskeyInitResponse } from "@/lib/types/generated/PasskeyInitResponse";
import { useState } from "react";

export default function LoginPage() {
  const GITHUB_AUTH_URL = process.env.NEXT_PUBLIC_GITHUB_URL || "";
  const GOOGLE_AUTH_URL = process.env.NEXT_PUBLIC_GOOGLE_URL || "";
  const [isLoading, setIsLoading] = useState(false);

  const user = useUser();

  if (user.user) {
    window.location.href = "/";
    return null;
  }

  return (
    <div className={styles.loginPage}>
      <div className={styles.content}>
        <div className={styles.top}>
          <h1 className={styles.title}>Welcome to Ledger</h1>
          <p className={styles.subtitle}>Please log in to view your files.</p>
        </div>
        <div className={styles.loginButtonContainer}>
          <LoginButton
            authUrl={GITHUB_AUTH_URL}
            title="Continue with GitHub"
            svg={
              <svg
                viewBox="0 0 48 48"
                version="1.1"
                id="Shopicons"
                xmlns="http://www.w3.org/2000/svg"
                x="0"
                y="0"
                className={styles.logo}
              >
                <g id="github_00000178918564504449926280000008731996709616696990_">
                  <path d="M0 .011h48v48H0v-48z" fill="none" />
                  <path
                    stroke="#24292e"
                    fill="#24292e"
                    d="M30 44.004v-10c0-.884-.197-1.722-.542-2.479.825-.167 1.65-.358 2.472-.601 2.527-.746 6.154-3.839 7.226-6.863 1.188-3.356 1.188-6.76 0-10.116l-.001-.001c-.213-.603-.537-1.211-.998-1.868.848-3.154.253-5.792.225-5.915l-.365-1.564-1.606.019c-.15.002-3.48.063-6.724 1.955a29.635 29.635 0 0 0-11.371 0c-3.243-1.892-6.573-1.953-6.724-1.955l-1.608-.019-.365 1.564c-.028.123-.623 2.761.225 5.915-.461.657-.785 1.266-.999 1.869-1.187 3.356-1.187 6.76.001 10.117 1.07 3.023 4.697 6.116 7.225 6.862.822.243 1.647.434 2.472.601A5.946 5.946 0 0 0 18 34.004v1.281c-.062.036-.127.065-.187.108-.289.211-2.869 1.967-5.505.09-.93-.946-1.386-1.639-1.826-2.309-.988-1.5-1.841-2.586-4.588-3.96a2 2 0 1 0-1.789 3.579c1.991.995 2.341 1.525 3.035 2.581.515.781 1.155 1.754 2.445 3.044l.215.186c1.692 1.27 3.447 1.723 5.053 1.723A9.286 9.286 0 0 0 18 39.76v4.253l12-.009z"
                  />
                </g>
              </svg>
            }
          />
          <LoginButton
            authUrl={GOOGLE_AUTH_URL}
            title="Continue with Google"
            svg={
              <svg
                viewBox="0 0 32 32"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                className={styles.logo}
              >
                <path
                  d="M30.0014 16.3109C30.0014 15.1598 29.9061 14.3198 29.6998 13.4487H16.2871V18.6442H24.1601C24.0014 19.9354 23.1442 21.8798 21.2394 23.1864L21.2127 23.3604L25.4536 26.58L25.7474 26.6087C28.4458 24.1665 30.0014 20.5731 30.0014 16.3109Z"
                  fill="#4285F4"
                />
                <path
                  d="M16.2863 29.9998C20.1434 29.9998 23.3814 28.7553 25.7466 26.6086L21.2386 23.1863C20.0323 24.0108 18.4132 24.5863 16.2863 24.5863C12.5086 24.5863 9.30225 22.1441 8.15929 18.7686L7.99176 18.7825L3.58208 22.127L3.52441 22.2841C5.87359 26.8574 10.699 29.9998 16.2863 29.9998Z"
                  fill="#34A853"
                />
                <path
                  d="M8.15964 18.769C7.85806 17.8979 7.68352 16.9645 7.68352 16.0001C7.68352 15.0356 7.85806 14.1023 8.14377 13.2312L8.13578 13.0456L3.67083 9.64746L3.52475 9.71556C2.55654 11.6134 2.00098 13.7445 2.00098 16.0001C2.00098 18.2556 2.55654 20.3867 3.52475 22.2845L8.15964 18.769Z"
                  fill="#FBBC05"
                />
                <path
                  d="M16.2864 7.4133C18.9689 7.4133 20.7784 8.54885 21.8102 9.4978L25.8419 5.64C23.3658 3.38445 20.1435 2 16.2864 2C10.699 2 5.8736 5.1422 3.52441 9.71549L8.14345 13.2311C9.30229 9.85555 12.5086 7.4133 16.2864 7.4133Z"
                  fill="#EB4335"
                />
              </svg>
            }
          />
          <div className={styles.divider}>
            <div className={styles.line} />
            <span className={styles.dividerText}>or</span>
            <div className={styles.line} />
          </div>
          <div className={styles.passkeyContainer}>
            <LoginButton
              procedure={async () => {
                setIsLoading(true);
                let username = "Testing";
                let existing_id = null;

                let res = await beginRegistration(username, existing_id);
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

                creds.publicKey.excludeCredentials?.forEach(
                  function (listItem) {
                    listItem.id = Uint8Array.from(
                      listItem.id as any,
                      (c: string) => c.charCodeAt(0),
                    );
                  },
                );

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

                let finishRes = await completeRegistration(user_id, response);
                if (finishRes.ok) {
                  document.dispatchEvent(new CustomEvent("reloadUser"));
                } else {
                  alert("Passkey registration failed");
                  setIsLoading(false);
                }
              }}
              title="Sign up with a Passkey"
              isLoading={isLoading}
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
                      fill="black"
                    />
                    <path
                      d="M22.1191 11.698C20.3809 11.698 19.0039 13.0945 19.0039 14.8035C19.0039 16.1316 19.7852 17.2546 20.9863 17.7234L20.9863 22.5574C20.9863 22.6746 21.0449 22.7625 21.123 22.8601L21.9434 23.6804C22.041 23.7781 22.1777 23.7878 22.2852 23.6804L23.8379 22.1375C23.9355 22.03 23.9355 21.8933 23.8379 21.7957L22.8711 20.8289L24.209 19.5203C24.3066 19.4324 24.3066 19.2859 24.1895 19.1687L22.8809 17.8699C24.3848 17.2546 25.2246 16.1609 25.2246 14.8035C25.2246 13.0945 23.8379 11.698 22.1191 11.698ZM22.1094 12.9675C22.6367 12.9675 23.0566 13.3972 23.0566 13.9148C23.0566 14.4519 22.6367 14.8816 22.1094 14.8816C21.5918 14.8816 21.1523 14.4519 21.1523 13.9148C21.1523 13.3972 21.5723 12.9675 22.1094 12.9675Z"
                      fill="black"
                    />
                  </g>
                </svg>
              }
            />
            <LoginButton
              procedure={async () => {
                setIsLoading(true);
                let res = await beginAuthentication();
                let data = await res.json();

                let creds = data.ccr as CredentialRequestOptions;
                let ticket = data.ticket as string;

                if (!creds.publicKey) {
                  console.error("No publicKey in credential request options");
                  return;
                }

                if (typeof creds.publicKey.challenge === "string") {
                  creds.publicKey.challenge = Uint8Array.from(
                    atob(
                      (creds.publicKey.challenge as any)
                        .replace(/-/g, "+")
                        .replace(/_/g, "/"),
                    ),
                    (c) => c.charCodeAt(0),
                  );
                }

                creds.mediation = "optional";

                let assertion = (await window.navigator.credentials
                  .get({
                    publicKey: creds.publicKey,
                    mediation:
                      creds.mediation as CredentialMediationRequirement,
                  })
                  .catch((err) => {
                    console.error("Error obtaining assertion:", err);
                    setIsLoading(false);
                    return null;
                  })) as PublicKeyCredential;

                if (!assertion) {
                  console.error("No assertion obtained from credentials.get");
                  setIsLoading(false);
                  return;
                }

                const authResponse =
                  assertion.response as AuthenticatorAssertionResponse;

                console.log("Assertion obtained:", assertion);

                let userHandleBase64 = authResponse.userHandle
                  ? Buffer.from(authResponse.userHandle).toString("base64")
                  : null;

                let response = {
                  id: assertion.id,
                  rawId: Buffer.from(assertion.rawId).toString("base64"),
                  type: assertion.type,
                  response: {
                    authenticatorData: Buffer.from(
                      authResponse.authenticatorData,
                    ).toString("base64"),
                    clientDataJSON: Buffer.from(
                      authResponse.clientDataJSON,
                    ).toString("base64"),
                    signature: Buffer.from(authResponse.signature).toString(
                      "base64",
                    ),
                    userHandle: userHandleBase64,
                  },
                };

                let result = await completeAuthentication(ticket, response);

                if (result.ok) {
                  document.dispatchEvent(new CustomEvent("reloadUser"));
                } else {
                  alert("Passkey authentication failed");
                }
                setIsLoading(false);
              }}
              title="Log in with a Passkey"
              isLoading={isLoading}
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
                      fill="black"
                    />
                    <path
                      d="M22.1191 11.698C20.3809 11.698 19.0039 13.0945 19.0039 14.8035C19.0039 16.1316 19.7852 17.2546 20.9863 17.7234L20.9863 22.5574C20.9863 22.6746 21.0449 22.7625 21.123 22.8601L21.9434 23.6804C22.041 23.7781 22.1777 23.7878 22.2852 23.6804L23.8379 22.1375C23.9355 22.03 23.9355 21.8933 23.8379 21.7957L22.8711 20.8289L24.209 19.5203C24.3066 19.4324 24.3066 19.2859 24.1895 19.1687L22.8809 17.8699C24.3848 17.2546 25.2246 16.1609 25.2246 14.8035C25.2246 13.0945 23.8379 11.698 22.1191 11.698ZM22.1094 12.9675C22.6367 12.9675 23.0566 13.3972 23.0566 13.9148C23.0566 14.4519 22.6367 14.8816 22.1094 14.8816C21.5918 14.8816 21.1523 14.4519 21.1523 13.9148C21.1523 13.3972 21.5723 12.9675 22.1094 12.9675Z"
                      fill="black"
                    />
                  </g>
                </svg>
              }
            />
          </div>
        </div>
      </div>
    </div>
  );
}
