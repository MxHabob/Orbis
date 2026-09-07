/**
 * @module @orbis/mobile/app/(app)/(tabs)/index
 *
 * Glovo-inspired Home Screen for Orbis Mobile.
 * Clean, declarative component composition powered by @orbis/sdk & TanStack Query.
 */

import React, { useCallback, useMemo, useState } from 'react';
import { View, StyleSheet, ScrollView, RefreshControl, Dimensions } from 'react-native';
import { router } from 'expo-router';
import { ThemedStatusBar } from '@/components/ui/themed-status-bar';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth } from '@/lib/api';
import { useI18n } from '@/lib/i18n';
import { useEffectiveContext } from '@/providers/effective-context';
import {
  useClassesQuery,
  useCurrentOrgQuery,
  useDashboardQuery,
  useEnrollmentsQuery,
} from '@/hooks/use-api-queries';
import {
  CampusHeaderPill,
  StudentBubbleCluster,
  PersonalizedFeaturesGrid,
  PrimeAiBanner,
  BubbleItemData,
} from '@/components/home';
import {
  StudentSeniorIllustration,
  StudentSophomoreIllustration,
  StudentMiddleIllustration,
  StudentElementaryIllustration,
  StudentKGIllustration,
  GradesFeatureIllustration,
  AttendanceFeatureIllustration,
  ScheduleFeatureIllustration,
  BehaviorFeatureIllustration,
  WaveCurveDivider,
} from '@/components/glovo-illustrations';
import { useTheme } from '@/hooks/use-theme';
import { useColorScheme } from '@/hooks/use-color-scheme';
import { useToast } from '@/components/ui/toast';
import {
  Drawer,
  DrawerContent,
  DrawerHeader,
  DrawerTitle,
  DrawerDescription,
  DrawerOptionRow,
  DrawerSubmitButton,
} from '@/components/ui';
import { Spacing } from '@/constants/theme';
import { Ionicons } from '@expo/vector-icons';

const { width: SCREEN_WIDTH } = Dimensions.get('window');

const ILLUSTRATION_LIST = [
  StudentSeniorIllustration,
  StudentSophomoreIllustration,
  StudentMiddleIllustration,
  StudentElementaryIllustration,
  StudentKGIllustration,
];

