/**
 * @module @orbis/mobile/components/profile/profile-identity-card
 *
 * User identity summary card with avatar, role badge, and school info.
 */

import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { Radii, Spacing, Typography } from '@/constants/theme';

interface ProfileIdentityCardProps {
  fullName: string;
  email: string;
  role: 'student' | 'parent' | 'teacher' | 'admin';
  isLoading?: boolean;
}

export const ProfileIdentityCard = React.memo<ProfileIdentityCardProps>(
  ({ fullName, email, role, isLoading = false }) => {
    const theme = useTheme();
    const { t } = useI18n();

    const roleLabel =
      role === 'student'
        ? t('profile_role_student')
        : role === 'parent'
          ? t('profile_role_parent')
          : role === 'teacher'
            ? t('profile_role_teacher')
            : t('profile_role_admin');

    return (
      <Card variant="elevated" style={styles.card}>
        {isLoading ? (
          <View style={styles.row}>
            <Skeleton width={68} height={68} borderRadius={34} />
            <View style={styles.info}>
              <Skeleton width={140} height={20} />
              <Skeleton width={180} height={14} style={{ marginTop: 6 }} />
              <Skeleton width={80} height={18} style={{ marginTop: 8 }} />
            </View>
          </View>
        ) : (
          <View style={styles.row}>
            <View style={[styles.avatarCircle, { backgroundColor: theme.primaryLight }]}>
              <Ionicons name="person" size={36} color={theme.primary} />
            </View>

            <View style={styles.info}>
              <Text style={[styles.userName, { color: theme.text }]}>{fullName}</Text>
              <Text style={[styles.userEmail, { color: theme.textSecondary }]}>{email}</Text>

              <View style={styles.badgeRow}>
                <Badge label={roleLabel} variant="primary" size="sm" />
                <Text style={[styles.orgName, { color: theme.textMuted }]}>
                  {t('nav_campus_default')}
                </Text>
              </View>
            </View>
          </View>
        )}
      </Card>
    );
  },
);

ProfileIdentityCard.displayName = 'ProfileIdentityCard';

const styles = StyleSheet.create({
  card: {
    padding: Spacing.four,
  },
  row: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.four,
  },
  avatarCircle: {
    width: 68,
    height: 68,
    borderRadius: 34,
    alignItems: 'center',
    justifyContent: 'center',
  },
  info: {
    flex: 1,
    gap: 2,
  },
  userName: {
    ...Typography.bodyBold,
    fontSize: 18,
  },
  userEmail: {
    ...Typography.small,
    fontSize: 13,
  },
  badgeRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
    marginTop: 4,
  },
  orgName: {
    ...Typography.small,
    fontSize: 11,
  },
});
