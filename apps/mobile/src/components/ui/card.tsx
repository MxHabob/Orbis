/**
 * @module @orbis/mobile/components/ui/card
 *
 * S.C.R.A.P.S. Tactile Card Component for Orbis Mobile.
 * 100% synchronized with @orbis/ui (packages/ui/globals.css .card-tactile & .card-tactile-interactive).
 */

import React from 'react';
import { View, StyleSheet, type ViewProps, type ViewStyle, Pressable } from 'react-native';
import { useTheme } from '@/hooks/use-theme';
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

    const getVariantStyle = (): ViewStyle => {
      switch (variant) {
        case 'elevated':
        case 'interactive':
          return {
            backgroundColor: theme.card,
            borderWidth: 0,
            shadowOpacity: 0,
            elevation: 0,
          };
        case 'outline':
          return {
            backgroundColor: 'transparent',
            borderColor: theme.border,
            borderWidth: 1,
          };
        case 'flat':
          return { backgroundColor: 'transparent', borderWidth: 0 };
        case 'default':
        default:
          return {
            backgroundColor: theme.card,
            borderWidth: 0,
            shadowOpacity: 0,
            elevation: 0,
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
