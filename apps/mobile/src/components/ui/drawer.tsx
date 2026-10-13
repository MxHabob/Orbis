/**
 * @module @orbis/mobile/components/ui/drawer
 *
 * S.C.R.A.P.S. Bottom Drawer / Sheet Component for Orbis Mobile.
 * Built with high-performance gesture physics and fluid spring curves.
 */

import React, {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import {
  Modal,
  Platform,
  Pressable,
  StyleSheet,
  Text,
  View,
  useWindowDimensions,
} from "react-native";
import { BlurView } from "expo-blur";
import {
  Gesture,
  GestureDetector,
  GestureHandlerRootView,
} from "react-native-gesture-handler";
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

interface DrawerContextValue {
  translateY: SharedValue<number>;
  onClose: () => void;
  theme: ReturnType<typeof useTheme>;
  isDark: boolean;
  createPanGesture: () => ReturnType<typeof Gesture.Pan>;
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
  const { height } = useWindowDimensions();
  const closedOffset = height;

  const [shouldRender, setShouldRender] = useState(visible);
  const [hasCloseButton, setHasCloseButton] = useState(false);

  if (visible && !shouldRender) {
    setShouldRender(true);
  }

  const translateY = useSharedValue(visible ? 0 : closedOffset);

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
        closedOffset,
        { duration: 180, easing: Easing.out(Easing.cubic) },
        (finished) => {
          if (finished) {
            runOnJS(setShouldRender)(false);
          }
        },
      );
    }
  }, [closedOffset, translateY, visible]);

  const animatedBackdropStyle = useAnimatedStyle(() => {
    const progress = Math.max(
      0,
      1 - Math.min(1, translateY.value / height),
    );
    const opacity = progress * 0.52;
    return { opacity };
  });

  const animatedSheetStyle = useAnimatedStyle(() => {
    return {
      transform: [{ translateY: translateY.value }],
    };
  });

  const createPanGesture = useCallback(
    () =>
      Gesture.Pan()
        .minDistance(8)
        .activeOffsetY([-12, 12])
        .failOffsetX([-20, 20])
        .onUpdate((event) => {
          translateY.value = Math.max(0, event.translationY);
        })
        .onEnd((event) => {
          if (event.translationY > height * 0.18 || event.velocityY > 800) {
            translateY.value = withTiming(
              closedOffset,
              { duration: 200, easing: Easing.out(Easing.cubic) },
              (finished) => {
                if (finished) runOnJS(onClose)();
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
        }),
    [closedOffset, height, onClose, translateY],
  );
  const contentPan = useMemo(
    () => createPanGesture(),
    [createPanGesture],
  );

  if (!shouldRender) return null;

  const content = (
    <Animated.View
      style={[
        styles.sheetContainer,
        {
          maxHeight: height * 0.92,
        },
        animatedSheetStyle,
      ]}
    >
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
        createPanGesture,
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
        <GestureHandlerRootView style={styles.modalContainer}>
          <Animated.View style={[styles.backdrop, animatedBackdropStyle]}>
            {Platform.OS === "android" ? null : (
              <BlurView
                pointerEvents="none"
                intensity={18}
                tint={isDark ? "dark" : "light"}
                style={StyleSheet.absoluteFill}
              />
            )}
            <View
              pointerEvents="none"
              style={[StyleSheet.absoluteFill, styles.backdropTint]}
            />
            <Pressable style={StyleSheet.absoluteFill} onPress={onClose} />
          </Animated.View>
          {disableContentDrag ? (
            content
          ) : (
            <GestureDetector gesture={contentPan}>{content}</GestureDetector>
          )}
        </GestureHandlerRootView>
      </Modal>
    </DrawerContext.Provider>
  );
}

export interface DrawerContentProps {
  children: React.ReactNode;
  showDragHandle?: boolean;
}

export function DrawerContent({
  children,
  showDragHandle = true,
}: DrawerContentProps) {
  const { theme, hasCloseButton, createPanGesture } = useDrawer();
  const insets = useSafeAreaInsets();
  const pan = useMemo(() => createPanGesture(), [createPanGesture]);

  const dragHandleNode = showDragHandle ? (
    <GestureDetector gesture={pan}>
      <View
        collapsable={false}
        style={styles.dragHandleContainer}
      >
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

export function DrawerHeader({ children }: { children: React.ReactNode }) {
  const { theme, createPanGesture } = useDrawer();
  const pan = useMemo(() => createPanGesture(), [createPanGesture]);
  return (
    <GestureDetector gesture={pan}>
      <View
        collapsable={false}
        style={[styles.header, { borderBottomColor: theme.border }]}
      >
        {children}
      </View>
    </GestureDetector>
  );
}

export function DrawerFooter({ children }: { children: React.ReactNode }) {
  const { theme } = useDrawer();
  return (
    <View style={[styles.footer, { borderTopColor: theme.border }]}>
      {children}
    </View>
  );
}

export function DrawerTitle({ children }: { children: string }) {
  const { theme } = useDrawer();
  return <Text style={[styles.title, { color: theme.text }]}>{children}</Text>;
}

export function DrawerDescription({ children }: { children: string }) {
  const { theme } = useDrawer();
  return (
    <Text style={[styles.description, { color: theme.textSecondary }]}>
      {children}
    </Text>
  );
}

export function DrawerSeparator() {
  const { theme } = useDrawer();
  return <View style={[styles.separator, { backgroundColor: theme.border }]} />;
}

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
    backgroundColor: "rgba(0, 0, 0, 0.12)",
  },
  backdropTint: {
    backgroundColor: "rgba(0, 0, 0, 0.4)",
  },
  sheetContainer: {
    width: "100%",
    maxWidth: 540,
    zIndex: 1,
    shadowColor: "#000000",
    shadowOpacity: 0.2,
    shadowRadius: 20,
    elevation: 16,
  },
  menuWrapper: {
    overflow: "hidden",
    borderTopWidth: StyleSheet.hairlineWidth,
    borderLeftWidth: StyleSheet.hairlineWidth,
    borderRightWidth: StyleSheet.hairlineWidth,
    borderBottomWidth: 0,
    borderTopLeftRadius: 24,
    borderTopRightRadius: 24,
  },
  cancelWrapper: {
    overflow: "hidden",
    marginTop: 12,
    borderTopWidth: 1,
    borderLeftWidth: 1,
    borderRightWidth: 1,
    borderBottomWidth: 0,
    borderTopLeftRadius: 16,
    borderTopRightRadius: 16,
  },
  bodyContent: {
    paddingVertical: 0,
  },
  dragHandleContainer: {
    width: "100%",
    height: 48,
    alignItems: "center",
    justifyContent: "center",
  },
  dragHandle: {
    width: 38,
    height: 5,
    borderRadius: 9999,
  },
  header: {
    paddingHorizontal: 20,
    paddingTop: 8,
    paddingBottom: 16,
    alignItems: "center",
    gap: 2,
    borderBottomWidth: StyleSheet.hairlineWidth,
  },
  footer: {
    paddingHorizontal: 20,
    paddingTop: 16,
    paddingBottom: 16,
    borderTopWidth: StyleSheet.hairlineWidth,
  },
  title: {
    ...{"fontSize":16,"lineHeight":22,"fontWeight":"600"},
    fontWeight: "700",
    textAlign: "center",
  },
  description: {
    ...{"fontSize":11,"lineHeight":14,"fontWeight":"500"},
    marginTop: 2,
    textAlign: "center",
  },
  menuItem: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    paddingHorizontal: 20,
    paddingVertical: 16,
  },
  menuItemText: {
    ...{"fontSize":14,"lineHeight":20,"fontWeight":"600"},
  },
  separator: {
    height: StyleSheet.hairlineWidth,
    marginLeft: 16,
  },
});
