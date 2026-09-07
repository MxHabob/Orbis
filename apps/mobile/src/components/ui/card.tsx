/**
 * @module @orbis/mobile/components/ui/card
 *
 * S.C.R.A.P.S. Tactile Card Component for Orbis Mobile.
 * 100% synchronized with @orbis/ui (packages/ui/globals.css .card-tactile & .card-tactile-interactive).
 */

import React from 'react';
import { View, StyleSheet, type ViewProps, type ViewStyle, Pressable } from 'react-native';
import { useTheme } from '@/hooks/use-theme';
import { useColorScheme } from '@/hooks/use-color-scheme';
import { Radii, Spacing } from '@/constants/theme';

export interface CardProps extends ViewProps {
  variant?: 'default' | 'elevated' | 'outline' | 'flat' | 'interactive';
  onPress?: () => void;
  padding?: keyof typeof Spacing;
  children?: React.ReactNode;
}

export const Card = React.memo<CardProps>(
  ({
    variant = 'default',
    onPress,
    padding = 'four',
    style,
    children,
    ...props
  }) => {
    const theme = useTheme();
    const colorScheme = useColorScheme();
    const isDark = colorScheme === 'dark';

    const getVariantStyle = (): ViewStyle => {
      switch (variant) {
        case 'elevated':
        case 'interactive':
          return {
            backgroundColor: theme.card,
            borderColor: theme.border,
            borderWidth: 1,
            shadowColor: '#000000',
            shadowOffset: { width: 0, height: 2 },
            shadowOpacity: isDark ? 0.35 : 0.06,
            shadowRadius: isDark ? 8 : 6,
            elevation: 2,
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
            borderColor: theme.border,
            borderWidth: 1,
            shadowColor: '#000000',
            shadowOffset: { width: 0, height: 1 },
            shadowOpacity: isDark ? 0.2 : 0.04,
            shadowRadius: 3,
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
            pressed && { opacity: 0.88, transform: [{ translateY: 1 }] },
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
  },
);

Card.displayName = 'Card';
