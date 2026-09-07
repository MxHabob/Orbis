/**
 * @module @orbis/mobile/components/auth/sign-in-form
 *
 * Highly refined, minimal SignIn Form Card.
 * Includes credentials inputs, validation, Google SSO, and student admission link.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, Pressable, Keyboard } from 'react-native';
import { Link } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { Radii, Spacing, Typography } from '@/constants/theme';

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
      Keyboard.dismiss();
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
      Keyboard.dismiss();
      setError(null);
      try {
        await onGoogleSignIn();
      } catch (err: any) {
        setError(err?.message || t('auth_error_google'));
      }
    };

    return (
      <View style={styles.container}>
        {/* Main Clean Card */}
        <Card variant="elevated" style={styles.card}>
          <View style={styles.cardHeader}>
            <Text style={[styles.title, { color: theme.text }]}>{t('auth_card_title')}</Text>
            <Text style={[styles.subtitle, { color: theme.textSecondary }]}>
              {t('auth_card_subtitle')}
            </Text>
          </View>

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
              autoCorrect={false}
              keyboardType="email-address"
              returnKeyType="next"
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
              returnKeyType="done"
              onSubmitEditing={handleSubmit}
              icon={<Ionicons name="lock-closed-outline" size={20} color={theme.textMuted} />}
            />

            <View style={styles.forgotRow}>
              <Link href="/(auth)/forgot-password" asChild>
                <Pressable hitSlop={10}>
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

        {/* Student Admission / Registration Card */}
        <Card
          variant="flat"
          style={[
            styles.admissionCard,
            { backgroundColor: theme.backgroundElement, borderColor: theme.border },
          ]}>
          <View style={styles.admissionContent}>
            <View style={[styles.admissionIconBox, { backgroundColor: theme.primaryLight }]}>
              <Ionicons name="person-add-outline" size={22} color={theme.primary} />
            </View>
            <View style={styles.admissionTextGroup}>
              <Text style={[styles.admissionPrompt, { color: theme.text }]}>
                {t('auth_no_account')}
              </Text>
              <Link href="/(auth)/register" asChild>
                <Pressable hitSlop={8} style={styles.admissionLinkBtn}>
                  <Text style={[styles.admissionLinkText, { color: theme.primary }]}>
                    {t('auth_register_link')}
                  </Text>
                  <Ionicons name="arrow-back" size={14} color={theme.primary} />
                </Pressable>
              </Link>
            </View>
          </View>
        </Card>
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
    gap: Spacing.three,
  },
  cardHeader: {
    gap: 4,
    marginBottom: Spacing.one,
  },
  title: {
    ...Typography.h2,
    fontSize: 20,
    fontWeight: '700',
  },
  subtitle: {
    ...Typography.caption,
  },
  errorBox: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
    padding: Spacing.three,
    borderRadius: Radii.md,
  },
  errorText: {
    ...Typography.captionBold,
    flex: 1,
  },
  fields: {
    gap: Spacing.three,
  },
  forgotRow: {
    alignItems: 'flex-end',
    marginTop: -2,
  },
  linkText: {
    ...Typography.captionBold,
  },
  dividerRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.three,
    marginVertical: Spacing.one,
  },
  dividerLine: {
    flex: 1,
    height: 1,
  },
  dividerText: {
    ...Typography.caption,
  },
  admissionCard: {
    padding: Spacing.three,
    borderWidth: 1,
    borderRadius: Radii.lg,
  },
  admissionContent: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.three,
  },
  admissionIconBox: {
    width: 44,
    height: 44,
    borderRadius: 22,
    alignItems: 'center',
    justifyContent: 'center',
  },
  admissionTextGroup: {
    flex: 1,
    gap: 2,
  },
  admissionPrompt: {
    ...Typography.caption,
  },
  admissionLinkBtn: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 4,
  },
  admissionLinkText: {
    ...Typography.bodyBold,
    fontSize: 14,
  },
});
