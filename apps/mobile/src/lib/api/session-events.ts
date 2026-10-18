type SessionInvalidationListener = () => void;
export interface SessionIdentity {
  orgId: string;
  userId: string;
}

const sessionInvalidationListeners = new Set<SessionInvalidationListener>();
const sessionEndedListeners = new Set<(identity: SessionIdentity) => void>();

export function subscribeToSessionInvalidation(
  listener: SessionInvalidationListener,
): () => void {
  sessionInvalidationListeners.add(listener);
  return () => sessionInvalidationListeners.delete(listener);
}

export function notifySessionInvalidated(): void {
  for (const listener of sessionInvalidationListeners) {
    listener();
  }
}

export function subscribeToSessionEnded(
  listener: (identity: SessionIdentity) => void,
): () => void {
  sessionEndedListeners.add(listener);
  return () => sessionEndedListeners.delete(listener);
}

export function notifySessionEnded(identity: SessionIdentity): void {
  for (const listener of sessionEndedListeners) {
    listener(identity);
  }
}
