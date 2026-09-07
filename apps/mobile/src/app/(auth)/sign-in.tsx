/**
 * @module @orbis/mobile/app/(auth)/sign-in
 *
 * Primary Authentication Screen for Orbis Mobile.
 * Clean, declarative component composition.
 */

import React, { useState } from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  KeyboardAvoidingView,
  Platform,
} from 'react-native';
import { router } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth } from '@/lib/api';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { SignInForm } from '@/components/auth';
import { Spacing, Typography } from '@/constants/theme';

export default function SignInScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { t } = useI18n();
  const { login, loginWithGoogle } = useAuth();
  const [loading, setLoading] = useState(false);

  const handleSignIn = async (email: string, pass: string) => {
    setLoading(true);
    try {
      const res = await login(email, pass);
      if (res?.token && res?.user) {
        router.replace('/(app)/(tabs)');
      }
    } finally {
      setLoading(false);
    }
  };

  const handleGoogleSignIn = async () => {
    setLoading(true);
    try {
      const res = await loginWithGoogle('google-oauth-token');
      if (res?.token && res?.user) {
        router.replace('/(app)/(tabs)');
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <KeyboardAvoidingView
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
      style={[styles.container, { backgroundColor: theme.background }]}>
      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          {
            paddingTop: insets.top + Spacing.six,
            paddingBottom: insets.bottom + Spacing.six,
          },
        ]}
        showsVerticalScrollIndicator={false}>
        {/* Brand Header */}
        <View style={styles.brandSection}>
          <View style={[styles.logoBadge, { backgroundColor: theme.primaryLight }]}>
            <Ionicons name="school" size={40} color={theme.primary} />
          </View>
          <Text style={[styles.brandTitle, { color: theme.text }]}>Orbis Mobile</Text>
          <Text style={[styles.brandSubtitle, { color: theme.textSecondary }]}>
            {t('auth_brand_subtitle')}
          </Text>
        </View>

        {/* Modular SignIn Form */}
        <SignInForm
          onSignIn={handleSignIn}
          onGoogleSignIn={handleGoogleSignIn}
          loading={loading}
        />
      </ScrollView>
    </KeyboardAvoidingView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: Spacing.four,
    alignItems: 'center',
    gap: Spacing.four,
  },
  brandSection: {
    alignItems: 'center',
    gap: Spacing.two,
    marginBottom: Spacing.two,
  },
  logoBadge: {
    width: 72,
    height: 72,
    borderRadius: 36,
    alignItems: 'center',
    justifyContent: 'center',
    marginBottom: Spacing.one,
  },
  brandTitle: {
    ...Typography.h1,
    fontSize: 26,
  },
  brandSubtitle: {
    ...Typography.small,
    textAlign: 'center',
  },
});
