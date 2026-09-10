/**
 * @module @orbis/mobile/components/ui/toast
 *
 * S.C.R.A.P.S. Tactile Notification Toast for Orbis Mobile.
 * Features variant icons, theme reactivity, safe-area floating pill, and spring animations.
 */

import React, {
  createContext,
  useCallback,
  useContext,
  useRef,
  useState,
} from "react";
import { StyleSheet, Text, View } from "react-native";
import Animated, {
  Easing,
  runOnJS,
  useAnimatedStyle,
  useSharedValue,
  withSpring,
  withTiming,
} from "react-native-reanimated";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Ionicons } from "@expo/vector-icons";
import { Radii, Spacing, Typography } from "@/constants/theme";
import { useTheme } from "@/hooks/use-theme";

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

  const opacity = useSharedValue(0);
  const translateY = useSharedValue(-20);
  const scale = useSharedValue(0.92);

  const dismiss = useCallback(() => {
    opacity.value = withTiming(0, {
      duration: 160,
      easing: Easing.out(Easing.ease),
    });
    translateY.value = withTiming(-20, {
      duration: 160,
      easing: Easing.out(Easing.ease),
    });
    scale.value = withTiming(
      0.92,
      { duration: 160, easing: Easing.out(Easing.ease) },
      () => {
        runOnJS(setCurrent)(null);
      },
    );
  }, [opacity, translateY, scale]);

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

      opacity.value = withTiming(1, { duration: 180 });
      translateY.value = withSpring(0, { damping: 20, stiffness: 320 });
      scale.value = withSpring(1, { damping: 20, stiffness: 320 });

      timeoutRef.current = setTimeout(() => {
        dismiss();
      }, duration);
    },
    [opacity, translateY, scale, dismiss],
  );

  const toastHandler = Object.assign(
    (payload: string | ToastPayload) => showToast(payload),
    {
      success: (message: string, duration?: number) =>
        showToast({ message, variant: "success", duration }),
      error: (message: string, duration?: number) =>
        showToast({ message, variant: "error", duration }),
      warning: (message: string, duration?: number) =>
        showToast({ message, variant: "warning", duration }),
      info: (message: string, duration?: number) =>
        showToast({ message, variant: "info", duration }),
    },
  );

  const animatedStyle = useAnimatedStyle(() => ({
    opacity: opacity.value,
    transform: [{ translateY: translateY.value }, { scale: scale.value }],
  }));

  const getVariantIcon = (variant: ToastVariant) => {
    switch (variant) {
      case "success":
        return (
          <Ionicons name="checkmark-circle" size={20} color={theme.success} />
        );
      case "error":
        return <Ionicons name="alert-circle" size={20} color={theme.error} />;
      case "warning":
        return <Ionicons name="warning" size={20} color={theme.warning} />;
      case "info":
        return (
          <Ionicons name="information-circle" size={20} color={theme.info} />
        );
      default:
        return <Ionicons name="sparkles" size={18} color={theme.primary} />;
    }
  };

  return (
    <ToastContext.Provider value={{ toast: toastHandler }}>
      {children}
      {current && (
        <View
          pointerEvents="none"
          style={[styles.container, { top: Math.max(insets.top, 16) + 8 }]}
        >
          <Animated.View
            style={[
              styles.toast,
              {
                backgroundColor: theme.card,
                borderColor: theme.border,
              },
              animatedStyle,
            ]}
          >
            {getVariantIcon(current.variant)}
            <Text style={[styles.message, { color: theme.text }]}>
              {current.message}
            </Text>
          </Animated.View>
        </View>
      )}
    </ToastContext.Provider>
  );
}

// ─── Styles ──────────────────────────────────────────────────────────────────

const styles = StyleSheet.create({
  container: {
    position: "absolute",
    left: 0,
    right: 0,
    alignItems: "center",
    justifyContent: "center",
    zIndex: 99999,
    elevation: 99999,
    paddingHorizontal: Spacing.four,
  },
  toast: {
    flexDirection: "row",
    alignItems: "center",
    paddingVertical: Spacing.three,
    paddingHorizontal: Spacing.four,
    borderRadius: Radii.full,
    borderWidth: 1,
    maxWidth: "92%",
    gap: Spacing.two,
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: 6 },
    shadowOpacity: 0.14,
    shadowRadius: 14,
    elevation: 8,
  },
  message: {
    ...Typography.bodyBold,
    fontSize: 13,
    lineHeight: 18,
    flexShrink: 1,
  },
});
