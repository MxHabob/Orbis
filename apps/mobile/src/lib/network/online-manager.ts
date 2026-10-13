import { onlineManager } from "@tanstack/react-query";

onlineManager.setOnline(false);

export interface NetworkStatus {
  isKnown: boolean;
  isOnline: boolean;
}

let networkStatus: NetworkStatus = {
  isKnown: false,
  isOnline: false,
};
const networkStatusListeners = new Set<(status: NetworkStatus) => void>();

export function updateNetworkStatus(isOnline: boolean): void {
  networkStatus = { isKnown: true, isOnline };
  onlineManager.setOnline(isOnline);
  for (const listener of networkStatusListeners) {
    listener(networkStatus);
  }
}

export function getNetworkStatus(): NetworkStatus {
  return networkStatus;
}

export function subscribeToNetworkStatus(
  listener: (status: NetworkStatus) => void,
): () => void {
  networkStatusListeners.add(listener);
  listener(networkStatus);
  return () => networkStatusListeners.delete(listener);
}

export { onlineManager };
