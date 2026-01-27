

import { auth } from "./index";
import { db } from "@midday/db/client";
import { getSessionTokenById } from "@midday/db/queries/sessions";

// Define a type that covers both Next.js ReadonlyHeaders and standard Headers
type RequestHeaders = Headers | { get: (name: string) => string | null } | Record<string, string>;

/**
 * Get the current session from better-auth in server-side context
 * Returns the session object with user and session data
 */
export async function getServerSession(headers?: RequestHeaders) {
  try {
    let headersList = headers;

    if (!headersList) {
      try {
        // @ts-ignore
        const { headers: nextHeaders } = await import("next/headers");
        headersList = await nextHeaders();
      } catch {
        // Not in Next.js context or next/headers not available
      }
    }

    if (!headersList) {
      return null;
    }

    // Create a new Headers object to ensure we can modify it
    const finalHeaders = new Headers();
    
    // Helper to safely get headers
    if (headersList instanceof Headers) {
      headersList.forEach((value, key) => {
        finalHeaders.set(key, value);
      });
    } else if (typeof (headersList as any).get === 'function') {
      // Handle Next.js ReadonlyHeaders or similar
      // We can't iterate easily, so we'll check common ones
      const auth = (headersList as any).get("authorization");
      if (auth) finalHeaders.set("authorization", auth);
      const cookie = (headersList as any).get("cookie");
      if (cookie) finalHeaders.set("cookie", cookie);
    } else {
      // Handle Record<string, string>
      for (const [key, value] of Object.entries(headersList)) {
        finalHeaders.set(key, value as string);
      }
    }

    // Support Bearer token by converting to cookie for better-auth compatibility if needed
    const authHeader = finalHeaders.get("authorization");
    if (authHeader?.startsWith("Bearer ")) {
      const token = authHeader.substring(7);
      const currentCookie = finalHeaders.get("cookie") || "";
      if (!currentCookie.includes("better-auth.session_token")) {
        finalHeaders.set("cookie", `${currentCookie}${currentCookie ? "; " : ""}better-auth.session_token=${token}`);
      }
    }

    const session = await auth.api.getSession({
      headers: finalHeaders,
    });

    return session;
  } catch (error) {
    return null;
  }
}

/**
 * Get the session token from the current session
 * This token can be used for server-to-server authentication
 * 
 * Since better-auth may not expose token directly in session object,
 * we get it from the database using the session ID
 */
export async function getSessionToken(headers?: RequestHeaders): Promise<string | null> {
  try {
    const session = await getServerSession(headers);
    
    if (!session?.session?.id) {
      return null;
    }

    // Get token from database using session ID
    return await getSessionTokenById(db, session.session.id);
  } catch (error) {
    return null;
  }
}
