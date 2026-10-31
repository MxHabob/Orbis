/**
 * @module @orbis/mobile/components/ui/toast
 *
 * S.C.R.A.P.S. Tactile Notification Toast for Orbis Mobile.
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
import { Text, View } from "react-native";
import Animated, {
  Easing,
  runOnJS,
  useAnimatedStyle,
  useSharedValue,
  withTiming,
} from "react-native-reanimated";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { CheckCircle, InfoCircle, Sparkles } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";

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

function createToastHandler(
  showToast: (payload: string | ToastPayload) => void,
): ToastContextValue["toast"] {
  const handler = (payload: string | ToastPayload) => showToast(payload);
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

  const opacity = useSharedValue(0);
  const translateY = useSharedValue(4);

  useEffect(
    () => () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    },
    [],
  );

  const dismiss = useCallback(() => {
    translateY.set(
      withTiming(4, {
        duration: 120,
        easing: Easing.out(Easing.ease),
      }),
    );
    opacity.set(
      withTiming(0, { duration: 120, easing: Easing.out(Easing.ease) }, () => {
        runOnJS(setCurrent)(null);
      }),
    );
  }, [opacity, translateY]);

  const showToast = useCallback(
    (payload: string | ToastPayload) => {
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

      opacity.set(
        withTiming(1, {
          duration: 140,
          easing: Easing.out(Easing.ease),
        }),
      );
      translateY.set(
        withTiming(0, {
          duration: 140,
          easing: Easing.out(Easing.ease),
        }),
      );

      timeoutRef.current = setTimeout(() => {
        dismiss();
      }, duration);
    },
    [opacity, translateY, dismiss],
  );

  const toastHandler = React.useMemo(
    () => createToastHandler(showToast),
    [showToast],
  );

  const animatedStyle = useAnimatedStyle(() => ({
    opacity: opacity.value,
    transform: [{ translateY: translateY.value }],
  }));

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
          <Animated.View
            accessibilityRole="alert"
            accessibilityLiveRegion="polite"
            className="w-full max-w-[420px] flex-row items-center gap-three rounded-2xl border border-card-border bg-card-elevated px-four py-three"
            style={animatedStyle}
          >
            <View
              className={`h-9 w-9 shrink-0 items-center justify-center rounded-xl ${getVariantIconBackground(current.variant)}`}
            >
              {getVariantIcon(current.variant)}
            </View>
            <Text className="shrink font-sans text-sm leading-5 font-medium text-card-foreground">
              {current.message}
            </Text>
          </Animated.View>
        </View>
      )}
    </ToastContext.Provider>
  );
}
