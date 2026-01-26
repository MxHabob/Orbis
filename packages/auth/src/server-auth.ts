import "server-only";

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

    const session = await auth.api.getSession({
      headers: headersList as Headers,
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
