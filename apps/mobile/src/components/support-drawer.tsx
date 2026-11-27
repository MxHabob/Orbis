import Constants from "expo-constants";
import { useEffect, useState } from "react";
import {
  Linking,
  Platform,
  View,
  useWindowDimensions,
} from "react-native";

import { Button } from "@/components/ui/button";
import {
  Drawer,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerScrollView,
  DrawerTitle,
} from "@/components/ui/drawer";
import { DrawerFormField } from "@/components/ui/drawer-form-primitives";
import { Select } from "@/components/ui/select";
import { Text } from "@/components/ui/text";
import { useFormValidation } from "@/hooks/use-form-validation";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";

type SupportCategory =
  | "technical_support"
  | "onboarding_setup"
  | "finance_accounting"
  | "academics_grading"
  | "feature_request"
  | "other";

const SUPPORT_EMAIL = "support@mulverse.com";
const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

const SUPPORT_CATEGORIES: {
  value: SupportCategory;
  labelKey:
    | "support_category_technical"
    | "support_category_onboarding"
    | "support_category_finance"
    | "support_category_academics"
    | "support_category_feature_request"
    | "support_category_other";
}[] = [
  { value: "technical_support", labelKey: "support_category_technical" },
  { value: "onboarding_setup", labelKey: "support_category_onboarding" },
  { value: "finance_accounting", labelKey: "support_category_finance" },
  { value: "academics_grading", labelKey: "support_category_academics" },
  { value: "feature_request", labelKey: "support_category_feature_request" },
  { value: "other", labelKey: "support_category_other" },
];

export interface SupportDrawerProps {
  visible: boolean;
  onClose: () => void;
}

export function SupportDrawer({ visible, onClose }: SupportDrawerProps) {
  const { t, locale } = useI18n();
  const { user } = useAuth();
  const { height } = useWindowDimensions();
  const [category, setCategory] =
    useState<SupportCategory>("technical_support");

  const {
    values,
    errors,
    formError,
    setFormError,
    setValue,
    markTouched,
    validateAll,
    reset,
  } = useFormValidation({
    initialValues: {
      subject: "",
      contactEmail: user?.email ?? "",
      message: "",
    },
    validators: {
      subject: (value) =>
        !value.trim() ? t("support_required_fields") : null,
      contactEmail: (value) => {
        const trimmed = value.trim();
        if (trimmed && !EMAIL_REGEX.test(trimmed)) {
          return t("support_invalid_email");
        }
        return null;
      },
      message: (value) =>
        !value.trim() ? t("support_required_fields") : null,
    },
  });

  useEffect(() => {
    if (visible && user?.email && !values.contactEmail) {
      setValue("contactEmail", user.email);
    }
  }, [setValue, user?.email, values.contactEmail, visible]);

  const submitSupportRequest = () => {
    if (!validateAll()) {
      return;
    }

    const trimmedSubject = values.subject.trim();
    const trimmedEmail = values.contactEmail.trim();
    const trimmedMessage = values.message.trim();

    setFormError(null);
    const selectedCategory = SUPPORT_CATEGORIES.find(
      (option) => option.value === category,
    );
    const diagnostics = [
      `${t("profile_about_version")}: ${Constants.nativeAppVersion ?? Constants.expoConfig?.version ?? "—"}`,
      `Platform: ${Platform.OS}`,
      `Locale: ${locale}`,
      `Timezone: ${Intl.DateTimeFormat().resolvedOptions().timeZone}`,
    ].join("\n");
    const body = [
      `${t("support_category_label")}: ${selectedCategory ? t(selectedCategory.labelKey) : t("support_category_other")}`,
      `${t("support_contact_email")}: ${trimmedEmail || "—"}`,
      "",
      `${t("support_details_label")}:`,
      trimmedMessage,
      "",
      `${t("support_diagnostics_label")}:`,
      diagnostics,
    ].join("\n");
    const mailto = `mailto:${SUPPORT_EMAIL}?subject=${encodeURIComponent(`[Orbis] ${trimmedSubject}`)}&body=${encodeURIComponent(body)}`;
    void Linking.openURL(mailto)
      .then(() => {
        onClose();
        reset({
          subject: "",
          contactEmail: user?.email ?? "",
          message: "",
        });
        setCategory("technical_support");
      })
      .catch((error: unknown) => {
        console.error(
          "[Orbis Mobile Support] Failed to open support email.",
          error,
        );
        setFormError(t("profile_support_email_error"));
      });
  };

  return (
    <Drawer visible={visible} onClose={onClose}>
      <DrawerContent>
        <View style={{ height: height * 0.82 }}>
          <DrawerHeader>
            <DrawerTitle>{t("profile_help_title")}</DrawerTitle>
          </DrawerHeader>

          <DrawerScrollView
            contentContainerStyle={{ gap: 16, paddingTop: 16, paddingBottom: 16 }}
            style={{ flex: 1, minHeight: 0 }}
          >
            <View className="px-five">
              <Select
                label={t("support_category_label")}
                value={category}
                options={SUPPORT_CATEGORIES.map((option) => ({
                  value: option.value,
                  label: t(option.labelKey),
                }))}
                onValueChange={setCategory}
              />
            </View>

            <DrawerFormField
              label={t("support_subject_label")}
              value={values.subject}
              onChangeText={(value) => setValue("subject", value)}
              onBlur={() => markTouched("subject")}
              error={errors.subject ?? undefined}
              placeholder={t("support_subject_placeholder")}
              returnKeyType="next"
              autoCapitalize="sentences"
            />

            <DrawerFormField
              label={t("support_contact_email")}
              value={values.contactEmail}
              onChangeText={(value) => setValue("contactEmail", value)}
              onBlur={() => markTouched("contactEmail")}
              error={errors.contactEmail ?? undefined}
              placeholder={t("profile_email")}
              keyboardType="email-address"
              autoCapitalize="none"
              autoComplete="email"
            />

            <DrawerFormField
              label={t("support_details_label")}
              value={values.message}
              onChangeText={(value) => setValue("message", value)}
              onBlur={() => markTouched("message")}
              error={errors.message ?? undefined}
              placeholder={t("support_details_placeholder")}
              multiline
              textAlignVertical="top"
              style={{ minHeight: 108 }}
              autoCapitalize="sentences"
            />

            {formError ? (
              <Text
                accessibilityRole="alert"
                className="px-five text-start text-xs font-medium text-destructive"
              >
                {formError}
              </Text>
            ) : null}

            <Text className="px-five text-start text-xs leading-4 text-muted-foreground">
              {t("support_mail_review_note")}
            </Text>
          </DrawerScrollView>

          <DrawerFooter>
            <View className="gap-two">
              <Button
                title={t("support_send_report")}
                onPress={submitSupportRequest}
                size="lg"
                fullWidth
              />
              <Button
                title={t("common_cancel")}
                onPress={onClose}
                variant="outline"
                size="lg"
                fullWidth
              />
            </View>
          </DrawerFooter>
        </View>
      </DrawerContent>
    </Drawer>
  );
}
