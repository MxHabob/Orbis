/**
 * @module @orbis/mobile/components/ui/drawer-form-primitives
 *
 * Form primitives and list item components for Drawer bottoms sheets.
 * Synchronized with S.C.R.A.P.S. design system tokens.
 */

import React from "react";
import {
  Pressable,
  StyleSheet,
  Text,
  type TextInputProps,
  View,
} from "react-native";
import { CheckCircle } from "@orbis/icons/native";
import { Button } from "@/components/ui/button";
import { useDrawer } from "@/components/ui/drawer";
import { Input } from "@/components/ui/input";

// ─── DrawerFormField ─────────────────────────────────────────────────────────
// A labeled text input styled for use inside a Drawer form.

export interface DrawerFormFieldProps extends TextInputProps {
  label: string;
  error?: string;
}

export function DrawerFormField({
  label,
  error,
  style,
  ...props
}: DrawerFormFieldProps) {
  return (
    <View className="gap-one px-five">
      <Input label={label} error={error} style={style} {...props} />
    </View>
  );
}

// ─── DrawerSubmitButton ──────────────────────────────────────────────────────
// A primary action button for drawer forms (submit / save).

export interface DrawerSubmitButtonProps {
  label: string;
  onPress: () => void;
  isLoading?: boolean;
  disabled?: boolean;
}

export function DrawerSubmitButton({
  label,
  onPress,
  isLoading = false,
  disabled = false,
}: DrawerSubmitButtonProps) {
  return (
    <View className="mx-five mb-two">
      <Button
        title={label}
        onPress={onPress}
        loading={isLoading}
        disabled={disabled || isLoading}
        fullWidth
        variant="default"
        size="lg"
      />
    </View>
  );
}

// ─── DrawerOptionRow ─────────────────────────────────────────────────────────
// A selectable option row with icon, label, and a checkmark when active.

export interface DrawerOptionRowProps {
  icon?: React.ReactNode;
  label: string;
  sublabel?: string;
  isActive?: boolean;
  onPress: () => void;
  isLast?: boolean;
}

export function DrawerOptionRow({
  icon,
  label,
  sublabel,
  isActive = false,
  onPress,
  isLast = false,
}: DrawerOptionRowProps) {
  const { theme } = useDrawer();

  return (
    <Pressable
      className="flex-row items-center justify-between px-five py-four"
      style={({ pressed }) => [
        !isLast && {
          borderBottomColor: theme.border,
          borderBottomWidth: StyleSheet.hairlineWidth,
        },
        pressed && { backgroundColor: theme.accent },
      ]}
      onPress={onPress}
    >
      <View className="flex-1 flex-row items-center">
        {icon && (
          <View className="me-three h-9 w-9 items-center justify-center rounded-md">
            {icon}
          </View>
        )}
        <View className="flex-1 gap-half">
          <Text className="text-sm leading-5 font-semibold text-foreground">
            {label}
          </Text>
          {sublabel && (
            <Text className="text-[11px] leading-[14px] font-medium text-muted-foreground">
              {sublabel}
            </Text>
          )}
        </View>
      </View>
      {isActive && (
        <CheckCircle size={22} color={theme.primary} />
      )}
    </Pressable>
  );
}
