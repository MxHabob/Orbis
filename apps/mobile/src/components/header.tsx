/**
 * @module @orbis/mobile/components/header
 *
 * Universal Screen Header with title, subtitle, back navigation,
 * and contextual action slots.
 */

import React from 'react';
import { View, Text, StyleSheet, Pressable, type ViewStyle } from 'react-native';
import { router } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { useTheme } from '@/hooks/use-theme';
import { Spacing, Typography } from '@/constants/theme';

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
        <View style={styles.leftGroup}>
          {showBack && (
            <View style={[styles.floatingLeft, { backgroundColor: theme.backgroundElement, borderColor: theme.border }]}>
              <Pressable
                onPress={handleBack}
                style={({ pressed }) => [
                  styles.backButton,
                  { backgroundColor: 'transparent' },
                  pressed && { opacity: 0.7 },
                ]}
                hitSlop={8}>
                <Ionicons name="chevron-back" size={22} color={theme.text} />
              </Pressable>
            </View>
          )}

          <View style={[styles.titleWrap, { backgroundColor: theme.backgroundElement, borderColor: theme.border }]}>
            {title ? (
              <Text style={[styles.title, { color: theme.text }]} numberOfLines={1}>
                {title}
              </Text>
            ) : null}
            {subtitle ? (
              <Text style={[styles.subtitle, { color: theme.textSecondary }]} numberOfLines={1}>
                {subtitle}
              </Text>
            ) : null}
          </View>
        </View>

        <View style={styles.rightSlot}>
          {rightAction ? (
            <View style={[styles.floatingRight, { backgroundColor: theme.backgroundElement, borderColor: theme.border }]}>
              {rightAction}
            </View>
          ) : null}
        </View>
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
    minHeight: 56,
  },
  leftGroup: {
    flexDirection: 'row',
    alignItems: 'center',
    flexShrink: 1,
    gap: Spacing.one,
  },
  floatingLeft: {
    borderRadius: 20,
    borderWidth: 1,
    overflow: 'hidden',
    shadowColor: '#000000',
    shadowOffset: { width: 0, height: 3 },
    shadowOpacity: 0.10,
    shadowRadius: 8,
    elevation: 3,
  },
  titleWrap: {
    alignItems: 'center',
    justifyContent: 'center',
    paddingHorizontal: Spacing.three,
    paddingVertical: Spacing.one,
    borderRadius: 22,
    borderWidth: 1,
    minWidth: 170,
    shadowColor: '#000000',
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.08,
    shadowRadius: 12,
    elevation: 3,
  },
  title: {
    ...Typography.bodyBold,
    fontSize: 17,
    textAlign: 'center',
  },
  subtitle: {
    ...Typography.caption,
    fontSize: 11,
    textAlign: 'center',
    marginTop: 2,
  },
  rightSlot: {
    width: 44,
    alignItems: 'flex-end',
    justifyContent: 'center',
    marginLeft: Spacing.two,
  },
  floatingRight: {
    minWidth: 38,
    minHeight: 38,
    borderRadius: 20,
    borderWidth: 1,
    alignItems: 'center',
    justifyContent: 'center',
    paddingHorizontal: Spacing.one,
    shadowColor: '#000000',
    shadowOffset: { width: 0, height: 3 },
    shadowOpacity: 0.10,
    shadowRadius: 8,
    elevation: 3,
  },
  backButton: {
    width: 36,
    height: 36,
    borderRadius: 18,
    alignItems: 'center',
    justifyContent: 'center',
  },
});
