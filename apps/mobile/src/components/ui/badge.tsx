/**
 * @module @orbis/mobile/components/badge
 *
 * Status and category badge pill component with distinct color variants.
 */

import React from 'react';
import { View, Text, StyleSheet, type ViewStyle, type TextStyle } from 'react-native';
import { useTheme } from '@/hooks/use-theme';
import { Radii, Spacing, Typography } from '@/constants/theme';

export type BadgeVariant = 'primary' | 'success' | 'warning' | 'error' | 'info' | 'neutral';

export interface BadgeProps {
  label: string;
  variant?: BadgeVariant;
  size?: 'sm' | 'md';
  icon?: React.ReactNode;
  style?: ViewStyle;
  textStyle?: TextStyle;
}

export function Badge({
  label,
  variant = 'primary',
  size = 'md',
  icon,
  style,
  textStyle,
}: BadgeProps) {
  const theme = useTheme();

  const getColors = () => {
    switch (variant) {
      case 'success':
        return { bg: theme.successLight, text: theme.success };
      case 'warning':
        return { bg: theme.warningLight, text: theme.warning };
      case 'error':
        return { bg: theme.errorLight, text: theme.error };
      case 'info':
        return { bg: theme.infoLight, text: theme.info };
      case 'neutral':
        return { bg: theme.backgroundSelected, text: theme.textSecondary };
      case 'primary':
      default:
        return { bg: theme.primaryLight, text: theme.primary };
    }
  };

  const { bg, text: textColor } = getColors();
  const isSm = size === 'sm';

  return (
    <View
      style={[
        styles.container,
        {
          backgroundColor: bg,
          paddingVertical: isSm ? 2 : Spacing.half,
          paddingHorizontal: isSm ? Spacing.two : Spacing.three,
        },
        style,
      ]}>
      {icon && <View style={styles.iconContainer}>{icon}</View>}
      <Text
        style={[
          styles.text,
          isSm ? Typography.small : Typography.captionBold,
          { color: textColor },
          textStyle,
        ]}>
        {label}
      </Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flexDirection: 'row',
    alignItems: 'center',
    alignSelf: 'flex-start',
    borderRadius: Radii.full,
    gap: 4,
  },
  iconContainer: {
    marginRight: 2,
  },
  text: {
    textAlign: 'center',
  },
});
