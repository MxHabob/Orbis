/**
 * @module @orbis/mobile/components/home/prime-ai-banner
 *
 * Glovo-style Prime promotional banner highlighting unlimited AI analytics.
 */

import React from "react";
import { View, Text, StyleSheet, Pressable } from "react-native";
import { Ionicons } from "@expo/vector-icons";
import { PrimeScooterMascot } from "@/components/glovo-illustrations";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import { useColorScheme } from "@/hooks/use-color-scheme";
import { Radii, Spacing, Typography } from "@/constants/theme";

interface PrimeAiBannerProps {
  onPress: () => void;
}

export const PrimeAiBanner = React.memo<PrimeAiBannerProps>(({ onPress }) => {
  const theme = useTheme();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const { t } = useI18n();

  return (
    <Pressable
      onPress={onPress}
      style={({ pressed }) => [
        styles.banner,
        pressed && { opacity: 0.92, transform: [{ scale: 0.99 }] },
      ]}
    >
      {/* Left Content */}
      <View style={styles.left}>
        <View style={styles.badge}>
          <Ionicons name="sparkles" size={13} color="#FFFFFF" />
          <Text style={styles.badgeText}>{t("prime_badge")}</Text>
        </View>

        <Text style={[styles.title, { color: theme.text }]}>
          {t("prime_title")}
        </Text>
        <Text style={[styles.subtitle, { color: theme.textSecondary }]}>
          {t("prime_subtitle")}
        </Text>
      </View>

      {/* Right Mascot Artwork & Button */}
      <View style={styles.right}>
        <PrimeScooterMascot size={78} />
        <View
          style={[
            styles.chevronButton,
            {
              backgroundColor: theme.background,
              borderColor: theme.border,
            },
          ]}
        >
          <Ionicons name="chevron-forward" size={18} color={theme.primary} />
        </View>
      </View>
    </Pressable>
  );
});

PrimeAiBanner.displayName = "PrimeAiBanner";

const styles = StyleSheet.create({
  banner: {
    paddingVertical: Spacing.four,
    paddingHorizontal: Spacing.four,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    marginTop: Spacing.two,
  },
  left: {
    flex: 1,
    gap: 4,
    paddingRight: Spacing.two,
  },
  badge: {
    flexDirection: "row",
    alignItems: "center",
    paddingVertical: 3,
    paddingHorizontal: 8,
    borderRadius: Radii.full,
    alignSelf: "flex-start",
    gap: 4,
    marginBottom: 4,
  },
  badgeText: {
    ...Typography.small,
    fontSize: 10,
    fontWeight: "800",
    color: "#FFFFFF",
    letterSpacing: 0.8,
  },
  title: {
    ...Typography.bodyBold,
    fontSize: 15,
    fontWeight: "800",
  },
  subtitle: {
    ...Typography.small,
    fontSize: 12,
  },
  right: {
    flexDirection: "row",
    alignItems: "center",
    gap: Spacing.two,
  },
  chevronButton: {
    width: 34,
    height: 34,
    borderRadius: 17,
    borderWidth: 1,
    alignItems: "center",
    justifyContent: "center",
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.08,
    shadowRadius: 4,
    elevation: 2,
  },
});
