/**
 * @module @orbis/mobile/app/(auth)/register
 *
 * Minimalist, elegant step-by-step registration directly inspired by Reference Image 3
 * (Single focused question per step, large pill inputs, tactile selection chips,
 * and bottom pill continue buttons).
 */

import{ useState } from "react";
import {
  Text,
  View,
  Pressable,
  TextInput,
  Platform,
} from "react-native";
import { useRouter } from "expo-router";
import { api, useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { useToast } from "@/components/ui/toast";
import {
  ArrowLeft,
  ArrowRight,
} from "@orbis/icons/native";
import { Button } from "@/components/ui/button";

type RoleOption = "teacher" | "student" | "parent" | "admin";

export default function RegisterScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { t, isArabic } = useI18n();
  const toast = useToast();
  const { login } = useAuth();

  const [step, setStep] = useState<1 | 2 >(1);
  const [fullName, setFullName] = useState("");
  const [selectedRole, setSelectedRole] = useState<RoleOption>("teacher");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);


  const handleNextStep1 = () => {
    setErrorMsg(null);
    if (!fullName.trim()) {
      setErrorMsg(t("auth_error_required"));
      return;
    }
    setStep(2);
  };


  const handleFinalSubmit = async () => {
    setErrorMsg(null);
    const trimmedEmail = email.trim();
    if (!trimmedEmail || !password) {
      setErrorMsg(t("auth_error_required"));
      return;
    }

    setIsSubmitting(true);
    try {
      await api.registerUser({
        body: {
          full_name: fullName.trim(),
          email: trimmedEmail,
          password,
          role: selectedRole,
        },
        throwOnError: true,
      });

      toast.success(t("auth_register_success"));

      try {
        await login(trimmedEmail, password);
        router.replace("/");
      } catch {
        router.replace("/(auth)/sign-in");
      }
    } catch (err: any) {
      const msg = err?.message || t("auth_error_failed");
      setErrorMsg(msg);
      toast.error(msg);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleBack = () => {
    setErrorMsg(null);
    if (step === 2) setStep(2);
    else if (step === 1) setStep(1);
    else router.back();
  };

  const BackIcon = isArabic ? ArrowLeft : ArrowRight;

  return (
    <View
      className="flex-1 bg-background"
    >

      <KeyboardAwareView
        style={{ flex: 1 }}
        contentContainerStyle={{
          flexGrow: 1,
          justifyContent: "space-between",
          paddingHorizontal: 20,
          paddingTop: Platform.OS === "ios" ? 54 : 36,
          paddingBottom: 32,
        }}
      >
        {/* Minimal Nav Bar */}
        <View className="mb-6 flex-row items-center justify-between">
          <Pressable
            onPress={handleBack}
            hitSlop={{ top: 12, bottom: 12, left: 12, right: 12 }}
          >
            <BackIcon size={18} color={theme.foreground} />
          </Pressable>

          {/* Minimal Step Dots Indicator */}
          <View className="flex-row items-center gap-[6px]">
            {[1, 2, 3].map((s) => (
              <View
                key={s}
                className={`h-[6px] rounded-[3px] ${s <= step ? "bg-primary" : "bg-border"}`}
                style={{ width: s === step ? 20 : 6 }}
              />
            ))}
          </View>

          <Pressable
            onPress={() => router.push("/(auth)/sign-in")}
            hitSlop={{ top: 10, bottom: 10, left: 10, right: 10 }}
          >
            <Text
              className="font-sans text-xs leading-4 font-semibold text-muted-foreground"
            >
              {t("auth_sign_in")}
            </Text>
          </Pressable>
        </View>

        {/* Dynamic Step Content */}
        <View className="flex-1 justify-center">
          {/* STEP 1: NAME */}
          {step === 1 && (
            <View className="w-full items-end">
              <View className="mb-6">
                <Text className="font-sans text-[28px] leading-[34px] font-extrabold tracking-[-0.5px] text-foreground">
                  {t("auth_full_name")}
                </Text>
              </View>

              <View>
                <TextInput
                  className="w-[370px] rounded-[6px] border border-border bg-background-element text-foreground"
                  placeholder={t("auth_full_name_ph")}
                  placeholderTextColor={theme.textMuted}
                  value={fullName}
                  onChangeText={setFullName}
                  autoFocus
                  onSubmitEditing={handleNextStep1}
                  returnKeyType="next"
                />
              </View>
            </View>
          )}

          {/* STEP 2: CREDENTIALS */}
          {step === 2 && (
            <View className="w-full items-end">
              <View className="mb-6">
                <Text className="font-sans text-[28px] leading-[34px] font-extrabold tracking-[-0.5px] text-foreground">
                  {t("auth_email_label")}
                </Text>
              </View>

              <View className="gap-3">
                  <TextInput
                    className="w-[370px] rounded-[6px] border border-border bg-background-element text-foreground"
                    placeholder={t("auth_email_placeholder")}
                    placeholderTextColor={theme.textMuted}
                    value={email}
                    onChangeText={setEmail}
                    keyboardType="email-address"
                    autoCapitalize="none"
                    autoFocus
                    />
   
                  <TextInput
                    className="w-[370px] rounded-[6px] border border-border bg-background-element text-foreground"
                    placeholder="••••••••"
                    placeholderTextColor={theme.textMuted}
                    value={password}
                    onChangeText={setPassword}
                    secureTextEntry={!showPassword}
                    onSubmitEditing={handleFinalSubmit}
                    returnKeyType="go"
                  />

              </View>
            </View>
          )}
        </View>

        {/* Bottom Tactile Pill Button */}
        <View className="mt-6">
          <Button
            onPress={
              step === 1
                ? handleNextStep1
                  : handleFinalSubmit
            }
            disabled={isSubmitting}
          >
            <Text
              className="font-sans text-base leading-5 font-bold"
            >
              {step === 2
                ? t("auth_create_account_btn")
                : t("admission_next_step")}
            </Text>
          </Button>
        </View>
      </KeyboardAwareView>
    </View>
  );
}
