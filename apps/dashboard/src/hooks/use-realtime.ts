"use client";

import { useEffect, useRef, useCallback } from "react";

interface RealtimeOptions {
  channelName: string;
  event?: string;
  table?: string;
  filter?: string;
  onEvent: (payload: any) => void;
}

/**
 * useRealtime hook - 2026 Best Practices
 * Implements Realtime functionality using Server-Sent Events (SSE) and Redis Pub/Sub.
 * This is a high-performance, scalable alternative to Supabase Realtime.
 * 
 * @param options - Subscription options (channel, onEvent callback)
 */
export function useRealtime({
  channelName,
  onEvent,
}: RealtimeOptions) {
  // Use a ref for the callback to prevent effect re-runs when the closure changes
  const onEventRef = useRef(onEvent);
  const reconnectTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    onEventRef.current = onEvent;
  }, [onEvent]);

  const connect = useCallback(() => {
    const apiUrl = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3002";
    
    // Establishing SSE connection with credentials (cookies) for auth
    const eventSource = new EventSource(`${apiUrl}/realtime`, {
      withCredentials: true,
    });

    console.debug(`[Realtime] Connecting to ${channelName}...`);

    const handleMessage = (data: any) => {
      try {
        const parsed = typeof data === "string" ? JSON.parse(data) : data;
        onEventRef.current(parsed);
      } catch (e) {
        onEventRef.current(data);
      }
    };

    // Specific channel listener
    eventSource.addEventListener(channelName, (e: any) => {
      handleMessage(e.data);
    });

    // Global listener
    eventSource.onmessage = (e) => {
      if (e.data === "heartbeat") return;
      handleMessage(e.data);
    };

    eventSource.onopen = () => {
      console.debug(`[Realtime] Connected to ${channelName}`);
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
        reconnectTimeoutRef.current = null;
      }
    };

    eventSource.onerror = (err) => {
      console.error(`[Realtime] Connection error for ${channelName}:`, err);
      eventSource.close();
      
      // Simple exponential backoff for reconnection if it doesn't auto-retry
      if (!reconnectTimeoutRef.current) {
        reconnectTimeoutRef.current = setTimeout(() => {
          reconnectTimeoutRef.current = null;
          connect();
        }, 5000);
      }
    };

    return eventSource;
  }, [channelName]);

  useEffect(() => {
    const es = connect();

    return () => {
      es.close();
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
      }
    };
  }, [connect]);
}
