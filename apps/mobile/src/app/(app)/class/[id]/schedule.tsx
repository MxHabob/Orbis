/**
 * @module @orbis/mobile/app/(app)/class/[id]/schedule
 *
 * Class Timetable Grid Screen for Teachers.
 *
 * - Full-width screen background.
 * - Gradient extends behind StatusBar.
 * - Header remains transparent.
 * - Day selector remains part of the screen content.
 * - No hardcoded UI colors.
 */

import React, {
  useState,
} from 'react';

import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  Pressable,
} from 'react-native';

import {
  useLocalSearchParams,
} from 'expo-router';

import {
  Ionicons,
} from '@expo/vector-icons';

import {
  useSafeAreaInsets,
} from 'react-native-safe-area-context';

import {
  useTheme,
} from '@/hooks/use-theme';

import {
  Header,
} from '@/components/header';

import {
  Card,
} from '@/components/ui/card';

import {
  Badge,
} from '@/components/ui/badge';

import {
  ScreenBackground,
} from '@/components/screen-background';

import {
  ThemedStatusBar,
} from '@/components/ui/themed-status-bar';

import {
  Radii,
  Spacing,
  Typography,
} from '@/constants/theme';


// ─────────────────────────────────────────────────────────────────────────
// Screen
// ─────────────────────────────────────────────────────────────────────────

export default function ClassScheduleScreen() {

  const insets =
    useSafeAreaInsets();

  const theme =
    useTheme();

  const {
    id,
  } =
    useLocalSearchParams<{
      id: string;
    }>();


  const [
    selectedDay,
    setSelectedDay,
  ] =
    useState(0);


  // ───────────────────────────────────────────────────────────────────────
  // Days
  // ───────────────────────────────────────────────────────────────────────

  const days = [
    {
      index: 0,
      name: 'الأحد',
    },

    {
      index: 1,
      name: 'الإثنين',
    },

    {
      index: 2,
      name: 'الثلاثاء',
    },

    {
      index: 3,
      name: 'الأربعاء',
    },

    {
      index: 4,
      name: 'الخميس',
    },
  ];


  // ───────────────────────────────────────────────────────────────────────
  // Render
  // ───────────────────────────────────────────────────────────────────────

  return (
    <ScreenBackground
      gradientHeight={
        0.48
      }>

      {/* ─────────────────────────────────────────────────────────────
          Status Bar
        ───────────────────────────────────────────────────────────── */}

      <ThemedStatusBar />


      {/* ─────────────────────────────────────────────────────────────
          Header
        ───────────────────────────────────────────────────────────── */}

      <Header
        title="جدول حصص الفصل"

        subtitle="الصف 3 علمي - أ • القاعة 104"

        showBack
      />


      {/* ─────────────────────────────────────────────────────────────
          Main Content
        ───────────────────────────────────────────────────────────── */}

      <ScrollView
        style={
          styles.scrollView
        }

        contentContainerStyle={[
          styles.scrollContent,

          {
            paddingBottom:
              insets.bottom +
              Spacing.eight,
          },
        ]}

        showsVerticalScrollIndicator={
          false
        }>

        {/* ─────────────────────────────────────────────────────────
            Day Selector
          ───────────────────────────────────────────────────────── */}

        <View
          style={[
            styles.dayTabBar,

            {
              backgroundColor:
                theme.card,

              borderColor:
                theme.borderSubtle,
            },
          ]}>

          {days.map(
            day => {

              const isSelected =
                selectedDay ===
                day.index;


              return (
                <Pressable
                  key={
                    day.index
                  }

                  onPress={() =>
                    setSelectedDay(
                      day.index,
                    )
                  }

                  style={[
                    styles.dayTab,

                    {
                      backgroundColor:
                        isSelected
                          ? theme.primary
                          : 'transparent',
                    },
                  ]}>

                  <Text
                    style={[
                      styles.dayTabText,

                      {
                        color:
                          isSelected
                            ? theme.primaryForeground
                            : theme.textSecondary,
                      },
                    ]}>

                    {
                      day.name
                    }

                  </Text>

                </Pressable>
              );
            },
          )}

        </View>


        {/* ─────────────────────────────────────────────────────────
            Schedule
          ───────────────────────────────────────────────────────── */}

        <View
          style={
            styles.scheduleList
          }>

          {[
            {
              slot:
                1,

              time:
                '08:00 - 08:45',

              subject:
                'الرياضيات المتقدمة',

              teacher:
                'أنت (معلم المادة)',

              room:
                'قاعة 104',
            },

            {
              slot:
                2,

              time:
                '08:50 - 09:35',

              subject:
                'الفيزياء التطبيقية',

              teacher:
                'د. أحمد العمري',

              room:
                'مختبر العلوم 2',
            },

            {
              slot:
                3,

              time:
                '09:40 - 10:25',

              subject:
                'اللغة الإنجليزية',

              teacher:
                'أ. مايكل براون',

              room:
                'قاعة 104',
            },

            {
              slot:
                4,

              time:
                '10:55 - 11:40',

              subject:
                'الكيمياء العضوية',

              teacher:
                'أ. عبدالعزيز الغامدي',

              room:
                'مختبر الكيمياء',
            },
          ].map(
            item => (

              <Card
                key={
                  item.slot
                }

                variant="elevated"

                style={
                  styles.slotCard
                }>

                <View
                  style={
                    styles.slotRow
                  }>

                  {/* ─────────────────────────────────────────
                      Time
                    ───────────────────────────────────────── */}

                  <View
                    style={
                      styles.timeCol
                    }>

                    <Text
                      style={[
                        styles.timeText,

                        {
                          color:
                            theme.text,
                        },
                      ]}>

                      {
                        item.time
                      }

                    </Text>


                    <Badge
                      label={
                        `الحصة ${item.slot}`
                      }

                      variant="neutral"

                      size="sm"
                    />

                  </View>


                  {/* ─────────────────────────────────────────
                      Details
                    ───────────────────────────────────────── */}

                  <View
                    style={
                      styles.detailsCol
                    }>

                    <Text
                      style={[
                        styles.subjectName,

                        {
                          color:
                            theme.text,
                        },
                      ]}>

                      {
                        item.subject
                      }

                    </Text>


                    <Text
                      style={[
                        styles.metaText,

                        {
                          color:
                            theme.textSecondary,
                        },
                      ]}>

                      {
                        item.teacher
                      }

                      {' • '}

                      {
                        item.room
                      }

                    </Text>

                  </View>

                </View>

              </Card>
            ),
          )}

        </View>

      </ScrollView>

    </ScreenBackground>
  );
}


