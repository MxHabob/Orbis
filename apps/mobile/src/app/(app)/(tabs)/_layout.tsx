/**
 * @module @orbis/mobile/app/(app)/(tabs)/_layout
 *
 * Floating Bottom Tab Navigation Bar with the exact canonical 3 tabs:
 * 1. الرئيسية
 * 2. السلوك والذكاء
 * 3. حسابي
 *
 * Styled as an elevated floating capsule pill above the bottom screen bezel.
 */

import { Tabs } from 'expo-router';
import { StyleSheet, View } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { Typography, Radii } from '@/constants/theme';
import { useI18n } from '@/lib/i18n';

export default function TabsLayout() {
  const insets = useSafeAreaInsets();
  const { t } = useI18n();
  const floatingBottom = Math.max(insets.bottom, 12) + 8;

  return (
    <Tabs
      screenOptions={{
        headerShown: false,
        tabBarActiveTintColor: '#18181B',
        tabBarInactiveTintColor: '#71717A',
        tabBarShowLabel: true,
        tabBarStyle: {
          position: 'absolute',
          bottom: floatingBottom,
          left: 20,
          right: 20,
          height: 66,
          borderRadius: 33,
          backgroundColor: '#FFFFFF',
          borderWidth: 1,
          borderColor: '#F4F4F5',
          paddingBottom: 8,
          paddingTop: 8,
          paddingHorizontal: 12,
          elevation: 16,
          shadowColor: '#000000',
          shadowOffset: { width: 0, height: 10 },
          shadowOpacity: 0.14,
          shadowRadius: 20,
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
                focused && styles.activeIconCircle,
              ]}>
              <Ionicons
                name={focused ? 'home' : 'home-outline'}
                size={22}
                color="#18181B"
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
                focused && styles.activeIconCircle,
              ]}>
              <Ionicons
                name={focused ? 'sparkles' : 'sparkles-outline'}
                size={22}
                color={focused ? '#18181B' : '#71717A'}
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
                focused && styles.activeIconCircle,
              ]}>
              <Ionicons
                name={focused ? 'person' : 'person-outline'}
                size={22}
                color={focused ? '#18181B' : '#71717A'}
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
    height: 30,
    alignItems: 'center',
    justifyContent: 'center',
  },
  activeIconCircle: {
    backgroundColor: '#FFE380',
    borderRadius: Radii.full,
  },
});
