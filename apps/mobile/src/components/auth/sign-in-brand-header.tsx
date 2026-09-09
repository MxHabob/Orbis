/**
 * @module @orbis/mobile/components/auth/sign-in-brand-header
 *
 * Organization branding for authentication.
 */

import React from 'react';

import {
  View,
  Text,
  StyleSheet,
  Pressable,
} from 'react-native';

import { Ionicons } from '@expo/vector-icons';

import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';

import {
  useCurrentOrgQuery,
} from '@/hooks/use-api-queries';

import {
  OrganizationLogo,
} from '@/components/ui/organization-logo';

import {
  Radii,
  Spacing,
  Typography,
} from '@/constants/theme';

export const SignInBrandHeader =
  React.memo(() => {
    const theme = useTheme();

    const {
      t,
      locale,
      setLocale,
    } = useI18n();

    const {
      data: orgData,
    } = useCurrentOrgQuery();

    const orgName =
      orgData?.name ||
      t('auth_org_portal');

    const toggleLanguage =
      async () => {
        await setLocale(
          locale === 'ar'
            ? 'en'
            : 'ar',
        );
      };

    return (
      <View
        style={styles.container}>

        {/* Language */}

        <View
          style={styles.topBar}>
          <Pressable
            onPress={toggleLanguage}
            accessibilityRole="button"
            hitSlop={8}
            style={({ pressed }) => [
              styles.langPill,

              {
                backgroundColor:
                  theme.backgroundElement,

                borderColor:
                  theme.border,

                opacity:
                  pressed
                    ? 0.75
                    : 1,
              },
            ]}>

            <Text
              style={[
                styles.langText,
                {
                  color:
                    theme.text,
                },
              ]}>
              {locale === 'ar'
                ? 'English'
                : 'العربية'}
            </Text>

            <Ionicons
              name="globe-outline"
              size={15}
              color={
                theme.textSecondary
              }
            />

          </Pressable>
        </View>

        {/* Organization Logo */}

        <OrganizationLogo
          size={64}
          color={
            theme.primary
          }
          dotColor={
            theme.brandPink
          }
        />

        {/* Organization Name */}

        <Text
          style={[
            styles.title,
            {
              color:
                theme.text,
            },
          ]}
          numberOfLines={2}>

          {orgName}

        </Text>

        {/* Subtitle */}

        <Text
          style={[
            styles.subtitle,
            {
              color:
                theme.textSecondary,
            },
          ]}>

          {t('auth_welcome_sub')}

        </Text>
      </View>
    );
  });

SignInBrandHeader.displayName =
  'SignInBrandHeader';

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',

    width: '100%',

    paddingTop:
      Spacing.three,

    paddingBottom:
      Spacing.six,
  },

  topBar: {
    width: '100%',

    flexDirection: 'row',

    justifyContent: 'flex-start',

    marginBottom:
      Spacing.ten,
  },

  langPill: {
    flexDirection: 'row',

    alignItems: 'center',

    gap: 7,

    paddingHorizontal: 14,

    paddingVertical: 8,

    borderRadius:
      Radii.full,

    borderWidth: 1,

    borderBottomWidth: 2,
  },

  langText: {
    ...Typography.captionBold,

    fontSize: 12,
  },

  title: {
    ...Typography.h1,

    fontSize: 25,

    lineHeight: 32,

    fontWeight: '800',

    textAlign: 'center',

    marginTop:
      Spacing.two,

    maxWidth: '90%',
  },

  subtitle: {
    ...Typography.body,

    fontSize: 15,

    lineHeight: 23,

    textAlign: 'center',

    maxWidth: '90%',

    marginTop:
      Spacing.one,
  },
});