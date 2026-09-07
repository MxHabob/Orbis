/**
 * @module @orbis/mobile/components/ui/input
 *
 * S.C.R.A.P.S. Sunken Input Component for Orbis Mobile.
 * 100% synchronized with @orbis/ui (packages/ui/src/components/ui/input/input.svelte & globals.css).
 *
 * Features:
 * - Sunken recessed surface styling with theme borders
 * - Focus ring state matching desktop `--ring`
 * - Clear icon slots and secure entry eye toggle
 * - Clean RTL / LTR layout and transparent native input surface
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
import { useColorScheme } from '@/hooks/use-color-scheme';
import { Radii, Spacing, Typography } from '@/constants/theme';

export interface InputProps extends TextInputProps {
  label?: string;
  error?: string | null;
  hint?: string;
  icon?: React.ReactNode;
  containerStyle?: ViewStyle;
}

export const Input = React.memo<InputProps>(
  ({
    label,
    error,
    hint,
    icon,
    secureTextEntry,
    containerStyle,
    style,
    ...props
  }) => {
    const theme = useTheme();
    const colorScheme = useColorScheme();
    const isDark = colorScheme === 'dark';

    const [isFocused, setIsFocused] = useState(false);
    const [showPassword, setShowPassword] = useState(false);

    const isPassword = !!secureTextEntry;
    const isActualSecure = isPassword && !showPassword;

    return (
      <View style={[styles.wrapper, containerStyle]}>
        {label && (
          <Text style={[styles.label, { color: theme.text }]}>
            {label}
          </Text>
        )}

        {/* Sunken Recessed Input Container */}
        <View
          style={[
            styles.inputContainer,
            {
              backgroundColor: isDark ? theme.backgroundElement : theme.background,
              borderColor: error
                ? theme.error
                : isFocused
                  ? theme.primary
                  : theme.border,
              borderWidth: 1,
              shadowColor: isFocused ? theme.primary : '#000000',
              shadowOffset: { width: 0, height: 1 },
              shadowOpacity: isFocused ? 0.15 : isDark ? 0.2 : 0.04,
              shadowRadius: isFocused ? 4 : 2,
              elevation: isFocused ? 2 : 1,
            },
          ]}>
          {icon && <View style={styles.iconBox}>{icon}</View>}

          <TextInput
            style={[
              styles.input,
              {
                color: theme.text,
                backgroundColor: 'transparent',
              },
              style,
            ]}
            placeholderTextColor={theme.textMuted}
            secureTextEntry={isActualSecure}
            onFocus={() => setIsFocused(true)}
            onBlur={() => setIsFocused(false)}
            cursorColor={theme.primary}
            selectionColor={theme.primaryLight}
            underlineColorAndroid="transparent"
            importantForAutofill="no"
            autoComplete="off"
            textContentType="none"
            {...props}
          />

          {isPassword && (
            <Pressable
              onPress={() => setShowPassword(!showPassword)}
              style={styles.eyeButton}
              hitSlop={8}>
              <Ionicons
                name={showPassword ? 'eye-off-outline' : 'eye-outline'}
                size={18}
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
  },
);

Input.displayName = 'Input';

const styles = StyleSheet.create({
  wrapper: {
    gap: 4,
    width: '100%',
  },
  label: {
    ...Typography.captionBold,
    fontSize: 12.5,
    marginBottom: 2,
  },
  inputContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    borderRadius: Radii.md,
    paddingHorizontal: 12,
    minHeight: 44,
  },
  iconBox: {
    marginEnd: Spacing.two,
    alignItems: 'center',
    justifyContent: 'center',
  },
  input: {
    flex: 1,
    ...Typography.body,
    fontSize: 14,
    paddingVertical: 8,
  },
  eyeButton: {
    padding: Spacing.one,
  },
  errorText: {
    ...Typography.small,
    fontSize: 11,
    marginTop: 2,
  },
  hintText: {
    ...Typography.small,
    fontSize: 11,
    marginTop: 2,
  },
});
