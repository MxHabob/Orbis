/**
 * @module @orbis/mobile/components/card
 *
 * Reusable Card container supporting standard, elevated, and outline variants.
 */

import React from 'react';
import { View, StyleSheet, type ViewProps, type ViewStyle, Pressable } from 'react-native';
import { useTheme } from '@/hooks/use-theme';
import { Radii, Spacing } from '@/constants/theme';

export interface CardProps extends ViewProps {
  variant?: 'default' | 'elevated' | 'outline' | 'flat';
  onPress?: () => void;
  padding?: keyof typeof Spacing;
  children?: React.ReactNode;
}

export function Card({
  variant = 'default',
  onPress,
  padding = 'four',
  style,
  children,
  ...props
}: CardProps) {
  const theme = useTheme();

  const getVariantStyle = (): ViewStyle => {
    switch (variant) {
      case 'elevated':
        return {
          backgroundColor: theme.cardElevated,
          borderColor: theme.cardBorder,
          borderWidth: 1,
          shadowColor: '#000000',
          shadowOffset: { width: 0, height: 4 },
          shadowOpacity: 0.06,
          shadowRadius: 12,
          elevation: 3,
        };
      case 'outline':
        return {
          backgroundColor: 'transparent',
          borderColor: theme.border,
          borderWidth: 1,
        };
      case 'flat':
        return {
          backgroundColor: theme.backgroundElement,
          borderWidth: 0,
        };
      case 'default':
      default:
        return {
          backgroundColor: theme.card,
          borderColor: theme.cardBorder,
          borderWidth: 1,
          shadowColor: '#000000',
          shadowOffset: { width: 0, height: 1 },
          shadowOpacity: 0.04,
          shadowRadius: 4,
          elevation: 1,
        };
    }
  };

  const containerStyle: ViewStyle = {
    borderRadius: Radii.lg,
    padding: Spacing[padding],
    ...getVariantStyle(),
  };

  if (onPress) {
    return (
      <Pressable
        onPress={onPress}
        style={({ pressed }) => [
          containerStyle,
          style,
          pressed && { opacity: 0.85, transform: [{ scale: 0.99 }] },
        ]}
        {...props}>
        {children}
      </Pressable>
    );
  }

  return (
    <View style={[containerStyle, style]} {...props}>
      {children}
    </View>
  );
}
