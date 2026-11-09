// components/header.tsx

/**
 * @module @orbis/mobile/components/header
 *
 * Orbis screen header.
 *
 * A minimal safe-area-aware bar with a centered title and optional circular
 * actions. The container itself is transparent so content can scroll beneath.
 */

import {
  AppNotification,
  ChevronLeft,
  ChevronRight,
} from "@orbis/icons/native";
import { GlassView, isLiquidGlassAvailable } from "expo-glass-effect";
import { useRouter } from "expo-router";
import type React from "react";
import {
  Platform,
  Pressable,
  type StyleProp,
  StyleSheet,
  Text,
  View,
  type ViewStyle,
} from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Avatar } from "@/components/ui/avatar";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useUnreadCountQuery } from "@/hooks/use-api-queries";

export const HEADER_CONTENT_OFFSET = 36;

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
  showCrown = false,

  style,
  children,
}: HeaderProps) {
  const router = useRouter();
  const theme = useTheme();
  const insets = useSafeAreaInsets();
  const { user } = useAuth();
  const { t, isArabic } = useI18n();
  const useNativeGlass = Platform.OS === "ios" && isLiquidGlassAvailable();

  const sideSize = 44;
  const headerHeight =
    insets.top + HEADER_CONTENT_OFFSET + (largeTitle && title ? 58 : 0);

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

  const { data: unreadCount } = useUnreadCountQuery();
  const leftLabel = showBack ? t("header_back") : t("header_profile");

  return (
    <View
      pointerEvents="box-none"
      className="w-full bg-background"
      style={[
        {
          position: "relative",
          paddingTop: insets.top + 4,
          paddingHorizontal: 16,
          paddingBottom: 4,
        },
        style,
      ]}
    >
      <View
        pointerEvents="box-none"
        className="min-h-11 w-full flex-row items-center justify-between"
      >
        {/* LEFT */}
        <View
          pointerEvents="box-none"
          className="shrink-0 items-center justify-center"
          style={{ width: sideSize, height: sideSize }}
        >
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
                  minWidth: sideSize,
                  minHeight: sideSize,
                  flexShrink: 0,
                  aspectRatio: 1,
                  borderRadius: sideSize / 2,
                  overflow: "hidden",
                },
                pressed && styles.pressed,
              ]}
            >
              {showBack && useNativeGlass ? (
                <GlassView
                  pointerEvents="none"
                  glassEffectStyle="regular"
                  tintColor={theme.cardElevated}
                  style={[
                    StyleSheet.absoluteFill,
                    styles.glassSurface,
                    { borderColor: theme.cardBorder },
                  ]}
                />
              ) : showBack ? (
                <View
                  pointerEvents="none"
                  style={[
                    StyleSheet.absoluteFill,
                    styles.glassSurface,
                    {
                      backgroundColor: theme.cardElevated,
                      borderColor: theme.cardBorder,
                      opacity: 0.76,
                    },
                  ]}
                />
              ) : null}
              {showBack ? (
                isArabic ? (
                  <ChevronRight size={21} color={theme.foreground} />
                ) : (
                  <ChevronLeft size={21} color={theme.foreground} />
                )
              ) : (
                <Avatar
                  name={
                    isArabic
                      ? user?.full_name_ar || user?.full_name
                      : user?.full_name
                  }
                  imageUrl={user?.avatar_url}
                  size={32}
                />
              )}
            </Pressable>
          )}
        </View>

        {/* CENTER — PAGE TITLE */}
        <View pointerEvents="none" style={[styles.titleContainer]}>
          {showTitle && title && !largeTitle ? (
            <View className="max-w-full items-center justify-center">
              <Text
                numberOfLines={1}
                ellipsizeMode="tail"
                className="text-center font-semibold"
                style={[styles.pageTitle, { color: theme.foreground }]}
              >
                {title}
              </Text>
              {subtitle ? (
                <Text
                  numberOfLines={1}
                  ellipsizeMode="tail"
                  className="text-center"
                  style={[
                    styles.pageSubtitle,
                    { color: theme.mutedForeground },
                  ]}
                >
                  {subtitle}
                </Text>
              ) : null}
            </View>
          ) : null}
        </View>

        {/* RIGHT */}
        <View
          pointerEvents="box-none"
          className="shrink-0 items-center justify-center"
          style={{ width: sideSize, height: sideSize }}
        >
          {rightAction ? (
            rightAction
          ) : showCrown ? (
            <Pressable
              accessibilityRole="button"
              accessibilityLabel={t("home_notifications")}
              hitSlop={8}
              onPress={handleCrown}
              style={({ pressed }) => [
                styles.floatingSurface,
                {
                  width: sideSize,
                  height: sideSize,
                  flexShrink: 0,
                  borderRadius: sideSize / 2,
                  backgroundColor: theme.cardElevated,
                },
                pressed && styles.pressed,
              ]}
              className="items-center justify-center"
            >
              <AppNotification size={20} color={theme.foreground} />
              {(unreadCount ?? 0) > 0 && (
                <View
                  style={{
                    position: "absolute",
                    top: 10,
                    right: 10,
                    width: 8,
                    height: 8,
                    borderRadius: 4,
                    backgroundColor: theme.destructive,
                    borderWidth: 1.5,
                    borderColor: theme.cardElevated,
                  }}
                />
              )}
            </Pressable>
          ) : (
            <View
              className="shrink-0"
              style={{ width: sideSize, height: sideSize }}
            />
          )}
        </View>
      </View>

      {largeTitle && title ? (
        <View pointerEvents="none" className="mt-3 px-1">
          <Text
            style={[
              {
                color: theme.foreground,
                ...{ fontSize: 24, lineHeight: 30, fontWeight: "800" },
              },
            ]}
          >
            {title}
          </Text>
          {subtitle ? (
            <Text
              style={[
                {
                  color: theme.mutedForeground,
                  ...{ fontSize: 13, lineHeight: 18, fontWeight: "400" },
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
  glassSurface: {
    borderRadius: 9999,
    borderWidth: StyleSheet.hairlineWidth,
  },
  floatingSurface: {
    alignItems: "center",
    justifyContent: "center",
  },
  titleContainer: {
    position: "absolute",
    left: 56,
    right: 56,
    alignItems: "center",
    justifyContent: "center",
  },
  pageTitle: {
    fontSize: 17,
    lineHeight: 24,
    fontWeight: "700",
    maxWidth: "100%",
  },
  pageSubtitle: {
    fontSize: 12,
    lineHeight: 16,
  },
  pressed: {
    transform: [{ scale: 0.96 }],
  },
});

export default Header;
