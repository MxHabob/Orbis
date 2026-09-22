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

import { Colors, Fonts, Radii, Spacing, Typography } from "@/constants/theme";
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

  const titleWidth = Math.min(Math.max(width * 0.42, 128), 190);

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
      style={[
        styles.container,
        {
          paddingTop: insets.top + Spacing.two,
          paddingHorizontal: Spacing.three,
        },
        style,
      ]}
    >
      <View style={styles.content}>
        {/* LEFT */}
        <View style={[styles.sideSlot, { width: sideSize, height: sideSize }]}>
          {leftAction ? (
            leftAction
          ) : (
            <Pressable
              accessibilityRole="button"
              accessibilityLabel={leftLabel}
              hitSlop={8}
              onPress={showBack ? handleBack : handleProfile}
              style={({ pressed }) => [
                styles.floatingControl,
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
                  shadowColor: Colors.light.text,
                },
                pressed && styles.pressed,
              ]}
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
        <View style={styles.centerSlot}>
          {showTitle && title && !largeTitle ? (
            <View
              style={[
                styles.titleControl,
                {
                  width: titleWidth,
                  backgroundColor: transparent
                    ? "rgba(255,255,255,0.10)"
                    : theme.card,
                  borderColor: transparent
                    ? "rgba(255,255,255,0.16)"
                    : theme.border,
                  shadowColor: Colors.light.text,
                },
              ]}
            >
              <View style={styles.titleContent}>
                <Animated.View
                  pointerEvents="none"
                  style={[
                    styles.titleMarquee,
                    {
                      width: shouldAnimateTitle ? estimatedTitleWidth + 28 : "100%",
                      transform: [{ translateX: shouldAnimateTitle ? marqueeX : 0 }],
                    },
                  ]}
                >
                  <Text
                    numberOfLines={1}
                    ellipsizeMode="tail"
                    style={[styles.title, { color: transparent ? "#FFFFFF" : theme.text }]}
                  >
                    {title}
                  </Text>
                </Animated.View>

                {subtitle ? (
                  <Text
                    numberOfLines={1}
                    ellipsizeMode="tail"
                    style={[
                      styles.subtitle,
                      { color: transparent ? "rgba(255,255,255,0.72)" : theme.textSecondary },
                    ]}
                  >
                    {subtitle}
                  </Text>
                ) : null}
              </View>
            </View>
          ) : null}
        </View>

        {/* RIGHT */}
        <View style={[styles.sideSlot, { width: sideSize, height: sideSize }]}>
          {rightAction ? (
            rightAction
          ) : showCrown ? (
            <Pressable
              accessibilityRole="button"
              accessibilityLabel="Notifications"
              hitSlop={8}
              onPress={handleCrown}
              style={({ pressed }) => [
                styles.floatingControl,
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
                  shadowColor: Colors.light.text,
                  opacity: 1,
                },
                pressed && styles.pressed,
              ]}
            >
              <AppNotification
                size={20}
                color={transparent ? "#FFFFFF" : theme.text}
              />
            </Pressable>
          ) : (
            <View style={{ width: sideSize, height: sideSize }} />
          )}
        </View>
      </View>

      {largeTitle && title ? (
        <View style={styles.largeTitleContainer}>
          <Text
            style={[
              styles.largeTitleText,
              { color: transparent ? "#FFFFFF" : theme.text },
            ]}
          >
            {title}
          </Text>
          {subtitle ? (
            <Text
              style={[
                styles.largeSubtitleText,
                {
                  color: transparent
                    ? "rgba(255,255,255,0.72)"
                    : theme.textSecondary,
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
  container: {
    width: "100%",
    backgroundColor: "transparent",
    zIndex: 10,
  },

  content: {
    width: "100%",
    minHeight: 48,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
  },

  sideSlot: {
    flexShrink: 0,
    alignItems: "center",
    justifyContent: "center",
  },

  centerSlot: {
    flex: 1,
    height: 48,
    marginHorizontal: Spacing.three,
    alignItems: "center",
    justifyContent: "center",
  },

  floatingControl: {
    alignItems: "center",
    justifyContent: "center",
    borderWidth: StyleSheet.hairlineWidth,
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.055,
    shadowRadius: 4,
    elevation: 2,
  },

  titleControl: {
    height: 40,
    paddingHorizontal: Spacing.four,
    borderRadius: Radii.full,
    borderWidth: StyleSheet.hairlineWidth,
    alignItems: "center",
    justifyContent: "center",
    overflow: "hidden",
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.055,
    shadowRadius: 4,
    elevation: 2,
  },

  titleContent: {
    alignItems: "center",
    justifyContent: "center",
    maxWidth: "100%",
    overflow: "hidden",
  },

  titleMarquee: {
    alignSelf: "center",
  },

  title: {
    ...Typography.bodyBold,
    fontFamily: Fonts.sans,
    textAlign: "center",
  },

  subtitle: {
    ...Typography.small,
    fontFamily: Fonts.sans,
    textAlign: "center",
    marginTop: 1,
  },

  pressed: {
    transform: [{ scale: 0.94 }],
  },

  largeTitleContainer: {
    marginTop: Spacing.three,
    paddingHorizontal: Spacing.one,
  },

  largeTitleText: {
    ...Typography.h1,
    fontFamily: Fonts.sans,
  },

  largeSubtitleText: {
    ...Typography.bodySmall,
    fontFamily: Fonts.sans,
    marginTop: Spacing.half,
  },
});

export default Header;

