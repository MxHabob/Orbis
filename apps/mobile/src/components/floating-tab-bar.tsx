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

import React, { useEffect } from "react";
import { Keyboard, Platform, Pressable, View } from "react-native";
import Animated, {
  cancelAnimation,
  useAnimatedStyle,
  useReducedMotion,
  useSharedValue,
  withTiming,
} from "react-native-reanimated";

import { ChartBar, Document, Grid, Sparkles } from "@orbis/icons/native";
import type { BottomTabBarProps } from "expo-router/tabs";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { FloatingActionButton } from "@/components/ui/floating-action-button";
import { useSystemSettingsQuery } from "@/hooks/use-api-queries";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";
import { Text } from "@/components/ui/text";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export type FloatingTabBarProps = BottomTabBarProps;

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

export function FloatingTabBar({
  state,
  emitter,
  navigateToTab,
}: FloatingTabBarProps) {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const router = useRouter();
  const { t, isArabic } = useI18n();
  const reduceMotion = useReducedMotion();
  const { data: systemSettings } = useSystemSettingsQuery();
  const isAiChatEnabled = Boolean(systemSettings?.ai?.mobile_chat_enabled);

  const translateY = useSharedValue(0);
  const opacity = useSharedValue(1);

  useEffect(() => {
    const showEvent =
      Platform.OS === "ios" ? "keyboardWillShow" : "keyboardDidShow";
    const hideEvent =
      Platform.OS === "ios" ? "keyboardWillHide" : "keyboardDidHide";

    const showSub = Keyboard.addListener(showEvent, () => {
      if (reduceMotion) {
        translateY.value = 80;
        opacity.value = 0;
        return;
      }
      translateY.value = withTiming(80, { duration: 200 });
      opacity.value = withTiming(0, { duration: 150 });
    });

    const hideSub = Keyboard.addListener(hideEvent, () => {
      if (reduceMotion) {
        translateY.value = 0;
        opacity.value = 1;
        return;
      }
      translateY.value = withTiming(0, { duration: 250 });
      opacity.value = withTiming(1, { duration: 200 });
    });

    return () => {
      showSub.remove();
      hideSub.remove();
      cancelAnimation(translateY);
      cancelAnimation(opacity);
    };
  }, [opacity, reduceMotion, translateY]);

  const barAnimatedStyle = useAnimatedStyle(() => ({
    transform: [{ translateY: translateY.value }],
    opacity: opacity.value,
  }));

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
    const event = emitter.emit({
      type: "tabPress",
      target: route.key,
      canPreventDefault: true,
    });

    if (!event.defaultPrevented) {
      navigateToTab(route.key);
    }
  };

  const isAppsFocused = isRouteFocused(ROUTE_APPS);
  const isAnalyticsFocused = isRouteFocused(ROUTE_ANALYTICS);

  const appsLabel = t("tab_home");
  const digitalIdLabel = t("tab_digital_id");
  const analyticsLabel = t("tab_behavior_ai");

  return (
    <>
      {isAiChatEnabled ? (
        <Animated.View
          pointerEvents="box-none"
          className="absolute inset-x-0 z-50"
          style={[{ bottom: bottom + 68 }, barAnimatedStyle]}
        >
          <FloatingActionButton
            icon={<Sparkles size={22} color={theme.primaryForeground} />}
            accessibilityLabel={t("ai_chat_title")}
            position={isArabic ? "bottom-left" : "bottom-right"}
            offsetBottom={0}
            offsetHorizontal={20}
            onPress={() => router.push("/(app)/ai-chat")}
          />
        </Animated.View>
      ) : null}

      <Animated.View
        pointerEvents="box-none"
        className="absolute left-five right-five flex-row items-center justify-between z-50 min-h-14"
        style={[{ bottom }, barAnimatedStyle]}
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
    </>
  );
}

export default FloatingTabBar;
