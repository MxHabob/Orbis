/**
 * @module @orbis/mobile/components/ui/drawer
 *
 * S.C.R.A.P.S. Bottom Drawer / Sheet Component for Orbis Mobile.
 * Built with high-performance gesture physics and fluid spring curves.
 */

import React, { createContext, useContext, useEffect, useState } from "react";
import {
  Modal,
  Platform,
  Pressable,
  StyleSheet,
  Text,
  View,
} from "react-native";
import { Gesture, GestureDetector } from "react-native-gesture-handler";
import Animated, {
  Easing,
  runOnJS,
  type SharedValue,
  useAnimatedStyle,
  useSharedValue,
  withSpring,
  withTiming,
} from "react-native-reanimated";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { useTheme } from "@/hooks/use-theme";
import { useColorScheme } from "@/hooks/use-color-scheme";
import { Radii, Spacing, Typography } from "@/constants/theme";

// ─── Drawer Context ─────────────────────────────────────────────────────────

interface DrawerContextValue {
  translateY: SharedValue<number>;
  onClose: () => void;
  theme: ReturnType<typeof useTheme>;
  isDark: boolean;
  pan: ReturnType<typeof Gesture.Pan>;
  hasCloseButton: boolean;
  setHasCloseButton: (val: boolean) => void;
}

const DrawerContext = createContext<DrawerContextValue | null>(null);

export function useDrawer() {
  const context = useContext(DrawerContext);
  if (!context) {
    throw new Error(
      "Drawer compound components must be used within a <Drawer />",
    );
  }
  return context;
}

// ─── Drawer Root ────────────────────────────────────────────────────────────

export interface DrawerProps {
  visible: boolean;
  onClose: () => void;
  children: React.ReactNode;
  disableContentDrag?: boolean;
}

export function Drawer({
  visible,
  onClose,
  children,
  disableContentDrag = true,
}: DrawerProps) {
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const theme = useTheme();

  const [shouldRender, setShouldRender] = useState(visible);
  const [hasCloseButton, setHasCloseButton] = useState(false);

  if (visible && !shouldRender) {
    setShouldRender(true);
  }

  const translateY = useSharedValue(visible ? 0 : 800);

  useEffect(() => {
    if (visible) {
      translateY.value = withSpring(0, {
        damping: 32,
        stiffness: 380,
        mass: 0.5,
        overshootClamping: true,
      });
    } else {
      translateY.value = withTiming(
        800,
        { duration: 180, easing: Easing.out(Easing.cubic) },
        (finished) => {
          if (finished) {
            runOnJS(setShouldRender)(false);
          }
        },
      );
    }
  }, [visible, translateY]);

  const animatedBackdropStyle = useAnimatedStyle(() => {
    const opacity = Math.max(0, 1 - translateY.value / 800);
    return { opacity };
  });

  const animatedSheetStyle = useAnimatedStyle(() => {
    return {
      transform: [{ translateY: translateY.value }],
    };
  });

  const pan = Gesture.Pan()
    .minDistance(10)
    .activeOffsetY([15, 99999])
    .onUpdate((event) => {
      if (event.translationY > 0) {
        translateY.value = event.translationY;
      } else {
        translateY.value = event.translationY * 0.15;
      }
    })
    .onEnd((event) => {
      if (event.translationY > 100 || event.velocityY > 400) {
        translateY.value = withTiming(
          800,
          {
            duration: 180,
            easing: Easing.out(Easing.cubic),
          },
          (finished) => {
            if (finished) {
              runOnJS(onClose)();
            }
          },
        );
      } else {
        translateY.value = withSpring(0, {
          damping: 32,
          stiffness: 380,
          mass: 0.5,
          overshootClamping: true,
        });
      }
    });

  if (!shouldRender) return null;

  const content = (
    <Animated.View style={[styles.sheetContainer, animatedSheetStyle]}>
      {children}
    </Animated.View>
  );

  return (
    <DrawerContext.Provider
      value={{
        translateY,
        onClose,
        theme,
        isDark,
        pan,
        hasCloseButton,
        setHasCloseButton,
      }}
    >
      <Modal
        visible
        transparent
        animationType="none"
        statusBarTranslucent
        onRequestClose={onClose}
      >
        <View style={styles.modalContainer}>
          <Animated.View style={[styles.backdrop, animatedBackdropStyle]}>
            <Pressable style={StyleSheet.absoluteFill} onPress={onClose} />
          </Animated.View>
          {disableContentDrag ? (
            content
          ) : (
            <GestureDetector gesture={pan}>{content}</GestureDetector>
          )}
        </View>
      </Modal>
    </DrawerContext.Provider>
  );
}

// ─── Drawer Content ─────────────────────────────────────────────────────────

export interface DrawerContentProps {
  children: React.ReactNode;
  showDragHandle?: boolean;
}

