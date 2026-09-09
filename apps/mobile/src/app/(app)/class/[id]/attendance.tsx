/**
 * @module @orbis/mobile/app/(app)/class/[id]/attendance
 *
 * Fast Class Attendance Sheet for Teachers.
 *
 * - Full-width screen gradient.
 * - Gradient extends behind StatusBar.
 * - Header remains transparent.
 * - Fast one-tap attendance controls.
 * - Bulk attendance action.
 * - Theme-aware UI.
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
  Alert,
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
  Button,
} from '@/components/ui/button';

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
// Types
// ─────────────────────────────────────────────────────────────────────────

type AttendanceStatus =
  | 'Present'
  | 'Late'
  | 'Excused'
  | 'Absent';


interface StudentAttendanceEntry {
  studentId: string;

  studentName: string;

  studentNumber: string;

  status: AttendanceStatus;
}


// ─────────────────────────────────────────────────────────────────────────
// Screen
// ─────────────────────────────────────────────────────────────────────────

export default function ClassAttendanceScreen() {

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


  // ───────────────────────────────────────────────────────────────────────
  // State
  // ───────────────────────────────────────────────────────────────────────

  const [
    saving,
    setSaving,
  ] =
    useState(false);


  const [
    entries,
    setEntries,
  ] =
    useState<StudentAttendanceEntry[]>([
      {
        studentId:
          'std-1',

        studentName:
          'عمر خالد المنصور',

        studentNumber:
          'STD-081',

        status:
          'Present',
      },

      {
        studentId:
          'std-2',

        studentName:
          'سلطان عبدالعزيز الراشد',

        studentNumber:
          'STD-082',

        status:
          'Present',
      },

      {
        studentId:
          'std-3',

        studentName:
          'فيصل محمد العتيبي',

        studentNumber:
          'STD-083',

        status:
          'Present',
      },

      {
        studentId:
          'std-4',

        studentName:
          'عبدالله ناصر القحطاني',

        studentNumber:
          'STD-084',

        status:
          'Late',
      },

      {
        studentId:
          'std-5',

        studentName:
          'تركي سعود الشمري',

        studentNumber:
          'STD-085',

        status:
          'Absent',
      },
    ]);


  // ───────────────────────────────────────────────────────────────────────
  // Attendance Actions
  // ───────────────────────────────────────────────────────────────────────

  const updateStatus = (
    studentId: string,
    status: AttendanceStatus,
  ) => {

    setEntries(
      previous =>
        previous.map(
          entry =>
            entry.studentId === studentId
              ? {
                  ...entry,
                  status,
                }
              : entry,
        ),
    );
  };


  const markAllPresent = () => {

    setEntries(
      previous =>
        previous.map(
          entry => ({
            ...entry,
            status:
              'Present',
          }),
        ),
    );
  };


  const handleSave = () => {

    setSaving(
      true,
    );


    setTimeout(
      () => {

        setSaving(
          false,
        );


        Alert.alert(
          'تم الحفظ بنجاح',
          'تم تسجيل واعتماد كشف الحضور للحصة الحالية وإرسال التنبيهات تلقائياً.',
        );
      },
      800,
    );
  };


  // ───────────────────────────────────────────────────────────────────────
  // Statistics
  // ───────────────────────────────────────────────────────────────────────

  const presentCount =
    entries.filter(
      entry =>
        entry.status ===
        'Present',
    ).length;


  const lateCount =
    entries.filter(
      entry =>
        entry.status ===
        'Late',
    ).length;


  const absentCount =
    entries.filter(
      entry =>
        entry.status ===
          'Absent' ||
        entry.status ===
          'Excused',
    ).length;


  // ───────────────────────────────────────────────────────────────────────
  // Render
  // ───────────────────────────────────────────────────────────────────────

  return (
    <ScreenBackground
      gradientHeight={
        0.48
      }>

      {/* ─────────────────────────────────────────────────────────────
          System Status Bar
        ───────────────────────────────────────────────────────────── */}

      <ThemedStatusBar />


      {/* ─────────────────────────────────────────────────────────────
          Header

          Header has NO gradient of its own.
          The screen background is responsible for the gradient.
        ───────────────────────────────────────────────────────────── */}

      <Header
        title="رصد حضور الحصة"

        subtitle="الصف 3 علمي - أ • الحصة الأولى"

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
            Attendance Summary
          ───────────────────────────────────────────────────────── */}

        <Card
          variant="elevated"
          style={
            styles.topCard
          }>

          <View
            style={
              styles.topRow
            }>

            <View
              style={
                styles.lessonDetails
              }>

              <Text
                style={[
                  styles.lessonDate,
                  {
                    color:
                      theme.text,
                  },
                ]}>

                اليوم، 7 سبتمبر 2026

              </Text>


              <Text
                style={[
                  styles.lessonInfo,
                  {
                    color:
                      theme.textSecondary,
                  },
                ]}>

                الرياضيات المتقدمة • القاعة 104

              </Text>

            </View>


            <Button
              title="تحضير الكل كـ حاضر"

              onPress={
                markAllPresent
              }

              variant="outline"

              size="sm"

              icon={
                <Ionicons
                  name="checkmark-done"
                  size={16}
                  color={
                    theme.primary
                  }
                />
              }
            />

          </View>


          {/* ─────────────────────────────────────────────────────
              Statistics
            ───────────────────────────────────────────────────── */}

          <View
            style={[
              styles.statsRow,

              {
                borderTopColor:
                  theme.borderSubtle,
              },
            ]}>

            <Badge
              label={
                `حاضر: ${presentCount}`
              }

              variant="success"

              size="md"
            />


            <Badge
              label={
                `متأخر: ${lateCount}`
              }

              variant="warning"

              size="md"
            />


            <Badge
              label={
                `غياب: ${absentCount}`
              }

              variant="error"

              size="md"
            />

          </View>

        </Card>


        {/* ─────────────────────────────────────────────────────────
            Section Header
          ───────────────────────────────────────────────────────── */}

        <View
          style={
            styles.sectionHeader
          }>

          <Text
            style={[
              styles.sectionTitle,
              {
                color:
                  theme.text,
              },
            ]}>

            كشف الطلاب ({entries.length})

          </Text>

        </View>


        {/* ─────────────────────────────────────────────────────────
            Students
          ───────────────────────────────────────────────────────── */}

        <View
          style={
            styles.entriesList
          }>

          {entries.map(
            (
              entry,
              index,
            ) => {

              return (
                <Card
                  key={
                    entry.studentId
                  }

                  variant="flat"

                  style={
                    styles.entryCard
                  }>

                  {/* ───────────────────────────────────────────
                      Student Information
                    ─────────────────────────────────────────── */}

                  <View
                    style={
                      styles.entryTop
                    }>

                    <Text
                      style={[
                        styles.entryNum,
                        {
                          color:
                            theme.textSecondary,
                        },
                      ]}>

                      {index + 1}.

                    </Text>


                    <View
                      style={
                        styles.studentInfo
                      }>

                      <Text
                        style={[
                          styles.entryName,
                          {
                            color:
                              theme.text,
                          },
                        ]}>

                        {
                          entry.studentName
                        }

                      </Text>


                      <Text
                        style={[
                          styles.entrySub,
                          {
                            color:
                              theme.textSecondary,
                          },
                        ]}>

                        {
                          entry.studentNumber
                        }

                      </Text>

                    </View>

                  </View>


                  {/* ───────────────────────────────────────────
                      Status Selector
                    ─────────────────────────────────────────── */}

                  <View
                    style={
                      styles.statusButtonsRow
                    }>

                    {[
                      {
                        key:
                          'Present',

                        label:
                          'حاضر',

                        color:
                          theme.success,

                        bg:
                          theme.successLight,
                      },

                      {
                        key:
                          'Late',

                        label:
                          'متأخر',

                        color:
                          theme.warning,

                        bg:
                          theme.warningLight,
                      },

                      {
                        key:
                          'Excused',

                        label:
                          'بعذر',

                        color:
                          theme.info,

                        bg:
                          theme.infoLight,
                      },

                      {
                        key:
                          'Absent',

                        label:
                          'غائب',

                        color:
                          theme.error,

                        bg:
                          theme.errorLight,
                      },
                    ].map(
                      status => {

                        const isSelected =
                          entry.status ===
                          status.key;


                        return (
                          <Pressable
                            key={
                              status.key
                            }

                            onPress={() =>
                              updateStatus(
                                entry.studentId,
                                status.key as AttendanceStatus,
                              )
                            }

                            style={[
                              styles.statusBtn,

                              {
                                backgroundColor:
                                  isSelected
                                    ? status.color
                                    : theme.card,

                                borderColor:
                                  isSelected
                                    ? status.color
                                    : theme.border,
                              },
                            ]}>

                            <Text
                              style={[
                                styles.statusBtnText,

                                {
                                  color:
                                    isSelected
                                      ? theme.primaryForeground
                                      : theme.textSecondary,
                                },
                              ]}>

                              {
                                status.label
                              }

                            </Text>

                          </Pressable>
                        );
                      },
                    )}

                  </View>

                </Card>
              );
            },
          )}

        </View>


        {/* ─────────────────────────────────────────────────────────
            Submit
          ───────────────────────────────────────────────────────── */}

        <Button
          title="اعتماد وحفظ الكشف"

          onPress={
            handleSave
          }

          loading={
            saving
          }

          size="lg"

          icon={
            <Ionicons
              name="save-outline"
              size={20}
              color={
                theme.primaryForeground
              }
            />
          }

          fullWidth

          style={
            styles.submitButton
          }
        />

      </ScrollView>

    </ScreenBackground>
  );
}


