// components/header.tsx

/**
 * @module @orbis/mobile/components/header
 *
 * Orbis screen header.
 *
 * A left floating control (back / profile), an optional centered floating
 * title pill, and a right floating control (custom action / crown).
 */

import { ChevronLeft, ChevronRight, User, AppNotification } from "@orbis/icons/native";
import { useRouter } from "expo-router";
import React from "react";
import {
  Animated,
  Easing,
  Pressable,
  StyleProp,
  StyleSheet,
  Text,
  View,
  ViewStyle,
  useWindowDimensions,
} from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

type HeaderProps = {
  title?: string;
  subtitle?: string;

  showBack?: boolean;
  onBack?: () => void;

  onProfilePress?: () => void;
  onCrownPress?: () => void;

  leftAction?: React.ReactNode;
  rightAction?: React.ReactNode;

  largeTitle?: boolean;

  /**
   * Kept for compatibility with existing screens.
   * Progress is no longer displayed in the header.
   */
  progress?: number;
  progressTotal?: number;
  onProgressPress?: () => void;

  /** Show the center page title. */
  showTitle?: boolean;

  /** Show crown on the right. */
  showCrown?: boolean;

  transparent?: boolean;

  style?: StyleProp<ViewStyle>;
  children?: React.ReactNode;
};

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

