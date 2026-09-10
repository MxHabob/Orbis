/**
 * @module @orbis/mobile/components/empty-state
 *
 * Polished empty state indicator with icons, explanatory text, and optional action button.
 */

import React from "react";
import { View, Text, StyleSheet, type ViewStyle } from "react-native";
import { Ionicons } from "@expo/vector-icons";
import { useTheme } from "@/hooks/use-theme";
import { Spacing, Typography } from "@/constants/theme";
import { Button } from "./button";

export interface EmptyStateProps {
  icon?: keyof typeof Ionicons.glyphMap;
  title: string;
  description?: string;
  actionTitle?: string;
  onAction?: () => void;
  style?: ViewStyle;
}

export function EmptyState({
  icon = "file-tray-outline",
  title,
  description,
  actionTitle,
  onAction,
  style,
}: EmptyStateProps) {
  const theme = useTheme();

  return (
    <View style={[styles.container, style]}>
      <View
        style={[
          styles.iconWrapper,
          { backgroundColor: theme.backgroundElement },
        ]}
      >
        <Ionicons name={icon} size={36} color={theme.textSecondary} />
      </View>
      <Text style={[styles.title, { color: theme.text }]}>{title}</Text>
      {description && (
        <Text style={[styles.description, { color: theme.textSecondary }]}>
          {description}
        </Text>
      )}
      {actionTitle && onAction && (
        <Button
          title={actionTitle}
          onPress={onAction}
          variant="secondary"
          size="sm"
          style={styles.actionButton}
        />
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    alignItems: "center",
    justifyContent: "center",
    paddingVertical: Spacing.six,
    paddingHorizontal: Spacing.five,
    gap: Spacing.two,
  },
  iconWrapper: {
    width: 68,
    height: 68,
    borderRadius: 34,
    alignItems: "center",
    justifyContent: "center",
    marginBottom: Spacing.two,
  },
  title: {
    ...Typography.h3,
    textAlign: "center",
  },
  description: {
    ...Typography.body,
    textAlign: "center",
    maxWidth: 320,
    lineHeight: 20,
  },
  actionButton: {
    marginTop: Spacing.three,
  },
});
