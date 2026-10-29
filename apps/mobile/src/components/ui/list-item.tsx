/**
 * @module @orbis/mobile/components/ui/list-item
 *
 * Interactive or static list row component with icon, title, subtitle, badge, and trailing accessory.
 */

import React from "react";
import { View, Text, Pressable, type ViewStyle } from "react-native";
import { ChevronRight, ChevronLeft } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";

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

  const titleClass = destructive
    ? "text-destructive"
    : disabled
      ? "text-muted-foreground"
      : "text-foreground";

  const content = (
    <View
      className="min-h-[52px] w-full flex-row items-center px-two py-two"
      style={style}
    >
      {left && (
        <View className="me-three items-center justify-center">{left}</View>
      )}

      <View className="flex-1 justify-center">
        <View className="flex-row items-center gap-two">
          <Text
            numberOfLines={1}
            className={`shrink text-sm leading-5 font-semibold ${titleClass}`}
          >
            {title}
          </Text>
          {badge && <View className="shrink-0">{badge}</View>}
        </View>

        {subtitle && (
          <Text
            numberOfLines={2}
            className="mt-0.5 text-xs leading-4 text-muted-foreground"
          >
            {subtitle}
          </Text>
        )}
      </View>

      {(right || chevron) && (
        <View className="ms-two flex-row items-center gap-two">
          {right}
          {chevron &&
            (isArabic ? (
              <ChevronLeft size={18} color={theme.textMuted} />
            ) : (
              <ChevronRight size={18} color={theme.textMuted} />
            ))}
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
        className="w-full"
        style={({ pressed }) => [
          pressed && {
            backgroundColor: theme.backgroundSelected,
            borderRadius: 6,
          },
        ]}
      >
        {content}
      </Pressable>
    );
  }

  return content;
}
