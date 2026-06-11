"use client";

import { useEffect } from "react";

interface RealtimeOptions {
  channelName: string;
  event?: string;
  table?: string;
  filter?: string;
  onEvent: (payload: any) => void;
}

/**
 * useRealtime hook - Disabled for stability
 * Realtime is currently disabled to prevent Bun runtime crashes on Windows.
 * This hook is now a no-op.
 */
export function useRealtime({ channelName, onEvent }: RealtimeOptions) {
  useEffect(() => {
    // Realtime is disabled.
    console.debug(
      `[Realtime] Subscription to ${channelName} ignored (Realtime is disabled)`,
    );
  }, [channelName]);
}
