/**
 * @module @orbis/mobile/components/behavior/behavior-score-banner
 *
 * Summary card presenting net behavior score and discipline indicator.
 */

import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { Card } from '@/components/ui/card';
import { SkeletonCard } from '@/components/ui/skeleton';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { Spacing, Typography } from '@/constants/theme';

interface BehaviorScoreBannerProps {
  totalPoints: number;
  isLoading?: boolean;
}

export const BehaviorScoreBanner = React.memo<BehaviorScoreBannerProps>(
  ({ totalPoints, isLoading = false }) => {
    const theme = useTheme();
    const { t } = useI18n();

    if (isLoading) {
      return <SkeletonCard />;
    }

    const isPositive = totalPoints >= 0;

    return (
      <Card variant="elevated" style={styles.card}>
        <View style={styles.row}>
          <View style={styles.col}>
            <Text style={[styles.label, { color: theme.textSecondary }]}>
              {t('behavior_score_net')}
            </Text>
            <Text
              style={[
                styles.value,
                { color: isPositive ? theme.success : theme.error },
              ]}>
              {t('behavior_score_unit', {
                count: totalPoints > 0 ? `+${totalPoints}` : totalPoints,
              })}
            </Text>
            <Text style={[styles.status, { color: theme.textSecondary }]}>
              {t('behavior_status_regular')}
            </Text>
          </View>

          <View
            style={[
              styles.badgeCircle,
              { backgroundColor: isPositive ? theme.successLight : theme.errorLight },
            ]}>
            <Ionicons
              name={isPositive ? 'shield-checkmark' : 'alert-circle'}
              size={36}
              color={isPositive ? theme.success : theme.error}
            />
          </View>
        </View>
      </Card>
    );
  },
);

BehaviorScoreBanner.displayName = 'BehaviorScoreBanner';

const styles = StyleSheet.create({
  card: {
    padding: Spacing.four,
  },
  row: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  col: {
    gap: 2,
  },
  label: {
    ...Typography.small,
    fontSize: 12,
  },
  value: {
    ...Typography.h1,
    fontSize: 32,
    marginTop: 2,
  },
  status: {
    ...Typography.small,
    marginTop: 4,
  },
  badgeCircle: {
    width: 64,
    height: 64,
    borderRadius: 32,
    alignItems: 'center',
    justifyContent: 'center',
  },
});
