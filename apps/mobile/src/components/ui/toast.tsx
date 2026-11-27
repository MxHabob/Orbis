/**
 * @module @orbis/mobile/components/ui/toast
 *
 * S.C.R.A.P.S. Notification Toast for Orbis Mobile.
 * Features variant icons, theme reactivity, and safe-area placement.
 */

import React, {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";
import { View } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { CheckCircle, InfoCircle, Sparkles } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";
import { Text } from "@/components/ui/text";

const FLOATING_TAB_BAR_HEIGHT = 48;
const TOAST_TAB_BAR_GAP = 16;

// ─── Types ───────────────────────────────────────────────────────────────────

export type ToastVariant = "default" | "success" | "error" | "warning" | "info";

export interface ToastPayload {
  message: string;
  variant?: ToastVariant;
  duration?: number;
}

export interface ToastContextValue {
  toast: {
    (payload: string | ToastPayload): void;
    success: (message: string, duration?: number) => void;
    error: (message: string, duration?: number) => void;
    warning: (message: string, duration?: number) => void;
    info: (message: string, duration?: number) => void;
  };
}

// ─── Context ─────────────────────────────────────────────────────────────────

const ToastContext = createContext<ToastContextValue | null>(null);

export function useToast(): ToastContextValue["toast"] {
  const ctx = useContext(ToastContext);
  if (!ctx) {
    throw new Error("useToast must be used within a <ToastProvider />");
  }
  return ctx.toast;
}

// ─── Provider ────────────────────────────────────────────────────────────────

interface ToastItem {
  id: string;
  message: string;
  variant: ToastVariant;
}

export function ToastProvider({ children }: { children: React.ReactNode }) {
  const theme = useTheme();
  const insets = useSafeAreaInsets();
  const [current, setCurrent] = useState<ToastItem | null>(null);
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(
    () => () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    },
    [],
  );

  const showToast = useCallback((payload: string | ToastPayload) => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }

    const message = typeof payload === "string" ? payload : payload.message;
    const variant =
      typeof payload === "string" ? "default" : payload.variant || "default";
    const duration =
      typeof payload === "string" ? 2600 : (payload.duration ?? 2600);

    setCurrent({
      id: Date.now().toString(),
      message,
      variant,
    });

    timeoutRef.current = setTimeout(() => {
      setCurrent(null);
      timeoutRef.current = null;
    }, duration);
  }, []);

  const toastHandler = React.useMemo(() => {
    const handler = (payload: string | ToastPayload) => showToast(payload);
    // This function is deferred as a consumer event handler, not invoked during render.
    // eslint-disable-next-line react-hooks/refs
    return Object.assign(handler, {
      success: (message: string, duration?: number) =>
        showToast({ message, variant: "success", duration }),
      error: (message: string, duration?: number) =>
        showToast({ message, variant: "error", duration }),
      warning: (message: string, duration?: number) =>
        showToast({ message, variant: "warning", duration }),
      info: (message: string, duration?: number) =>
        showToast({ message, variant: "info", duration }),
    });
  }, [showToast]);

  const getVariantIcon = (variant: ToastVariant) => {
    switch (variant) {
      case "success":
        return <CheckCircle size={18} color={theme.success} />;
      case "error":
        return <InfoCircle size={18} color={theme.error} />;
      case "warning":
        return <InfoCircle size={18} color={theme.warning} />;
      case "info":
        return <InfoCircle size={18} color={theme.info} />;
      default:
        return <Sparkles size={18} color={theme.primary} />;
    }
  };

  const getVariantIconBackground = (variant: ToastVariant) => {
    switch (variant) {
      case "success":
        return "bg-success/10";
      case "error":
        return "bg-destructive/10";
      case "warning":
        return "bg-warning/10";
      case "info":
        return "bg-info/10";
      default:
        return "bg-primary/10";
    }
  };

  return (
    <ToastContext.Provider value={{ toast: toastHandler }}>
      {children}
      {current && (
        <View
          pointerEvents="none"
          className="absolute left-0 right-0 z-[99999] items-center justify-center px-four"
          style={{
            bottom:
              Math.max(insets.bottom, 12) +
              FLOATING_TAB_BAR_HEIGHT +
              TOAST_TAB_BAR_GAP,
          }}
        >
          <View
            accessibilityRole="alert"
            accessibilityLiveRegion="polite"
            className="w-full max-w-[420px] flex-row items-center gap-three rounded-md border px-four py-three"
            style={{
              backgroundColor: theme.cardElevated,
              borderColor: theme.cardBorder,
              shadowColor: "#000000",
              shadowOffset: { width: 0, height: 4 },
              shadowOpacity: 0.16,
              shadowRadius: 12,
              elevation: 6,
            }}
          >
            <View
              className={`h-9 w-9 shrink-0 items-center justify-center rounded-md ${getVariantIconBackground(current.variant)}`}
            >
              {getVariantIcon(current.variant)}
            </View>
            <Text
              className="flex-1 font-sans text-sm leading-5 font-medium"
              style={{ color: theme.cardForeground }}
            >
              {current.message}
            </Text>
          </View>
        </View>
      )}
    </ToastContext.Provider>
  );
}
