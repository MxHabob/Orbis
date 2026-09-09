/**
 * @module @orbis/mobile/components/home/campus-header-pill
 *
 * Floating header capsule pill displaying campus name and dropdown indicator.
 */

import React from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useTheme } from '@/hooks/use-theme';
import { Skeleton } from '@/components/ui/skeleton';
import { Radii, Spacing, Typography } from '@/constants/theme';

interface CampusHeaderPillProps {
  campusName?: string;
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
        {campusName ? (
          <Text style={[styles.text, { color: theme.text }]} numberOfLines={1}>
            {campusName}
          </Text>
        ) : (
          <Skeleton width={150} height={16} />
        )}
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
    maxWidth: '90%',
  },
  text: {
    ...Typography.bodyBold,
    fontSize: 13,
  },
});
