/**
 * @module @orbis/mobile/components/auth/sign-in-brand-header
 *
 * Highly refined, minimal Organization Brand Header for the Sign-In flow.
 * Displays official organization logo, language toggle, and institutional identity.
 */

import React from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useTheme } from '@/hooks/use-theme';
import { useColorScheme } from '@/hooks/use-color-scheme';
import { useI18n } from '@/lib/i18n';
import { useCurrentOrgQuery } from '@/hooks/use-api-queries';
import { OrganizationLogo } from '@/components/ui/organization-logo';
import { Radii, Spacing, Typography } from '@/constants/theme';

export const SignInBrandHeader = React.memo(() => {
  const theme = useTheme();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === 'dark';
  const { t, locale, setLocale } = useI18n();
  const { data: orgData } = useCurrentOrgQuery();

  const orgName = orgData?.name || t('nav_campus_default');
  const orgColor = orgData?.primary_color || theme.primary;

  const toggleLanguage = () => {
    setLocale(locale === 'ar' ? 'en' : 'ar');
  };

  return (
    <View style={styles.container}>
      {/* Top Language Switcher Bar */}
      <View style={styles.topBar}>
        <Pressable
          onPress={toggleLanguage}
          style={({ pressed }) => [
            styles.langPill,
            {
              backgroundColor: theme.backgroundElement,
              borderColor: theme.border,
              opacity: pressed ? 0.8 : 1,
            },
          ]}>
          <Ionicons name="globe-outline" size={14} color={theme.textSecondary} />
          <Text style={[styles.langText, { color: theme.text }]}>
            {locale === 'ar' ? 'English' : 'العربية'}
          </Text>
        </Pressable>
      </View>

      {/* Official Organization / Institution Vector Logo */}
      <OrganizationLogo
        size={46}
        color={isDark ? '#FFFFFF' : orgColor}
        dotColor="#A821D9"
        withContainer
      />

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
  topBar: {
    width: '100%',
    flexDirection: 'row',
    justifyContent: 'flex-end',
    marginBottom: -Spacing.two,
  },
  langPill: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 6,
    paddingHorizontal: Spacing.three,
    paddingVertical: 6,
    borderRadius: Radii.full,
    borderWidth: 1,
  },
  langText: {
    ...Typography.captionBold,
    fontSize: 12,
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
