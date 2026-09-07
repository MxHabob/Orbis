/**
 * @module @orbis/mobile/app/(auth)/mfa-challenge
 *
 * Multi-Factor Authentication (TOTP) Challenge Verification Screen.
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
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Header } from '@/components/header';
import { KeyboardAwareView } from '@/components/ui/keyboard-aware-view';
import { Radii, Spacing, Typography } from '@/constants/theme';

export default function MfaChallengeScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { mfaChallenge, user } = useAuth();

  const [code, setCode] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleVerify = async () => {
    if (!code.trim() || code.length < 6) {
      setError('يرجى إدخال رمز التحقق المكون من 6 أرقام');
      return;
    }

    setLoading(true);
    setError(null);
    if (!user?.id) {
      setError('تعذر تحديد جلسة المستخدم، يرجى إعادة تسجيل الدخول');
      setLoading(false);
      return;
    }

    try {
      const res = await mfaChallenge({
        user_id: user.id,
        code: code.trim(),
      });
      if (res?.token) {
        router.replace('/(app)/(tabs)');
      }
    } catch (err: any) {
      setError(err?.message || 'رمز التحقق غير صحيح أو منتهي الصلاحية');
    } finally {
      setLoading(false);
    }
  };

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="التحقق بخطوتين (MFA)" showBack />

      <KeyboardAwareView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.six },
        ]}>
        <Card variant="elevated" style={styles.card}>
          <View style={[styles.iconCircle, { backgroundColor: theme.primaryLight }]}>
            <Ionicons name="shield-checkmark" size={36} color={theme.primary} />
          </View>

          <Text style={[styles.title, { color: theme.text }]}>أدخل رمز الأمان</Text>
          <Text style={[styles.subtitle, { color: theme.textSecondary }]}>
            أدخل الرمز المكون من 6 أرقام من تطبيق المصادقة (Google Authenticator أو ما يماثله)
          </Text>

          {error && (
            <View style={[styles.errorBox, { backgroundColor: theme.errorLight }]}>
              <Ionicons name="alert-circle" size={18} color={theme.error} />
              <Text style={[styles.errorText, { color: theme.error }]}>{error}</Text>
            </View>
          )}

          <Input
            placeholder="123456"
            value={code}
            onChangeText={(t) => {
              setCode(t);
              setError(null);
            }}
            maxLength={6}
            keyboardType="number-pad"
            style={styles.codeInput}
          />

          <Button
            title="تأكيد ومتابعة"
            onPress={handleVerify}
            loading={loading}
            fullWidth
            size="lg"
          />
        </Card>
      </KeyboardAwareView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: Spacing.four,
    alignItems: 'center',
    paddingTop: Spacing.four,
  },
  card: {
    width: '100%',
    maxWidth: 440,
    padding: Spacing.five,
    alignItems: 'center',
    gap: Spacing.three,
  },
  iconCircle: {
    width: 68,
    height: 68,
    borderRadius: 34,
    alignItems: 'center',
    justifyContent: 'center',
    marginBottom: Spacing.two,
  },
  title: {
    ...Typography.h2,
    textAlign: 'center',
  },
  subtitle: {
    ...Typography.caption,
    textAlign: 'center',
    marginBottom: Spacing.two,
  },
  errorBox: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: Spacing.three,
    borderRadius: Radii.md,
    gap: Spacing.two,
    width: '100%',
  },
  errorText: {
    ...Typography.captionBold,
    flex: 1,
  },
  codeInput: {
    textAlign: 'center',
    fontSize: 24,
    letterSpacing: 8,
    fontWeight: '700',
  },
});
