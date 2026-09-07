/**
 * @module @orbis/mobile/components/home/campus-header-pill
 *
 * Floating header capsule pill displaying campus name and dropdown indicator.
 */

import React from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useTheme } from '@/hooks/use-theme';
import { Radii, Spacing, Typography } from '@/constants/theme';

interface CampusHeaderPillProps {
  campusName: string;
  onPress?: () => void;
}

export const CampusHeaderPill = React.memo<CampusHeaderPillProps>(({ campusName, onPress }) => {
  const theme = useTheme();

  return (
    <View style={styles.container}>
      <Pressable
        onPress={onPress}
        style={({ pressed }) => [
          styles.pill,
          {
            backgroundColor: theme.card,
            borderColor: theme.border,
          },
          pressed && { opacity: 0.85 },
        ]}>
        <Ionicons name="school-outline" size={17} color={theme.primary} />
        <Text style={[styles.text, { color: theme.text }]} numberOfLines={1}>
          {campusName}
        </Text>
        <Ionicons name="chevron-down" size={15} color={theme.textSecondary} />
      </Pressable>
    </View>
  );
});

CampusHeaderPill.displayName = 'CampusHeaderPill';

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    marginBottom: Spacing.four,
  },
  pill: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingVertical: 7,
    paddingHorizontal: 16,
    borderRadius: Radii.full,
    borderWidth: 1,
    gap: 6,
    shadowColor: '#000000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.08,
    shadowRadius: 8,
    elevation: 3,
    maxWidth: '90%',
  },
  text: {
    ...Typography.bodyBold,
    fontSize: 13,
  },
});