// ─────────────────────────────────────────────────────────────────────────
// Styles
// ─────────────────────────────────────────────────────────────────────────

const styles = StyleSheet.create({

  // ───────────────────────────────────────────────────────────────────────
  // Scroll
  // ───────────────────────────────────────────────────────────────────────

  scrollView: {
    flex:
      1,

    width:
      '100%',
  },


  scrollContent: {
    paddingHorizontal:
      Spacing.four,

    paddingTop:
      Spacing.four,

    gap:
      Spacing.four,
  },


  // ───────────────────────────────────────────────────────────────────────
  // Top Card
  // ───────────────────────────────────────────────────────────────────────

  topCard: {
    padding:
      Spacing.four,

    gap:
      Spacing.three,
  },


  topRow: {
    flexDirection:
      'row',

    justifyContent:
      'space-between',

    alignItems:
      'center',

    gap:
      Spacing.three,
  },


  lessonDetails: {
    flex:
      1,

    minWidth:
      0,
  },


  lessonDate: {
    ...Typography.bodyBold,
  },


  lessonInfo: {
    ...Typography.small,

    marginTop:
      2,
  },


  // ───────────────────────────────────────────────────────────────────────
  // Statistics
  // ───────────────────────────────────────────────────────────────────────

  statsRow: {
    flexDirection:
      'row',

    gap:
      Spacing.two,

    borderTopWidth:
      1,

    paddingTop:
      Spacing.three,
  },


  // ───────────────────────────────────────────────────────────────────────
  // Section
  // ───────────────────────────────────────────────────────────────────────

  sectionHeader: {
    marginTop:
      Spacing.two,
  },


  sectionTitle: {
    ...Typography.h3,
  },


  // ───────────────────────────────────────────────────────────────────────
  // Students
  // ───────────────────────────────────────────────────────────────────────

  entriesList: {
    gap:
      Spacing.three,
  },


  entryCard: {
    padding:
      Spacing.four,

    gap:
      Spacing.three,
  },


  entryTop: {
    flexDirection:
      'row',

    alignItems:
      'center',

    gap:
      8,
  },


  entryNum: {
    ...Typography.bodyBold,

    width:
      24,

    textAlign:
      'center',
  },


  studentInfo: {
    flex:
      1,

    minWidth:
      0,
  },


  entryName: {
    ...Typography.bodyBold,
  },


  entrySub: {
    ...Typography.small,

    marginTop:
      2,
  },


  // ───────────────────────────────────────────────────────────────────────
  // Status Buttons
  // ───────────────────────────────────────────────────────────────────────

  statusButtonsRow: {
    flexDirection:
      'row',

    gap:
      Spacing.two,
  },


  statusBtn: {
    flex:
      1,

    minHeight:
      38,

    paddingVertical:
      8,

    borderRadius:
      Radii.md,

    borderWidth:
      1,

    alignItems:
      'center',

    justifyContent:
      'center',
  },


  statusBtnText: {
    ...Typography.captionBold,
  },


  // ───────────────────────────────────────────────────────────────────────
  // Submit
  // ───────────────────────────────────────────────────────────────────────

  submitButton: {
    marginTop:
      Spacing.two,
  },
});