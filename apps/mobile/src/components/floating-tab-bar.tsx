/**
 * @module @orbis/mobile/components/floating-tab-bar
 *
 * Orbis Floating Tab Bar.
 *
 * - Built entirely with NativeWind utility classes and 100% synchronized Orbis design tokens.
 * - Zero StyleSheet usage.
 * - Uses official icons from @orbis/icons/native.
 * - Matches the reference aesthetic and desktop design tokens exactly:
 *   [ Apps / Home (with desktop lime badge) ]    [ Digital ID ]    [ Analytics / Behavior AI ]
 */

import React, { useEffect, useRef } from "react";
import {
  Animated,
  Keyboard,
  Platform,
  Pressable,
  Text,
  View,
} from "react-native";

import { ChartBar, Document, Grid } from "@orbis/icons/native";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";

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

const ROUTE_APPS = "index";
const ROUTE_ANALYTICS = "behavior";

// Ambient shadow for floating controls matching desktop card elevation
const floatingShadow = {
  shadowColor: "#000000",
  shadowOffset: { width: 0, height: 4 },
  shadowOpacity: 0.08,
  shadowRadius: 12,
  elevation: 4,
};

const pressedScale = { transform: [{ scale: 0.94 }] };

export function FloatingTabBar({ state, navigation }: FloatingTabBarProps) {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const router = useRouter();
  const { t, isArabic } = useI18n();

  const translateY = useRef(new Animated.Value(0)).current;
  const opacity = useRef(new Animated.Value(1)).current;

  useEffect(() => {
    const showEvent =
      Platform.OS === "ios" ? "keyboardWillShow" : "keyboardDidShow";
    const hideEvent =
      Platform.OS === "ios" ? "keyboardWillHide" : "keyboardDidHide";

    const showSub = Keyboard.addListener(showEvent, () => {
      Animated.parallel([
        Animated.timing(translateY, {
          toValue: 80,
          duration: 200,
          useNativeDriver: true,
        }),
        Animated.timing(opacity, {
          toValue: 0,
          duration: 150,
          useNativeDriver: true,
        }),
      ]).start();
    });

    const hideSub = Keyboard.addListener(hideEvent, () => {
      Animated.parallel([
        Animated.timing(translateY, {
          toValue: 0,
          duration: 250,
          useNativeDriver: true,
        }),
        Animated.timing(opacity, {
          toValue: 1,
          duration: 200,
          useNativeDriver: true,
        }),
      ]).start();
    });

    return () => {
      showSub.remove();
      hideSub.remove();
    };
  }, [opacity, translateY]);

  const bottom = Math.max(insets.bottom, 12) + 4;

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

  const appsLabel = t("tab_home");
  const digitalIdLabel = t("tab_digital_id");
  const analyticsLabel = t("tab_behavior_ai");

  return (
    <Animated.View
      pointerEvents="box-none"
      className="absolute left-five right-five flex-row items-center justify-between z-50 min-h-14"
      style={{
        bottom,
        transform: [{ translateY }],
        opacity,
      }}
    >
      {/* LEFT — Apps (Home) with NEW badge synchronized with desktop badge-brand-lime */}
      <View className="w-14 h-14 relative items-center justify-center shrink-0">
        <View
          pointerEvents="none"
          className={`absolute -top-[9px] ${
            isArabic ? "-right-1" : "-left-1"
          } h-[22px] px-[9px] rounded-full items-center justify-center z-10 border border-brand-lime/30 bg-brand-lime/15`}
        >
          <Text className="text-[11px] leading-[14px] font-bold tracking-wider uppercase text-brand-lime-dark">
            {t("badge_new")}
          </Text>
        </View>

        <Pressable
          accessibilityRole="button"
          accessibilityState={{ selected: isAppsFocused }}
          accessibilityLabel={appsLabel}
          onPress={() => navigateToRoute(ROUTE_APPS)}
          className="w-14 h-14 rounded-full bg-card items-center justify-center shrink-0"
          style={({ pressed }) => [floatingShadow, pressed && pressedScale]}
        >
          <Grid size={22} color={theme.foreground} strokeWidth={1.8} />
        </Pressable>
      </View>

      {/* CENTER — Digital ID (Document icon + Label) */}
      <Pressable
        accessibilityRole="button"
        accessibilityLabel={digitalIdLabel}
        onPress={() => router.push("/Id")}
        className="h-14 min-w-[140px] px-five rounded-full bg-card flex-row items-center justify-center gap-two shrink-0"
        style={({ pressed }) => [floatingShadow, pressed && pressedScale]}
      >
        <Document size={21} color={theme.foreground} strokeWidth={1.8} />
        <Text
          numberOfLines={1}
          className="text-[15px] leading-5 font-medium text-foreground shrink"
        >
          {digitalIdLabel}
        </Text>
      </Pressable>

      {/* RIGHT — Analytics (Behavior & AI) */}
      <Pressable
        accessibilityRole="button"
        accessibilityState={{ selected: isAnalyticsFocused }}
        accessibilityLabel={analyticsLabel}
        onPress={() => navigateToRoute(ROUTE_ANALYTICS)}
        className="w-14 h-14 rounded-full bg-card items-center justify-center shrink-0"
        style={({ pressed }) => [floatingShadow, pressed && pressedScale]}
      >
        <ChartBar size={21} color={theme.foreground} strokeWidth={1.8} />
      </Pressable>
    </Animated.View>
  );
}

export default FloatingTabBar;
