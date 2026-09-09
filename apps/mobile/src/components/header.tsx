/**
 * @module @orbis/mobile/components/header
 *
 * Universal Screen Header with title, subtitle, back navigation,
 * and contextual action slots.
 */

import React from 'react';
import { View, StyleSheet, Pressable, type ViewStyle } from 'react-native';
import { router } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { useTheme } from '@/hooks/use-theme';
import { Spacing } from '@/constants/theme';

export interface HeaderProps {
  title: string;
  subtitle?: string;
  showBack?: boolean;
  onBack?: () => void;
  rightAction?: React.ReactNode;
  style?: ViewStyle;
  transparent?: boolean;
}

export function Header({
  title,
  subtitle,
  showBack = false,
  onBack,
  rightAction,
  style,
  transparent = false,
}: HeaderProps) {
  const insets = useSafeAreaInsets();
  const theme = useTheme();

  const handleBack = () => {
    if (onBack) {
      onBack();
    } else if (router.canGoBack()) {
      router.back();
    } else {
      router.replace('/');
    }
  };

  return (
    <View
      style={[
        styles.container,
        {
          paddingTop: insets.top + Spacing.two,
          backgroundColor: 'transparent',
          borderBottomWidth: 0,
        },
        style,
      ]}>
      <View style={styles.contentRow}>
        <View style={styles.leftSlot}>
          {showBack && (
            <Pressable
              onPress={handleBack}
              style={({ pressed }) => [
                styles.backButton,
                { backgroundColor: theme.backgroundElement },
                pressed && { opacity: 0.7 },
              ]}
              hitSlop={8}>
              <Ionicons name="chevron-back" size={22} color={theme.text} />
            </Pressable>
          )}
        </View>

        <View style={styles.centerSlot} />

        <View style={styles.rightSlot}>{rightAction}</View>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    paddingHorizontal: Spacing.four,
    paddingBottom: 0,
  },
  contentRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    minHeight: 40,
  },
  leftSlot: {
    width: 40,
    alignItems: 'flex-start',
    justifyContent: 'center',
  },
  centerSlot: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    paddingHorizontal: Spacing.two,
  },
  rightSlot: {
    width: 40,
    alignItems: 'flex-end',
    justifyContent: 'center',
  },
  backButton: {
    width: 36,
    height: 36,
    borderRadius: 18,
    alignItems: 'center',
    justifyContent: 'center',
  },
});