export default function HomeScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const toast = useToast();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === 'dark';
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

  // ── TanStack Query Data Fetching (Zero useEffect) ──────────────────────
  const { data: orgData, isLoading: isOrgLoading, refetch: refetchOrg } = useCurrentOrgQuery();
  const { data: dashboardData, isLoading: isDashLoading, refetch: refetchDash } = useDashboardQuery();
  const { data: classesData, isLoading: isClassesLoading, refetch: refetchClasses } = useClassesQuery();
  const { data: enrollmentsData, isLoading: isEnrollmentsLoading, refetch: refetchEnrollments } =
    useEnrollmentsQuery(isParent ? {} : { studentId: user?.id });

  const isQueryLoading = isOrgLoading || isDashLoading || isClassesLoading || isEnrollmentsLoading;

  const onRefresh = useCallback(async () => {
    await Promise.all([refetchOrg(), refetchDash(), refetchClasses(), refetchEnrollments()]);
  }, [refetchOrg, refetchDash, refetchClasses, refetchEnrollments]);

  const effectiveId = targetStudentId || user?.id || '';
  const firstName = user?.full_name?.split(' ')[0] || user?.email?.split('@')[0] || '';
  const campusName = orgData?.name || t('nav_campus_default');

  // ── Dynamic Bubble Items ───────────────────────────────────────────────
  const bubbleItems = useMemo<BubbleItemData[]>(() => {
    if (isParent && childrenList.length > 0) {
      return childrenList.slice(0, 5).map((child, idx) => ({
        id: child.id,
        title: child.name,
        sub: child.className || child.gradeLevel || '',
        type: 'child' as const,
        Illustration: ILLUSTRATION_LIST[idx % ILLUSTRATION_LIST.length],
      }));
    }

    if (isTeacher && classesData && classesData.length > 0) {
      return classesData.slice(0, 5).map((c, idx) => ({
        id: c.id,
        title: c.name,
        sub: (c as any).grade_level || '',
        type: 'class' as const,
        Illustration: ILLUSTRATION_LIST[idx % ILLUSTRATION_LIST.length],
      }));
    }

    return [
      {
        id: 'grades',
        title: t('domain_grades'),
        type: 'student_domain' as const,
        route: `/(app)/student/${effectiveId}/grades`,
        Illustration: GradesFeatureIllustration,
      },
      {
        id: 'attendance',
        title: t('domain_attendance'),
        type: 'student_domain' as const,
        route: `/(app)/student/${effectiveId}/attendance`,
        Illustration: AttendanceFeatureIllustration,
      },
      {
        id: 'schedule',
        title: t('domain_schedule'),
        type: 'student_domain' as const,
        route: `/(app)/student/${effectiveId}/schedule`,
        Illustration: ScheduleFeatureIllustration,
      },
      {
        id: 'behavior',
        title: t('domain_behavior'),
        type: 'student_domain' as const,
        route: '/(app)/(tabs)/behavior',
        Illustration: BehaviorFeatureIllustration,
      },
      {
        id: 'exams',
        title: t('domain_exams'),
        type: 'student_domain' as const,
        route: `/(app)/student/${effectiveId}/exams`,
        Illustration: StudentSeniorIllustration,
      },
    ];
  }, [isParent, isTeacher, childrenList, classesData, effectiveId, t]);

  const handleBubblePress = useCallback(
    (item: BubbleItemData) => {
      if (item.type === 'child') {
        setSelectedChildId(item.id);
        router.push(`/(app)/student/${item.id}` as any);
      } else if (item.type === 'class') {
        setSelectedClassId(item.id);
        router.push(`/(app)/class/${item.id}` as any);
      } else if (item.route) {
        router.push(item.route as any);
      }
    },
    [setSelectedChildId, setSelectedClassId],
  );

  return (
    <View style={[styles.container, { backgroundColor: theme.primary }]}>
      <ThemedStatusBar style="light" backgroundColor={theme.primary} />
      <ScrollView
        style={styles.scrollView}
        contentContainerStyle={styles.scrollContent}
        showsVerticalScrollIndicator={false}
        refreshControl={
          <RefreshControl refreshing={isQueryLoading} onRefresh={onRefresh} tintColor="#FFFFFF" />
        }>
        {/* 1. Primary Hero Header */}
        <View style={[styles.heroSection, { paddingTop: Math.max(insets.top, 16) + 4 }]}>
          <CampusHeaderPill campusName={campusName} />
          <StudentBubbleCluster
            items={bubbleItems}
            isLoading={isClassesLoading}
            onItemPress={handleBubblePress}
          />
        </View>

        {/* 2. Themed Wave Curve Divider */}
        <WaveCurveDivider
          width={SCREEN_WIDTH}
          height={38}
          primaryColor={theme.primary}
          sheetColor={theme.background}
        />

        {/* 3. Features & Prime Banner Sheet extending to the bottom */}
        <View
          style={[
            styles.sheet,
            {
              backgroundColor: theme.background,
              paddingBottom: Math.max(insets.bottom, 16) + 100,
            },
          ]}>
          <PersonalizedFeaturesGrid
            userName={firstName}
            isLoading={isQueryLoading}
            onAttendancePress={() =>
              effectiveId && router.push(`/(app)/student/${effectiveId}/attendance` as any)
            }
            onGradesPress={() =>
              effectiveId && router.push(`/(app)/student/${effectiveId}/grades` as any)
            }
            onBehaviorPress={() => router.push('/(app)/(tabs)/behavior' as any)}
            onSchedulePress={() =>
              effectiveId && router.push(`/(app)/student/${effectiveId}/schedule` as any)
            }
          />
          <PrimeAiBanner onPress={() => setShowPrimeDrawer(true)} />
        </View>
      </ScrollView>

      {/* Prime AI Information Bottom Drawer */}
      <Drawer visible={showPrimeDrawer} onClose={() => setShowPrimeDrawer(false)}>
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
              router.push('/(app)/(tabs)/behavior' as any);
            }}
          />

          <DrawerOptionRow
            icon={<Ionicons name="bulb" size={20} color="#F59E0B" />}
            label="توصيات دراسية مخصصة"
            sublabel="اقتراحات يومية لرفع الكفاءة التعليمية"
            onPress={() => {
              setShowPrimeDrawer(false);
              router.push('/(app)/(tabs)/behavior' as any);
            }}
          />

          <DrawerOptionRow
            icon={<Ionicons name="shield-checkmark" size={20} color="#10B981" />}
            label="رصد الحضور والسلوك الإيجابي"
            sublabel="إشعارات فورية وتقارير أسبوعية تفصيلية"
            isLast
            onPress={() => {
              setShowPrimeDrawer(false);
              router.push('/(app)/(tabs)/behavior' as any);
            }}
          />

          <DrawerSubmitButton
            label="استكشاف جميع ميزات Prime"
            onPress={() => {
              setShowPrimeDrawer(false);
              router.push('/(app)/(tabs)/behavior' as any);
            }}
          />
        </DrawerContent>
      </Drawer>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollView: {
    flex: 1,
  },
  scrollContent: {
    flexGrow: 1,
  },
  heroSection: {
    paddingHorizontal: Spacing.four,
    paddingBottom: Spacing.four,
  },
  sheet: {
    flex: 1,
    minHeight: 450,
    marginTop: -2,
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.three,
    gap: Spacing.four,
  },
});
