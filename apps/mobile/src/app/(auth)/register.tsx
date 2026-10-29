import { useState } from "react";
import { Redirect, useRouter } from "expo-router";
import { Text, View } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { api, useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";

export default function RegisterScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const { t } = useI18n();
  const { login, selectedInstitution } = useAuth();
  const [fullName, setFullName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  if (!selectedInstitution) {
    return <Redirect href="/institution-select" />;
  }

  const createAccount = async () => {
    const normalizedEmail = email.trim();
    if (!fullName.trim() || !normalizedEmail || !password) {
      setErrorMessage(t("auth_error_required"));
      return;
    }

    setIsSubmitting(true);
    setErrorMessage(null);
    try {
      await api.registerUser({
        body: {
          full_name: fullName.trim(),
          email: normalizedEmail,
          password,
          role: "teacher",
        },
        throwOnError: true,
      });
      try {
        await login(normalizedEmail, password);
      } catch {
        router.replace("/(auth)/sign-in");
      }
    } catch (error: unknown) {
      const message =
        error instanceof Error ? error.message : t("auth_error_failed");
      setErrorMessage(message);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <View className="flex-1 bg-background">
      <KeyboardAwareView
        style={{ flex: 1 }}
        contentContainerStyle={{
          flexGrow: 1,
          justifyContent: "space-between",
          paddingHorizontal: 24,
          paddingTop: insets.top + 24,
          paddingBottom: Math.max(insets.bottom, 20) + 16,
        }}
      >
        <View className="gap-6">
          <View className="gap-2">
            <Text className="font-sans text-[28px] font-extrabold text-foreground">
              {t("auth_create_account_btn")}
            </Text>
            <Text className="font-sans text-sm text-muted-foreground">
              {selectedInstitution.name}
            </Text>
          </View>
          <Input
            label={t("auth_full_name")}
            placeholder={t("auth_full_name_ph")}
            value={fullName}
            onChangeText={setFullName}
            autoComplete="name"
            textContentType="name"
            autoCapitalize="words"
            returnKeyType="next"
          />
          <Input
            label={t("auth_email_label")}
            placeholder={t("auth_email_placeholder")}
            value={email}
            onChangeText={setEmail}
            keyboardType="email-address"
            autoCapitalize="none"
            autoCorrect={false}
            autoComplete="email"
            textContentType="emailAddress"
            returnKeyType="next"
          />
          <Input
            label={t("auth_password")}
            placeholder="••••••••"
            value={password}
            onChangeText={setPassword}
            secureTextEntry
            autoComplete="new-password"
            textContentType="newPassword"
            onSubmitEditing={() => void createAccount()}
            returnKeyType="go"
          />
          {errorMessage ? (
            <Text
              accessibilityRole="alert"
              className="text-sm text-destructive"
            >
              {errorMessage}
            </Text>
          ) : null}
        </View>
        <View className="gap-3 pt-6">
          <Button
            onPress={() => void createAccount()}
            disabled={isSubmitting}
            loading={isSubmitting}
            fullWidth
          >
            <Text className="font-sans text-base font-bold text-primary-foreground">
              {t("auth_create_account_btn")}
            </Text>
          </Button>
          <Button
            title={t("auth_sign_in")}
            variant="outline"
            fullWidth
            onPress={() => router.replace("/(auth)/sign-in")}
          />
        </View>
      </KeyboardAwareView>
    </View>
  );
}
