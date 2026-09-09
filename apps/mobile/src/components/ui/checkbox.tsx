import React from 'react';
import { Pressable, StyleSheet } from 'react-native';
import { Radii } from '@/constants/theme';
import { useTheme } from '@/hooks/use-theme';

export interface CheckboxProps {
  value: boolean;
  onValueChange?: (value: boolean) => void;
  disabled?: boolean;
  accessibilityLabel?: string;
}

export function Checkbox({
  value,
  onValueChange,
  disabled = false,
  accessibilityLabel,
}: CheckboxProps) {
  const theme = useTheme();

  return (
    <Pressable
      accessibilityRole="checkbox"
      accessibilityState={{ checked: value, disabled }}
      accessibilityLabel={accessibilityLabel}
      disabled={disabled}
      onPress={() => onValueChange?.(!value)}
      style={({ pressed }) => [
        styles.box,
        {
          backgroundColor: 'transparent',
          borderColor: value ? theme.primary : theme.input,
          borderWidth: value ? 4 : 1,
        },
        disabled && styles.disabled,
        pressed && styles.pressed,
      ]}>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  box: {
    width: 16,
    height: 16,
    borderRadius: Radii.full,
    alignItems: 'center',
    justifyContent: 'center',
  },
  disabled: {
    opacity: 0.5,
  },
  pressed: {
    opacity: 0.75,
  },
});