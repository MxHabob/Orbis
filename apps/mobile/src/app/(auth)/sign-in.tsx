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
import { Fonts, Radii, Spacing, Typography } from "@/constants/theme";
import { useColorScheme } from "@/hooks/use-color-scheme";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import {
  AcademicCap,
  ArrowLeft,
  ArrowRight,
  Bolt,
  BuildingLibrary,
  ChartBar,
  Eye,
  EyeSlash
} from "@orbis/icons/native";
import { useRouter } from "expo-router";
import { useEffect, useState } from "react";
import {
  Platform,
  StyleSheet,
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
  const theme = useTheme();
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
      router.replace("/");
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
      router.replace("/");
    } catch (err: any) {
      toast.error(err?.message || t("auth_error_google"));
    } finally {
      setIsSubmitting(false);
    }
  };

  const ArrowIcon = isArabic ? ArrowRight : ArrowLeft;

  return (
    <View
      style={[
        styles.container,
        { backgroundColor: isDark ? "#09090B" : "#F8FAFC" },
      ]}
    >

      <KeyboardAwareView
        style={styles.keyboardContainer}
        contentContainerStyle={styles.scrollContent}
      >
        <View style={styles.upperSection}>
            {CAPABILITIES.map((cap, idx) => {
              return (
                    <View>
                      <Text
                        style={[
                          styles.activeCapText,
                        ]}
                      >
                        {t(cap.labelKey)}
                      </Text>
                    </View>
                  )})}
        </View>


        <View
          style={[
            styles.bottomCard,
          ]}
        >
          {authMode === "welcome" && (
            <View style={styles.actionsContainer}>
              <Button
                onPress={handleGoogleSignIn}
              >
                <Text              
                style={[
                    styles.pillButtonSecondaryText,
                    { color: isDark ? "#FFFFFF" : "#0F172A" },
                  ]}>
                  {t("auth_continue_google")}
                </Text>
              </Button>

              {/* Secondary Translucent Pill Button: Email */}
              <Button
                onPress={() => setAuthMode("email")}
                variant="secondary"
              >
                <Text
                  style={[
                    styles.pillButtonSecondaryText,
                    { color: isDark ? "#FFFFFF" : "#0F172A" },
                  ]}
                >
                  {t("auth_sign_in")}
                </Text>
              </Button>
            </View>
          )}
          {authMode === "email" && (
            <View style={[
              styles.pillInputContainer
            ]}>
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
                <View style={{ gap: Spacing.two }}>
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
            <View style={[ styles.pillInputContainer]}>
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
              <View style={{ gap: Spacing.two }}>
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
          <View style={styles.cardFooter}>
            <Text style={[Typography.bodyMedium, { color: theme.foreground }]}>
              {t("auth_no_account")}
            </Text>
            <Button
              onPress={() => router.push("/(auth)/register")}
              variant="link"
            >
              <Text style={[Typography.bodyMedium, { color: theme.primary }]}>
                {t("auth_register_title")}
              </Text>
            </Button>
          </View>
        </View>
      </KeyboardAwareView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  keyboardContainer: {
    flex: 1,
  },
  scrollContent: {
    flexGrow: 1,
    justifyContent: "space-between",
    paddingHorizontal: Spacing.five,
    paddingTop: Platform.OS === "ios" ? 80 : 50,
    paddingBottom: Spacing.eight,
  },
  upperSection: {
    flex: 1,
    alignItems: "flex-end",
    justifyContent: "center",
    minHeight: 220,
    gap: "8"
  },
  activeCapText: {
    ...Typography.h1,
    fontFamily: Fonts.sans,
    fontWeight: "800",
  },

  bottomCard: {
    paddingHorizontal: Spacing.six,
    paddingTop: Spacing.six,
    paddingBottom: Spacing.five,
  },
  actionsContainer: {
    gap: Spacing.three,
  },

  pillButtonSecondaryText: {
    ...Typography.bodyBold,
    fontFamily: Fonts.sans,
    fontWeight: "600",
  },

  pillInputContainer: {
    gap: Spacing.four,
    width: "auto"
  },
  cardFooter: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    marginTop: Spacing.five,
  },

});
