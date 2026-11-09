/**
 * @module @orbis/mobile/app/(auth)/forgot-password
 *
 * Minimalist, elegant Password Recovery screen directly inspired by Reference Image 3.
 */

import React, { useState } from "react";
import { Text, View, Pressable, TextInput } from "react-native";
import { useRouter } from "expo-router";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { Button } from "@/components/ui/button";
import { IconButton } from "@/components/ui/icon-button";
import { ArrowLeft, ArrowRight, CheckCircle } from "@orbis/icons/native";

export default function ForgotPasswordScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { t, isArabic } = useI18n();

  const [email, setEmail] = useState("");
  const [isSubmitted, setIsSubmitted] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const handleSend = async () => {
    setErrorMsg(null);
    const trimmed = email.trim();
    if (!trimmed || !trimmed.includes("@")) {
      setErrorMsg(t("auth_error_required"));
      return;
    }

    setIsLoading(true);
    try {
      await new Promise((resolve) => setTimeout(resolve, 600));
      setIsSubmitted(true);
    } catch {
      setErrorMsg(t("auth_error_failed"));
    } finally {
      setIsLoading(false);
    }
  };

  const BackIcon = isArabic ? ArrowRight : ArrowLeft;
  const ArrowIcon = isArabic ? ArrowLeft : ArrowRight;

  return (
    <View className="flex-1 bg-background">
      <KeyboardAwareView
        style={{ flex: 1 }}
        contentContainerStyle={{
          flexGrow: 1,
          justifyContent: "space-between",
          paddingHorizontal: 24,
          paddingTop: 48,
          paddingBottom: 32,
        }}
      >
        {/* Minimal Nav Header */}
        <View className="mb-6 flex-row items-center">
          <Pressable
            onPress={() => router.back()}
            className="h-11 w-11 items-center justify-center"
            hitSlop={{ top: 12, bottom: 12, left: 12, right: 12 }}
          >
            <BackIcon size={20} color={theme.foreground} />
          </Pressable>
        </View>

        {!isSubmitted ? (
          <View className="flex-1 justify-center">
            {/* Title Block */}
            <View className="mb-6">
              <Text className="font-sans text-[28px] leading-8.5 font-extrabold tracking-[-0.5px] text-foreground">
                {t("auth_forgot_title")}
              </Text>
              <Text className="mt-one font-sans text-sm leading-5 text-muted-foreground">
                {t("auth_forgot_subtitle")}
              </Text>
            </View>

            {errorMsg ? (
              <View className="mb-four rounded-lg border border-destructive bg-destructive/10 px-three py-two">
                <Text className="font-sans text-xs leading-4 font-semibold text-destructive">
                  {errorMsg}
                </Text>
              </View>
            ) : null}

            {/* Large Pill Input with embedded arrow */}
            <View
              className="min-h-12 flex-row items-center rounded-md border border-border bg-card px-four"
              style={{
                shadowColor: "#000000",
                shadowOffset: { width: 0, height: 2 },
                shadowOpacity: 0.06,
                shadowRadius: 4,
                elevation: 2,
              }}
            >
              <TextInput
                className="flex-1 px-two font-sans text-base text-foreground"
                style={{
                  textAlign: isArabic ? "right" : "left",
                }}
                placeholder={t("auth_email_placeholder")}
                placeholderTextColor={theme.textMuted}
                value={email}
                onChangeText={setEmail}
                keyboardType="email-address"
                autoCapitalize="none"
                autoFocus
                onSubmitEditing={handleSend}
                returnKeyType="go"
              />

              <IconButton
                icon={<ArrowIcon size={16} color={theme.primaryForeground} />}
                onPress={() => void handleSend()}
                accessibilityLabel={t("auth_forgot_title")}
                disabled={isLoading}
                loading={isLoading}
                size="md"
              />
            </View>
          </View>
        ) : (
          /* Confirmation State */
          <View className="flex-1 items-center justify-center px-4">
            <View className="mb-five h-16 w-16 items-center justify-center rounded-full bg-success/10">
              <CheckCircle size={44} color={theme.success} />
            </View>

            <Text className="mb-two text-center font-sans text-2xl leading-7.5 font-extrabold text-foreground">
              {t("auth_forgot_success_title")}
            </Text>

            <Text className="mb-eight text-center font-sans text-sm leading-5.5 text-muted-foreground">
              {t("auth_forgot_success_msg")}
            </Text>

            <View className="w-full">
              <Button
                title={t("auth_back_to_sign_in")}
                onPress={() => router.replace("/(auth)/sign-in")}
                fullWidth
                size="lg"
              />
            </View>
          </View>
        )}
      </KeyboardAwareView>
    </View>
  );
}
