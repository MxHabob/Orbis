import { useLocalSearchParams, useRouter } from "expo-router";
import { useMemo, useState } from "react";
import { StyleSheet, View } from "react-native";
import { WebView } from "react-native-webview";

import { Header } from "@/components/header";
import { Skeleton } from "@/components/ui/skeleton";
import { Spacing } from "@/constants/theme";
import { useTheme } from "@/hooks/use-theme";

const DEFAULT_URL = "https://www.mulverse.com/";

function getParam(value?: string | string[]) {
  return Array.isArray(value) ? value[0] : value;
}

export default function AppWebViewerScreen() {
  const router = useRouter();
  const theme = useTheme();

  const params = useLocalSearchParams<{
    url?: string | string[];
    title?: string | string[];
  }>();

  const [loading, setLoading] = useState(true);

  const url = useMemo(() => {
    const requestedUrl = getParam(params.url)?.trim();

    return requestedUrl?.startsWith("https://") ? requestedUrl : DEFAULT_URL;
  }, [params.url]);

  const title = useMemo(() => {
    return getParam(params.title)?.trim() || "Orbis";
  }, [params.title]);

  return (
    <View
      style={[
        styles.container,
        {
          backgroundColor: theme.background,
        },
      ]}
    >
      <Header title={title} showBack onBack={() => router.back()}  />

      <View style={styles.webviewContainer}>
        {loading && (
          <View pointerEvents="none" style={styles.loading}>
            <View style={styles.skeletonBlock}>
              <Skeleton width="80%" height={16} borderRadius={8} />
              <Skeleton width="55%" height={12} style={{ marginTop: 10 }} />
              <Skeleton width="100%" height={220} style={{ marginTop: 18 }} />
              <Skeleton width="100%" height={12} style={{ marginTop: 12 }} />
              <Skeleton width="88%" height={12} style={{ marginTop: 10 }} />
            </View>
          </View>
        )}

        <WebView
          source={{ uri: url }}
          onLoadStart={() => setLoading(true)}
          onLoadEnd={() => setLoading(false)}
          onError={() => setLoading(false)}
          javaScriptEnabled
          domStorageEnabled
          originWhitelist={["https://*"]}
          startInLoadingState={false}
          style={[
            styles.webview,
            {
              backgroundColor: theme.background,
            },
          ]}
        />
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },

  webviewContainer: {
    flex: 1,
    position: "relative",
  },

  webview: {
    flex: 1,
  },

  loading: {
    position: "absolute",
    zIndex: 10,
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    justifyContent: "center",
    backgroundColor: "transparent",
  },

  skeletonBlock: {
    width: "100%",
    paddingHorizontal: Spacing.three,
    paddingVertical: Spacing.two,
  },
});
