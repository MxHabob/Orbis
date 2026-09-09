/**
 * @module @orbis/mobile/components/auth/sign-in-form
 *
 * Minimalist, high-aesthetic SignIn Form Card.
 * Streamlined credentials form, tactile submit button, Google SSO, and admission link.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, Pressable, Keyboard } from 'react-native';
import { Link } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { Card } from '@/components/ui/card';
import { Checkbox } from '@/components/ui/checkbox';
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
    const [rememberMe, setRememberMe] = useState(false);
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
        const msg = String(err?.message || '');
        const code = String(err?.code || '');
        if (
          code === 'NETWORK_ERROR' ||
          msg.includes('ConnectException') ||
          msg.includes('Network request failed') ||
          msg.includes('Failed to connect') ||
          msg.includes('NetworkError') ||
          msg.includes('fetch failed') ||
          msg.includes('ECONNREFUSED') ||
          msg.includes('ENOTFOUND') ||
          msg.includes('ETIMEDOUT')
        ) {
          setError(t('auth_error_network'));
        } else if (
          err?.status === 401 ||
          code === 'UNAUTHORIZED' ||
          code === 'INVALID_CREDENTIALS' ||
          msg.toLowerCase().includes('invalid credentials') ||
          msg.toLowerCase().includes('unauthorized') ||
          msg.toLowerCase().includes('user not found') ||
          msg.toLowerCase().includes('password')
        ) {
          setError(t('auth_error_invalid_credentials'));
        } else if (msg && !msg.includes('java.') && !msg.includes('http') && msg.length < 120) {
          setError(msg);
        } else {
          setError(t('auth_error_failed'));
        }
      }
    };

    const handleGoogle = async () => {
      Keyboard.dismiss();
      setError(null);
      try {
        await onGoogleSignIn();
      } catch (err: any) {
        const msg = String(err?.message || '');
        const code = String(err?.code || '');
        if (
          code === 'NETWORK_ERROR' ||
          msg.includes('ConnectException') ||
          msg.includes('Network request failed') ||
          msg.includes('Failed to connect')
        ) {
          setError(t('auth_error_network'));
        } else {
          setError(t('auth_error_google'));
        }
      }
    };

    return (
      <View style={styles.container}>
        {/* Main Minimalist Card */}
        <Card variant="elevated" style={styles.card}>
          {error && (
            <View
              style={[
                styles.errorBox,
                {
                  backgroundColor: theme.errorLight,
                  borderColor: theme.error,
                },
              ]}>
              <Ionicons name="alert-circle" size={18} color={theme.error} />
              <Text style={[styles.errorText, { color: theme.error }]}>{error}</Text>
            </View>
          )}

          <View style={styles.fields}>
            <Button
              title={t('auth_continue_google')}
              onPress={handleGoogle}
              variant="secondary"
              fullWidth
              icon={<Ionicons name="logo-google" size={18} color={theme.foreground} />}
            />

            <View style={styles.dividerRow}>
              <View style={[styles.dividerLine, { backgroundColor: theme.border }]} />
              <Text style={[styles.dividerText, { color: theme.textMuted }]}>
                {t('auth_or_divider')}
              </Text>
              <View style={[styles.dividerLine, { backgroundColor: theme.border }]} />
            </View>

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
              icon={<Ionicons name="mail-outline" size={18} color={theme.textMuted} />}
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
              icon={<Ionicons name="lock-closed-outline" size={18} color={theme.textMuted} />}
            />

            <View style={styles.optionsRow}>
              <Pressable
                onPress={() => setRememberMe((value) => !value)}
                style={styles.rememberOption}
                hitSlop={8}>
                <Checkbox
                  value={rememberMe}
                  onValueChange={setRememberMe}
                  accessibilityLabel="تذكرني"
                />
                <Text style={[styles.optionText, { color: theme.textSecondary }]}>تذكرني</Text>
              </Pressable>

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
          </View>

          <View style={styles.footerRow}>
            <Text style={[styles.footerPrompt, { color: theme.textSecondary }]}> 
              {t('auth_no_account')}
            </Text>
            <Link href="/(auth)/register" asChild>
              <Pressable hitSlop={8}>
                <Text style={[styles.footerLink, { color: theme.primary }]}> 
                  {t('auth_register_link')}
                </Text>
              </Pressable>
            </Link>
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
    gap: Spacing.three,
    marginTop: -Spacing.six,
    zIndex: 1,
  },
  card: {
    padding: Spacing.four,
    gap: Spacing.three,
  },
  errorBox: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
    paddingHorizontal: Spacing.three,
    paddingVertical: Spacing.two + 2,
    borderRadius: Radii.md,
    borderWidth: 1,
  },
  errorText: {
    ...Typography.captionBold,
    fontSize: 12.5,
    flex: 1,
    lineHeight: 18,
  },
  fields: {
    gap: Spacing.three,
  },
  optionsRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    marginTop: -4,
  },
  rememberOption: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
  },
  optionText: {
    ...Typography.body,
    fontSize: 13,
  },
  linkText: {
    ...Typography.captionBold,
    fontSize: 12,
  },
  dividerRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
    marginVertical: 2,
  },
  dividerLine: {
    flex: 1,
    height: 1,
  },
  dividerText: {
    ...Typography.caption,
    fontSize: 11,
  },
  footerRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    flexWrap: 'wrap',
    gap: 6,
    paddingVertical: Spacing.two,
  },
  footerPrompt: {
    ...Typography.caption,
    fontSize: 13,
  },
  footerLink: {
    ...Typography.captionBold,
    fontSize: 13,
  },
});
