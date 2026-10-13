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
import { Header } from "@/components/header";
import { useI18n } from "@/lib/i18n";

export default function TabsLayout() {
  const { t } = useI18n();

  return (
    <Tabs
      screenOptions={{
        headerTransparent: true,
        headerShadowVisible: false,
        headerStyle: {
          backgroundColor: "transparent",
        },
        header: ({ route }) => (
          <Header
            title={t(
              route.name === "behavior" ? "tab_behavior_ai" : "tab_home",
            )}
            showCrown
          />
        ),
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
