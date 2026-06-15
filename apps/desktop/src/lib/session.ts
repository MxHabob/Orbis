/** Session helpers for IPC calls — reads from localStorage set by auth flow. */

const SESSION_USER_KEY = "session_user";
const ACCESS_TOKEN_KEY = "access_token";

export function getAccessToken(): string | null {
  return localStorage.getItem(ACCESS_TOKEN_KEY);
}

export function getSessionUser(): Record<string, unknown> | null {
  const raw = localStorage.getItem(SESSION_USER_KEY);
  if (!raw) return null;
  try {
    return JSON.parse(raw) as Record<string, unknown>;
  } catch {
    return null;
  }
}

export function getOrgId(): string {
  const user = getSessionUser();
  if (!user) return "";
  return String(user.orgId ?? user.org_id ?? "");
}

export function getUserId(): string {
  const user = getSessionUser();
  if (!user) return "";
  return String(user.id ?? user.userId ?? "");
}

export function setSession(
  user: Record<string, unknown>,
  accessToken: string,
): void {
  localStorage.setItem(SESSION_USER_KEY, JSON.stringify(user));
  localStorage.setItem(ACCESS_TOKEN_KEY, accessToken);
}

export function clearSession(): void {
  localStorage.removeItem(SESSION_USER_KEY);
  localStorage.removeItem(ACCESS_TOKEN_KEY);
}
