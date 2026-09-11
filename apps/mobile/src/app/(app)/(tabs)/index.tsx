/**
 * @module @orbis/mobile/app/(app)/(tabs)/index
 *
 * Glovo-inspired Home Screen for Orbis Mobile.
 *
 * - Theme-aware gradient background.
 * - Gradient does not consume layout space.
 * - Gradient starts at the top of the viewport.
 * - Gradient extends visually behind the StatusBar.
 * - StatusBar is handled independently.
 * - No hardcoded UI colors.
 * - Clean declarative composition powered by @orbis/sdk & TanStack Query.
 */

import { useCallback, useMemo, useState } from "react";

import {
  RefreshControl,
  ScrollView,
  StyleSheet,
  useWindowDimensions,
  View,
} from "react-native";

import { router } from "expo-router";

import { useSafeAreaInsets } from "react-native-safe-area-context";

import { ThemedStatusBar } from "@/components/ui/themed-status-bar";

import { useAuth } from "@/lib/api";

import { useI18n } from "@/lib/i18n";

import { useEffectiveContext } from "@/providers/effective-context";

import {
  useClassesQuery,
  useCurrentOrgQuery,
  useDashboardQuery,
  useEnrollmentsQuery,
} from "@/hooks/use-api-queries";

import {
  BubbleItemData,
  PersonalizedFeaturesGrid,
  PrimeAiBanner,
  StudentBubbleCluster,
} from "@/components/home";

import {
  AttendanceFeatureIllustration,
  BehaviorFeatureIllustration,
  GradesFeatureIllustration,
  ScheduleFeatureIllustration,
  StudentElementaryIllustration,
  StudentKGIllustration,
  StudentMiddleIllustration,
  StudentSeniorIllustration,
  StudentSophomoreIllustration,
} from "@/components/glovo-illustrations";

import { GradientHeader } from "@/components/gradient-header";

import { Header } from "@/components/header";

import { useTheme } from "@/hooks/use-theme";

import {
  Drawer,
  DrawerContent,
  DrawerDescription,
  DrawerHeader,
  DrawerOptionRow,
  DrawerSubmitButton,
  DrawerTitle,
} from "@/components/ui";

import { Spacing } from "@/constants/theme";

import { Ionicons } from "@expo/vector-icons";

// ─────────────────────────────────────────────────────────────────────────
// Illustrations
// ─────────────────────────────────────────────────────────────────────────

const ILLUSTRATION_LIST = [
  StudentSeniorIllustration,
  StudentSophomoreIllustration,
  StudentMiddleIllustration,
  StudentElementaryIllustration,
  StudentKGIllustration,
];

// ─────────────────────────────────────────────────────────────────────────
// Screen
// ─────────────────────────────────────────────────────────────────────────

