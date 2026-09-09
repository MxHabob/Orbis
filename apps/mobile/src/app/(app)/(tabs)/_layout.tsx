/**
 * @module @orbis/mobile/app/(app)/(tabs)/_layout
 *
 * Shared tabs layout.
 *
 * - iOS uses NativeTabs.
 * - Android uses the custom FloatingTabBar.
 * - System StatusBar follows the current color scheme.
 */

import {
  Platform,
  useColorScheme,
} from 'react-native';

import { StatusBar } from 'expo-status-bar';
import { Tabs } from 'expo-router';
import { NativeTabs } from 'expo-router/unstable-native-tabs';

import { FloatingTabBar } from '@/components/floating-tab-bar';
import { useI18n } from '@/lib/i18n';

export default function TabsLayout() {
  const { t } = useI18n();
  const colorScheme = useColorScheme();

  return (
    <>
      <StatusBar
        style={
          colorScheme === 'dark'
            ? 'light'
            : 'dark'
        }
      />

      {Platform.OS === 'ios' ? (
        <NativeTabs>
          <NativeTabs.Trigger name="index">
            <NativeTabs.Trigger.Icon
              sf={{
                default: 'house',
                selected: 'house.fill',
              }}
            />

            <NativeTabs.Trigger.Label>
              {t('tab_home')}
            </NativeTabs.Trigger.Label>
          </NativeTabs.Trigger>

          <NativeTabs.Trigger name="behavior">
            <NativeTabs.Trigger.Icon
              sf={{
                default: 'sparkles',
                selected: 'sparkles',
              }}
            />

            <NativeTabs.Trigger.Label>
              {t('tab_behavior_ai')}
            </NativeTabs.Trigger.Label>
          </NativeTabs.Trigger>

          <NativeTabs.Trigger name="profile">
            <NativeTabs.Trigger.Icon
              sf={{
                default: 'person',
                selected: 'person.fill',
              }}
            />

            <NativeTabs.Trigger.Label>
              {t('tab_profile')}
            </NativeTabs.Trigger.Label>
          </NativeTabs.Trigger>
        </NativeTabs>
      ) : (
        <Tabs
          screenOptions={{
            headerShown: false,
            tabBarShowLabel: false,
            tabBarStyle: {
              display: 'none',
            },
          }}
          tabBar={(props) => (
            <FloatingTabBar {...props} />
          )}
        >
          <Tabs.Screen
            name="index"
            options={{
              title: t('tab_home'),
            }}
          />

          <Tabs.Screen
            name="behavior"
            options={{
              title: t('tab_behavior_ai'),
            }}
          />

          <Tabs.Screen
            name="profile"
            options={{
              title: t('tab_profile'),
            }}
          />
        </Tabs>
      )}
    </>
  );
}