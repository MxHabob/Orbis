/**
 * @module @orbis/mobile/app/(app)/(tabs)/_layout
 *
 * Shared tabs layout.
 *
 * - iOS uses NativeTabs.
 * - Android uses the custom FloatingTabBar.
 * - System StatusBar follows the current color scheme.
 */

import { Tabs } from "expo-router";
import { FloatingTabBar } from "@/components/floating-tab-bar";

export default function TabsLayout() {
  return (
    <Tabs
      screenOptions={{
        headerShown: false,
        tabBarShowLabel: false,
        tabBarStyle: {
          display: "none",
        },
      }}
      tabBar={(props: any) => <FloatingTabBar {...(props as any)} />}
    >
      <Tabs.Screen name="index" />
      <Tabs.Screen name="behavior" />
    </Tabs>
  );
}

