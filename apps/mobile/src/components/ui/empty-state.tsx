/**
 * @module @orbis/mobile/components/empty-state
 *
 * Polished empty state indicator with icons, explanatory text, and optional action button.
 */

import React from "react";
import { View, Text, type ViewStyle } from "react-native";
import { ArchiveBox } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";
import { Button } from "./button";

export interface EmptyStateProps {
  icon?: React.ComponentType<{ size?: number; color?: string }> | React.ReactNode;
  title: string;
  description?: string;
  actionTitle?: string;
  onAction?: () => void;
  style?: ViewStyle;
}

export function EmptyState({
  icon: IconComponent,
  title,
  description,
  actionTitle,
  onAction,
  style,
}: EmptyStateProps) {
  const theme = useTheme();

  return (
    <View
      className="items-center justify-center gap-two px-five py-six"
      style={style}
    >
      <View
        className="mb-two h-[68px] w-[68px] items-center justify-center rounded-full bg-background-element"
      >
        {IconComponent ? (
          typeof IconComponent === "function" ? (
            <IconComponent size={36} color={theme.textSecondary} />
          ) : (
            IconComponent
          )
        ) : (
          <ArchiveBox size={36} color={theme.textSecondary} />
        )}
      </View>
      <Text className="text-center text-base leading-[22px] font-semibold text-foreground">
        {title}
      </Text>
      {description && (
        <Text className="max-w-[320px] text-center text-sm leading-5 text-muted-foreground">
          {description}
        </Text>
      )}
      {actionTitle && onAction && (
        <View className="mt-three">
          <Button
            title={actionTitle}
            onPress={onAction}
            variant="secondary"
            size="sm"
          />
        </View>
      )}
    </View>
  );
}
