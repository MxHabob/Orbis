/**
 * @module @orbis/mobile/components/input
 *
 * Form text input component with labels, error states, leading/trailing icons,
 * and secure entry toggle.
 */

import React, { useState } from 'react';
import {
  View,
  Text,
  TextInput,
  type TextInputProps,
  StyleSheet,
  Pressable,
  type ViewStyle,
} from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useTheme } from '@/hooks/use-theme';
import { Radii, Spacing, Typography } from '@/constants/theme';

export interface InputProps extends TextInputProps {
  label?: string;
  error?: string | null;
  hint?: string;
  icon?: React.ReactNode;
  containerStyle?: ViewStyle;
}

export function Input({
  label,
  error,
  hint,
  icon,
  secureTextEntry,
  containerStyle,
  style,
  ...props
}: InputProps) {
  const theme = useTheme();
  const [isFocused, setIsFocused] = useState(false);
  const [showPassword, setShowPassword] = useState(false);

  const isPassword = !!secureTextEntry;
  const isActualSecure = isPassword && !showPassword;

  return (
    <View style={[styles.wrapper, containerStyle]}>
      {label && <Text style={[styles.label, { color: theme.text }]}>{label}</Text>}

      <View
        style={[
          styles.inputContainer,
          {
            backgroundColor: theme.backgroundElement,
            borderColor: error ? theme.error : isFocused ? theme.primary : theme.border,
          },
        ]}>
        {icon && <View style={styles.iconBox}>{icon}</View>}

        <TextInput
          style={[styles.input, { color: theme.text }, style]}
          placeholderTextColor={theme.textMuted}
          secureTextEntry={isActualSecure}
          onFocus={() => setIsFocused(true)}
          onBlur={() => setIsFocused(false)}
          {...props}
        />

        {isPassword && (
          <Pressable
            onPress={() => setShowPassword(!showPassword)}
            style={styles.eyeButton}
            hitSlop={8}>
            <Ionicons
              name={showPassword ? 'eye-off-outline' : 'eye-outline'}
              size={20}
              color={theme.textSecondary}
            />
          </Pressable>
        )}
      </View>

      {error ? (
        <Text style={[styles.errorText, { color: theme.error }]}>{error}</Text>
      ) : hint ? (
        <Text style={[styles.hintText, { color: theme.textMuted }]}>{hint}</Text>
      ) : null}
    </View>
  );
}

const styles = StyleSheet.create({
  wrapper: {
    gap: 6,
    width: '100%',
  },
  label: {
    ...Typography.captionBold,
    marginBottom: 2,
  },
  inputContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    borderWidth: 1.5,
    borderRadius: Radii.md,
    paddingHorizontal: Spacing.three,
    minHeight: 48,
  },
  iconBox: {
    marginEnd: Spacing.two,
  },
  input: {
    flex: 1,
    ...Typography.body,
    paddingVertical: Spacing.two,
  },
  eyeButton: {
    padding: Spacing.one,
  },
  errorText: {
    ...Typography.small,
    marginTop: 2,
  },
  hintText: {
    ...Typography.small,
    marginTop: 2,
  },
});
