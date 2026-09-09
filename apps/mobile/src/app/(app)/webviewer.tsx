import { useLocalSearchParams, useRouter } from 'expo-router';
import { useMemo, useState } from 'react';
import { ActivityIndicator, StyleSheet, Text, View } from 'react-native';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { WebView } from 'react-native-webview';
import { BackButton } from '@/components/back-button';
import { Spacing, Typography } from '@/constants/theme';
import { useTheme } from '@/hooks/use-theme';

const DEFAULT_URL = 'https://www.mulverse.com/';

function getParam(value?: string | string[]) {
  return Array.isArray(value) ? value[0] : value;
}

export default function AppWebViewerScreen() {
  const router = useRouter();
  const theme = useTheme();
  const insets = useSafeAreaInsets();
  const params = useLocalSearchParams<{ url?: string | string[]; title?: string | string[] }>();
  const [loading, setLoading] = useState(true);

  const url = useMemo(() => {
    const requestedUrl = getParam(params.url);
    return requestedUrl?.startsWith('https://') ? requestedUrl : DEFAULT_URL;
  }, [params.url]);

  const title = useMemo(() => getParam(params.title)?.trim() || 'Orbis', [params.title]);

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}> 
      <View style={[styles.header, { paddingTop: insets.top + Spacing.two }]}> 
          <BackButton onPress={() => router.back()} />
        <Text numberOfLines={1} style={[styles.title, { color: theme.text }]}>{title}</Text>
        <View style={styles.iconButton} />
      </View>

      <View style={styles.webviewContainer}>
        {loading && <ActivityIndicator color={theme.primary} style={styles.loader} />}
        <WebView
          source={{ uri: url }}
          onLoadStart={() => setLoading(true)}
          onLoadEnd={() => setLoading(false)}
          onError={() => setLoading(false)}
          javaScriptEnabled
          domStorageEnabled
          originWhitelist={['https://*']}
          style={{ backgroundColor: theme.background }}
        />
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1 },
  header: {
    minHeight: 76,
    paddingHorizontal: Spacing.three,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  iconButton: {
    width: 40,
    height: 40,
    minHeight: 0,
    paddingHorizontal: 0,
    justifyContent: 'center',
    alignItems: 'center',
  },
  title: { ...Typography.bodyBold, flex: 1, textAlign: 'center' },
  webviewContainer: {
    flex: 1,
  },
  loader: { marginTop: Spacing.two },
});