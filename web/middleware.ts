import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

export function middleware(request: NextRequest) {
  const refreshToken = request.cookies.get("refresh_token");
  const { pathname } = request.nextUrl;

  const validSession = (refreshToken && refreshToken.value != "");

  if (!validSession && !pathname.startsWith("/login") && !pathname.startsWith("/signup")) {
    const response = NextResponse.redirect(new URL("/login", request.url));
    response.cookies.set("session_exists", "false", { path: "/" });
    return response;
  }

  if (validSession && pathname.startsWith("/login")) {
    return NextResponse.redirect(new URL("/", request.url));
  }

  const response = NextResponse.next();

  if (validSession) {
    response.cookies.set("session_exists", "true", {
      path: "/",
      httpOnly: false,
      sameSite: "lax",
    });
  } else {
    response.cookies.set("session_exists", "false", { path: "/" });
  }

  return response;
}

export const config = {
  matcher: [
    '/', 
    '/((?!api|signup|auth|about|callback|_next/static|_next/image|favicon.ico|sw.js|mitm.js).*)'
  ],
};