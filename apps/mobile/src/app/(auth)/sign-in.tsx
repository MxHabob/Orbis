/**
 * @module @orbis/mobile/app/(auth)/sign-in
 *
 * Minimalist authentication screen.
 *
 * Layout:
 * - Clean theme surface at the top
 * - Large animated gradient anchored to the bottom
 * - Transparent authentication surface over the gradient
 */

import { useState } from 'react';

import {
  StyleSheet,
  View,
  useWindowDimensions,
} from 'react-native';

import { router } from 'expo-router';

import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth } from '@/lib/api';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { useToast } from '@/components/ui/toast';

import {
  SignInBrandHeader,
  SignInForm,
} from '@/components/auth';

import { GradientHeader } from '@/components/gradient-header';

import { KeyboardAwareView } from '@/components/ui/keyboard-aware-view';

import {
  Spacing,
} from '@/constants/theme';

export default function SignInScreen() {
  const insets = useSafeAreaInsets();

  const { height } = useWindowDimensions();

  const theme = useTheme();

  const { locale } = useI18n();

  const toast = useToast();

  const {
    login,
    loginWithGoogle,
  } = useAuth();

  const [loading, setLoading] = useState(false);

  const gradientHeight = Math.max(
    620,
    height * 0.68,
  );

  const handleSignIn = async (
    email: string,
    pass: string,
  ) => {
    setLoading(true);

    try {
      const res = await login(
        email,
        pass,
      );

      if (res?.token && res?.user) {
        toast.success(
          `مرحباً بك ${res.user.full_name || ''}`,
        );

        router.replace(
          '/(app)/(tabs)',
        );
      } else {
        toast.error(
          'بيانات الدخول غير صحيحة، يرجى التحقق والمحاولة مجدداً',
        );
      }
    } catch (err: any) {
      toast.error(
        err?.message ||
          'تعذر تسجيل الدخول، يرجى المحاولة لاحقاً',
      );
    } finally {
      setLoading(false);
    }
  };

  const handleGoogleSignIn = async () => {
    setLoading(true);

    try {
      const res = await loginWithGoogle(
        'google-oauth-token',
      );

      if (res?.token && res?.user) {
        toast.success(
          `مرحباً بك ${res.user.full_name || ''}`,
        );

        router.replace(
          '/(app)/(tabs)',
        );
      }
    } catch (err: any) {
      toast.error(
        err?.message ||
          'تعذر تسجيل الدخول بواسطة Google',
      );
    } finally {
      setLoading(false);
    }
  };

  return (
    <View
      style={[
        styles.container,
        {
          backgroundColor:
            theme.background,
        },
      ]}>

      {/* =====================================================
          Bottom Gradient Surface
          ===================================================== */}

      <GradientHeader
        height={gradientHeight}
        colors={
          theme.gradientHeader.colors
        }
        locations={[
          0,
          0.08,
          0.20,
          0.40,
          0.68,
          1,
        ]}
        start={
          theme.gradientHeader.start
        }
        end={
          theme.gradientHeader.end
        }
        blobPrimary={
          theme.gradientHeader.blobPrimary
        }
        blobSecondary={
          theme.gradientHeader.blobSecondary
        }
        blobTertiary={
          theme.gradientHeader.blobTertiary
        }
        animated
        duration={18000}
        style={styles.gradient}
      />

      {/* =====================================================
          Content
          ===================================================== */}

      <KeyboardAwareView
        style={styles.keyboardView}
        contentContainerStyle={[
          styles.scrollContent,

          {
            paddingTop:
              Math.max(
                insets.top,
                Spacing.four,
              ),

            paddingBottom:
              Math.max(
                insets.bottom,
                Spacing.four,
              ),

            direction:
              locale === 'ar'
                ? 'rtl'
                : 'ltr',
          },
        ]}>

        <SignInBrandHeader
          key={locale}
        />

        <SignInForm
          onSignIn={handleSignIn}
          onGoogleSignIn={
            handleGoogleSignIn
          }
          loading={loading}
        />

      </KeyboardAwareView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },

  keyboardView: {
    flex: 1,
  },

  gradient: {
    position: 'absolute',

    left: 0,
    right: 0,
    bottom: 0,

    zIndex: 0,
  },

  scrollContent: {
    width: '100%',

    alignItems: 'center',

    paddingHorizontal:
      Spacing.four,

    gap: 0,

    zIndex: 1,
  },
});