export function DrawerContent({
  children,
  showDragHandle = true,
}: DrawerContentProps) {
  const { theme, pan, hasCloseButton } = useDrawer();
  const insets = useSafeAreaInsets();

  const dragHandleNode = showDragHandle ? (
    <GestureDetector gesture={pan}>
      <View style={styles.dragHandleContainer}>
        <View style={[styles.dragHandle, { backgroundColor: theme.border }]} />
      </View>
    </GestureDetector>
  ) : null;

  return (
    <View
      style={[
        styles.menuWrapper,
        {
          borderColor: theme.border,
          backgroundColor: theme.card,
          paddingBottom: hasCloseButton
            ? 0
            : Math.max(insets.bottom, Platform.OS === "ios" ? 34 : 16),
        },
      ]}
    >
      <View style={styles.bodyContent}>
        {dragHandleNode}
        {children}
      </View>
    </View>
  );
}

// ─── Drawer Header ──────────────────────────────────────────────────────────

export function DrawerHeader({ children }: { children: React.ReactNode }) {
  const { pan } = useDrawer();
  return (
    <GestureDetector gesture={pan}>
      <View style={styles.header}>{children}</View>
    </GestureDetector>
  );
}

// ─── Drawer Footer ──────────────────────────────────────────────────────────

export function DrawerFooter({ children }: { children: React.ReactNode }) {
  return <View style={styles.footer}>{children}</View>;
}

// ─── Drawer Title ───────────────────────────────────────────────────────────

export function DrawerTitle({ children }: { children: string }) {
  const { theme } = useDrawer();
  return <Text style={[styles.title, { color: theme.text }]}>{children}</Text>;
}

// ─── Drawer Description ─────────────────────────────────────────────────────

export function DrawerDescription({ children }: { children: string }) {
  const { theme } = useDrawer();
  return (
    <Text style={[styles.description, { color: theme.textSecondary }]}>
      {children}
    </Text>
  );
}

// ─── Drawer Separator ───────────────────────────────────────────────────────

export function DrawerSeparator() {
  const { theme } = useDrawer();
  return <View style={[styles.separator, { backgroundColor: theme.border }]} />;
}

// ─── Drawer Close Button ────────────────────────────────────────────────────

export function DrawerCloseButton({ title = "Cancel" }: { title?: string }) {
  const { onClose, theme, setHasCloseButton } = useDrawer();
  const insets = useSafeAreaInsets();

  useEffect(() => {
    setHasCloseButton(true);
    return () => {
      setHasCloseButton(false);
    };
  }, [setHasCloseButton]);

  return (
    <View
      style={[
        styles.cancelWrapper,
        {
          borderColor: theme.border,
          backgroundColor: theme.card,
          paddingBottom: Math.max(
            insets.bottom,
            Platform.OS === "ios" ? 34 : 16,
          ),
        },
      ]}
    >
      <Pressable
        style={({ pressed }) => [
          styles.menuItem,
          pressed && { backgroundColor: theme.accent },
        ]}
        onPress={onClose}
      >
        <Text
          style={[
            styles.menuItemText,
            {
              color: theme.text,
              fontWeight: "700",
              textAlign: "center",
              flex: 1,
            },
          ]}
        >
          {title}
        </Text>
      </Pressable>
    </View>
  );
}

// ─── Styles ──────────────────────────────────────────────────────────────────

const styles = StyleSheet.create({
  modalContainer: {
    flex: 1,
    justifyContent: "flex-end",
    alignItems: "center",
  },
  backdrop: {
    position: "absolute",
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
    backgroundColor: "rgba(0, 0, 0, 0.52)",
  },
  sheetContainer: {
    width: "100%",
    maxWidth: 540,
    paddingHorizontal: 0,
    paddingBottom: 0,
  },
  menuWrapper: {
    overflow: "hidden",
    borderTopWidth: 1,
    borderLeftWidth: 1,
    borderRightWidth: 1,
    borderBottomWidth: 0,
    marginBottom: 0,
    borderTopLeftRadius: Radii["3xl"],
    borderTopRightRadius: Radii["3xl"],
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: -4 },
    shadowOpacity: 0.1,
    shadowRadius: 16,
    elevation: 10,
  },
  cancelWrapper: {
    overflow: "hidden",
    marginTop: Spacing.three,
    borderTopWidth: 1,
    borderLeftWidth: 1,
    borderRightWidth: 1,
    borderBottomWidth: 0,
    borderTopLeftRadius: Radii["2xl"],
    borderTopRightRadius: Radii["2xl"],
  },
  bodyContent: {
    paddingVertical: 0,
  },
  dragHandleContainer: {
    width: "100%",
    alignItems: "center",
    paddingTop: Spacing.three,
    paddingBottom: Spacing.two,
  },
  dragHandle: {
    width: 38,
    height: 5,
    borderRadius: Radii.full,
  },
  header: {
    paddingHorizontal: Spacing.five,
    paddingTop: Spacing.two,
    paddingBottom: Spacing.four,
    alignItems: "center",
    gap: Spacing.half,
  },
  footer: {
    paddingHorizontal: Spacing.five,
    paddingTop: Spacing.four,
    paddingBottom: Spacing.four,
  },
  title: {
    ...Typography.h3,
    fontWeight: "700",
    textAlign: "center",
  },
  description: {
    ...Typography.small,
    marginTop: 2,
    textAlign: "center",
  },
  menuItem: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    paddingHorizontal: Spacing.five,
    paddingVertical: Spacing.four,
  },
  menuItemText: {
    ...Typography.bodyBold,
  },
  separator: {
    height: StyleSheet.hairlineWidth,
    marginLeft: Spacing.four,
  },
});
