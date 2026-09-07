/**
 * @module @orbis/mobile/components/ui/drawer-form-primitives
 *
 * Form primitives and list item components for Drawer bottoms sheets.
 * Synchronized with S.C.R.A.P.S. design system tokens.
 */

import React from 'react';
import {
  Pressable,
  StyleSheet,
  Text,
  type TextInputProps,
  View,
} from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { Button } from '@/components/ui/button';
import { useDrawer } from '@/components/ui/drawer';
import { Input } from '@/components/ui/input';
import { Radii, Spacing, Typography } from '@/constants/theme';

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
    <View style={formStyles.fieldWrapper}>
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
    <View style={formStyles.submitButtonWrapper}>
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
      style={({ pressed }) => [
        formStyles.optionRow,
        !isLast && {
          borderBottomColor: theme.border,
          borderBottomWidth: StyleSheet.hairlineWidth,
        },
        pressed && { backgroundColor: theme.accent },
      ]}
      onPress={onPress}
    >
      <View style={formStyles.optionLeft}>
        {icon && <View style={formStyles.optionIcon}>{icon}</View>}
        <View style={formStyles.optionLabels}>
          <Text style={[formStyles.optionLabel, { color: theme.text }]}>
            {label}
          </Text>
          {sublabel && (
            <Text
              style={[
                formStyles.optionSublabel,
                { color: theme.textSecondary },
              ]}
            >
              {sublabel}
            </Text>
          )}
        </View>
      </View>
      {isActive && (
        <Ionicons name="checkmark-circle" size={22} color={theme.primary} />
      )}
    </Pressable>
  );
}

// ─── Styles ──────────────────────────────────────────────────────────────────

const formStyles = StyleSheet.create({
  fieldWrapper: {
    gap: Spacing.one,
    paddingHorizontal: Spacing.five,
  },
  submitButtonWrapper: {
    marginHorizontal: Spacing.five,
    marginBottom: Spacing.two,
  },
  optionRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingHorizontal: Spacing.five,
    paddingVertical: Spacing.four,
  },
  optionLeft: {
    flexDirection: 'row',
    alignItems: 'center',
    flex: 1,
  },
  optionIcon: {
    width: 36,
    height: 36,
    borderRadius: Radii.md,
    alignItems: 'center',
    justifyContent: 'center',
    marginRight: Spacing.three,
  },
  optionLabels: {
    flex: 1,
    gap: 2,
  },
  optionLabel: {
    ...Typography.bodyBold,
  },
  optionSublabel: {
    ...Typography.small,
  },
});
