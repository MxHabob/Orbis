/**
 * @module @orbis/mobile/components/empty-state
 *
 * Polished empty state indicator with icons, explanatory text, and optional action button.
 */

import React from "react";
import { View, Text, StyleSheet, type ViewStyle } from "react-native";
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
    <View style={[styles.container, style]}>
      <View
        style={[
          styles.iconWrapper,
          { backgroundColor: theme.backgroundElement },
        ]}
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
    paddingVertical: 24,
    paddingHorizontal: 20,
    gap: 8,
  },
  iconWrapper: {
    width: 68,
    height: 68,
    borderRadius: 34,
    alignItems: "center",
    justifyContent: "center",
    marginBottom: 8,
  },
  title: {
    ...{"fontSize":16,"lineHeight":22,"fontWeight":"600"},
    textAlign: "center",
  },
  description: {
    ...{"fontSize":14,"lineHeight":20,"fontWeight":"400"},
    textAlign: "center",
    maxWidth: 320,
    lineHeight: 20,
  },
  actionButton: {
    marginTop: 12,
  },
});
