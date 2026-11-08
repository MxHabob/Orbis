import { onlineManager } from "@tanstack/query-core";

onlineManager.setOnline(false);

export interface NetworkStatus {
  isKnown: boolean;
  isOnline: boolean;
  isConnected: boolean;
}

let networkStatus: NetworkStatus = {
  isKnown: false,
  isOnline: false,
  isConnected: false,
};
const networkStatusListeners = new Set<(status: NetworkStatus) => void>();

export function updateNetworkStatus(
  isOnline: boolean,
  isConnected: boolean = isOnline,
): void {
  networkStatus = { isKnown: true, isOnline, isConnected };
  onlineManager.setOnline(isConnected);
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
