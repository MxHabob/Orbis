/**
 * @module @orbis/mobile/components/ui/floating-tab-bar
 *
 * Floating iOS-like bottom tab bar extracted as a reusable component.
 */

import React from 'react';
import { View, Pressable, StyleSheet } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { Radii, Spacing } from '@/constants/theme';
import { useTheme } from '@/hooks/use-theme';
import { useColorScheme } from '@/hooks/use-color-scheme';

type FloatingTabBarProps = {
  state: { index: number; routes: Array<{ key: string; name: string; params?: unknown }> };
  navigation: {
    emit: (event: { type: string; target: string; canPreventDefault: boolean }) => { defaultPrevented: boolean };
    navigate: (name: string, params?: unknown) => void;
  };
  descriptors: Record<string, { options: { title?: string } }>;
};

export function FloatingTabBar({ state, navigation, descriptors }: FloatingTabBarProps) {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === 'dark';
  const floatingBottom = Math.max(insets.bottom, 12) + 6;

  const getTabIcon = (routeName: string, focused: boolean) => {
    const iconMap: Record<string, keyof typeof Ionicons.glyphMap> = {
      index: focused ? 'home' : 'home-outline',
      behavior: focused ? 'sparkles' : 'sparkles-outline',
      profile: focused ? 'person' : 'person-outline',
    };

    return iconMap[routeName] ?? (focused ? 'home' : 'home-outline');
  };

  return (
    <View
      style={[
        styles.tabBar,
        {
          backgroundColor: isDark ? theme.card : '#FFFFFF',
          borderColor: theme.border,
          bottom: floatingBottom,
        },
      ]}>
      {state.routes.map((route, index) => {
        const focused = state.index === index;
        const { options } = descriptors[route.key];
        const label = options.title ?? route.name;

        return (
          <Pressable
            key={route.key}
            accessibilityRole="button"
            accessibilityState={focused ? { selected: true } : {}}
            accessibilityLabel={label}
            onPress={() => {
              const event = navigation.emit({
                type: 'tabPress',
                target: route.key,
                canPreventDefault: true,
              });

              if (!event.defaultPrevented) {
                navigation.navigate(route.name, route.params);
              }
            }}
            style={({ pressed }) => [
              styles.tabItem,
              focused && {
                backgroundColor: theme.background,
              },
              pressed && { opacity: 0.9 },
            ]}>
            <Ionicons
              name={getTabIcon(route.name, focused)}
              size={22}
              color={focused ? theme.primary : theme.textSecondary}
            />
          </Pressable>
        );
      })}
    </View>
  );
}

const styles = StyleSheet.create({
  tabBar: {
    position: 'absolute',
    left: 36,
    right: 36,
    minHeight: 48,
    borderRadius: Radii.xl,
    borderWidth: 1,
    paddingVertical: 2,
    paddingHorizontal: 8,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-around',
    elevation: 8,
    shadowColor: '#000000',
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.08,
    shadowRadius: 10,
  },
  tabItem: {
    flex: 1,
    minHeight: 36,
    alignItems: 'center',
    justifyContent: 'center',
    borderRadius: Radii.md,
    paddingVertical: 0,
  },
});
