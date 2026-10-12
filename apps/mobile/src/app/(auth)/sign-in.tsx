/**
 * @module @orbis/mobile/app/(auth)/sign-in
 *
 * Ultra-modern, elegant Sign In screen directly modeled after the Mobbin Fuse
 * and minimalist reference designs (Atmospheric radiant glow, rotating capability
 * badges, frosted glass bottom container, sleek pill inputs, and tactile buttons).
 */

import { Button, Input } from "@/components/ui";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { useToast } from "@/components/ui/toast";
import { useColorScheme } from "@/hooks/use-color-scheme";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import {
  AcademicCap,
  ArrowLeft,
  ArrowRight,
  Bolt,
  BuildingLibrary,
  ChartBar
} from "@orbis/icons/native";
import { useRouter } from "expo-router";
import { useState } from "react";
import {
  Platform,
  Text,
  View
} from "react-native";

const CAPABILITIES = [
  { labelKey: "auth_capability_teach", icon: AcademicCap, color: "#8B5CF6" },
  { labelKey: "auth_capability_analyze", icon: ChartBar, color: "#06B6D4" },
  { labelKey: "auth_capability_automate", icon: Bolt, color: "#84CC16" },
  { labelKey: "auth_capability_govern", icon: BuildingLibrary, color: "#EC4899" },
] as const;

export default function SignInScreen() {
  const router = useRouter();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const { t, isArabic } = useI18n();
  const toast = useToast();
  const { login, loginWithGoogle } = useAuth();

  const [authMode, setAuthMode] = useState<"welcome" | "email" | "password">("welcome");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const handleProceedFromEmail = () => {
    setErrorMsg(null);
    const trimmed = email.trim();
    if (!trimmed || !trimmed.includes("@")) {
      setErrorMsg(t("auth_error_required"));
      return;
    }
    setAuthMode("password");
  };

  const handleSignIn = async () => {
    setErrorMsg(null);
    const trimmedEmail = email.trim();
    if (!trimmedEmail || !password) {
      setErrorMsg(t("auth_error_required"));
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
      setErrorMsg(msg);
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

  const ArrowIcon = isArabic ? ArrowRight : ArrowLeft;

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
          paddingTop: Platform.OS === "ios" ? 80 : 50,
          paddingBottom: 32,
        }}
      >
        <View className="flex-1 items-end justify-center min-h-[220px] gap-2">
            {CAPABILITIES.map((cap) => {
              return (
                    <View key={cap.labelKey}>
                      <Text className="font-sans text-2xl leading-[30px] font-extrabold text-foreground">
                        {t(cap.labelKey)}
                      </Text>
                    </View>
                  )})}
        </View>


        <View
          className="px-6 pt-6 pb-5"
        >
          {authMode === "welcome" && (
            <View className="gap-3">
              <Button
                onPress={handleGoogleSignIn}
              >
                <Text
                  className="font-sans text-sm leading-5 font-semibold text-foreground"
                >
                  {t("auth_continue_google")}
                </Text>
              </Button>

              {/* Secondary Translucent Pill Button: Email */}
              <Button
                onPress={() => setAuthMode("email")}
                variant="secondary"
              >
                <Text
                  className="font-sans text-sm leading-5 font-semibold text-foreground"
                >
                  {t("auth_sign_in")}
                </Text>
              </Button>
            </View>
          )}
          {authMode === "email" && (
            <View className="gap-4">
                <Input
                  placeholder={t("auth_email_placeholder")}
                  value={email}
                  onChangeText={setEmail}
                  keyboardType="email-address"
                  autoCapitalize="none"
                  autoFocus
                  onSubmitEditing={handleProceedFromEmail}
                  returnKeyType="next"
                />
                <View className="gap-2">
                <Button onPress={handleProceedFromEmail}>
                  <ArrowIcon size={16} />
                </Button>
              <Button  variant="outline" onPress={() => setAuthMode("welcome")} >
                <Text>
                  {t("header_back")}
                </Text>
              </Button>
              </View>
            </View>
          )}

          {authMode === "password" && (
            <View className="gap-4">
                <Input
                  placeholder="••••••••"
                  placeholderTextColor={
                    isDark ? "rgba(255, 255, 255, 0.4)" : "#94A3B8"
                  }
                  value={password}
                  onChangeText={setPassword}
                  secureTextEntry={!showPassword}
                  autoFocus
                  onSubmitEditing={handleSignIn}
                  returnKeyType="go"
                />
              <View className="gap-2">
                <Button
                  onPress={handleSignIn}
                  disabled={isSubmitting}
                >
                  <ArrowIcon size={16} />
                </Button>
                <Button variant="outline" onPress={() => setAuthMode("email")}>
                  <Text>
                    {t("header_back")}
                  </Text>
                </Button>

                <Button variant="outline" onPress={() => router.push("/(auth)/forgot-password")}>
                  <Text>
                    {t("auth_forgot_password")}
                  </Text>
                </Button>
              </View>
            </View>
          )}

          {/* Bottom Link: Create Account */}
          <View className="mt-5 flex-row items-center justify-center">
            <Text className="font-sans text-sm leading-5 text-foreground">
              {t("auth_no_account")}
            </Text>
            <Button
              onPress={() => router.push("/(auth)/register")}
              variant="link"
            >
              <Text className="font-sans text-sm leading-5 text-primary">
                {t("auth_register_title")}
              </Text>
            </Button>
          </View>
          <Button
            onPress={() => router.push("/(auth)/explore")}
            variant="link"
          >
            <Text className="font-sans text-sm leading-5 text-primary">
              {t("visitor_explore")}
            </Text>
          </Button>
        </View>
      </KeyboardAwareView>
    </View>
  );
}
