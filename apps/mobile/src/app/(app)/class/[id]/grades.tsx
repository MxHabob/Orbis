/**
 * @module @orbis/mobile/app/(app)/class/[id]/grades
 *
 * Class Gradebook & Score Entry Screen for Teachers.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, TextInput, Alert, Pressable } from 'react-native';
import { useLocalSearchParams } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Radii, Spacing, Typography } from '@/constants/theme';

interface StudentGradeRow {
  id: string;
  name: string;
  studentNumber: string;
  score: string;
  notes?: string;
  isExcused?: boolean;
}

export default function ClassGradesScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { id } = useLocalSearchParams<{ id: string }>();

  const [activeScheme, setActiveScheme] = useState<'midterm' | 'coursework' | 'practical'>('midterm');
  const [saving, setSaving] = useState(false);

  const [grades, setGrades] = useState<StudentGradeRow[]>([
    {
      id: 'std-1',
      name: 'عمر خالد المنصور',
      studentNumber: 'STD-081',
      score: '29',
    },
    {
      id: 'std-2',
      name: 'سلطان عبدالعزيز الراشد',
      studentNumber: 'STD-082',
      score: '30',
    },
    {
      id: 'std-3',
      name: 'فيصل محمد العتيبي',
      studentNumber: 'STD-083',
      score: '26',
    },
    {
      id: 'std-4',
      name: 'عبدالله ناصر القحطاني',
      studentNumber: 'STD-084',
      score: '24',
    },
    {
      id: 'std-5',
      name: 'تركي سعود الشمري',
      studentNumber: 'STD-085',
      score: '28',
    },
  ]);

  const updateScore = (id: string, score: string) => {
    setGrades((prev) => prev.map((g) => (g.id === id ? { ...g, score } : g)));
  };

  const handleSave = () => {
    setSaving(true);
    setTimeout(() => {
      setSaving(false);
      Alert.alert('تم حفظ الدرجات', 'تم تسجيل درجات التقييم بنجاح وتحديث السجلات الأكاديمية للطلاب.');
    }, 700);
  };

  const maxScore = activeScheme === 'midterm' ? 30 : activeScheme === 'coursework' ? 20 : 10;

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header
        title="دفتر رصد الدرجات"
        subtitle="الصف 3 علمي - أ • الرياضيات"
        showBack
      />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.eight },
        ]}
        showsVerticalScrollIndicator={false}>
        {/* Assessment Scheme Selector */}
        <View style={styles.schemeSelector}>
          {[
            { key: 'midterm', label: 'الاختبار النصفي (30 درجة)' },
            { key: 'coursework', label: 'الأعمال والمشاريع (20 درجة)' },
            { key: 'practical', label: 'التقييم العملي (10 درجات)' },
          ].map((s) => {
            const isSelected = activeScheme === s.key;
            return (
              <Pressable
                key={s.key}
                onPress={() => setActiveScheme(s.key as any)}
                style={[
                  styles.schemeTab,
                  {
                    backgroundColor: isSelected ? theme.primary : theme.card,
                    borderColor: isSelected ? theme.primary : theme.border,
                  },
                ]}>
                <Text
                  style={[
                    styles.schemeText,
                    { color: isSelected ? '#FFFFFF' : theme.textSecondary },
                  ]}>
                  {s.label}
                </Text>
              </Pressable>
            );
          })}
        </View>

        {/* Student Score List */}
        <View style={styles.sectionHeader}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>درجات الطلاب</Text>
          <Text style={[styles.sectionSub, { color: theme.textSecondary }]}>
            الدرجة العظمى: {maxScore}
          </Text>
        </View>

        <View style={styles.gradesList}>
          {grades.map((item, idx) => (
            <Card key={item.id} variant="elevated" style={styles.gradeCard}>
              <View style={styles.gradeRow}>
                <View style={styles.stdCol}>
                  <Text style={[styles.stdName, { color: theme.text }]}>
                    {idx + 1}. {item.name}
                  </Text>
                  <Text style={[styles.stdNum, { color: theme.textSecondary }]}>
                    {item.studentNumber}
                  </Text>
                </View>

                <View style={styles.scoreInputBox}>
                  <TextInput
                    value={item.score}
                    onChangeText={(val) => updateScore(item.id, val)}
                    keyboardType="numeric"
                    maxLength={3}
                    style={[
                      styles.scoreInput,
                      {
                        backgroundColor: theme.backgroundElement,
                        color: theme.text,
                        borderColor: theme.border,
                      },
                    ]}
                  />
                  <Text style={[styles.maxScoreLbl, { color: theme.textSecondary }]}>
                    / {maxScore}
                  </Text>
                </View>
              </View>
            </Card>
          ))}
        </View>

        {/* Save Button */}
        <Button
          title="حفظ واعتماد الدرجات"
          onPress={handleSave}
          loading={saving}
          size="lg"
          icon={<Ionicons name="cloud-upload-outline" size={20} color="#FFFFFF" />}
          fullWidth
          style={{ marginTop: Spacing.two }}
        />
      </ScrollView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.four,
    gap: Spacing.four,
  },
  schemeSelector: {
    gap: Spacing.two,
  },
  schemeTab: {
    paddingVertical: Spacing.three,
    paddingHorizontal: Spacing.four,
    borderRadius: Radii.md,
    borderWidth: 1,
    alignItems: 'center',
  },
  schemeText: {
    ...Typography.captionBold,
  },
  sectionHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginTop: Spacing.two,
  },
  sectionTitle: {
    ...Typography.h3,
  },
  sectionSub: {
    ...Typography.captionBold,
  },
  gradesList: {
    gap: Spacing.two,
  },
  gradeCard: {
    padding: Spacing.three,
  },
  gradeRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  stdCol: {
    flex: 1,
    gap: 2,
  },
  stdName: {
    ...Typography.bodyBold,
  },
  stdNum: {
    ...Typography.small,
  },
  scoreInputBox: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 6,
  },
  scoreInput: {
    width: 60,
    height: 44,
    borderRadius: Radii.md,
    borderWidth: 1.5,
    textAlign: 'center',
    fontSize: 18,
    fontWeight: '700',
  },
  maxScoreLbl: {
    ...Typography.captionBold,
  },
});
