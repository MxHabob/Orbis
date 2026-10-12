/**
 * @module @orbis/mobile/app/(auth)/sign-in
 *
 * Professional Sign In screen for Orbis Mobile.
 * Clean enterprise authentication with email/password, Google sign-in,
 * bilingual language toggle, and fluid keyboard avoidance.
 */

import React, { useState } from "react";
import { View, Text, Pressable, Platform } from "react-native";
import { useRouter } from "expo-router";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { useToast } from "@/components/ui/toast";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import {
  AcademicCap,
  Eye,
  EyeSlash,
  ArrowRight,
  ArrowLeft,
  Language,
} from "@orbis/icons/native";

export default function SignInScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { t, isArabic, locale, setLocale } = useI18n();
  const toast = useToast();
  const { login, loginWithGoogle } = useAuth();

  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const ArrowIcon = isArabic ? ArrowLeft : ArrowRight;

  const handleSignIn = async () => {
    const trimmedEmail = email.trim();
    if (!trimmedEmail || !password) {
      toast.error(t("auth_error_required") || "Please enter your email and password.");
      return;
    }

    setIsSubmitting(true);
    try {
      const res: any = await login(trimmedEmail, password);
      if (res?.mfa_required && (res?.user_id || res?.user?.id)) {
        router.push({
          pathname: "/(auth)/mfa-challenge",
          params: { userId: res.user_id || res.user.id },
        });
        return;
      }
      router.replace("/(app)/(tabs)");
    } catch (err: any) {
      const msg = err?.message || t("auth_error_invalid_credentials");
      toast.error(msg);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleGoogleSignIn = async () => {
    setIsSubmitting(true);
    try {
      await loginWithGoogle("google-token");
      router.replace("/(app)/(tabs)");
    } catch (err: any) {
      toast.error(err?.message || t("auth_error_google"));
    } finally {
      setIsSubmitting(false);
    }
  };

  const toggleLanguage = () => {
    const next = locale === "ar" ? "en" : "ar";
    void setLocale(next);
  };

  return (
    <View className="flex-1 bg-background">
      <KeyboardAwareView
        style={{ flex: 1 }}
        contentContainerStyle={{
          flexGrow: 1,
          justifyContent: "space-between",
          paddingHorizontal: 24,
          paddingTop: Platform.OS === "ios" ? 64 : 44,
          paddingBottom: 36,
        }}
      >
        {/* Top Bar: Language Switcher */}
        <View className="flex-row items-center justify-between">
          <View />
          <Pressable
            accessibilityRole="button"
            onPress={toggleLanguage}
            className="flex-row items-center gap-one rounded-full border border-border bg-card px-three py-one active:opacity-70"
          >
            <Language size={14} color={theme.primary} />
            <Text className="text-xs font-bold text-foreground">
              {locale === "ar" ? "English" : "العربية"}
            </Text>
          </Pressable>
        </View>

        {/* Center: Brand Identity & Form */}
        <View className="gap-eight py-four">
          {/* Brand Header */}
          <View className="items-center gap-two">
            <View className="h-16 w-16 items-center justify-center rounded-2xl bg-primary shadow-md">
              <AcademicCap size={36} color="#FFFFFF" />
            </View>
            <View className="items-center gap-half">
              <Text className="text-3xl font-extrabold tracking-tight text-foreground">
                Orbis
              </Text>
              <Text className="text-sm font-medium text-muted-foreground text-center">
                {t("auth_sign_in_subtitle")}
              </Text>
            </View>
          </View>

          {/* Form Inputs */}
          <View className="gap-four">
            <View className="gap-one">
              <Text className="text-xs font-semibold text-foreground">
                {t("auth_username_or_email")}
              </Text>
              <Input
                placeholder="name@school.org"
                value={email}
                onChangeText={setEmail}
                keyboardType="email-address"
                autoCapitalize="none"
                returnKeyType="next"
              />
            </View>

            <View className="gap-one">
              <View className="flex-row items-center justify-between">
                <Text className="text-xs font-semibold text-foreground">
                  {t("auth_password")}
                </Text>
                <Pressable onPress={() => router.push("/(auth)/forgot-password")}>
                  <Text className="text-xs font-semibold text-primary">
                    {t("auth_forgot_password")}
                  </Text>
                </Pressable>
              </View>

              <View className="relative justify-center">
                <Input
                  placeholder="••••••••"
                  value={password}
                  onChangeText={setPassword}
                  secureTextEntry={!showPassword}
                  returnKeyType="go"
                  onSubmitEditing={handleSignIn}
                />
                <Pressable
                  accessibilityRole="button"
                  onPress={() => setShowPassword(!showPassword)}
                  className={`absolute top-0 bottom-0 justify-center px-three ${
                    isArabic ? "left-0" : "right-0"
                  }`}
                >
                  {showPassword ? (
                    <EyeSlash size={18} color={theme.mutedForeground} />
                  ) : (
                    <Eye size={18} color={theme.mutedForeground} />
                  )}
                </Pressable>
              </View>
            </View>

            {/* Primary Sign In Button */}
            <Button
              title={t("auth_sign_in")}
              onPress={handleSignIn}
              disabled={isSubmitting}
              loading={isSubmitting}
              fullWidth
            />

            {/* Google Sign In Option */}
            <Button
              title={t("auth_continue_google")}
              onPress={handleGoogleSignIn}
              disabled={isSubmitting}
              variant="outline"
              fullWidth
            />
          </View>
        </View>

        {/* Bottom Actions: Register & Explore */}
        <View className="items-center gap-three pt-four">
          <View className="flex-row items-center gap-one">
            <Text className="text-xs text-muted-foreground">
              {t("auth_no_account")}
            </Text>
            <Pressable onPress={() => router.push("/(auth)/register")}>
              <Text className="text-xs font-bold text-primary">
                {t("auth_register_title")}
              </Text>
            </Pressable>
          </View>

          <Pressable onPress={() => router.push("/(auth)/explore")}>
            <Text className="text-xs font-medium text-muted-foreground underline">
              {t("visitor_explore")}
            </Text>
          </Pressable>
        </View>
      </KeyboardAwareView>
    </View>
  );
}
