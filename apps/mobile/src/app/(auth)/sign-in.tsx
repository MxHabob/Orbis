/**
 * @module @orbis/mobile/app/(auth)/sign-in
 *
 * Primary Authentication Screen for Orbis Mobile.
 * Supports Email/Password, Google OAuth, and One-Tap Demo Profiles.
 */

import React, { useState } from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  KeyboardAvoidingView,
  Platform,
  Pressable,
  Alert,
} from 'react-native';
import { router, Link } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth } from '@/lib/api';
import { useTheme } from '@/hooks/use-theme';
import { useEffectiveContext } from '@/providers/effective-context';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Radii, Spacing, Typography } from '@/constants/theme';

export default function SignInScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { login, loginWithGoogle, setSession } = useAuth();
  const { setRole } = useEffectiveContext();

  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSignIn = async () => {
    if (!email.trim() || !password) {
      setError('يرجى إدخال البريد الإلكتروني وكلمة المرور');
      return;
    }

    setLoading(true);
    setError(null);
    try {
      const res = await login(email.trim(), password);
      if (res?.token && res?.user) {
        router.replace('/(app)/(tabs)');
      }
    } catch (err: any) {
      setError(err?.message || 'فشل تسجيل الدخول. يرجى التحقق من البيانات.');
    } finally {
      setLoading(false);
    }
  };

  const handleGoogleSignIn = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await loginWithGoogle('google-oauth-token');
      if (res?.token && res?.user) {
        router.replace('/(app)/(tabs)');
      }
    } catch (err: any) {
      setError(err?.message || 'فشل تسجيل الدخول عبر Google');
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
            منصة الإدارة التعليمية والعمليات الموحدة
          </Text>
        </View>

        {/* Login Form Card */}
        <Card variant="elevated" style={styles.formCard}>
          <Text style={[styles.cardTitle, { color: theme.text }]}>تسجيل الدخول</Text>
          <Text style={[styles.cardSubtitle, { color: theme.textSecondary }]}>
            أدخل بريدك الإلكتروني المؤسسي وكلمة المرور للمتابعة
          </Text>

          {error && (
            <View style={[styles.errorBox, { backgroundColor: theme.errorLight }]}>
              <Ionicons name="alert-circle" size={18} color={theme.error} />
              <Text style={[styles.errorText, { color: theme.error }]}>{error}</Text>
            </View>
          )}

          <View style={styles.fields}>
            <Input
              label="البريد الإلكتروني"
              placeholder="name@school.edu.sa"
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
              label="كلمة المرور"
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
                    نسيت كلمة المرور؟
                  </Text>
                </Pressable>
              </Link>
            </View>

            <Button
              title="دخول"
              onPress={handleSignIn}
              loading={loading}
              fullWidth
              size="lg"
            />

            <View style={styles.dividerRow}>
              <View style={[styles.dividerLine, { backgroundColor: theme.border }]} />
              <Text style={[styles.dividerText, { color: theme.textMuted }]}>أو</Text>
              <View style={[styles.dividerLine, { backgroundColor: theme.border }]} />
            </View>

            <Button
              title="المتابعة باستخدام Google"
              onPress={handleGoogleSignIn}
              variant="secondary"
              fullWidth
              icon={<Ionicons name="logo-google" size={18} color={theme.text} />}
            />
          </View>
        </Card>

        {/* Registration Link Footer */}
        <View style={styles.footerRow}>
          <Text style={[styles.footerText, { color: theme.textSecondary }]}>
            ليس لديك حساب بعد؟
          </Text>
          <Link href="/(auth)/register" asChild>
            <Pressable hitSlop={8}>
              <Text style={[styles.registerLink, { color: theme.primary }]}>تسجيل حساب جديد</Text>
            </Pressable>
          </Link>
        </View>
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
    marginBottom: Spacing.two,
  },
  logoBadge: {
    width: 80,
    height: 80,
    borderRadius: 40,
    alignItems: 'center',
    justifyContent: 'center',
    marginBottom: Spacing.three,
  },
  brandTitle: {
    ...Typography.h1,
    letterSpacing: -0.5,
  },
  brandSubtitle: {
    ...Typography.caption,
    marginTop: 4,
    textAlign: 'center',
  },
  formCard: {
    width: '100%',
    maxWidth: 440,
    padding: Spacing.five,
  },
  cardTitle: {
    ...Typography.h2,
    marginBottom: 4,
  },
  cardSubtitle: {
    ...Typography.caption,
    marginBottom: Spacing.four,
  },
  errorBox: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: Spacing.three,
    borderRadius: Radii.md,
    gap: Spacing.two,
    marginBottom: Spacing.three,
  },
  errorText: {
    ...Typography.captionBold,
    flex: 1,
  },
  fields: {
    gap: Spacing.three,
  },
  forgotRow: {
    alignItems: 'flex-start',
    marginTop: -2,
    marginBottom: 4,
  },
  linkText: {
    ...Typography.captionBold,
  },
  dividerRow: {
    flexDirection: 'row',
    alignItems: 'center',
    marginVertical: Spacing.two,
    gap: Spacing.three,
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
    gap: 6,
    marginTop: Spacing.two,
  },
  footerText: {
    ...Typography.body,
  },
  registerLink: {
    ...Typography.bodyBold,
  },
});
