/**
 * @module @orbis/mobile/components/behavior/behavior-filter-bar
 *
 * Filter tabs allowing toggling between All, Merits, and Violations.
 */

import React from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { Radii, Spacing, Typography } from '@/constants/theme';

export type BehaviorFilterType = 'all' | 'merit' | 'violation';

interface BehaviorFilterBarProps {
  activeFilter: BehaviorFilterType;
  onFilterChange: (filter: BehaviorFilterType) => void;
}

export const BehaviorFilterBar = React.memo<BehaviorFilterBarProps>(
  ({ activeFilter, onFilterChange }) => {
    const theme = useTheme();
    const { t } = useI18n();

    const filters: { key: BehaviorFilterType; label: string }[] = [
      { key: 'all', label: t('behavior_filter_all') },
      { key: 'merit', label: t('behavior_filter_merit') },
      { key: 'violation', label: t('behavior_filter_violation') },
    ];

    return (
      <View style={styles.container}>
        <View style={[styles.bar, { backgroundColor: theme.backgroundElement }]}>
          {filters.map((f) => {
            const isSelected = activeFilter === f.key;
            return (
              <Pressable
                key={f.key}
                onPress={() => onFilterChange(f.key)}
                style={[
                  styles.pill,
                  isSelected && { backgroundColor: theme.primary },
                ]}>
                <Text
                  style={[
                    styles.label,
                    { color: isSelected ? '#FFFFFF' : theme.textSecondary },
                  ]}>
                  {f.label}
                </Text>
              </Pressable>
            );
          })}
        </View>
      </View>
    );
  },
);

BehaviorFilterBar.displayName = 'BehaviorFilterBar';

const styles = StyleSheet.create({
  container: {
    marginVertical: Spacing.one,
  },
  bar: {
    flexDirection: 'row',
    padding: 4,
    borderRadius: Radii.lg,
    gap: 4,
  },
  pill: {
    flex: 1,
    paddingVertical: 8,
    borderRadius: Radii.md,
    alignItems: 'center',
    justifyContent: 'center',
  },
  label: {
    ...Typography.bodyBold,
    fontSize: 13,
  },
});
