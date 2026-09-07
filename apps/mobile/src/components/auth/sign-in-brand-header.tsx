/**
 * @module @orbis/mobile/components/auth/sign-in-brand-header
 *
 * Minimal, ultra-clean Organization Brand Header for the Sign-In flow.
 * Displays official organization logo and name without redundant clutter.
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
      {/* Top Language Switcher Toggle */}
      <View style={styles.topBar}>
        <Pressable
          onPress={toggleLanguage}
          hitSlop={8}
          style={({ pressed }) => [
            styles.langPill,
            {
              backgroundColor: theme.backgroundElement,
              borderColor: theme.border,
              opacity: pressed ? 0.75 : 1,
              transform: [{ translateY: pressed ? 1 : 0 }],
            },
          ]}>
          <Ionicons name="globe-outline" size={13} color={theme.textSecondary} />
          <Text style={[styles.langText, { color: theme.text }]}>
            {locale === 'ar' ? 'English' : 'العربية'}
          </Text>
        </Pressable>
      </View>

      {/* Official Organization Logo */}
      <OrganizationLogo
        size={52}
        color={isDark ? '#FFFFFF' : orgColor}
        dotColor="#A821D9"
        withContainer
      />

      {/* Organization Name */}
      <Text style={[styles.title, { color: theme.text }]} numberOfLines={2}>
        {orgName}
      </Text>
    </View>
  );
});

SignInBrandHeader.displayName = 'SignInBrandHeader';

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    gap: Spacing.two,
    marginBottom: Spacing.three,
    width: '100%',
  },
  topBar: {
    width: '100%',
    flexDirection: 'row',
    justifyContent: 'flex-end',
    paddingHorizontal: Spacing.one,
    marginBottom: Spacing.two,
  },
  langPill: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 6,
    paddingHorizontal: 12,
    paddingVertical: 6,
    borderRadius: Radii.full,
    borderWidth: 1,
    borderBottomWidth: 2,
  },
  langText: {
    ...Typography.captionBold,
    fontSize: 12,
  },
  title: {
    ...Typography.h1,
    fontSize: 22,
    textAlign: 'center',
    fontWeight: '800',
    marginTop: Spacing.one,
    maxWidth: '90%',
  },
});