export function Header({
  title,
  subtitle,

  showBack = false,
  onBack,

  onProfilePress,
  onCrownPress,

  leftAction,
  rightAction,

  largeTitle = false,

  showTitle = true,
  showCrown = true,
  transparent = false,

  style,
  children,
}: HeaderProps) {

  const router = useRouter();
  const theme = useTheme();
  const insets = useSafeAreaInsets();
  const { width } = useWindowDimensions();
  const { t, isArabic } = useI18n();

  const sideSize = width < 360 ? 46 : 48;

  const titleWidth = Math.min(Math.max(width * 0.42, 128), 160);

  const marqueeX = React.useRef(new Animated.Value(0)).current;
  const estimatedTitleWidth = Math.max((title?.length ?? 0) * 8.6, 0);
  const shouldAnimateTitle = Boolean(title) && estimatedTitleWidth > titleWidth;

  React.useEffect(() => {
    if (!shouldAnimateTitle) {
      marqueeX.stopAnimation();
      marqueeX.setValue(0);
      return;
    }

    const distance = Math.max(estimatedTitleWidth - titleWidth + 18, 20);

    const animation = Animated.loop(
      Animated.sequence([
        Animated.timing(marqueeX, {
          toValue: -distance,
          duration: 3800 + distance * 14,
          easing: Easing.linear,
          useNativeDriver: true,
        }),
        Animated.delay(800),
        Animated.timing(marqueeX, {
          toValue: 0,
          duration: 0,
          useNativeDriver: true,
        }),
      ]),
    );

    animation.start();

    return () => {
      animation.stop();
    };
  }, [estimatedTitleWidth, marqueeX, shouldAnimateTitle, titleWidth]);

  const handleBack = () => {
    if (onBack) {
      onBack();
      return;
    }

    router.back();
  };

  const handleProfile = () => {
    if (onProfilePress) {
      onProfilePress();
      return;
    }

    router.push("/profile");
  };

  const handleCrown = () => {
    if (onCrownPress) {
      onCrownPress();
      return;
    }

    router.push("/notifications");
  };

  const leftLabel = showBack ? t("header_back") : t("header_profile");

  return (
    <View
      className="z-10 w-full bg-transparent"
      style={[
        {
          paddingTop: insets.top + 8,
          paddingHorizontal: 16,
        },
        style,
      ]}
    >
      <View className="min-h-12 w-full flex-row items-center justify-between gap-2">
        {/* LEFT */}
        <View className="shrink-0 items-center justify-center" style={{ width: sideSize, height: sideSize }}>
          {leftAction ? (
            leftAction
          ) : (
            <Pressable
              accessibilityRole="button"
              accessibilityLabel={leftLabel}
              hitSlop={8}
              onPress={showBack ? handleBack : handleProfile}
              style={({ pressed }) => [
                styles.floatingSurface,
                {
                  width: sideSize,
                  height: sideSize,
                  borderRadius: sideSize / 2,
                  backgroundColor: transparent
                    ? "rgba(255,255,255,0.10)"
                    : theme.card,
                  borderColor: transparent
                    ? "rgba(255,255,255,0.16)"
                    : theme.border,
                  shadowColor: theme.text,
                },
                pressed && { transform: [{ scale: 0.94 }] },
              ]}
              className="items-center justify-center"
            >
              {showBack ? (
                isArabic ? (
                  <ChevronRight
                    size={21}
                    color={transparent ? "#FFFFFF" : theme.text}
                  />
                ) : (
                  <ChevronLeft
                    size={21}
                    color={transparent ? "#FFFFFF" : theme.text}
                  />
                )
              ) : (
                <User
                  size={21}
                  color={transparent ? "#FFFFFF" : theme.text}
                />
              )}
            </Pressable>
          )}
        </View>

        {/* CENTER — PAGE TITLE */}
        <View className="mx-three h-12 flex-1 items-center justify-center">
          {showTitle && title && !largeTitle ? (
            <View
              className="h-11 items-center justify-center overflow-hidden rounded-full px-3"
              style={[
                styles.floatingSurface,
                {
                  width: titleWidth,
                  backgroundColor: transparent
                    ? "rgba(255,255,255,0.10)"
                    : theme.card,
                  borderColor: transparent
                    ? "rgba(255,255,255,0.16)"
                    : theme.border,
                  shadowColor: theme.text,
                },
              ]}
            >
              <View className="max-w-full items-center justify-center overflow-hidden">
                <Animated.View
                  pointerEvents="none"
                  style={[
                    {
                      width: shouldAnimateTitle ? estimatedTitleWidth + 28 : "100%",
                      transform: [{ translateX: shouldAnimateTitle ? marqueeX : 0 }],
                    },
                  ]}
                >
                  <Text
                    numberOfLines={1}
                    ellipsizeMode="tail"
                    className="text-center font-semibold"
                    style={{ color: transparent ? "#FFFFFF" : theme.text }}
                  >
                    {title}
                  </Text>
                </Animated.View>

                {subtitle ? (
                  <Text
                    numberOfLines={1}
                    ellipsizeMode="tail"
                    className="mt-px text-center"
                    style={{ color: transparent ? "rgba(255,255,255,0.72)" : theme.textSecondary }}
                  >
                    {subtitle}
                  </Text>
                ) : null}
              </View>
            </View>
          ) : null}
        </View>

        {/* RIGHT */}
        <View className="shrink-0 items-center justify-center" style={{ width: sideSize, height: sideSize }}>
          {rightAction ? (
            rightAction
          ) : showCrown ? (
            <Pressable
              accessibilityRole="button"
              accessibilityLabel="Notifications"
              hitSlop={8}
              onPress={handleCrown}
              style={({ pressed }) => [
                styles.floatingSurface,
                {
                  width: sideSize,
                  height: sideSize,
                  borderRadius: sideSize / 2,
                  backgroundColor: transparent
                    ? "rgba(255,255,255,0.10)"
                    : theme.card,
                  borderColor: transparent
                    ? "rgba(255,255,255,0.16)"
                    : theme.border,
                  shadowColor: theme.text,
                  opacity: 1,
                },
                pressed && { transform: [{ scale: 0.94 }] },
              ]}
              className="items-center justify-center"
            >
              <AppNotification
                size={20}
                color={transparent ? "#FFFFFF" : theme.text}
              />
            </Pressable>
          ) : (
            <View className="shrink-0" style={{ width: sideSize, height: sideSize }} />
          )}
        </View>
      </View>

      {largeTitle && title ? (
        <View className="mt-3 px-1">
          <Text
            style={[
              { color: transparent ? "#FFFFFF" : theme.text, ...{"fontSize":24,"lineHeight":30,"fontWeight":"800"} },
            ]}
          >
            {title}
          </Text>
          {subtitle ? (
            <Text
              style={[
                {
                  color: transparent
                    ? "rgba(255,255,255,0.72)"
                    : theme.textSecondary,
                  ...{"fontSize":13,"lineHeight":18,"fontWeight":"400"},
                  marginTop: 2,
                },
              ]}
            >
              {subtitle}
            </Text>
          ) : null}
        </View>
      ) : null}

      {children}
    </View>
  );

}

// ---------------------------------------------------------------------------
// Styles
// ---------------------------------------------------------------------------

const styles = StyleSheet.create({
  floatingSurface: {
    borderWidth: StyleSheet.hairlineWidth,
    elevation: 4,
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.07,
    shadowRadius: 6,
  },
});

export default Header;
