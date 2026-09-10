import { useLocalSearchParams, useRouter } from 'expo-router';
import { useMemo, useState } from 'react';
import { ActivityIndicator, StyleSheet, View } from 'react-native';
import { WebView } from 'react-native-webview';

import { Header } from '@/components/header';
import { Spacing } from '@/constants/theme';
import { useTheme } from '@/hooks/use-theme';

const DEFAULT_URL = 'https://www.mulverse.com/';

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

    return requestedUrl?.startsWith('https://')
      ? requestedUrl
      : DEFAULT_URL;
  }, [params.url]);

  const title = useMemo(() => {
    return getParam(params.title)?.trim() || 'Orbis';
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
      <Header
        title={title}
        showBack
        onBack={() => router.back()}
        transparent
      />

      <View style={styles.webviewContainer}>
        {loading && (
          <View
            pointerEvents="none"
            style={styles.loading}
          >
            <ActivityIndicator
              size="small"
              color={theme.primary}
            />
          </View>
        )}

        <WebView
          source={{ uri: url }}
          onLoadStart={() => setLoading(true)}
          onLoadEnd={() => setLoading(false)}
          onError={() => setLoading(false)}
          javaScriptEnabled
          domStorageEnabled
          originWhitelist={['https://*']}
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
    position: 'relative',
  },

  webview: {
    flex: 1,
  },

  loading: {
    position: 'absolute',
    zIndex: 10,
    top: Spacing.two,
    left: 0,
    right: 0,
    alignItems: 'center',
  },
});