// ─────────────────────────────────────────────────────────────────────────
// Styles
// ─────────────────────────────────────────────────────────────────────────

const styles = StyleSheet.create({

  // ───────────────────────────────────────────────────────────────────────
  // Screen
  // ───────────────────────────────────────────────────────────────────────

  scrollView: {
    flex:
      1,

    width:
      '100%',
  },


  scrollContent: {
    width:
      '100%',

    paddingHorizontal:
      Spacing.four,

    paddingTop:
      Spacing.four,

    gap:
      Spacing.four,
  },


  // ───────────────────────────────────────────────────────────────────────
  // Day Selector
  // ───────────────────────────────────────────────────────────────────────

  dayTabBar: {
    width:
      '100%',

    flexDirection:
      'row',

    padding:
      Spacing.two,

    gap:
      Spacing.one,

    borderWidth:
      1,

    borderRadius:
      Radii.xl,
  },


  dayTab: {
    flex:
      1,

    minHeight:
      40,

    paddingHorizontal:
      Spacing.one,

    paddingVertical:
      Spacing.two,

    borderRadius:
      Radii.md,

    alignItems:
      'center',

    justifyContent:
      'center',
  },


  dayTabText: {
    ...Typography.captionBold,

    textAlign:
      'center',
  },


  // ───────────────────────────────────────────────────────────────────────
  // Schedule
  // ───────────────────────────────────────────────────────────────────────

  scheduleList: {
    width:
      '100%',

    gap:
      Spacing.three,
  },


  slotCard: {
    width:
      '100%',

    padding:
      Spacing.four,
  },


  slotRow: {
    width:
      '100%',

    flexDirection:
      'row',

    alignItems:
      'center',

    gap:
      Spacing.three,
  },


  // ───────────────────────────────────────────────────────────────────────
  // Time
  // ───────────────────────────────────────────────────────────────────────

  timeCol: {
    width:
      100,

    flexShrink:
      0,

    gap:
      4,
  },


  timeText: {
    ...Typography.small,

    fontWeight:
      '600',
  },


  // ───────────────────────────────────────────────────────────────────────
  // Details
  // ───────────────────────────────────────────────────────────────────────

  detailsCol: {
    flex:
      1,

    minWidth:
      0,

    gap:
      2,
  },


  subjectName: {
    ...Typography.bodyBold,
  },


  metaText: {
    ...Typography.small,
  },
});