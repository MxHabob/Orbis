import React, { PropsWithChildren, useState } from 'react';
import { Pressable, StyleSheet, View } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import Animated, { FadeIn, FadeOut } from 'react-native-reanimated';
import { Radii, Spacing, Typography } from '@/constants/theme';
import { ThemedText } from '@/components/themed-text';
import { useTheme } from '@/hooks/use-theme';

export interface AccordionProps extends PropsWithChildren {
  title: string;
  defaultOpen?: boolean;
  disabled?: boolean;
  onOpenChange?: (open: boolean) => void;
}

export function Accordion({
  title,
  children,
  defaultOpen = false,
  disabled = false,
  onOpenChange,
}: AccordionProps) {
  const theme = useTheme();
  const [open, setOpen] = useState(defaultOpen);

  const toggle = () => {
    if (disabled) return;
    const nextOpen = !open;
    setOpen(nextOpen);
    onOpenChange?.(nextOpen);
  };

  return (
    <View style={[styles.container, { borderBottomColor: theme.borderSubtle }]}>
      <Pressable
        accessibilityRole="button"
        accessibilityState={{ expanded: open, disabled }}
        disabled={disabled}
        onPress={toggle}
        style={({ pressed }) => [styles.trigger, pressed && styles.pressed]}>
        <ThemedText type="smallBold" style={styles.title}>
          {title}
        </ThemedText>
        <Ionicons
          name={open ? 'chevron-up' : 'chevron-down'}
          size={18}
          color={theme.textSecondary}
        />
      </Pressable>
      {open && (
        <Animated.View entering={FadeIn.duration(160)} exiting={FadeOut.duration(120)}>
          <View style={styles.content}>{children}</View>
        </Animated.View>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    borderBottomWidth: 1,
  },
  trigger: {
    minHeight: 48,
    paddingVertical: Spacing.three,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    gap: Spacing.three,
  },
  title: {
    ...Typography.bodyBold,
    flex: 1,
  },
  content: {
    paddingBottom: Spacing.four,
    paddingRight: Spacing.two,
  },
  pressed: {
    opacity: 0.7,
  },
});