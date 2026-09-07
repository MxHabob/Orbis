/**
 * @module @orbis/mobile/components/button
 *
 * Polished Button component supporting primary, secondary, outline, and ghost styles,
 * with loading spinner and disabled states.
 */

import React from 'react';
import {
  Pressable,
  Text,
  ActivityIndicator,
  StyleSheet,
  type ViewStyle,
  type TextStyle,
  View,
} from 'react-native';
import { useTheme } from '@/hooks/use-theme';
import { Radii, Spacing, Typography } from '@/constants/theme';

export type ButtonVariant = 'primary' | 'secondary' | 'outline' | 'danger' | 'ghost';

export interface ButtonProps {
  title: string;
  onPress: () => void;
  variant?: ButtonVariant;
  size?: 'sm' | 'md' | 'lg';
  loading?: boolean;
  disabled?: boolean;
  icon?: React.ReactNode;
  iconPosition?: 'left' | 'right';
  style?: ViewStyle;
  textStyle?: TextStyle;
  fullWidth?: boolean;
}

export function Button({
  title,
  onPress,
  variant = 'primary',
  size = 'md',
  loading = false,
  disabled = false,
  icon,
  iconPosition = 'left',
  style,
  textStyle,
  fullWidth = false,
}: ButtonProps) {
  const theme = useTheme();

  const getVariantStyles = (): { container: ViewStyle; text: TextStyle; indicatorColor: string } => {
    switch (variant) {
      case 'secondary':
        return {
          container: {
            backgroundColor: theme.backgroundElement,
            borderWidth: 1,
            borderColor: theme.border,
          },
          text: { color: theme.text },
          indicatorColor: theme.text,
        };
      case 'outline':
        return {
          container: {
            backgroundColor: 'transparent',
            borderWidth: 1.5,
            borderColor: theme.primary,
          },
          text: { color: theme.primary },
          indicatorColor: theme.primary,
        };
      case 'danger':
        return {
          container: {
            backgroundColor: theme.error,
          },
          text: { color: '#FFFFFF' },
          indicatorColor: '#FFFFFF',
        };
      case 'ghost':
        return {
          container: {
            backgroundColor: 'transparent',
          },
          text: { color: theme.primary },
          indicatorColor: theme.primary,
        };
      case 'primary':
      default:
        return {
          container: {
            backgroundColor: theme.primary,
          },
          text: { color: '#FFFFFF' },
          indicatorColor: '#FFFFFF',
        };
    }
  };

  const getSizeStyles = () => {
    switch (size) {
      case 'sm':
        return {
          paddingVertical: Spacing.one + 2,
          paddingHorizontal: Spacing.three,
          typography: Typography.captionBold,
        };
      case 'lg':
        return {
          paddingVertical: Spacing.four,
          paddingHorizontal: Spacing.five,
          typography: Typography.bodyBold,
        };
      case 'md':
      default:
        return {
          paddingVertical: Spacing.three,
          paddingHorizontal: Spacing.four,
          typography: Typography.bodyBold,
        };
    }
  };

  const vStyles = getVariantStyles();
  const sStyles = getSizeStyles();
  const isDisabled = disabled || loading;

  return (
    <Pressable
      onPress={isDisabled ? undefined : onPress}
      disabled={isDisabled}
      style={({ pressed }) => [
        styles.base,
        vStyles.container,
        {
          paddingVertical: sStyles.paddingVertical,
          paddingHorizontal: sStyles.paddingHorizontal,
          borderRadius: Radii.md,
          width: fullWidth ? '100%' : undefined,
          opacity: isDisabled ? 0.6 : pressed ? 0.88 : 1,
          transform: [{ scale: pressed && !isDisabled ? 0.98 : 1 }],
        },
        style,
      ]}>
      {loading ? (
        <ActivityIndicator size="small" color={vStyles.indicatorColor} />
      ) : (
        <View style={styles.contentRow}>
          {icon && iconPosition === 'left' && <View style={styles.leftIcon}>{icon}</View>}
          <Text style={[styles.text, sStyles.typography, vStyles.text, textStyle]}>{title}</Text>
          {icon && iconPosition === 'right' && <View style={styles.rightIcon}>{icon}</View>}
        </View>
      )}
    </Pressable>
  );
}

const styles = StyleSheet.create({
  base: {
    alignItems: 'center',
    justifyContent: 'center',
    flexDirection: 'row',
  },
  contentRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
  },
  text: {
    textAlign: 'center',
  },
  leftIcon: {
    marginRight: 6,
  },
  rightIcon: {
    marginLeft: 6,
  },
});
