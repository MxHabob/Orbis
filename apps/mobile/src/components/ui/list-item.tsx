/**
 * @module @orbis/mobile/components/ui/list-item
 *
 * Interactive or static list row component with icon, title, subtitle, badge, and trailing accessory.
 */

import React from "react";
import {
  View,
  Text,
  Pressable,
  StyleSheet,
  type ViewStyle,
} from "react-native";
import { ChevronRight, ChevronLeft } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";
import { Radii, Spacing, Typography } from "@/constants/theme";

export interface ListItemProps {
  title: string;
  subtitle?: string;
  left?: React.ReactNode;
  right?: React.ReactNode;
  badge?: React.ReactNode;
  chevron?: boolean;
  onPress?: () => void;
  destructive?: boolean;
  disabled?: boolean;
  style?: ViewStyle;
}

export function ListItem({
  title,
  subtitle,
  left,
  right,
  badge,
  chevron = false,
  onPress,
  destructive = false,
  disabled = false,
  style,
}: ListItemProps) {
  const theme = useTheme();
  const { isArabic } = useI18n();

  const isInteractive = Boolean(onPress) && !disabled;

  const titleColor = destructive
    ? theme.destructive
    : disabled
      ? theme.textMuted
      : theme.foreground;

  const content = (
    <View style={[styles.container, style]}>
      {left && <View style={styles.leftBox}>{left}</View>}

      <View style={styles.centerBox}>
        <View style={styles.titleRow}>
          <Text
            numberOfLines={1}
            style={[styles.title, { color: titleColor }]}
          >
            {title}
          </Text>
          {badge && <View style={styles.badgeBox}>{badge}</View>}
        </View>

        {subtitle && (
          <Text
            numberOfLines={2}
            style={[styles.subtitle, { color: theme.textSecondary }]}
          >
            {subtitle}
          </Text>
        )}
      </View>

      {(right || chevron) && (
        <View style={styles.rightBox}>
          {right}
          {chevron && (
            isArabic ? (
              <ChevronLeft size={18} color={theme.textMuted} />
            ) : (
              <ChevronRight size={18} color={theme.textMuted} />
            )
          )}
        </View>
      )}
    </View>
  );

  if (isInteractive) {
    return (
      <Pressable
        accessibilityRole="button"
        accessibilityState={{ disabled }}
        onPress={onPress}
        style={({ pressed }) => [
          styles.pressable,
          pressed && {
            backgroundColor: theme.backgroundSelected,
            borderRadius: Radii.md,
          },
        ]}
      >
        {content}
      </Pressable>
    );
  }

  return content;
}

const styles = StyleSheet.create({
  pressable: {
    width: "100%",
  },
  container: {
    width: "100%",
    minHeight: 52,
    flexDirection: "row",
    alignItems: "center",
    paddingVertical: Spacing.two,
    paddingHorizontal: Spacing.two,
  },
  leftBox: {
    marginRight: Spacing.three,
    alignItems: "center",
    justifyContent: "center",
  },
  centerBox: {
    flex: 1,
    justifyContent: "center",
  },
  titleRow: {
    flexDirection: "row",
    alignItems: "center",
    gap: Spacing.two,
  },
  title: {
    ...Typography.bodyBold,
    flexShrink: 1,
  },
  badgeBox: {
    flexShrink: 0,
  },
  subtitle: {
    ...Typography.caption,
    marginTop: Spacing.half,
  },
  rightBox: {
    marginLeft: Spacing.two,
    flexDirection: "row",
    alignItems: "center",
    gap: Spacing.two,
  },
});
