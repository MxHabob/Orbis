/**
 * @module @orbis/mobile/components/ui/drop-down-menu
 *
 * S.C.R.A.P.S. Tactile Dropdown Menu for Orbis Mobile.
 * Built with native Modal positioning, Reanimated spring curves, and design tokens.
 */

import React, {
  type ComponentProps,
  cloneElement,
  createContext,
  isValidElement,
  type ReactElement,
  type ReactNode,
  useCallback,
  useContext,
  useRef,
  useState,
} from "react";
import {
  Modal,
  Platform,
  Pressable,
  StyleSheet,
  Text,
  TouchableOpacity,
  useWindowDimensions,
  View,
} from "react-native";
import Animated, {
  Easing,
  useAnimatedStyle,
  useSharedValue,
  withSpring,
  withTiming,
} from "react-native-reanimated";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { CheckCircle } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";

// ─── Types ──────────────────────────────────────────────────────────────────────

export type DropdownMenuItemProps = ComponentProps<typeof TouchableOpacity> & {
  icon?: ReactNode | string;
  avatar?: ReactNode;
  selected?: boolean;
  disabled?: boolean;
  onSelect?: () => void;
  children: ReactNode;
};

export type DropdownMenuTriggerProps = {
  children: ReactNode;
  asChild?: boolean;
};

export type DropdownMenuContentProps = {
  children: ReactNode;
  menuWidth?: number;
};

interface DropdownMenuContextValue {
  open: boolean;
  openMenu: () => void;
  closeMenu: (callback?: () => void) => void;
  triggerRef: React.RefObject<View | null>;
  menuPosition: {
    top: number;
    left?: number;
    right?: number;
    width: number;
  };
}

const DropdownMenuContext = createContext<DropdownMenuContextValue | null>(
  null,
);

// ─── Trigger Measurement ────────────────────────────────────────────────────────

function measureTrigger(
  ref: React.RefObject<View | null>,
  callback: (x: number, y: number, width: number, height: number) => void,
) {
  if (!ref.current) return;
  ref.current.measureInWindow((x, y, width, height) => {
    callback(x, y, width, height);
  });
}

// ─── DropdownMenu Root ──────────────────────────────────────────────────────────

export type DropdownMenuProps = {
  children: React.ReactNode;
  itemCount?: number;
  menuWidth?: number;
};

export const DropdownMenu = ({
  children,
  itemCount = 1,
  menuWidth = 240,
}: DropdownMenuProps) => {
  const [visible, setVisible] = useState(false);
  const triggerRef = useRef<View>(null);
  const { width: screenWidth, height: screenHeight } = useWindowDimensions();
  const insets = useSafeAreaInsets();

  const [menuPosition, setMenuPosition] = useState<{
    top: number;
    left?: number;
    right?: number;
    width: number;
  }>({
    top: 0,
    left: undefined,
    right: undefined,
    width: menuWidth,
  });

  const openMenu = useCallback(() => {
    if (visible) return;

    measureTrigger(triggerRef, (x, y, width, height) => {
      const itemHeight = 48;
      const verticalPadding = 16;
      const verticalOffset = 6;
      const minHorizontalGap = 16;
      const menuHeight = itemCount * itemHeight + verticalPadding;

      // Vertical: prefer below trigger
      let top = y + height + verticalOffset;
      if (top + menuHeight > screenHeight - insets.bottom - 20) {
        top = y - menuHeight - verticalOffset;
      }
      if (top < insets.top + 10) {
        top = insets.top + 10;
      }

      // Horizontal: center on trigger or snap within bounds
      const centerX = x + width / 2;
      let left: number | undefined = centerX - menuWidth / 2;

      if (left < minHorizontalGap) {
        left = minHorizontalGap;
      } else if (left + menuWidth > screenWidth - minHorizontalGap) {
        left = screenWidth - menuWidth - minHorizontalGap;
      }

      setMenuPosition({ top, left, width: menuWidth });
      setVisible(true);
    });
  }, [visible, itemCount, menuWidth, screenWidth, screenHeight, insets]);

  const closeMenu = useCallback((callback?: () => void) => {
    setVisible(false);
    callback?.();
  }, []);

  const contextValue: DropdownMenuContextValue = {
    open: visible,
    openMenu,
    closeMenu,
    triggerRef,
    menuPosition,
  };

  return (
    <DropdownMenuContext.Provider value={contextValue}>
      <View>{children}</View>
    </DropdownMenuContext.Provider>
  );
};

// ─── DropdownMenuTrigger ────────────────────────────────────────────────────────

export const DropdownMenuTrigger = ({
  children,
  asChild = false,
}: DropdownMenuTriggerProps) => {
  const context = useContext(DropdownMenuContext);

  if (!context) {
    return null;
  }

  const handlePress = () => context.openMenu();
  const suppressLongPress = () => undefined;

  if (asChild && isValidElement(children)) {
    return (
      <View ref={context.triggerRef} collapsable={false}>
        {cloneElement(children as ReactElement<any>, {
          onPress: (event: any) => {
            (children as ReactElement<any>).props?.onPress?.(event);
            handlePress();
          },
          onLongPress: suppressLongPress,
          delayLongPress: 1000000,
        })}
      </View>
    );
  }

  return (
    <View ref={context.triggerRef} collapsable={false}>
      <TouchableOpacity
        activeOpacity={0.7}
        onPress={handlePress}
        onLongPress={suppressLongPress}
        delayLongPress={1000000}
        accessibilityRole="button"
        accessibilityLabel="Open dropdown menu"
      >
        {children}
      </TouchableOpacity>
    </View>
  );
};

// ─── DropdownMenuContent ────────────────────────────────────────────────────────

