/**
 * @module @orbis/mobile/components/auth/sign-in-form
 *
 * Transparent authentication surface designed
 * to sit over the bottom gradient.
 */

import React, { useState } from "react";

import {
  View,
  Text,
  StyleSheet,
  Pressable,
  Keyboard,
  TextInput,
} from "react-native";

import { Link } from "expo-router";

import { Ionicons } from "@expo/vector-icons";

import { Checkbox } from "@/components/ui/checkbox";

import { Button } from "@/components/ui/button";

import { useTheme } from "@/hooks/use-theme";

import { useI18n } from "@/lib/i18n";

import { Radii, Spacing, Typography } from "@/constants/theme";

interface SignInFormProps {
  onSignIn: (email: string, pass: string) => Promise<void>;

  onGoogleSignIn: () => Promise<void>;

  loading?: boolean;
}

export const SignInForm = React.memo<SignInFormProps>(
  ({ onSignIn, onGoogleSignIn, loading = false }) => {
    const theme = useTheme();

    const { t } = useI18n();

    const [email, setEmail] = useState("");

    const [password, setPassword] = useState("");

    const [rememberMe, setRememberMe] = useState(false);

    const [showPassword, setShowPassword] = useState(false);

    const [error, setError] = useState<string | null>(null);

    const handleSubmit = async () => {
      Keyboard.dismiss();

      if (!email.trim() || !password) {
        setError(t("auth_error_required"));

        return;
      }

      setError(null);

      try {
        await onSignIn(email.trim(), password);
      } catch (err: any) {
        const msg = String(err?.message || "");

        const code = String(err?.code || "");

        if (
          code === "NETWORK_ERROR" ||
          msg.includes("ConnectException") ||
          msg.includes("Network request failed") ||
          msg.includes("Failed to connect") ||
          msg.includes("NetworkError") ||
          msg.includes("fetch failed") ||
          msg.includes("ECONNREFUSED") ||
          msg.includes("ENOTFOUND") ||
          msg.includes("ETIMEDOUT")
        ) {
          setError(t("auth_error_network"));
        } else if (
          err?.status === 401 ||
          code === "UNAUTHORIZED" ||
          code === "INVALID_CREDENTIALS" ||
          msg.toLowerCase().includes("invalid credentials") ||
          msg.toLowerCase().includes("unauthorized") ||
          msg.toLowerCase().includes("user not found") ||
          msg.toLowerCase().includes("password")
        ) {
          setError(t("auth_error_invalid_credentials"));
        } else if (
          msg &&
          !msg.includes("java.") &&
          !msg.includes("http") &&
          msg.length < 120
        ) {
          setError(msg);
        } else {
          setError(t("auth_error_failed"));
        }
      }
    };

    const handleGoogle = async () => {
      Keyboard.dismiss();

      setError(null);

      try {
        await onGoogleSignIn();
      } catch {
        setError(t("auth_error_google"));
      }
    };

    return (
      <View style={styles.container}>
        {/* Error */}

        {error && (
          <View
            style={[
              styles.errorBox,
              {
                backgroundColor: theme.auth.errorBackground,

                borderColor: theme.auth.errorBorder,
              },
            ]}
          >
            <Ionicons
              name="alert-circle"
              size={19}
              color={theme.auth.errorText}
            />

            <Text
              style={[
                styles.errorText,
                {
                  color: theme.auth.errorText,
                },
              ]}
            >
              {error}
            </Text>
          </View>
        )}

        {/* Email */}

        <View style={styles.field}>
          <Text
            style={[
              styles.label,
              {
                color: theme.auth.surfaceText,
              },
            ]}
          >
            {t("auth_email_label")}
          </Text>

          <View
            style={[
              styles.inputContainer,
              {
                backgroundColor: theme.auth.surfaceInput,

                borderColor: theme.auth.surfaceBorder,
              },
            ]}
          >
            <TextInput
              value={email}
              onChangeText={(text) => {
                setEmail(text);
                setError(null);
              }}
              placeholder={t("auth_email_placeholder")}
              placeholderTextColor={theme.auth.surfaceTextSecondary}
              autoCapitalize="none"
              autoCorrect={false}
              keyboardType="email-address"
              returnKeyType="next"
              style={[
                styles.input,
                {
                  color: theme.auth.surfaceText,
                },
              ]}
            />

            <Ionicons
              name="mail-outline"
              size={21}
              color={theme.auth.surfaceText}
            />
          </View>
        </View>

        {/* Password */}

        <View style={styles.field}>
          <Text
            style={[
              styles.label,
              {
                color: theme.auth.surfaceText,
              },
            ]}
          >
            {t("auth_password_label")}
          </Text>

          <View
            style={[
              styles.inputContainer,
              {
                backgroundColor: theme.auth.surfaceInput,

                borderColor: theme.auth.surfaceBorder,
              },
            ]}
          >
            <TextInput
              value={password}
              onChangeText={(text) => {
                setPassword(text);
                setError(null);
              }}
              placeholder="••••••••"
              placeholderTextColor={theme.auth.surfaceTextSecondary}
              secureTextEntry={!showPassword}
              returnKeyType="done"
              onSubmitEditing={handleSubmit}
              style={[
                styles.input,
                {
                  color: theme.auth.surfaceText,
                },
              ]}
            />

            <Pressable
              onPress={() => setShowPassword((value) => !value)}
              hitSlop={8}
            >
              <Ionicons
                name={showPassword ? "eye-outline" : "eye-off-outline"}
                size={21}
                color={theme.auth.surfaceText}
              />
            </Pressable>
          </View>
        </View>

        {/* Options */}

        <View style={styles.optionsRow}>
          <Link href="/(auth)/forgot-password" asChild>
            <Pressable hitSlop={10}>
              <Text
                style={[
                  styles.forgotText,
                  {
                    color: theme.auth.surfaceText,
                  },
                ]}
              >
                {t("auth_forgot_password")}
              </Text>
            </Pressable>
          </Link>

          <Pressable
            onPress={() => setRememberMe((value) => !value)}
            style={styles.rememberOption}
            hitSlop={8}
          >
            <Text
              style={[
                styles.optionText,
                {
                  color: theme.auth.surfaceText,
                },
              ]}
            >
              تذكرني
            </Text>

            <Checkbox
              value={rememberMe}
              onValueChange={setRememberMe}
              accessibilityLabel="تذكرني"
            />
          </Pressable>
        </View>

        {/* Login */}

        <Button
          title={t("auth_submit")}
          onPress={handleSubmit}
          loading={loading}
          fullWidth
          size="lg"
          variant="secondary"
        />

        {/* Google */}

        <Button
          title={t("auth_continue_google")}
          onPress={handleGoogle}
          loading={loading}
          disabled={loading}
          fullWidth
          size="lg"
          variant="secondary"
          style={{
            backgroundColor: theme.auth.secondaryButton,
            borderColor: theme.auth.surfaceBorder,
          }}
          textStyle={{
            color: theme.auth.secondaryButtonText,
          }}
          icon={
            <Text
              style={[
                styles.googleIcon,
                {
                  color: theme.auth.secondaryButtonText,
                },
              ]}
            >
              G
            </Text>
          }
          iconPosition="right"
        />

        {/* Register */}

        <View style={styles.registerRow}>
          <Text
            style={[
              styles.registerPrompt,
              {
                color: theme.auth.surfaceTextSecondary,
              },
            ]}
          >
            {t("auth_no_account")}
          </Text>

          <Link href="/(auth)/register" asChild>
            <Pressable hitSlop={8}>
              <Text
                style={[
                  styles.registerLink,
                  {
                    color: theme.auth.surfaceText,
                  },
                ]}
              >
                {t("auth_register_link")}
              </Text>
            </Pressable>
          </Link>
        </View>
      </View>
    );
  },
);

