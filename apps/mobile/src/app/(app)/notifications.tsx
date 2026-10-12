/**
 * @module @orbis/mobile/app/(app)/notifications
 *
 * Notification Center for Orbis Mobile.
 * Features categorized notifications, unread badges, and clear back navigation.
 */

import React, { useState } from "react";
import { View, Text, Pressable } from "react-native";
import { Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { EmptyState } from "@/components/ui/empty-state";
import { Chip } from "@/components/ui/chip";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { useI18n } from "@/lib/i18n";
import { useRouter } from "expo-router";
import {
  Bell,
  AcademicCap,
  Calendar,
  Document,
  Sparkles,
} from "@orbis/icons/native";

type FilterTab = "all" | "academic" | "attendance" | "system";

export default function NotificationsScreen() {
  const { t, isArabic } = useI18n();
  const router = useRouter();
  const [activeFilter, setActiveFilter] = useState<FilterTab>("all");

  return (
    <View className="flex-1 bg-background">
      <Header
        title={t("home_notifications")}
        showBack={true}
        onBack={() => router.back()}
      />

      <ScreenScrollView
        edges={["bottom", "left", "right"]}
        scrollViewProps={{
          contentContainerClassName: "gap-four px-four pb-[100px] pt-two",
        }}
      >
        {/* Category Filters */}
        <View className="flex-row items-center gap-two pb-one">
          <Chip
            label={t("filter_all") || "All"}
            selected={activeFilter === "all"}
            onPress={() => setActiveFilter("all")}
          />
          <Chip
            label={t("domain_academic") || "Academic"}
            selected={activeFilter === "academic"}
            onPress={() => setActiveFilter("academic")}
          />
          <Chip
            label={t("domain_attendance") || "Attendance"}
            selected={activeFilter === "attendance"}
            onPress={() => setActiveFilter("attendance")}
          />
        </View>

        {/* Empty State */}
        <View className="flex-1 items-center justify-center py-sixteen">
          <EmptyState
            title={t("notifications_empty_title") || "All Caught Up"}
            description={t("notifications_inbox_unavailable_description")}
            icon={<Bell size={36} className="text-muted-foreground" />}
          />
        </View>
      </ScreenScrollView>
    </View>
  );
}
