/**
 * @module @orbis/mobile/components/ui/themed-status-bar
 *
 * Universal Status Bar component for Orbis Mobile.
 * Synchronizes status bar background color on Android and text/icon contrast on iOS/Android
 * to match the screen's background color and color scheme.
 */

import React from 'react';
import { StatusBar as RNStatusBar, Platform } from 'react-native';
import { StatusBar as ExpoStatusBar } from 'expo-status-bar';
import { useColorScheme } from '@/hooks/use-color-scheme';
import { useTheme } from '@/hooks/use-theme';

interface ThemedStatusBarProps {
  backgroundColor?: string;
  style?: 'light' | 'dark' | 'auto';
}

export const ThemedStatusBar = React.memo<ThemedStatusBarProps>(
  ({ backgroundColor, style }) => {
    const colorScheme = useColorScheme();
    const theme = useTheme();
    const isDark = colorScheme === 'dark';

    const resolvedBg = backgroundColor || theme.background;
    const resolvedStyle = style === 'auto' ? (isDark ? 'light' : 'dark') : style || (isDark ? 'light' : 'dark');

    return (
      <>
        <ExpoStatusBar style={resolvedStyle} />
        {Platform.OS === 'android' && (
          <RNStatusBar
            backgroundColor={resolvedBg}
            barStyle={resolvedStyle === 'light' ? 'light-content' : 'dark-content'}
            translucent={false}
          />
        )}
      </>
    );
  },
);

ThemedStatusBar.displayName = 'ThemedStatusBar';