SignInForm.displayName = "SignInForm";

const styles = StyleSheet.create({
  container: {
    width: "100%",

    paddingHorizontal: Spacing.one,

    gap: Spacing.three,

    zIndex: 2,
  },

  field: {
    width: "100%",
    gap: Spacing.one,
  },

  label: {
    ...Typography.captionBold,

    fontSize: 13,

    textAlign: "right",

    paddingHorizontal: Spacing.two,
  },

  inputContainer: {
    minHeight: 58,

    width: "100%",

    flexDirection: "row",

    alignItems: "center",

    borderWidth: 1,

    borderRadius: Radii.xl,

    paddingHorizontal: Spacing.three,
  },

  input: {
    flex: 1,

    minHeight: 56,

    ...Typography.body,

    fontSize: 16,

    paddingVertical: 0,
  },

  errorBox: {
    minHeight: 56,

    width: "100%",

    flexDirection: "row",

    alignItems: "center",

    gap: Spacing.two,

    paddingHorizontal: Spacing.three,

    borderRadius: Radii.lg,

    borderWidth: 1,
  },

  errorText: {
    ...Typography.captionBold,

    fontSize: 12.5,

    flex: 1,

    lineHeight: 18,
  },

  optionsRow: {
    width: "100%",

    flexDirection: "row",

    alignItems: "center",

    justifyContent: "space-between",

    marginTop: -Spacing.one,
  },

  rememberOption: {
    flexDirection: "row",

    alignItems: "center",

    gap: Spacing.two,
  },

  optionText: {
    ...Typography.body,

    fontSize: 13,
  },

  forgotText: {
    ...Typography.captionBold,

    fontSize: 13,
  },

  googleButton: {
    minHeight: 54,

    width: "100%",

    flexDirection: "row",

    alignItems: "center",

    justifyContent: "center",

    gap: Spacing.two,

    borderRadius: Radii.full,

    borderWidth: 1,
  },

  googleText: {
    ...Typography.bodyBold,

    fontSize: 15,
  },

  googleIcon: {
    fontSize: 21,

    fontWeight: "800",
  },

  registerRow: {
    flexDirection: "row",

    alignItems: "center",

    justifyContent: "center",

    flexWrap: "wrap",

    gap: 5,

    paddingTop: Spacing.two,
  },

  registerPrompt: {
    ...Typography.body,

    fontSize: 13,
  },

  registerLink: {
    ...Typography.bodyBold,

    fontSize: 13,
  },

  walletButton: {
    alignSelf: "flex-end",

    flexDirection: "row",

    alignItems: "center",

    gap: Spacing.two,

    paddingHorizontal: Spacing.three,

    paddingVertical: Spacing.two,

    borderRadius: Radii.full,

    borderWidth: 1,
  },

  walletText: {
    ...Typography.captionBold,

    fontSize: 12,
  },
});
