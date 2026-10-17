import { CheckCircle } from "@orbis/icons/native";
import Constants from "expo-constants";
import { useState } from "react";
import {
  Linking,
  Platform,
  Pressable,
  Text,
  View,
  useWindowDimensions,
} from "react-native";

import { Button } from "@/components/ui/button";
import {
  Drawer,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerScrollView,
  DrawerTitle,
} from "@/components/ui/drawer";
import {
  DrawerFormField,
  DrawerSubmitButton,
} from "@/components/ui/drawer-form-primitives";
import { useTheme } from "@/hooks/use-theme";
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
const SUPPORT_CATEGORIES: Array<{
  value: SupportCategory;
  labelKey:
    | "support_category_technical"
    | "support_category_onboarding"
    | "support_category_finance"
    | "support_category_academics"
    | "support_category_feature_request"
    | "support_category_other";
}> = [
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
  const theme = useTheme();
  const { t, locale } = useI18n();
  const { user } = useAuth();
  const { height } = useWindowDimensions();
  const [category, setCategory] =
    useState<SupportCategory>("technical_support");
  const [subject, setSubject] = useState("");
  const [contactEmail, setContactEmail] = useState(user?.email ?? "");
  const [message, setMessage] = useState("");
  const [formError, setFormError] = useState<string | null>(null);

  const submitSupportRequest = () => {
    const trimmedSubject = subject.trim();
    const trimmedMessage = message.trim();
    if (!trimmedSubject || !trimmedMessage) {
      setFormError(t("support_required_fields"));
      return;
    }
    if (
      contactEmail.trim() &&
      !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(contactEmail.trim())
    ) {
      setFormError(t("support_invalid_email"));
      return;
    }

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
      `${t("support_contact_email")}: ${contactEmail.trim() || "—"}`,
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
        setSubject("");
        setMessage("");
        setCategory("technical_support");
      })
      .catch((error: unknown) => {
        console.error("[Orbis Mobile Support] Failed to open support email.", error);
        setFormError(t("profile_support_email_error"));
      });
  };

  return (
    <Drawer visible={visible} onClose={onClose}>
      <DrawerContent>
        <View style={{ maxHeight: height * 0.9 }}>
          <DrawerHeader>
            <DrawerTitle>{t("profile_help_title")}</DrawerTitle>
            <DrawerDescription>
              {t("profile_support_page_description")}
            </DrawerDescription>
          </DrawerHeader>
          <DrawerScrollView
            contentContainerStyle={{ gap: 16, paddingBottom: 16 }}
            style={{ flex: 1, maxHeight: height * 0.58, flexShrink: 1 }}
          >
            <View className="gap-two px-five">
              <Text className="text-start text-xs font-semibold text-foreground">
                {t("support_category_label")}
              </Text>
              <View className="gap-one">
                {SUPPORT_CATEGORIES.map((option) => {
                  const selected = category === option.value;
                  return (
                    <Pressable
                      key={option.value}
                      accessibilityRole="radio"
                      accessibilityState={{ selected }}
                      onPress={() => setCategory(option.value)}
                      className={`min-h-11 flex-row items-center justify-between rounded-md border px-three ${
                        selected
                          ? "border-primary bg-primary/5"
                          : "border-border bg-background"
                      }`}
                    >
                      <Text className="text-start text-sm text-foreground">
                        {t(option.labelKey)}
                      </Text>
                      {selected ? (
                        <CheckCircle size={18} color={theme.primary} />
                      ) : null}
                    </Pressable>
                  );
                })}
              </View>
            </View>
            <DrawerFormField
              label={t("support_subject_label")}
              value={subject}
              onChangeText={setSubject}
              placeholder={t("support_subject_placeholder")}
              returnKeyType="next"
              autoCapitalize="sentences"
            />
            <DrawerFormField
              label={t("support_contact_email")}
              value={contactEmail}
              onChangeText={setContactEmail}
              placeholder={t("profile_email")}
              keyboardType="email-address"
              autoCapitalize="none"
              autoComplete="email"
            />
            <DrawerFormField
              label={t("support_details_label")}
              value={message}
              onChangeText={setMessage}
              placeholder={t("support_details_placeholder")}
              multiline
              textAlignVertical="top"
              style={{ minHeight: 108 }}
              autoCapitalize="sentences"
            />
            {formError ? (
              <Text className="px-five text-start text-xs font-medium text-error">
                {formError}
              </Text>
            ) : null}
            <Text className="px-five text-start text-xs leading-4 text-muted-foreground">
              {t("support_mail_review_note")}
            </Text>
          </DrawerScrollView>
          <DrawerFooter>
            <View className="gap-two">
              <DrawerSubmitButton
                label={t("support_send_report")}
                onPress={submitSupportRequest}
                disabled={!subject.trim() || !message.trim()}
              />
              <Button
                title={t("common_cancel")}
                onPress={onClose}
                variant="outline"
                size="default"
                fullWidth
              />
            </View>
          </DrawerFooter>
        </View>
      </DrawerContent>
    </Drawer>
  );
}