export default function HomeScreen() {
  const { height } = useWindowDimensions();

  const insets = useSafeAreaInsets();

  const theme = useTheme();

  const { user } = useAuth();

  const { t } = useI18n();

  const [showPrimeDrawer, setShowPrimeDrawer] = useState(false);

  const {
    isTeacher,
    isParent,
    childrenList,
    setSelectedChildId,
    setSelectedClassId,
    targetStudentId,
  } = useEffectiveContext();

  // ───────────────────────────────────────────────────────────────────────
  // TanStack Query
  // ───────────────────────────────────────────────────────────────────────

  const {
    data: orgData,
    isLoading: isOrgLoading,
    refetch: refetchOrg,
  } = useCurrentOrgQuery();

  const { isLoading: isDashLoading, refetch: refetchDash } =
    useDashboardQuery();

  const {
    data: classesData,
    isLoading: isClassesLoading,
    refetch: refetchClasses,
  } = useClassesQuery();

  const { isLoading: isEnrollmentsLoading, refetch: refetchEnrollments } =
    useEnrollmentsQuery(
      isParent
        ? {}
        : {
            studentId: user?.id,
          },
    );

  const isQueryLoading =
    isOrgLoading ||
    isDashLoading ||
    (isParent ? isClassesLoading : isEnrollmentsLoading);

  const onRefresh = useCallback(async () => {
    await Promise.all([
      refetchOrg(),
      refetchDash(),
      refetchClasses(),
      refetchEnrollments(),
    ]);
  }, [refetchOrg, refetchDash, refetchClasses, refetchEnrollments]);

  // ───────────────────────────────────────────────────────────────────────
  // Effective Context
  // ───────────────────────────────────────────────────────────────────────

  const effectiveId = targetStudentId || user?.id || "";

  const campusName = orgData?.name;

  // ───────────────────────────────────────────────────────────────────────
  // Dynamic Bubble Items
  // ───────────────────────────────────────────────────────────────────────

  const bubbleItems = useMemo<BubbleItemData[]>(() => {
    // Parent → Children
    if (isParent) {
      return childrenList.slice(0, 5).map((child, idx) => ({
        id: child.id,

        title: child.name,

        sub: child.className || child.gradeLevel || "",

        avatarUrl: child.avatarUrl,

        type: "child" as const,

        Illustration: ILLUSTRATION_LIST[idx % ILLUSTRATION_LIST.length],
      }));
    }

    // Teacher → Classes
    if (isTeacher && classesData && classesData.length > 0) {
      return classesData.slice(0, 5).map((c, idx) => ({
        id: c.id,

        title: c.name,

        sub: (c as any).grade_level || "",

        type: "class" as const,

        Illustration: ILLUSTRATION_LIST[idx % ILLUSTRATION_LIST.length],
      }));
    }

    // Student → Domains
    return [
      {
        id: "grades",

        title: t("domain_grades"),

        type: "student_domain" as const,

        route: `/(app)/student/${effectiveId}/grades`,

        Illustration: GradesFeatureIllustration,
      },

      {
        id: "attendance",

        title: t("domain_attendance"),

        type: "student_domain" as const,

        route: `/(app)/student/${effectiveId}/attendance`,

        Illustration: AttendanceFeatureIllustration,
      },

      {
        id: "schedule",

        title: t("domain_schedule"),

        type: "student_domain" as const,

        route: `/(app)/student/${effectiveId}/schedule`,

        Illustration: ScheduleFeatureIllustration,
      },

      {
        id: "behavior",

        title: t("domain_behavior"),

        type: "student_domain" as const,

        route: "/(app)/(tabs)/behavior",

        Illustration: BehaviorFeatureIllustration,
      },

      {
        id: "exams",

        title: t("domain_exams"),

        type: "student_domain" as const,

        route: `/(app)/student/${effectiveId}/exams`,

        Illustration: StudentSeniorIllustration,
      },
    ];
  }, [isParent, isTeacher, childrenList, classesData, effectiveId, t]);

  // ───────────────────────────────────────────────────────────────────────
  // Bubble Navigation
  // ───────────────────────────────────────────────────────────────────────

  const handleBubblePress = useCallback(
    (item: BubbleItemData) => {
      // Parent → Child
      if (item.type === "child") {
        setSelectedChildId(item.id);

        router.push(`/(app)/student/${item.id}` as any);

        return;
      }

      // Teacher → Class
      if (item.type === "class") {
        setSelectedClassId(item.id);

        router.push(`/(app)/class/${item.id}` as any);

        return;
      }

      // Student → Domain
      if (item.route) {
        router.push(item.route as any);
      }
    },
    [setSelectedChildId, setSelectedClassId],
  );

  // ───────────────────────────────────────────────────────────────────────
  // Render
  // ───────────────────────────────────────────────────────────────────────

  return (
    <View
      style={[
        styles.container,
        {
          backgroundColor: theme.background,
        },
      ]}
    >
      <ThemedStatusBar backgroundColor="transparent" />

      {/* ─────────────────────────────────────────────────────────────
          HOME GRADIENT

          Background layer only.

          - Does not consume layout space.
          - Starts from the very top of the viewport.
          - Extends behind the StatusBar.
          - Ends gradually before the bottom of the screen.
          - Remains behind ScrollView.
          - No hard horizontal section.
        ───────────────────────────────────────────────────────────── */}

      <GradientHeader
        height={height * 0.72}
        colors={theme.homeGradient.colors}
        locations={[0, 0.16, 0.34, 0.52, 0.68, 0.84, 1]}
        start={theme.homeGradient.start}
        end={theme.homeGradient.end}
        blobPrimary={theme.homeGradient.blobPrimary}
        blobSecondary={theme.homeGradient.blobSecondary}
        blobTertiary={theme.homeGradient.blobTertiary}
        animated
        duration={18000}
        style={styles.homeGradient}
      />

      {/* ─────────────────────────────────────────────────────────────
          Scroll Content

          Content respects the Safe Area.
          Gradient does NOT.
        ───────────────────────────────────────────────────────────── */}

      <ScrollView
        style={styles.scrollView}
        contentContainerStyle={styles.scrollContent}
        showsVerticalScrollIndicator={false}
        refreshControl={
          <RefreshControl
            refreshing={isQueryLoading}
            onRefresh={onRefresh}
            tintColor={theme.primary}
          />
        }
      >
        {/* ─────────────────────────────────────────────────────────
            Hero
          ───────────────────────────────────────────────────────── */}

        <View style={styles.heroSection}>
          <Header
            title={campusName}
          />

          <View style={styles.bubbleClusterWrapper}>
            <StudentBubbleCluster
              items={bubbleItems}
              isLoading={isParent}
              onItemPress={handleBubblePress}
            />
          </View>
        </View>

        {/* ─────────────────────────────────────────────────────────
            Features

            No backgroundColor here.

            The gradient is allowed to remain visible behind
            the upper part of the content.
          ───────────────────────────────────────────────────────── */}

        <View
          style={[
            styles.sheet,
            {
              paddingBottom: Math.max(insets.bottom, Spacing.four) + 100,
            },
          ]}
        >
          <PersonalizedFeaturesGrid
            isLoading={isQueryLoading}
            onAttendancePress={() =>
              effectiveId &&
              router.push(`/(app)/student/${effectiveId}/attendance` as any)
            }
            onGradesPress={() =>
              effectiveId &&
              router.push(`/(app)/student/${effectiveId}/grades` as any)
            }
            onBehaviorPress={() => router.push("/(app)/(tabs)/behavior" as any)}
            onSchedulePress={() =>
              effectiveId &&
              router.push(`/(app)/student/${effectiveId}/schedule` as any)
            }
          />

          <PrimeAiBanner onPress={() => setShowPrimeDrawer(true)} />
        </View>
      </ScrollView>

      {/* ─────────────────────────────────────────────────────────────
          Prime AI Drawer
        ───────────────────────────────────────────────────────────── */}

      <Drawer
        visible={showPrimeDrawer}
        onClose={() => setShowPrimeDrawer(false)}
      >
        <DrawerContent>
          <DrawerHeader>
            <Ionicons name="sparkles" size={28} color={theme.primary} />

            <DrawerTitle>Orbis Prime AI</DrawerTitle>

            <DrawerDescription>
              تحليلات الذكاء الاصطناعي الأكاديمية والتنبؤ بالأداء الدراسي
            </DrawerDescription>
          </DrawerHeader>

          <DrawerOptionRow
            icon={<Ionicons name="analytics" size={20} color={theme.primary} />}
            label="التحليل التنبؤي للدرجات"
            sublabel="توقعات مبكرة لمستوى التحصيل الدراسي"
            onPress={() => {
              setShowPrimeDrawer(false);

              router.push("/(app)/(tabs)/behavior" as any);
            }}
          />

          <DrawerOptionRow
            icon={<Ionicons name="bulb" size={20} color={theme.warning} />}
            label="توصيات دراسية مخصصة"
            sublabel="اقتراحات يومية لرفع الكفاءة التعليمية"
            onPress={() => {
              setShowPrimeDrawer(false);

              router.push("/(app)/(tabs)/behavior" as any);
            }}
          />

          <DrawerOptionRow
            icon={
              <Ionicons
                name="shield-checkmark"
                size={20}
                color={theme.success}
              />
            }
            label="رصد الحضور والسلوك الإيجابي"
            sublabel="إشعارات فورية وتقارير أسبوعية تفصيلية"
            isLast
            onPress={() => {
              setShowPrimeDrawer(false);

              router.push("/(app)/(tabs)/behavior" as any);
            }}
          />

          <DrawerSubmitButton
            label="استكشاف جميع ميزات Prime"
            onPress={() => {
              setShowPrimeDrawer(false);

              router.push("/(app)/(tabs)/behavior" as any);
            }}
          />
        </DrawerContent>
      </Drawer>
    </View>
  );
}

