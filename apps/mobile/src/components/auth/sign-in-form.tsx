/**
 * @module @orbis/mobile/components/auth/sign-in-form
 *
 * Reusable SignIn Form card with input validation and OAuth triggers.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import { Link } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { Spacing, Typography } from '@/constants/theme';

interface SignInFormProps {
  onSignIn: (email: string, pass: string) => Promise<void>;
  onGoogleSignIn: () => Promise<void>;
  loading?: boolean;
}

export const SignInForm = React.memo<SignInFormProps>(
  ({ onSignIn, onGoogleSignIn, loading = false }) => {
    const theme = useTheme();
    const { t } = useI18n();

    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState<string | null>(null);

    const handleSubmit = async () => {
      if (!email.trim() || !password) {
        setError(t('auth_error_required'));
        return;
      }
      setError(null);
      try {
        await onSignIn(email.trim(), password);
      } catch (err: any) {
        setError(err?.message || t('auth_error_failed'));
      }
    };

    const handleGoogle = async () => {
      setError(null);
      try {
        await onGoogleSignIn();
      } catch (err: any) {
        setError(err?.message || t('auth_error_google'));
      }
    };

    return (
      <View style={styles.container}>
        <Card variant="elevated" style={styles.card}>
          <Text style={[styles.title, { color: theme.text }]}>{t('auth_card_title')}</Text>
          <Text style={[styles.subtitle, { color: theme.textSecondary }]}>
            {t('auth_card_subtitle')}
          </Text>

          {error && (
            <View style={[styles.errorBox, { backgroundColor: theme.errorLight }]}>
              <Ionicons name="alert-circle" size={18} color={theme.error} />
              <Text style={[styles.errorText, { color: theme.error }]}>{error}</Text>
            </View>
          )}

          <View style={styles.fields}>
            <Input
              label={t('auth_email_label')}
              placeholder={t('auth_email_placeholder')}
              value={email}
              onChangeText={(text) => {
                setEmail(text);
                setError(null);
              }}
              autoCapitalize="none"
              keyboardType="email-address"
              icon={<Ionicons name="mail-outline" size={20} color={theme.textMuted} />}
            />

            <Input
              label={t('auth_password_label')}
              placeholder="••••••••"
              value={password}
              onChangeText={(text) => {
                setPassword(text);
                setError(null);
              }}
              secureTextEntry
              icon={<Ionicons name="lock-closed-outline" size={20} color={theme.textMuted} />}
            />

            <View style={styles.forgotRow}>
              <Link href="/(auth)/forgot-password" asChild>
                <Pressable hitSlop={8}>
                  <Text style={[styles.linkText, { color: theme.primary }]}>
                    {t('auth_forgot_password')}
                  </Text>
                </Pressable>
              </Link>
            </View>

            <Button
              title={t('auth_submit')}
              onPress={handleSubmit}
              loading={loading}
              fullWidth
              size="lg"
            />

            <View style={styles.dividerRow}>
              <View style={[styles.dividerLine, { backgroundColor: theme.border }]} />
              <Text style={[styles.dividerText, { color: theme.textMuted }]}>
                {t('auth_or_divider')}
              </Text>
              <View style={[styles.dividerLine, { backgroundColor: theme.border }]} />
            </View>

            <Button
              title={t('auth_continue_google')}
              onPress={handleGoogle}
              variant="secondary"
              fullWidth
              icon={<Ionicons name="logo-google" size={18} color={theme.text} />}
            />
          </View>
        </Card>

        {/* Footer Link */}
        <View style={styles.footerRow}>
          <Text style={[styles.footerText, { color: theme.textSecondary }]}>
            {t('auth_no_account')}
          </Text>
          <Link href="/(auth)/register" asChild>
            <Pressable hitSlop={8}>
              <Text style={[styles.registerLink, { color: theme.primary }]}>
                {t('auth_register_link')}
              </Text>
            </Pressable>
          </Link>
        </View>
      </View>
    );
  },
);

SignInForm.displayName = 'SignInForm';

const styles = StyleSheet.create({
  container: {
    width: '100%',
    gap: Spacing.four,
  },
  card: {
    padding: Spacing.four,
  },
  title: {
    ...Typography.h2,
    fontSize: 22,
    marginBottom: Spacing.one,
  },
  subtitle: {
    ...Typography.small,
    marginBottom: Spacing.four,
  },
  errorBox: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
    padding: Spacing.three,
    borderRadius: 8,
    marginBottom: Spacing.three,
  },
  errorText: {
    ...Typography.small,
    flex: 1,
  },
  fields: {
    gap: Spacing.three,
  },
  forgotRow: {
    alignItems: 'flex-end',
    marginTop: -Spacing.one,
  },
  linkText: {
    ...Typography.small,
    fontWeight: '600',
  },
  dividerRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.three,
    marginVertical: Spacing.two,
  },
  dividerLine: {
    flex: 1,
    height: 1,
  },
  dividerText: {
    ...Typography.small,
  },
  footerRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    gap: Spacing.two,
  },
  footerText: {
    ...Typography.body,
  },
  registerLink: {
    ...Typography.bodyBold,
  },
});
