/**
 * @module @orbis/mobile/components/floating-tab-bar
 *
 * Orbis Floating Tab Bar.
 *
 * Three independent floating controls — there is intentionally no shared
 * floating container:
 *
 *   [ Apps ]        [ Digital ID ]        [ Analytics ]
 *
 * The left and right controls navigate between tab routes as usual. The
 * center control does not navigate: it opens `DigitalIdDialog`, which
 * presents the signed-in user's digital ID card.
 */

import React from "react";
import {
  Pressable,
  StyleSheet,
  Text,
  View,
  type StyleProp,
  type ViewStyle,
} from "react-native";

import { Grid, Document, ChartBar } from "@orbis/icons/native";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { Fonts, Radii, Spacing, Typography } from "@/constants/theme";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

type TabRoute = {
  key: string;
  name: string;
  params?: unknown;
};

type TabBarState = {
  index: number;
  routes: TabRoute[];
};

type TabBarNavigation = {
  emit: (event: {
    type: string;
    target: string;
    canPreventDefault: boolean;
  }) => { defaultPrevented: boolean };

  navigate: (name: string, params?: unknown) => void;
};

type TabBarDescriptors = Record<string, { options: { title?: string } }>;

type FloatingTabBarProps = {
  state: TabBarState;
  navigation: TabBarNavigation;
  descriptors: TabBarDescriptors;
};

type FloatingControlProps = {
  children: React.ReactNode;
  onPress: () => void;
  accessibilityLabel: string;
  style?: StyleProp<ViewStyle>;
};

const ROUTE_APPS = "index";
const ROUTE_ANALYTICS = "behavior";

export function FloatingTabBar({ state, navigation }: FloatingTabBarProps) {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const router = useRouter();
  const { t } = useI18n();

  const bottom = Math.max(insets.bottom, Spacing.three) + Spacing.one;

  const findRouteIndex = (name: string) =>
    state.routes.findIndex((route) => route.name === name);

  const isRouteFocused = (name: string) => {
    const index = findRouteIndex(name);
    return index !== -1 && state.index === index;
  };

  const navigateToRoute = (name: string) => {
    const routeIndex = findRouteIndex(name);
    if (routeIndex === -1) {
      return;
    }

    const route = state.routes[routeIndex];
    const event = navigation.emit({
      type: "tabPress",
      target: route.key,
      canPreventDefault: true,
    });

    if (!event.defaultPrevented) {
      navigation.navigate(route.name, route.params);
    }
  };

  const isAppsFocused = isRouteFocused(ROUTE_APPS);
  const isAnalyticsFocused = isRouteFocused(ROUTE_ANALYTICS);

  const digitalIdLabel = t("tab_digital_id");

  // Shared floating surface — identical for every control.
  const surface = theme.cardElevated;
  const border = theme.cardBorder;
  const iconColor = theme.foreground;

  return (
    <View pointerEvents="box-none" style={[styles.navigation, { bottom }]}>
      {/* LEFT — Apps */}
      <View style={styles.appsWrapper}>
        <View
          pointerEvents="none"
          style={[
            styles.newBadge,
            { backgroundColor: theme.backgroundSelected },
          ]}
        >
          <Text
            style={[
              styles.newBadgeText,
              { color: theme.foreground, fontFamily: Fonts.sans },
            ]}
          >
            {t("badge_new")}
          </Text>
        </View>

        <FloatingControl
          accessibilityLabel="Apps"
          onPress={() => navigateToRoute(ROUTE_APPS)}
          style={[
            styles.iconButton,
            { backgroundColor: surface, borderColor: border },
          ]}
        >
          <Grid
            size={21}
            color={iconColor}
          />
        </FloatingControl>
      </View>

      {/* CENTER — Digital ID */}
      <FloatingControl
        accessibilityLabel="Digital ID"
        onPress={() => router.push("/Id")}
        style={[
          styles.digitalIdButton,
          { backgroundColor: surface, borderColor: border },
        ]}
      >
        <Document size={21} color={iconColor} />

        <Text
          numberOfLines={1}
          style={[
            styles.digitalIdText,
            { color: theme.foreground, fontFamily: Fonts.sans },
          ]}
        >
          {digitalIdLabel}
        </Text>
      </FloatingControl>

      {/* RIGHT — Analytics */}
      <FloatingControl
        accessibilityLabel="Analytics"
        onPress={() => navigateToRoute(ROUTE_ANALYTICS)}
        style={[
          styles.iconButton,
          { backgroundColor: surface, borderColor: border },
        ]}
      >
        <ChartBar
          size={21}
          color={iconColor}
        />
      </FloatingControl>
    </View>
  );
}

// ---------------------------------------------------------------------------
// Floating control
// ---------------------------------------------------------------------------

function FloatingControl({
  children,
  onPress,
  accessibilityLabel,
  style,
}: FloatingControlProps) {
  return (
    <Pressable
      accessibilityRole="button"
      accessibilityLabel={accessibilityLabel}
      onPress={onPress}
      style={({ pressed }) => [
        styles.control,
        style,
        pressed && styles.pressed,
      ]}
    >
      {children}
    </Pressable>
  );
}

// ---------------------------------------------------------------------------
// Styles
// ---------------------------------------------------------------------------

const styles = StyleSheet.create({
  /**
   * The navigation container is only a layout container.
   * No background, no border, no shadow.
   */
  navigation: {
    position: "absolute",
    left: Spacing.four,
    right: Spacing.four,
    minHeight: 48,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    gap: Spacing.two,
    zIndex: 100,
  },

  /** Shared floating surface. */
  control: {
    alignItems: "center",
    justifyContent: "center",
    borderWidth: StyleSheet.hairlineWidth,
    elevation: 4,
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.07,
    shadowRadius: 6,
  },

  /** LEFT + RIGHT. 48 × 48. */
  iconButton: {
    width: 48,
    height: 48,
    flexShrink: 0,
    borderRadius: Radii.full,
  },

  /** Wrapper only exists so NEW can sit above Apps. */
  appsWrapper: {
    width: 48,
    height: 48,
    flexShrink: 0,
    alignItems: "center",
    justifyContent: "center",
    position: "relative",
  },

  /** NEW badge — small and visually subordinate to the button. */
  newBadge: {
    position: "absolute",
    top: -16,
    left: -2,
    minWidth: 58,
    height: 30,
    paddingHorizontal: Spacing.two,
    borderRadius: Radii.full,
    alignItems: "center",
    justifyContent: "center",
    zIndex: 10,
  },

  newBadgeText: {
    ...Typography.small,
    fontSize: 12,
    lineHeight: 14,
    fontWeight: "500",
    letterSpacing: 0.1,
  },

  /** CENTER. Flexible width, typical 140–160 × 44. */
  digitalIdButton: {
    flex: 1,
    height: 44,
    minWidth: 120,
    maxWidth: 160,
    paddingHorizontal: Spacing.three,
    borderRadius: Radii.full,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    gap: Spacing.one,
  },

  digitalIdText: {
    ...Typography.body,
    fontSize: 14,
    lineHeight: 18,
    fontWeight: "500",
    flexShrink: 1,
  },

  pressed: {
    transform: [{ scale: 0.96 }],
  },
});

export default FloatingTabBar;
