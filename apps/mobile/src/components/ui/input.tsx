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

import React, { useState } from "react";
import {
  View,
  Text,
  TextInput,
  type TextInputProps,
  Pressable,
  type ViewStyle,
} from "react-native";
import { Eye, EyeSlash } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";
import { useColorScheme } from "@/hooks/use-color-scheme";

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
    const isDark = colorScheme === "dark";

    const [isFocused, setIsFocused] = useState(false);
    const [showPassword, setShowPassword] = useState(false);

    const isPassword = !!secureTextEntry;
    const isActualSecure = isPassword && !showPassword;

    return (
      <View className="w-full gap-1" style={containerStyle}>
        {label && (
          <Text className="mb-0.5 text-xs leading-4 font-semibold text-foreground">
            {label}
          </Text>
        )}

        {/* Sunken Recessed Input Container */}
        <View
          className={`min-h-11 flex-row items-center rounded-md border px-3 ${error ? "border-error" : isFocused ? "border-primary" : "border-border"} ${isDark ? "bg-background-element" : "bg-background"}`}
          style={[
            {
              shadowColor: isFocused ? theme.primary : "#000000",
              shadowOffset: { width: 0, height: 1 },
              shadowOpacity: isFocused ? 0.15 : isDark ? 0.2 : 0.04,
              shadowRadius: isFocused ? 4 : 2,
              elevation: isFocused ? 2 : 1,
            },
          ]}
        >
          {icon && (
            <View className="me-2 items-center justify-center">{icon}</View>
          )}

          <TextInput
            className="flex-1 py-2 text-sm leading-5 text-foreground"
            style={[
              {
                backgroundColor: "transparent",
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
              className="p-one"
              onPress={() => setShowPassword(!showPassword)}
              hitSlop={8}
            >
              {showPassword ? (
                <EyeSlash size={18} color={theme.textSecondary} />
              ) : (
                <Eye size={18} color={theme.textSecondary} />
              )}
            </Pressable>
          )}
        </View>

        {error ? (
          <Text className="mt-0.5 text-[11px] leading-[14px] font-medium text-error">
            {error}
          </Text>
        ) : hint ? (
          <Text className="mt-0.5 text-[11px] leading-[14px] font-medium text-muted-foreground">
            {hint}
          </Text>
        ) : null}
      </View>
    );
  },
);

Input.displayName = "Input";
