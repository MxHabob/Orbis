import React, { useState } from "react";
import { ActivityIndicator, Text, View } from "react-native";
import { useLocalSearchParams, useRouter } from "expo-router";
import { WebView } from "react-native-webview";

import { Header } from "@/components/header";
import { Screen } from "@/components/layout/screen";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";
import { normalizeWebUrl } from "@/lib/web-urls";

const ORBIS_WEB_URL = process.env.EXPO_PUBLIC_ORBIS_WEB_URL;
const ORBIS_LAB_URL = "https://lab.mulverse.com";

function getParam(value: string | string[] | undefined): string | undefined {
  return Array.isArray(value) ? value[0] : value;
}

export default function WebViewerScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { t } = useI18n();
  const params = useLocalSearchParams<{
    page?: string | string[];
    url?: string | string[];
    title?: string | string[];
  }>();
  const [retryKey, setRetryKey] = useState(0);
  const [isLoading, setIsLoading] = useState(true);
  const [loadFailed, setLoadFailed] = useState(false);

  const page = getParam(params.page);
  const requestedUrl = getParam(params.url);
  const requestedTitle = getParam(params.title);
  const url = requestedUrl
    ? normalizeWebUrl(requestedUrl)
    : page === "about"
      ? normalizeWebUrl(ORBIS_LAB_URL)
      : page === "mulverse"
        ? normalizeWebUrl(ORBIS_WEB_URL)
        : null;
  const isInvalidUrl =
    (Boolean(requestedUrl) || page === "about" || page === "mulverse") && !url;
  const pageTitle =
    requestedTitle ||
    (page === "about"
      ? t("profile_about_title")
      : page === "mulverse"
        ? t("profile_about_mulverse")
        : t("profile_organization_website"));

  return (
    <Screen
      edges={["bottom", "left", "right"]}
      header={
        <Header
          title={pageTitle}
          showBack
          showCrown={false}
          onBack={() => router.back()}
        />
      }
    >
      {url && !isInvalidUrl ? (
        <View className="flex-1">
          <WebView
            key={`${url}:${retryKey}`}
            source={{ uri: url }}
            onLoadStart={() => {
              setIsLoading(true);
              setLoadFailed(false);
            }}
            onLoadEnd={() => setIsLoading(false)}
            onError={() => {
              setIsLoading(false);
              setLoadFailed(true);
            }}
            onHttpError={(event) => {
              if (event.nativeEvent.statusCode >= 400) setLoadFailed(true);
            }}
            onShouldStartLoadWithRequest={(request) =>
              normalizeWebUrl(request.url) !== null
            }
            className="flex-1 bg-background"
            style={{ backgroundColor: theme.background }}
          />
          {isLoading && !loadFailed ? (
            <View className="absolute inset-0 items-center justify-center bg-background">
              <ActivityIndicator size="large" color={theme.primary} />
            </View>
          ) : null}
          {loadFailed ? (
            <View className="absolute inset-0 items-center justify-center gap-three bg-background px-six">
              <Text className="text-center text-sm text-muted-foreground">
                {t("webviewer_load_error")}
              </Text>
              <Text
                accessibilityRole="button"
                onPress={() => {
                  setLoadFailed(false);
                  setRetryKey((key) => key + 1);
                }}
                className="rounded-md border border-border px-four py-three text-sm font-medium text-foreground"
              >
                {t("webviewer_retry")}
              </Text>
            </View>
          ) : null}
        </View>
      ) : (
        <View className="flex-1 items-center justify-center px-six">
          <Text className="text-center text-sm text-muted-foreground">
            {t("webviewer_invalid_url")}
          </Text>
        </View>
      )}
    </Screen>
  );
}
