import { useState } from "react";
import { Pressable, Text, View } from "react-native";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { useToast } from "@/components/ui/toast";
import { useTheme } from "@/hooks/use-theme";
import { api, configureApiBaseUrl, useAuth } from "@/lib/api";
import {
  saveSelectedInstitution,
  searchInstitutionDirectory,
  type DirectoryInstitution,
} from "@/lib/api/institution-discovery";
import { useI18n } from "@/lib/i18n";
import { ArrowLeft, ArrowRight } from "@orbis/icons/native";

type RegistrationStep = 1 | 2 | 3 | 4;

export default function RegisterScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { t, isArabic } = useI18n();
  const toast = useToast();
  const { login } = useAuth();

  const [step, setStep] = useState<RegistrationStep>(1);
  const [institutionSearch, setInstitutionSearch] = useState("");
  const [institutions, setInstitutions] = useState<DirectoryInstitution[]>([]);
  const [institution, setInstitution] = useState<DirectoryInstitution | null>(null);
  const [fullName, setFullName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const handleSearchInstitutions = async () => {
    setErrorMsg(null);
    if (institutionSearch.trim().length < 2) {
      setErrorMsg(t("auth_error_required"));
      return;
    }

    setIsSubmitting(true);
    try {
      const matches = await searchInstitutionDirectory(institutionSearch);
      setInstitutions(matches);
      if (matches.length === 0) {
        setErrorMsg(t("auth_institution_no_matches"));
      }
    } catch (error: unknown) {
      console.warn("[Register] Institution directory search failed:", error);
      const message = t("auth_institution_search_error");
      setErrorMsg(message);
      toast.error(message);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleApproveInstitution = async () => {
    setErrorMsg(null);
    if (!institution) {
      setErrorMsg(t("auth_institution_not_selected"));
      return;
    }

    setIsSubmitting(true);
    try {
      await saveSelectedInstitution(institution);
      configureApiBaseUrl(institution.endpoint_url);
      setStep(3);
    } catch (error: unknown) {
      const message =
        error instanceof Error ? error.message : t("auth_error_failed");
      setErrorMsg(message);
      toast.error(message);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleNextToCredentials = () => {
    setErrorMsg(null);
    if (!fullName.trim()) {
      setErrorMsg(t("auth_error_required"));
      return;
    }
    setStep(4);
  };

  const handleCreateAccount = async () => {
    setErrorMsg(null);
    if (!institution) {
      setErrorMsg(t("auth_institution_not_selected"));
      setStep(1);
      return;
    }
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
          role: "teacher",
        },
        throwOnError: true,
      });

      toast.success(t("auth_register_success"));
      try {
        await login(trimmedEmail, password);
      } catch {
        router.replace("/(auth)/sign-in");
      }
    } catch (error: unknown) {
      const message =
        error instanceof Error ? error.message : t("auth_error_failed");
      setErrorMsg(message);
      toast.error(message);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleBack = () => {
    setErrorMsg(null);
    if (step === 4) {
      setStep(3);
    } else if (step === 3) {
      setStep(2);
    } else if (step === 2) {
      setStep(1);
      setInstitution(null);
    } else {
      router.back();
    }
  };

  const handleContinue = () => {
    if (step === 1) {
      void handleSearchInstitutions();
    } else if (step === 2) {
      void handleApproveInstitution();
    } else if (step === 3) {
      handleNextToCredentials();
    } else {
      void handleCreateAccount();
    }
  };

  const BackIcon = isArabic ? ArrowRight : ArrowLeft;

  return (
    <View className="flex-1 bg-background">
      <KeyboardAwareView
        style={{ flex: 1 }}
        contentContainerStyle={{
          flexGrow: 1,
          justifyContent: "space-between",
          paddingHorizontal: 24,
          paddingTop: insets.top + 12,
          paddingBottom: Math.max(insets.bottom, 20) + 12,
        }}
      >
        <View className="mb-6 min-h-11 flex-row items-center justify-between">
          <Pressable
            accessibilityRole="button"
            accessibilityLabel={isArabic ? "رجوع" : "Back"}
            onPress={handleBack}
            hitSlop={{ top: 12, bottom: 12, left: 12, right: 12 }}
          >
            <BackIcon size={18} color={theme.foreground} />
          </Pressable>

          <View className="flex-row items-center gap-[6px]">
            {[1, 2, 3, 4].map((currentStep) => (
              <View
                key={currentStep}
                className={`h-[6px] rounded-[3px] ${currentStep <= step ? "bg-primary" : "bg-border"}`}
                style={{ width: currentStep === step ? 20 : 6 }}
              />
            ))}
          </View>

          <Pressable
            onPress={() => router.push("/(auth)/sign-in")}
            hitSlop={{ top: 10, bottom: 10, left: 10, right: 10 }}
          >
            <Text className="font-sans text-xs font-semibold text-muted-foreground">
              {t("auth_sign_in")}
            </Text>
          </Pressable>
        </View>

        <View className="flex-1 justify-center">
          {step === 1 && (
            <View className="w-full gap-4">
              <View className="gap-2">
                <Text className="font-sans text-[28px] font-extrabold tracking-[-0.5px] text-foreground">
                  {t("auth_institution_search_title")}
                </Text>
                <Text className="font-sans text-sm leading-5 text-muted-foreground">
                  {t("auth_institution_search_hint")}
                </Text>
              </View>
              <Input
                label={t("auth_institution_code")}
                placeholder={t("auth_institution_search_placeholder")}
                value={institutionSearch}
                onChangeText={(value) => {
                  setInstitutionSearch(value);
                  setInstitutions([]);
                  setErrorMsg(null);
                }}
                autoCapitalize="words"
                autoCorrect={false}
                autoFocus
                onSubmitEditing={handleContinue}
                returnKeyType="go"
              />
              <View className="gap-2">
                {institutions.map((match) => (
                  <Pressable
                    key={`${match.deployment_id}:${match.endpoint_url}`}
                    accessibilityRole="button"
                    onPress={() => {
                      setInstitution(match);
                      setStep(2);
                      setErrorMsg(null);
                    }}
                    className="min-h-14 justify-center gap-1 rounded-xl border border-border bg-card px-4 py-3"
                  >
                    <Text className="font-sans text-sm font-semibold text-foreground">
                      {isArabic && match.name_ar ? match.name_ar : match.name}
                    </Text>
                    <Text className="font-sans text-xs text-muted-foreground">
                      {match.deployment_name}
                    </Text>
                  </Pressable>
                ))}
              </View>
            </View>
          )}

          {step === 2 && institution && (
            <View className="w-full gap-5">
              <View className="gap-2">
                <Text className="font-sans text-[28px] font-extrabold tracking-[-0.5px] text-foreground">
                  {t("auth_institution_connection_title")}
                </Text>
                <Text className="font-sans text-sm leading-5 text-muted-foreground">
                  {t("auth_institution_connection_hint")}
                </Text>
              </View>
              <View className="gap-3 rounded-2xl border border-border bg-card p-5">
                <Text className="font-sans text-xl font-bold text-foreground">
                  {isArabic && institution.name_ar
                    ? institution.name_ar
                    : institution.name}
                </Text>
                <Text className="font-sans text-sm text-muted-foreground">
                  {institution.deployment_name}
                </Text>
                <Text className="font-sans text-xs text-muted-foreground">
                  {institution.endpoint_url}
                </Text>
              </View>
              <Pressable
                onPress={() => {
                  setInstitution(null);
                  setErrorMsg(null);
                  setStep(1);
                }}
                hitSlop={8}
                className="min-h-11 justify-center"
              >
                <Text className="font-sans text-sm font-semibold text-primary">
                  {t("auth_institution_change")}
                </Text>
              </Pressable>
            </View>
          )}

          {step === 3 && (
            <View className="w-full gap-6">
              <Text className="font-sans text-[28px] font-extrabold tracking-[-0.5px] text-foreground">
                {t("auth_full_name")}
              </Text>
              <Input
                label={t("auth_full_name")}
                placeholder={t("auth_full_name_ph")}
                value={fullName}
                onChangeText={setFullName}
                autoFocus
                onSubmitEditing={handleNextToCredentials}
                returnKeyType="next"
                autoComplete="name"
                textContentType="name"
              />
            </View>
          )}

          {step === 4 && (
            <View className="w-full gap-6">
              <Text className="font-sans text-[28px] font-extrabold tracking-[-0.5px] text-foreground">
                {t("auth_email_label")}
              </Text>
              <View className="gap-three">
                <Input
                  placeholder={t("auth_email_placeholder")}
                  label={t("auth_email_label")}
                  value={email}
                  onChangeText={setEmail}
                  keyboardType="email-address"
                  autoCapitalize="none"
                  autoCorrect={false}
                  autoFocus
                  autoComplete="email"
                  textContentType="emailAddress"
                />
                <Input
                  placeholder="••••••••"
                  label={t("auth_password")}
                  value={password}
                  onChangeText={setPassword}
                  secureTextEntry
                  onSubmitEditing={handleContinue}
                  returnKeyType="go"
                  autoComplete="new-password"
                  textContentType="newPassword"
                />
              </View>
            </View>
          )}

          {errorMsg ? (
            <Text
              accessibilityRole="alert"
              className="mt-three text-sm font-medium text-destructive"
            >
              {errorMsg}
            </Text>
          ) : null}
        </View>

        <View className="mt-6">
          <Button onPress={handleContinue} disabled={isSubmitting}>
            <Text className="font-sans text-base font-bold text-primary-foreground">
              {isSubmitting
                ? step === 1
                  ? t("auth_institution_searching")
                  : t("common_loading")
                : step === 1
                  ? t("auth_institution_search")
                  : step === 2
                    ? t("auth_institution_approve")
                    : step === 4
                      ? t("auth_create_account_btn")
                      : t("auth_next_step")}
            </Text>
          </Button>
        </View>
      </KeyboardAwareView>
    </View>
  );
}
