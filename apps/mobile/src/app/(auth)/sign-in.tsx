/**
 * @module @orbis/mobile/app/(auth)/sign-in
 *
 * Minimalist, elegant Authentication Screen for Orbis Mobile.
 * Thin orchestrator delegating presentation to SignInBrandHeader and SignInForm.
 */

import React, { useState } from 'react';
import { StyleSheet } from 'react-native';
import { router } from 'expo-router';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth } from '@/lib/api';
import { useTheme } from '@/hooks/use-theme';
import { useToast } from '@/components/ui/toast';
import { SignInBrandHeader, SignInForm } from '@/components/auth';
import { KeyboardAwareView } from '@/components/ui/keyboard-aware-view';
import { Spacing } from '@/constants/theme';

export default function SignInScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const toast = useToast();
  const { login, loginWithGoogle } = useAuth();
  const [loading, setLoading] = useState(false);

  const handleSignIn = async (email: string, pass: string) => {
    setLoading(true);
    try {
      const res = await login(email, pass);
      if (res?.token && res?.user) {
        toast.success(`مرحباً بك ${res.user.full_name || ''}`);
        router.replace('/(app)/(tabs)');
      } else {
        toast.error('بيانات الدخول غير صحيحة، يرجى التحقق والمحاولة مجدداً');
      }
    } catch (err: any) {
      toast.error(err?.message || 'تعذر تسجيل الدخول، يرجى المحاولة لاحقاً');
    } finally {
      setLoading(false);
    }
  };

  const handleGoogleSignIn = async () => {
    setLoading(true);
    try {
      const res = await loginWithGoogle('google-oauth-token');
      if (res?.token && res?.user) {
        toast.success(`مرحباً بك ${res.user.full_name || ''}`);
        router.replace('/(app)/(tabs)');
      }
    } catch (err: any) {
      toast.error(err?.message || 'تعذر تسجيل الدخول بواسطة Google');
    } finally {
      setLoading(false);
    }
  };

  return (
    <KeyboardAwareView
      style={[styles.container, { backgroundColor: theme.background }]}
      contentContainerStyle={[
        styles.scrollContent,
        {
          paddingTop: Math.max(insets.top, 24) + Spacing.two,
          paddingBottom: Math.max(insets.bottom, 24) + Spacing.six,
        },
      ]}>
      {/* Organization Logo & Identity Header */}
      <SignInBrandHeader />

      {/* Elegant Minimalist SignIn Form & Admission Entry */}
      <SignInForm
        onSignIn={handleSignIn}
        onGoogleSignIn={handleGoogleSignIn}
        loading={loading}
      />
    </KeyboardAwareView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: Spacing.four,
    alignItems: 'center',
    gap: Spacing.three,
  },
});
