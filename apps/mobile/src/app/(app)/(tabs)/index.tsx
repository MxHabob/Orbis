/**
 * @module @orbis/mobile/app/(app)/(tabs)/index
 *
 * Glovo-inspired Home Screen for Orbis Mobile.
 * Built with @orbis/sdk & TanStack Query:
 * - 0 useEffect calls for data fetching
 * - Skeletons for bubbles & feature cards during query loading
 * - Highest performance with memoized layout and zero dummy data
 */

import React, { useCallback, useMemo } from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  Pressable,
  RefreshControl,
  Dimensions,
} from 'react-native';
import { router } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { StatusBar } from 'expo-status-bar';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth } from '@/lib/api';
import { useEffectiveContext } from '@/providers/effective-context';
import {
  useClassesQuery,
  useCurrentOrgQuery,
  useDashboardQuery,
  useEnrollmentsQuery,
} from '@/hooks/use-api-queries';
import {
  OrganicBubbleRing,
  StudentSeniorIllustration,
  StudentSophomoreIllustration,
  StudentMiddleIllustration,
  StudentElementaryIllustration,
  StudentKGIllustration,
  WaveCurveDivider,
  AttendanceFeatureIllustration,
  GradesFeatureIllustration,
  BehaviorFeatureIllustration,
  ScheduleFeatureIllustration,
  PrimeScooterMascot,
} from '@/components/glovo-illustrations';
import { SkeletonBubble, SkeletonFeedCard, Skeleton } from '@/components/ui/skeleton';
import { Radii, Spacing, Typography } from '@/constants/theme';

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
  const { user } = useAuth();
  const {
    isTeacher,
    isParent,
    isStudent,
    childrenList,
    setSelectedChildId,
    setSelectedClassId,
    targetStudentId,
    selectedClassId,
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
  const effectiveClassId = selectedClassId || (classesData && classesData[0]?.id) || '';
  const firstName = user?.full_name?.split(' ')[0] || user?.email?.split('@')[0] || '';
  const campusName = orgData?.name || 'مجمع الرياض التعليمي';

  const handleStudentPress = useCallback(
    (childId: string) => {
      setSelectedChildId(childId);
      router.push(`/(app)/student/${childId}` as any);
    },
    [setSelectedChildId],
  );

  const handleClassPress = useCallback(
    (classId: string) => {
      setSelectedClassId(classId);
      router.push(`/(app)/class/${classId}` as any);
    },
    [setSelectedClassId],
  );

  // ── Build Dynamic Bubble Items with Memoization ────────────────────────
  const bubbleItems = useMemo(() => {
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

    // Default student academic domain cards
    return [
      {
        id: 'grades',
        title: 'الدرجات',
        type: 'student_domain' as const,
        route: `/(app)/student/${effectiveId}/grades`,
        Illustration: GradesFeatureIllustration,
      },
      {
        id: 'attendance',
        title: 'الحضور',
        type: 'student_domain' as const,
        route: `/(app)/student/${effectiveId}/attendance`,
        Illustration: AttendanceFeatureIllustration,
      },
      {
        id: 'schedule',
        title: 'الجدول',
        type: 'student_domain' as const,
        route: `/(app)/student/${effectiveId}/schedule`,
        Illustration: ScheduleFeatureIllustration,
      },
      {
        id: 'behavior',
        title: 'السلوك',
        type: 'student_domain' as const,
        route: '/(app)/(tabs)/behavior',
        Illustration: BehaviorFeatureIllustration,
      },
      {
        id: 'exams',
        title: 'الاختبارات',
        type: 'student_domain' as const,
        route: `/(app)/student/${effectiveId}/exams`,
        Illustration: StudentSeniorIllustration,
      },
    ];
  }, [isParent, isTeacher, childrenList, classesData, effectiveId]);

  const topTwo = useMemo(() => bubbleItems.slice(0, 2), [bubbleItems]);
  const bottomItems = useMemo(() => bubbleItems.slice(2, 5), [bubbleItems]);

  return (
    <View style={styles.container}>
      <StatusBar style="dark" />
      <ScrollView
        style={styles.scrollView}
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: Math.max(insets.bottom, 16) + 70 },
        ]}
        showsVerticalScrollIndicator={false}
        refreshControl={
          <RefreshControl refreshing={isQueryLoading} onRefresh={onRefresh} tintColor="#FFFFFF" />
        }>
        {/* ========================================================
            1. TOP WARM GOLDEN HERO (Glovo Style)
            ======================================================== */}
        <View
          style={[
            styles.heroSection,
            { paddingTop: Math.max(insets.top, 16) + 4 },
          ]}>
          {/* Centered Top Pill: Home ⌄ */}
          <View style={styles.topPillContainer}>
            <View style={styles.topCapsulePill}>
              <Ionicons name="home-outline" size={18} color="#18181B" />
              <Text style={styles.topCapsuleText}>{campusName}</Text>
              <Ionicons name="chevron-down" size={16} color="#18181B" />
            </View>
          </View>

          {/* Staggered Circular Bubbles Cluster */}
          {isClassesLoading ? (
            <View style={styles.bubblesCluster}>
              <View style={styles.bubblesRowTop}>
                <SkeletonBubble size={118} />
                <SkeletonBubble size={118} />
              </View>
              <View style={styles.bubblesRowBottom}>
                <SkeletonBubble size={100} />
                <SkeletonBubble size={100} />
                <SkeletonBubble size={100} />
              </View>
            </View>
          ) : (
            <View style={styles.bubblesCluster}>
              {/* Top Row */}
              <View style={styles.bubblesRowTop}>
                {topTwo.map((item) => {
                  const ItemIllustration = item.Illustration;
                  return (
                    <Pressable
                      key={item.id}
                      onPress={() => {
                        if (item.type === 'child') handleStudentPress(item.id);
                        else if (item.type === 'class') handleClassPress(item.id);
                        else if ((item as any).route) router.push((item as any).route);
                      }}
                      style={({ pressed }) => [
                        styles.bubbleItemLarge,
                        pressed && { transform: [{ scale: 0.94 }] },
                      ]}>
                      <View style={styles.bubbleRingWrapperLarge}>
                        <OrganicBubbleRing size={118} />
                        <View style={styles.bubbleCircleLarge}>
                          <ItemIllustration size={68} />
                        </View>
                        <View style={styles.bubblePill}>
                          <Text style={styles.bubblePillText} numberOfLines={1}>
                            {item.title}
                          </Text>
                        </View>
                      </View>
                    </Pressable>
                  );
                })}
              </View>

              {/* Bottom Row */}
              {bottomItems.length > 0 && (
                <View style={styles.bubblesRowBottom}>
                  {bottomItems.map((item, idx) => {
                    const ItemIllustration = item.Illustration;
                    const isMiddle = idx === 1;
                    return (
                      <Pressable
                        key={item.id}
                        onPress={() => {
                          if (item.type === 'child') handleStudentPress(item.id);
                          else if (item.type === 'class') handleClassPress(item.id);
                          else if ((item as any).route) router.push((item as any).route);
                        }}
                        style={({ pressed }) => [
                          styles.bubbleItemSmall,
                          isMiddle && styles.bubbleItemArcMiddle,
                          pressed && {
                            transform: isMiddle
                              ? [{ translateY: 6 }, { scale: 0.94 }]
                              : [{ scale: 0.94 }],
                          },
                        ]}>
                        <View style={styles.bubbleRingWrapperSmall}>
                          <OrganicBubbleRing size={100} />
                          <View style={styles.bubbleCircleSmall}>
                            <ItemIllustration size={56} />
                          </View>
                          <View style={styles.bubblePill}>
                            <Text style={styles.bubblePillText} numberOfLines={1}>
                              {item.title}
                            </Text>
                          </View>
                        </View>
                      </Pressable>
                    );
                  })}
                </View>
              )}
            </View>
          )}
        </View>

        {/* ========================================================
            2. ORGANIC WAVE DIVIDER
            ======================================================== */}
        <WaveCurveDivider width={SCREEN_WIDTH} height={38} />

        {/* ========================================================
            3. WHITE SHEET: "Sam, these are for you"
            ======================================================== */}
        <View style={styles.whiteSheet}>
          {/* Section Header */}
          <View style={styles.sectionHeaderRow}>
            {isOrgLoading ? (
              <Skeleton width={180} height={24} />
            ) : (
              <Text style={styles.personalizedTitle}>
                {firstName ? `${firstName}، هذه مخصصة لك` : 'هذه مخصصة لك'}
              </Text>
            )}
            <Pressable hitSlop={8}>
              <Ionicons name="information-circle-outline" size={22} color="#71717A" />
            </Pressable>
          </View>

          {/* 4 Feature Cards */}
          {isQueryLoading ? (
            <View style={styles.featuresRow}>
              <SkeletonFeedCard size={(SCREEN_WIDTH - 56) / 4} />
              <SkeletonFeedCard size={(SCREEN_WIDTH - 56) / 4} />
              <SkeletonFeedCard size={(SCREEN_WIDTH - 56) / 4} />
              <SkeletonFeedCard size={(SCREEN_WIDTH - 56) / 4} />
            </View>
          ) : (
            <View style={styles.featuresRow}>
              {/* Card 1: Attendance */}
              <Pressable
                onPress={() =>
                  effectiveId && router.push(`/(app)/student/${effectiveId}/attendance` as any)
                }
                style={({ pressed }) => [
                  styles.featureItem,
                  pressed && { opacity: 0.85, transform: [{ scale: 0.96 }] },
                ]}>
                <View style={styles.squircleBox}>
                  <AttendanceFeatureIllustration size={44} />
                </View>
                <Text style={styles.featureLabel} numberOfLines={1}>
                  سجل الحضور
                </Text>
              </Pressable>

              {/* Card 2: Grades */}
              <Pressable
                onPress={() =>
                  effectiveId && router.push(`/(app)/student/${effectiveId}/grades` as any)
                }
                style={({ pressed }) => [
                  styles.featureItem,
                  pressed && { opacity: 0.85, transform: [{ scale: 0.96 }] },
                ]}>
                <View style={styles.squircleBox}>
                  <GradesFeatureIllustration size={44} />
                </View>
                <Text style={styles.featureLabel} numberOfLines={1}>
                  الدرجات
                </Text>
              </Pressable>

              {/* Card 3: Behavior */}
              <Pressable
                onPress={() => router.push('/(app)/(tabs)/behavior' as any)}
                style={({ pressed }) => [
                  styles.featureItem,
                  pressed && { opacity: 0.85, transform: [{ scale: 0.96 }] },
                ]}>
                <View style={styles.squircleBox}>
                  <BehaviorFeatureIllustration size={44} />
                </View>
                <Text style={styles.featureLabel} numberOfLines={1}>
                  التميز والسلوك
                </Text>
              </Pressable>

              {/* Card 4: Schedule */}
              <Pressable
                onPress={() =>
                  effectiveId && router.push(`/(app)/student/${effectiveId}/schedule` as any)
                }
                style={({ pressed }) => [
                  styles.featureItem,
                  pressed && { opacity: 0.85, transform: [{ scale: 0.96 }] },
                ]}>
                <View style={styles.squircleBox}>
                  <ScheduleFeatureIllustration size={44} />
                </View>
                <Text style={styles.featureLabel} numberOfLines={1}>
                  الجدول
                </Text>
              </Pressable>
            </View>
          )}

          {/* Pagination Dots */}
          <View style={styles.paginationRow}>
            <View style={[styles.dot, styles.dotActive]} />
            <View style={styles.dot} />
          </View>

          {/* ========================================================
              4. PRIME BANNER
              ======================================================== */}
          <Pressable
            onPress={() => router.push('/(app)/(tabs)/behavior' as any)}
            style={({ pressed }) => [
              styles.primeBanner,
              pressed && { opacity: 0.92, transform: [{ scale: 0.99 }] },
            ]}>
            {/* Left Content */}
            <View style={styles.primeLeft}>
              <View style={styles.primeBadge}>
                <Ionicons name="sparkles" size={13} color="#FFFFFF" />
                <Text style={styles.primeBadgeText}>PRIME</Text>
              </View>

              <Text style={styles.primeTitle}>متابعة ذكية وتقارير غير محدودة</Text>
              <Text style={styles.primeSub}>تحليل فوري بالذكاء الاصطناعي مجاناً</Text>
            </View>

            {/* Right Mascot Artwork & Button */}
            <View style={styles.primeRight}>
              <PrimeScooterMascot size={78} />
              <View style={styles.primeChevronButton}>
                <Ionicons name="chevron-forward" size={18} color="#18181B" />
              </View>
            </View>
          </Pressable>
        </View>
      </ScrollView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#FFC244',
  },
  scrollView: {
    flex: 1,
  },
  scrollContent: {
    flexGrow: 1,
  },

  /* Hero Section */
  heroSection: {
    paddingHorizontal: Spacing.four,
    paddingBottom: Spacing.four,
    backgroundColor: '#FFC244',
  },

  /* Top Capsule: Home ⌄ */
  topPillContainer: {
    alignItems: 'center',
    marginBottom: Spacing.four,
  },
  topCapsulePill: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 8,
    backgroundColor: '#FFE68C',
    paddingHorizontal: 22,
    paddingVertical: 9,
    borderRadius: Radii.full,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.08,
    shadowRadius: 4,
    elevation: 2,
  },
  topCapsuleText: {
    ...Typography.bodyBold,
    fontSize: 16,
    color: '#18181B',
  },

  /* Bubbles Cluster */
  bubblesCluster: {
    alignItems: 'center',
    gap: Spacing.three,
    marginTop: 2,
  },
  bubblesRowTop: {
    flexDirection: 'row',
    justifyContent: 'center',
    gap: 32,
  },
  bubblesRowBottom: {
    flexDirection: 'row',
    justifyContent: 'center',
    gap: 14,
  },

  /* Large Bubble (Top Row) */
  bubbleItemLarge: {
    alignItems: 'center',
  },
  bubbleRingWrapperLarge: {
    width: 118,
    height: 118,
    alignItems: 'center',
    justifyContent: 'center',
    position: 'relative',
  },
  bubbleCircleLarge: {
    width: 104,
    height: 104,
    borderRadius: 52,
    backgroundColor: '#FFFFFF',
    alignItems: 'center',
    justifyContent: 'center',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.12,
    shadowRadius: 8,
    elevation: 4,
  },

  /* Small Bubble (Bottom Row) */
  bubbleItemSmall: {
    alignItems: 'center',
  },
  bubbleItemArcMiddle: {
    transform: [{ translateY: 6 }],
  },
  bubbleRingWrapperSmall: {
    width: 100,
    height: 100,
    alignItems: 'center',
    justifyContent: 'center',
    position: 'relative',
  },
  bubbleCircleSmall: {
    width: 88,
    height: 88,
    borderRadius: 44,
    backgroundColor: '#FFFFFF',
    alignItems: 'center',
    justifyContent: 'center',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 3 },
    shadowOpacity: 0.1,
    shadowRadius: 6,
    elevation: 3,
  },

  /* Overlapping White Pill Label */
  bubblePill: {
    position: 'absolute',
    bottom: -8,
    backgroundColor: '#FFFFFF',
    paddingVertical: 4,
    paddingHorizontal: 12,
    borderRadius: 14,
    borderWidth: 1.5,
    borderColor: '#18181B',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
    maxWidth: 110,
  },
  bubblePillText: {
    ...Typography.small,
    fontWeight: '800',
    color: '#18181B',
    fontSize: 12,
    textAlign: 'center',
  },

  /* White Sheet Section */
  whiteSheet: {
    backgroundColor: '#FFFFFF',
    paddingTop: Spacing.two,
    paddingHorizontal: Spacing.four,
    gap: Spacing.four,
    minHeight: 440,
  },
  sectionHeaderRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingHorizontal: Spacing.one,
    marginTop: Spacing.two,
  },
  personalizedTitle: {
    ...Typography.h2,
    fontSize: 22,
    fontWeight: '800',
    color: '#18181B',
    letterSpacing: -0.4,
  },

  /* 4 Squircle Feature Cards */
  featuresRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginTop: Spacing.one,
  },
  featureItem: {
    width: (SCREEN_WIDTH - 56) / 4,
    alignItems: 'center',
  },
  squircleBox: {
    width: (SCREEN_WIDTH - 56) / 4,
    height: (SCREEN_WIDTH - 56) / 4,
    backgroundColor: '#F4F4F5',
    borderRadius: 22,
    alignItems: 'center',
    justifyContent: 'center',
  },
  featureLabel: {
    ...Typography.small,
    fontWeight: '700',
    fontSize: 12,
    color: '#18181B',
    textAlign: 'center',
    marginTop: 6,
  },

  /* Pagination Dots */
  paginationRow: {
    flexDirection: 'row',
    justifyContent: 'center',
    alignItems: 'center',
    gap: 6,
    marginVertical: Spacing.one,
  },
  dot: {
    width: 7,
    height: 7,
    borderRadius: 3.5,
    backgroundColor: '#D4D4D8',
  },
  dotActive: {
    backgroundColor: '#18181B',
  },

  /* PRIME Banner */
  primeBanner: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    backgroundColor: '#EEF2FF',
    borderRadius: 24,
    padding: Spacing.four,
    marginTop: Spacing.two,
    overflow: 'hidden',
    position: 'relative',
  },
  primeLeft: {
    flex: 1,
    gap: 4,
  },
  primeBadge: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#4F46E5',
    paddingVertical: 4,
    paddingHorizontal: 10,
    borderRadius: Radii.sm,
    alignSelf: 'flex-start',
    gap: 4,
  },
  primeBadgeText: {
    color: '#FFFFFF',
    fontSize: 11,
    fontWeight: '800',
    letterSpacing: 0.5,
  },
  primeTitle: {
    ...Typography.bodyBold,
    fontSize: 16,
    color: '#18181B',
    marginTop: 4,
  },
  primeSub: {
    ...Typography.small,
    fontSize: 12,
    color: '#4F46E5',
  },
  primeRight: {
    flexDirection: 'row',
    alignItems: 'center',
    position: 'relative',
  },
  primeChevronButton: {
    position: 'absolute',
    right: -4,
    width: 32,
    height: 32,
    borderRadius: 16,
    backgroundColor: '#FFFFFF',
    alignItems: 'center',
    justifyContent: 'center',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
});
