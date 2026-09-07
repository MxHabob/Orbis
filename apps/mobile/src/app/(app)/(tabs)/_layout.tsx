/**
 * @module @orbis/mobile/app/(app)/(tabs)/_layout
 *
 * Floating Bottom Tab Navigation Bar with the exact canonical 3 tabs:
 * 1. الرئيسية
 * 2. السلوك والذكاء
 * 3. حسابي
 *
 * Styled as an elevated floating capsule pill with side margins and theme support.
 */

import { Tabs } from 'expo-router';
import { StyleSheet, View } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { Typography, Radii } from '@/constants/theme';
import { useI18n } from '@/lib/i18n';
import { useTheme } from '@/hooks/use-theme';
import { useColorScheme } from '@/hooks/use-color-scheme';

export default function TabsLayout() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === 'dark';
  const { t } = useI18n();
  const floatingBottom = Math.max(insets.bottom, 12) + 6;

  return (
    <Tabs
      screenOptions={{
        headerShown: false,
        tabBarActiveTintColor: theme.primary,
        tabBarInactiveTintColor: theme.textSecondary,
        tabBarShowLabel: true,
        tabBarStyle: {
          position: 'absolute',
          bottom: floatingBottom,
          marginHorizontal: 32,
          left: 20,
          right: 20,
          height: 64,
          borderRadius: 32,
          backgroundColor: isDark ? theme.card : '#FFFFFF',
          borderWidth: 1,
          borderColor: theme.border,
          paddingBottom: 6,
          paddingTop: 6,
          paddingHorizontal: 12,
          elevation: 14,
          shadowColor: '#000000',
          shadowOffset: { width: 0, height: 8 },
          shadowOpacity: isDark ? 0.4 : 0.12,
          shadowRadius: 18,
        },
        tabBarLabelStyle: {
          ...Typography.small,
          fontWeight: '700',
          fontSize: 11,
          marginTop: -2,
        },
        tabBarItemStyle: {
          justifyContent: 'center',
          alignItems: 'center',
        },
      }}>
      <Tabs.Screen
        name="index"
        options={{
          title: t('tab_home'),
          tabBarIcon: ({ focused }) => (
            <View
              style={[
                styles.iconWrapper,
                focused && [
                  styles.activeIconCircle,
                  { backgroundColor: isDark ? theme.primaryLight : '#F5F3FF' },
                ],
              ]}>
              <Ionicons
                name={focused ? 'home' : 'home-outline'}
                size={22}
                color={focused ? theme.primary : theme.textSecondary}
              />
            </View>
          ),
        }}
      />
      <Tabs.Screen
        name="behavior"
        options={{
          title: t('tab_behavior_ai'),
          tabBarIcon: ({ focused }) => (
            <View
              style={[
                styles.iconWrapper,
                focused && [
                  styles.activeIconCircle,
                  { backgroundColor: isDark ? theme.primaryLight : '#F5F3FF' },
                ],
              ]}>
              <Ionicons
                name={focused ? 'sparkles' : 'sparkles-outline'}
                size={22}
                color={focused ? theme.primary : theme.textSecondary}
              />
            </View>
          ),
        }}
      />
      <Tabs.Screen
        name="profile"
        options={{
          title: t('tab_profile'),
          tabBarIcon: ({ focused }) => (
            <View
              style={[
                styles.iconWrapper,
                focused && [
                  styles.activeIconCircle,
                  { backgroundColor: isDark ? theme.primaryLight : '#F5F3FF' },
                ],
              ]}>
              <Ionicons
                name={focused ? 'person' : 'person-outline'}
                size={22}
                color={focused ? theme.primary : theme.textSecondary}
              />
            </View>
          ),
        }}
      />
    </Tabs>
  );
}

const styles = StyleSheet.create({
  iconWrapper: {
    width: 46,
    height: 28,
    alignItems: 'center',
    justifyContent: 'center',
  },
  activeIconCircle: {
    borderRadius: Radii.full,
  },
});