export const DropdownMenuContent = ({
  children,
  menuWidth = 240,
}: DropdownMenuContentProps) => {
  const theme = useTheme();
  const context = useContext(DropdownMenuContext);

  const animProgress = useSharedValue(0);

  React.useEffect(() => {
    if (context?.open) {
      animProgress.value = withSpring(1, {
        damping: 22,
        stiffness: 340,
        mass: 0.6,
      });
    } else {
      animProgress.value = withTiming(0, {
        duration: 120,
        easing: Easing.out(Easing.ease),
      });
    }
  }, [context?.open, animProgress]);

  const animatedMenuStyle = useAnimatedStyle(() => ({
    opacity: animProgress.value,
    transform: [
      { scale: 0.94 + animProgress.value * 0.06 },
      { translateY: -6 + animProgress.value * 6 },
    ],
  }));

  if (!context) {
    return null;
  }

  const { open, closeMenu, menuPosition } = context;

  if (!open) return null;

  return (
    <Modal
      visible={open}
      transparent
      animationType="none"
      statusBarTranslucent
      onRequestClose={() => closeMenu()}
    >
      <DropdownMenuContext.Provider value={context}>
        <View style={styles.modalContainer}>
          {/* Backdrop */}
          <Pressable
            style={StyleSheet.absoluteFill}
            onPress={() => closeMenu()}
            accessibilityRole="button"
            accessibilityLabel="Close dropdown"
          />

          {/* Menu Content */}
          <Animated.View
            collapsable={false}
            accessibilityRole="menu"
            style={[
              styles.menuContainer,
              {
                top: menuPosition.top,
                left: menuPosition.left,
                right: menuPosition.right,
                width: menuPosition.width || menuWidth,
                borderColor: theme.border,
                backgroundColor: theme.card,
              },
              animatedMenuStyle,
            ]}
          >
            <View style={styles.menuBody}>{children}</View>
          </Animated.View>
        </View>
      </DropdownMenuContext.Provider>
    </Modal>
  );
};

// ─── DropdownMenuItem ───────────────────────────────────────────────────────────

export const DropdownMenuItem = ({
  children,
  avatar,
  selected = false,
  disabled = false,
  onSelect,
  onPress,
  style,
  ...rest
}: DropdownMenuItemProps) => {
  const theme = useTheme();
  const context = useContext(DropdownMenuContext);

  if (!context) {
    return null;
  }

  const handlePress = () => {
    if (disabled) return;
    onSelect?.();
    context.closeMenu();
    onPress?.({} as any);
  };

  return (
    <TouchableOpacity
      activeOpacity={0.72}
      onPress={handlePress}
      disabled={disabled}
      accessibilityRole="menuitem"
      accessibilityState={{ selected, disabled }}
      style={[
        styles.menuItem,
        selected && { backgroundColor: theme.accentLight },
        disabled && { opacity: 0.5 },
        style,
      ]}
      {...rest}
    >
      <View style={styles.menuItemLeading}>
        {avatar ? <View style={styles.menuAvatar}>{avatar}</View> : null}
        <Text
          style={[
            styles.menuItemText,
            {
              color: selected ? theme.primary : theme.text,
              fontWeight: selected ? "700" : "500",
            },
          ]}
        >
          {children}
        </Text>
      </View>

      {selected ? (
        <CheckCircle size={18} color={theme.primary} />
      ) : null}
    </TouchableOpacity>
  );
};

// ─── DropdownMenuSeparator ──────────────────────────────────────────────────────

export const DropdownMenuSeparator = () => {
  const theme = useTheme();

  return <View style={[styles.separator, { backgroundColor: theme.border }]} />;
};

// ─── DropdownMenuLabel ──────────────────────────────────────────────────────────

export const DropdownMenuLabel = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  const theme = useTheme();

  return (
    <View style={styles.labelContainer}>
      <Text style={[styles.labelText, { color: theme.textSecondary }]}>
        {children}
      </Text>
    </View>
  );
};

// Legacy alias for backwards compatibility
export const DropDownMenu = DropdownMenu;

// ─── Styles ─────────────────────────────────────────────────────────────────────

const styles = StyleSheet.create({
  modalContainer: {
    position: "absolute",
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
    zIndex: 99998,
    elevation: 99998,
  },
  menuContainer: {
    position: "absolute",
    borderRadius: 12,
    overflow: "hidden",
    borderWidth: 1,
    ...Platform.select({
      ios: {
        shadowColor: "#000000",
        shadowOffset: { width: 0, height: 6 },
        shadowOpacity: 0.16,
        shadowRadius: 18,
      },
      android: {
        elevation: 10,
      },
    }),
  },
  menuBody: {
    paddingVertical: 4,
  },
  menuItem: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    paddingHorizontal: 16,
    paddingVertical: 12,
    marginHorizontal: 4,
    borderRadius: 6,
    minHeight: 44,
  },
  menuItemLeading: {
    flexDirection: "row",
    alignItems: "center",
    gap: 8,
    flex: 1,
  },
  menuItemText: {
    ...{"fontSize":14,"lineHeight":20,"fontWeight":"400"},
    flexShrink: 1,
  },
  menuAvatar: {
    marginRight: 0,
  },
  separator: {
    height: StyleSheet.hairlineWidth,
    marginHorizontal: 16,
    marginVertical: 4,
  },
  labelContainer: {
    paddingHorizontal: 16,
    paddingTop: 8,
    paddingBottom: 4,
  },
  labelText: {
    ...{"fontSize":12,"lineHeight":16,"fontWeight":"600"},
    textTransform: "uppercase",
    letterSpacing: 0.5,
  },
});
