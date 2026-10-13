/**
 * @module @orbis/mobile/providers/query-provider
 *
 * Configures and provides TanStack Query client for server-state management.
 */

import React, { useEffect, useState } from "react";
import { AppState, type AppStateStatus, Platform } from "react-native";
import * as Network from "expo-network";
import {
  QueryClient,
  focusManager,
} from "@tanstack/react-query";
import { PersistQueryClientProvider } from "@tanstack/react-query-persist-client";
import { useAuth } from "@/lib/api";
import { updateNetworkStatus } from "@/lib/network/online-manager";
import { createOfflineQueryPersister } from "@/lib/offline/offline-query-persister";
import { OFFLINE_QUERY_CACHE_MAX_AGE } from "@/lib/offline/offline-query-cache";
import { subscribeToSessionEnded } from "@/lib/api/session-events";

function onAppStateChange(status: AppStateStatus) {
  if (Platform.OS !== "web") {
    focusManager.setFocused(status === "active");
  }
}

export function AppQueryProvider({ children }: { children: React.ReactNode }) {
  const { isAuthenticated, orgId, userId } = useAuth();
  const cacheScope = isAuthenticated ? `${orgId}:${userId}` : "guest";

  const [queryClient] = useState(
    () =>
      new QueryClient({
        defaultOptions: {
          queries: {
            staleTime: 1000 * 60 * 2,
            gcTime: OFFLINE_QUERY_CACHE_MAX_AGE,
            retry: 1,
            networkMode: "online",
            refetchOnReconnect: true,
            refetchOnWindowFocus: true,
          },
          mutations: {
            networkMode: "always",
            retry: 0,
          },
        },
      }),
  );
  const persister = React.useMemo(
    () => createOfflineQueryPersister(cacheScope),
    [cacheScope],
  );
  const activePersister = React.useRef(persister);
  activePersister.current = persister;

  useEffect(() => {
    const subscription = AppState.addEventListener("change", onAppStateChange);
    return () => subscription.remove();
  }, []);

  useEffect(() => {
    let receivedNetworkEvent = false;
    const updateOnlineStatus = (state: Network.NetworkState) => {
      receivedNetworkEvent = true;
      updateNetworkStatus(
        state.isConnected === true && state.isInternetReachable !== false,
      );
    };

    const subscription = Network.addNetworkStateListener(updateOnlineStatus);
    void Network.getNetworkStateAsync()
      .then((state) => {
        if (!receivedNetworkEvent) {
          updateOnlineStatus(state);
        }
      })
      .catch((error: unknown) => {
        console.error("[Orbis Mobile Network] Failed to read network status.", error);
        if (!receivedNetworkEvent) {
          updateNetworkStatus(false);
        }
      });

    return () => subscription.remove();
  }, []);

  useEffect(
    () =>
      subscribeToSessionEnded((identity) => {
        if (`${identity.orgId}:${identity.userId}` !== cacheScope) return;

        void queryClient.cancelQueries().catch((error: unknown) => {
          console.error("[Orbis Mobile Cache] Failed to cancel session queries.", error);
        });
        queryClient.clear();

        const removeClient = activePersister.current.removeClient;
        if (removeClient) {
          void Promise.resolve(removeClient()).catch((error: unknown) => {
            console.error("[Orbis Mobile Cache] Failed to clear the user cache.", error);
          });
        }
      }),
    [cacheScope, queryClient],
  );

  return (
    <PersistQueryClientProvider
      client={queryClient}
      persistOptions={{
        persister,
        maxAge: OFFLINE_QUERY_CACHE_MAX_AGE,
        buster: "orbis-mobile-query-cache-v1",
      }}
      onError={() => {
        console.error("[Orbis Mobile Cache] Failed to restore the offline cache.");
      }}
    >
      {children}
    </PersistQueryClientProvider>
  );
}
