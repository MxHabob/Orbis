/**
 * @module @orbis/mobile/components/layout/section
 *
 * Page section layout wrapper with title, subtitle, action button, and content slots.
 */

import React from "react";
import {
  View,
  Text,
  StyleSheet,
  type ViewProps,
  useColorScheme,
} from "react-native";
import { Colors, Spacing, Typography } from "@/constants/theme";

export interface SectionProps extends ViewProps {
  title?: string;
  subtitle?: string;
  action?: React.ReactNode;
  spacing?: keyof typeof Spacing | number;
  headerSpacing?: keyof typeof Spacing | number;
  children?: React.ReactNode;
}

function resolveSpacing(val?: keyof typeof Spacing | number): number | undefined {
  if (val === undefined) return undefined;
  if (typeof val === "number") return val;
  return Spacing[val];
}

export function Section({
  title,
  subtitle,
  action,
  spacing = "four",
  headerSpacing = "two",
  style,
  children,
  ...props
}: SectionProps) {
  const colorScheme = useColorScheme();
  const theme = Colors[colorScheme === "dark" ? "dark" : "light"];

  const hasHeader = title || subtitle || action;

  return (
    <View
      style={[
        styles.container,
        { marginBottom: resolveSpacing(spacing) },
        style,
      ]}
      {...props}
    >
      {hasHeader && (
        <View
          style={[
            styles.header,
            { marginBottom: resolveSpacing(headerSpacing) },
          ]}
        >
          <View style={styles.titleContainer}>
            {title && (
              <Text style={[styles.title, { color: theme.foreground }]}>
                {title}
              </Text>
            )}
            {subtitle && (
              <Text style={[styles.subtitle, { color: theme.textSecondary }]}>
                {subtitle}
              </Text>
            )}
          </View>
          {action && <View style={styles.actionContainer}>{action}</View>}
        </View>
      )}
      {children}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    width: "100%",
  },
  header: {
    flexDirection: "row",
    alignItems: "flex-end",
    justifyContent: "space-between",
  },
  titleContainer: {
    flex: 1,
  },
  title: {
    ...Typography.h3,
  },
  subtitle: {
    ...Typography.caption,
    marginTop: Spacing.half,
  },
  actionContainer: {
    marginLeft: Spacing.two,
  },
});
