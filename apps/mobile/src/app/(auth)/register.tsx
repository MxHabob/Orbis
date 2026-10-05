/**
 * @module @orbis/mobile/app/(auth)/register
 *
 * Minimalist, elegant step-by-step registration directly inspired by Reference Image 3
 * (Single focused question per step, large pill inputs, tactile selection chips,
 * and bottom pill continue buttons).
 */

import{ useState } from "react";
import {
  StyleSheet,
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
import { useColorScheme } from "@/hooks/use-color-scheme";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { useToast } from "@/components/ui/toast";
import { Radii, Spacing, Typography, Fonts } from "@/constants/theme";
import {
  ArrowLeft,
  ArrowRight,
} from "@orbis/icons/native";
import { Button } from "@/components/ui/button";

type RoleOption = "teacher" | "student" | "parent" | "admin";

export default function RegisterScreen() {
  const router = useRouter();
  const theme = useTheme();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
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
      style={[
        styles.container,
        { backgroundColor: isDark ? "#09090B" : "#F8FAFC" },
      ]}
    >

      <KeyboardAwareView
        style={styles.keyboardContainer}
        contentContainerStyle={styles.scrollContent}
      >
        {/* Minimal Nav Bar */}
        <View style={styles.navHeader}>
          <Pressable
            onPress={handleBack}
            hitSlop={{ top: 12, bottom: 12, left: 12, right: 12 }}
          >
            <BackIcon size={18} color={isDark ? "#FFFFFF" : "#0F172A"} />
          </Pressable>

          {/* Minimal Step Dots Indicator */}
          <View style={styles.stepDots}>
            {[1, 2, 3].map((s) => (
              <View
                key={s}
                style={[
                  styles.dot,
                  {
                    backgroundColor:
                      s <= step
                        ? theme.primary
                        : isDark
                          ? "rgba(255, 255, 255, 0.2)"
                          : "rgba(0, 0, 0, 0.1)",
                    width: s === step ? 20 : 6,
                  },
                ]}
              />
            ))}
          </View>

          <Pressable
            onPress={() => router.push("/(auth)/sign-in")}
            hitSlop={{ top: 10, bottom: 10, left: 10, right: 10 }}
          >
            <Text
              style={[
                styles.navLinkText,
                { color: isDark ? "rgba(255, 255, 255, 0.6)" : "#64748B" },
              ]}
            >
              {t("auth_sign_in")}
            </Text>
          </Pressable>
        </View>

        {/* Dynamic Step Content */}
        <View style={styles.stepContent}>
          {/* STEP 1: NAME */}
          {step === 1 && (
            <View style={styles.stepInner}>
              <View style={styles.titleBlock}>
                <Text
                  style={[
                    styles.stepQuestion,
                  ]}
                >
                  {t("auth_full_name")}
                </Text>
              </View>

              <View>
                <TextInput
                  style={{ borderRadius: Radii.md, borderWidth: 1, width: 370}}
                  placeholder={t("auth_full_name_ph")}
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
            <View style={styles.stepInner}>
              <View style={styles.titleBlock}>
                <Text
                  style={[
                    styles.stepQuestion,
                  ]}
                >
                  {t("auth_email_label")}
                </Text>
              </View>

              <View style={{ gap: Spacing.three,}}>
                  <TextInput
                    style={{ borderRadius: Radii.md, borderWidth: 1, width: 370}}
                    placeholder={t("auth_email_placeholder")}
                    value={email}
                    onChangeText={setEmail}
                    keyboardType="email-address"
                    autoCapitalize="none"
                    autoFocus
                    />
   
                  <TextInput
                    style={{ borderRadius: Radii.md, borderWidth: 1, width: 370}}
                    placeholder="••••••••"
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
        <View style={styles.footer}>
          <Button
            onPress={
              step === 1
                ? handleNextStep1
                  : handleFinalSubmit
            }
            disabled={isSubmitting}
          >
            <Text
              style={[
                styles.continuePillBtnText,
              ]}
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
    paddingTop: Platform.OS === "ios" ? 54 : 36,
    paddingBottom: Spacing.eight,
  },
  navHeader: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    marginBottom: Spacing.six,
  },
  navIconBtn: {
    width: 40,
    height: 40,

    alignItems: "center",
    justifyContent: "center",
  },
  stepDots: {
    flexDirection: "row",
    alignItems: "center",
    gap: 6,
  },
  dot: {
    height: 6,
    borderRadius: 3,
  },
  navLinkText: {
    ...Typography.captionBold,
    fontFamily: Fonts.sans,
  },
  stepContent: {
    flex: 1,
    justifyContent: "center",
  },
  stepInner: {
    width: "100%",
    alignItems: "flex-end",
  },
  titleBlock: {
    marginBottom: Spacing.six,
  },
  stepQuestion: {
    ...Typography.display,
    fontSize: 28,
    lineHeight: 34,
    fontFamily: Fonts.sans,
    fontWeight: "800",
    letterSpacing: -0.5,
  },
  largePillInputContainer: {
    // flexDirection: "row",
    width:"auto",
    height:46,
    alignItems: "center",
    borderRadius: Radii.md,
    borderWidth: 1,
    paddingHorizontal: Spacing.five,
  },
  credentialsInputs: {
    flex: 1,
    gap: Spacing.two,
    alignItems: "center",
    justifyContent: "center",
  },
  footer: {
    marginTop: Spacing.six,
  },
  continuePillBtn: {
    height: 54,
    borderRadius: Radii.md,
    alignItems: "center",
    justifyContent: "center",
  },
  continuePillBtnText: {
    ...Typography.bodyBold,
    fontFamily: Fonts.sans,
    fontSize: 16,
    fontWeight: "700",
  },
});
