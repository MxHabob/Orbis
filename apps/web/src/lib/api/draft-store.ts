/**
 * Explicit local draft persistence for high-friction entry forms (grades, attendance).
 *
 * Preserves unsaved edits across brief network drops or accidental refreshes
 * without ever pretending the data is committed on the server.
 */

const DRAFT_PREFIX = "orbis:draft:";
const DEFAULT_TTL_MS = 7 * 24 * 60 * 60 * 1000;

interface DraftEnvelope<T> {
  savedAt: number;
  payload: T;
}

export function saveDraft<T>(key: string, payload: T): void {
  if (typeof localStorage === "undefined") return;
  try {
    const envelope: DraftEnvelope<T> = {
      savedAt: Date.now(),
      payload,
    };
    localStorage.setItem(`${DRAFT_PREFIX}${key}`, JSON.stringify(envelope));
  } catch {
    // Ignore storage quota or private-mode errors.
  }
}

export function loadDraft<T>(key: string, ttlMs = DEFAULT_TTL_MS): T | null {
  if (typeof localStorage === "undefined") return null;
  try {
    const raw = localStorage.getItem(`${DRAFT_PREFIX}${key}`);
    if (!raw) return null;
    const envelope = JSON.parse(raw) as DraftEnvelope<T>;
    if (
      !envelope ||
      typeof envelope.savedAt !== "number" ||
      Date.now() - envelope.savedAt > ttlMs
    ) {
      localStorage.removeItem(`${DRAFT_PREFIX}${key}`);
      return null;
    }
    return envelope.payload;
  } catch {
    return null;
  }
}

export function clearDraft(key: string): void {
  if (typeof localStorage === "undefined") return;
  try {
    localStorage.removeItem(`${DRAFT_PREFIX}${key}`);
  } catch {
    // Ignore storage errors.
  }
}
