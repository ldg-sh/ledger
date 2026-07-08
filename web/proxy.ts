import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

export function proxy(request: NextRequest) {
  const refreshToken = request.cookies.get("refresh_token");
  const { pathname } = request.nextUrl;
  const validSession = Boolean(refreshToken?.value);

  let response: NextResponse;

  if (!validSession && pathname === "/") {
    response = NextResponse.redirect(new URL("/login", request.url));
  } else if (validSession && pathname.startsWith("/login")) {
    response = NextResponse.redirect(new URL("/", request.url));
  } else {
    response = NextResponse.next();
  }

  response.cookies.set("session_exists", String(validSession), {
    path: "/",
    httpOnly: false,
    sameSite: "lax",
  });

  return response;
}

export const config = {
  matcher: [
    '/',
    '/((?!api|share|signup|download|auth|about|callback|_next/static|_next/image|favicon.ico|sw.js|mitm.html)(?!.*\\.png$).*)'
  ],
};
