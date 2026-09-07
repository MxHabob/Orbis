/**
 * @module @orbis/mobile/components/auth/sign-in-brand-header
 *
 * Highly refined, minimal Organization Brand Header for the Sign-In flow.
 * Displays organization logo/badge and institutional identity with subtle elevation.
 */

import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { useCurrentOrgQuery } from '@/hooks/use-api-queries';
import { Radii, Spacing, Typography } from '@/constants/theme';

export const SignInBrandHeader = React.memo(() => {
  const theme = useTheme();
  const { t } = useI18n();
  const { data: orgData } = useCurrentOrgQuery();

  const orgName = orgData?.name || t('nav_campus_default');
  const orgColor = orgData?.primary_color || theme.primary;

  return (
    <View style={styles.container}>
      {/* Organic Elegant Emblem Container */}
      <View
        style={[
          styles.emblemWrapper,
          {
            backgroundColor: theme.backgroundElement,
            borderColor: theme.border,
            shadowColor: orgColor,
          },
        ]}>
        <View
          style={[
            styles.emblemInner,
            {
              backgroundColor: theme.primaryLight,
            },
          ]}>
          <Ionicons name="school" size={38} color={orgColor} />
        </View>
      </View>

      {/* Organization Title & System Subtitle */}
      <View style={styles.textWrapper}>
        <View style={styles.badgeRow}>
          <View style={[styles.portalPill, { backgroundColor: theme.primaryLight }]}>
            <View style={[styles.onlineDot, { backgroundColor: orgColor }]} />
            <Text style={[styles.portalText, { color: orgColor }]}>
              {t('auth_org_portal')}
            </Text>
          </View>
        </View>

        <Text style={[styles.title, { color: theme.text }]} numberOfLines={2}>
          {orgName}
        </Text>

        <Text style={[styles.subtitle, { color: theme.textSecondary }]}>
          {t('auth_brand_subtitle')}
        </Text>
      </View>
    </View>
  );
});

SignInBrandHeader.displayName = 'SignInBrandHeader';

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    gap: Spacing.three,
    marginBottom: Spacing.two,
    width: '100%',
  },
  emblemWrapper: {
    width: 80,
    height: 80,
    borderRadius: 24,
    borderWidth: 1.5,
    alignItems: 'center',
    justifyContent: 'center',
    shadowOffset: { width: 0, height: 6 },
    shadowOpacity: 0.12,
    shadowRadius: 16,
    elevation: 4,
  },
  emblemInner: {
    width: 64,
    height: 64,
    borderRadius: 18,
    alignItems: 'center',
    justifyContent: 'center',
  },
  textWrapper: {
    alignItems: 'center',
    gap: Spacing.one,
    paddingHorizontal: Spacing.two,
  },
  badgeRow: {
    marginBottom: Spacing.one,
  },
  portalPill: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 6,
    paddingHorizontal: Spacing.three,
    paddingVertical: 4,
    borderRadius: Radii.full,
  },
  onlineDot: {
    width: 6,
    height: 6,
    borderRadius: 3,
  },
  portalText: {
    ...Typography.captionBold,
    fontSize: 11,
    letterSpacing: 0.2,
  },
  title: {
    ...Typography.h1,
    fontSize: 24,
    textAlign: 'center',
    fontWeight: '800',
  },
  subtitle: {
    ...Typography.caption,
    textAlign: 'center',
    maxWidth: 280,
  },
});