// ─────────────────────────────────────────────────────────────────────────
// Styles
// ─────────────────────────────────────────────────────────────────────────

const styles = StyleSheet.create({
  /**
   * Gradient background layer.
   *
   * IMPORTANT:
   * `top: 0` is required.
   *
   * Do NOT use `bottom: 0` here because the gradient
   * must originate from the top of the viewport.
   */
  homeGradient: {
    position: "absolute",

    top: 0,

    left: 0,

    right: 0,

    width: "100%",

    zIndex: 0,
  },

  /**
   * Main screen container.
   */
  container: {
    flex: 1,
  },

  /**
   * ScrollView sits above the gradient.
   */
  scrollView: {
    flex: 1,

    zIndex: 1,
  },

  /**
   * Scroll content itself remains transparent.
   *
   * This is important because any opaque background
   * here would hide the gradient.
   */
  scrollContent: {
    flexGrow: 1,
  },

  /**
   * Hero section.
   *
   * Top safe-area padding is now handled inside <Header />, so this
   * section no longer applies it directly.
   */
  heroSection: {
    width: "100%",
  },

  /**
   * Spacing between the header and the bubble cluster below it.
   */
  bubbleClusterWrapper: {
    marginTop: Spacing.four,
  },

  /**
   * Features section.
   *
   * No backgroundColor.
   */
  sheet: {
    width: "100%",
  },
});
