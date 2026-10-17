/**
 * @module @orbis/mobile/components/ui/drawer
 *
 * S.C.R.A.P.S. Bottom Drawer / Sheet Component for Orbis Mobile.
 * Built with high-performance gesture physics and fluid spring curves.
 */

import { BlurView } from "expo-blur";
import type { ReactNode } from "react";
import {
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
  useWindowDimensions,
  View,
} from "react-native";
import {
  Gesture,
  GestureDetector,
  GestureHandlerRootView,
} from "react-native-gesture-handler";
import {
  KeyboardAwareScrollView,
  type KeyboardAwareScrollViewProps,
  KeyboardStickyView,
} from "react-native-keyboard-controller";
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
import { useColorScheme } from "@/hooks/use-color-scheme";
import { useTheme } from "@/hooks/use-theme";

interface DrawerContextValue {
  translateY: SharedValue<number>;
  sheetHeight: SharedValue<number>;
  onClose: () => void;
  theme: ReturnType<typeof useTheme>;
  isDark: boolean;
  createPanGesture: () => ReturnType<typeof Gesture.Pan>;
  isSnapping: boolean;
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
  children: ReactNode;
  disableContentDrag?: boolean;
  snapPoints?: readonly number[];
}

export function Drawer({
  visible,
  onClose,
  children,
  disableContentDrag = true,
  snapPoints,
}: DrawerProps) {
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const theme = useTheme();
  const { height } = useWindowDimensions();
  const closedOffset = height;
  const normalizedSnapPoints = useMemo(() => {
    if (!snapPoints?.length) return null;
    const points = [...new Set(snapPoints)]
      .filter((point) => Number.isFinite(point) && point > 0 && point <= 0.92)
      .sort((left, right) => left - right);
    return points.length ? points : null;
  }, [snapPoints]);
  const isSnapping = normalizedSnapPoints !== null;
  const minSheetHeight = isSnapping
    ? (normalizedSnapPoints[0] ?? 0) * height
    : 0;
  const maxSheetHeight = isSnapping
    ? (normalizedSnapPoints[normalizedSnapPoints.length - 1] ?? 0) * height
    : 0;

  const [shouldRender, setShouldRender] = useState(visible);
  const [hasCloseButton, setHasCloseButton] = useState(false);

  if (visible && !shouldRender) {
    setShouldRender(true);
  }

  const translateY = useSharedValue(visible && !isSnapping ? 0 : closedOffset);
  const sheetHeight = useSharedValue(
    visible && isSnapping ? minSheetHeight : 0,
  );
  const dragStart = useSharedValue(0);

  useEffect(() => {
    if (visible) {
      if (isSnapping) {
        sheetHeight.value = withSpring(minSheetHeight, {
          damping: 32,
          stiffness: 380,
          mass: 0.5,
          overshootClamping: true,
        });
      } else {
        translateY.value = withSpring(0, {
          damping: 32,
          stiffness: 380,
          mass: 0.5,
          overshootClamping: true,
        });
      }
    } else {
      if (isSnapping) {
        sheetHeight.value = withTiming(
          0,
          { duration: 180, easing: Easing.out(Easing.cubic) },
          (finished) => {
            if (finished) runOnJS(setShouldRender)(false);
          },
        );
      } else {
        translateY.value = withTiming(
          closedOffset,
          { duration: 180, easing: Easing.out(Easing.cubic) },
          (finished) => {
            if (finished) runOnJS(setShouldRender)(false);
          },
        );
      }
    }
  }, [
    closedOffset,
    isSnapping,
    minSheetHeight,
    sheetHeight,
    translateY,
    visible,
  ]);

  const animatedBackdropStyle = useAnimatedStyle(() => {
    const progress = isSnapping
      ? Math.min(1, sheetHeight.value / height)
      : Math.max(0, 1 - Math.min(1, translateY.value / height));
    const opacity = progress * 0.52;
    return { opacity };
  }, [height, isSnapping]);

  const animatedSheetStyle = useAnimatedStyle(() => {
    return isSnapping
      ? { height: sheetHeight.value }
      : { transform: [{ translateY: translateY.value }] };
  }, [isSnapping]);

  const createPanGesture = useCallback(
    () =>
      Gesture.Pan()
        .minDistance(8)
        .activeOffsetY([-12, 12])
        .failOffsetX([-20, 20])
        .onStart(() => {
          dragStart.value = isSnapping ? sheetHeight.value : translateY.value;
        })
        .onUpdate((event) => {
          if (isSnapping) {
            sheetHeight.value = Math.max(
              minSheetHeight,
              Math.min(maxSheetHeight, dragStart.value - event.translationY),
            );
          } else {
            translateY.value = Math.max(0, event.translationY);
          }
        })
        .onEnd((event) => {
          if (isSnapping) {
            const nextHeight = dragStart.value - event.translationY;
            const shouldClose =
              (nextHeight < minSheetHeight - height * 0.14 &&
                event.translationY > 0) ||
              (dragStart.value <= minSheetHeight + 1 && event.velocityY > 900);
            if (shouldClose) {
              sheetHeight.value = withTiming(
                0,
                { duration: 200, easing: Easing.out(Easing.cubic) },
                (finished) => {
                  if (finished) runOnJS(onClose)();
                },
              );
              return;
            }

            const middleHeight = (minSheetHeight + maxSheetHeight) / 2;
            let targetHeight =
              nextHeight < middleHeight ? minSheetHeight : maxSheetHeight;
            if (event.velocityY > 350) targetHeight = minSheetHeight;
            if (event.velocityY < -350) targetHeight = maxSheetHeight;
            sheetHeight.value = withSpring(targetHeight, {
              damping: 32,
              stiffness: 380,
              mass: 0.5,
              overshootClamping: true,
            });
          } else if (
            event.translationY > height * 0.18 ||
            event.velocityY > 800
          ) {
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
    [
      closedOffset,
      dragStart,
      height,
      isSnapping,
      maxSheetHeight,
      minSheetHeight,
      onClose,
      sheetHeight,
      translateY,
    ],
  );
  const contentPan = useMemo(() => createPanGesture(), [createPanGesture]);

  if (!shouldRender) return null;

  const content = (
    <Animated.View
      style={[
        styles.sheetContainer,
        {
          maxHeight: height * 0.92,
        },
        isSnapping ? styles.snappingSheetContainer : undefined,
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
        sheetHeight,
        onClose,
        theme,
        isDark,
        createPanGesture,
        isSnapping,
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
  children: ReactNode;
  showDragHandle?: boolean;
}

export function DrawerContent({
  children,
  showDragHandle = true,
}: DrawerContentProps) {
  const { theme, hasCloseButton, createPanGesture, isSnapping } = useDrawer();
  const insets = useSafeAreaInsets();
  const pan = useMemo(() => createPanGesture(), [createPanGesture]);

  const dragHandleNode = showDragHandle ? (
    <GestureDetector gesture={pan}>
      <View collapsable={false} style={styles.dragHandleContainer}>
        <View style={[styles.dragHandle, { backgroundColor: theme.border }]} />
      </View>
    </GestureDetector>
  ) : null;

  return (
    <View
      style={[
        styles.menuWrapper,
        isSnapping ? styles.snappingMenuWrapper : undefined,
        {
          borderColor: theme.border,
          backgroundColor: theme.card,
          paddingBottom: hasCloseButton
            ? 0
            : Math.max(insets.bottom, Platform.OS === "ios" ? 34 : 16),
        },
      ]}
    >
      <View
        style={[
          styles.bodyContent,
          isSnapping ? styles.snappingBodyContent : undefined,
        ]}
      >
        {dragHandleNode}
        {children}
      </View>
    </View>
  );
}

export function DrawerHeader({ children }: { children: ReactNode }) {
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

export function DrawerFooter({ children }: { children: ReactNode }) {
  const { theme } = useDrawer();
  return (
    <KeyboardStickyView style={styles.keyboardStickyFooter}>
      <View style={[styles.footer, { borderTopColor: theme.border }]}>
        {children}
      </View>
    </KeyboardStickyView>
  );
}

export function DrawerScrollView({
  children,
  ...props
}: KeyboardAwareScrollViewProps) {
  return (
    <KeyboardAwareScrollView
      keyboardShouldPersistTaps="handled"
      keyboardDismissMode="on-drag"
      showsVerticalScrollIndicator={false}
      {...props}
    >
      {children}
    </KeyboardAwareScrollView>
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
  snappingSheetContainer: {
    overflow: "hidden",
  },
  menuWrapper: {
    overflow: "hidden",
    borderTopWidth: StyleSheet.hairlineWidth,
    borderLeftWidth: StyleSheet.hairlineWidth,
    borderRightWidth: StyleSheet.hairlineWidth,
    borderBottomWidth: 0,
    borderTopLeftRadius: 14,
    borderTopRightRadius: 14,
  },
  snappingMenuWrapper: {
    flex: 1,
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
  snappingBodyContent: {
    flex: 1,
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
  keyboardStickyFooter: {
    width: "100%",
  },
  title: {
    ...{ fontSize: 16, lineHeight: 22, fontWeight: "600" },
    fontWeight: "700",
    textAlign: "center",
  },
  description: {
    ...{ fontSize: 11, lineHeight: 14, fontWeight: "500" },
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
    ...{ fontSize: 14, lineHeight: 20, fontWeight: "600" },
  },
  separator: {
    height: StyleSheet.hairlineWidth,
    marginLeft: 16,
  },
});
