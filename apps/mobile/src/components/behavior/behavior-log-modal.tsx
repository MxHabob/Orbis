/**
 * @module @orbis/mobile/components/behavior/behavior-log-modal
 *
 * Modal for educators to record student behavior observations and merits.
 */

import React, { useState } from 'react';
import {
  View,
  Text,
  StyleSheet,
  Modal,
  ScrollView,
  Pressable,
  KeyboardAvoidingView,
  Platform,
  TouchableWithoutFeedback,
  Keyboard,
} from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { Radii, Spacing, Typography } from '@/constants/theme';
import { BehaviorRecordItem } from './behavior-timeline-list';

interface BehaviorLogModalProps {
  visible: boolean;
  onClose: () => void;
  onSave: (record: BehaviorRecordItem) => void;
}

export const BehaviorLogModal = React.memo<BehaviorLogModalProps>(
  ({ visible, onClose, onSave }) => {
    const theme = useTheme();
    const { t } = useI18n();

    const [student, setStudent] = useState('');
    const [type, setType] = useState<'merit' | 'violation'>('merit');
    const [category, setCategory] = useState('');
    const [points, setPoints] = useState('2');
    const [desc, setDesc] = useState('');

    const handleSubmit = () => {
      if (!desc.trim()) return;
      const numPoints = parseInt(points, 10) || 1;
      const record: BehaviorRecordItem = {
        id: `rec-${Date.now()}`,
        type,
        category: category.trim() || t('behavior_modal_type_merit'),
        points: type === 'merit' ? Math.abs(numPoints) : -Math.abs(numPoints),
        description: desc.trim(),
        reportedBy: t('common_you_teacher'),
        date: t('common_now'),
      };
      onSave(record);
      setDesc('');
      setCategory('');
      onClose();
    };

    return (
      <Modal visible={visible} transparent animationType="slide">
        <KeyboardAvoidingView
          behavior={Platform.OS === 'ios' ? 'padding' : undefined}
          style={styles.overlay}>
          <TouchableWithoutFeedback onPress={Keyboard.dismiss} accessible={false}>
            <View style={styles.overlayTouchable}>
              <View style={[styles.container, { backgroundColor: theme.card }]}>
                {/* Header */}
                <View style={styles.header}>
                  <Text style={[styles.title, { color: theme.text }]}>
                    {t('behavior_modal_title')}
                  </Text>
                  <Pressable onPress={onClose} hitSlop={8}>
                    <Ionicons name="close" size={24} color={theme.textSecondary} />
                  </Pressable>
                </View>

                {/* Form Fields */}
                <ScrollView
                  style={{ maxHeight: 420 }}
                  keyboardShouldPersistTaps="handled"
                  showsVerticalScrollIndicator={false}>
                  <View style={styles.fields}>
                    <Input
                      label={t('behavior_modal_student_label')}
                      placeholder={t('behavior_modal_student_placeholder')}
                      value={student}
                      onChangeText={setStudent}
                    />

                    <View style={{ gap: 6 }}>
                      <Text style={[Typography.captionBold, { color: theme.text }]}>
                        {t('behavior_modal_type_label')}
                      </Text>
                      <View style={{ flexDirection: 'row', gap: Spacing.two }}>
                        <Button
                          title={t('behavior_modal_type_merit')}
                          onPress={() => setNewTypeSafely('merit')}
                          variant={type === 'merit' ? 'primary' : 'outline'}
                          size="sm"
                          style={{ flex: 1 }}
                        />
                        <Button
                          title={t('behavior_modal_type_violation')}
                          onPress={() => setNewTypeSafely('violation')}
                          variant={type === 'violation' ? 'danger' : 'outline'}
                          size="sm"
                          style={{ flex: 1 }}
                        />
                      </View>
                    </View>

                    <Input
                      label={t('behavior_modal_category_label')}
                      placeholder={t('behavior_modal_category_placeholder')}
                      value={category}
                      onChangeText={setCategory}
                    />

                    <Input
                      label={t('behavior_modal_points_label')}
                      placeholder="2"
                      keyboardType="numeric"
                      value={points}
                      onChangeText={setPoints}
                    />

                    <Input
                      label={t('behavior_modal_desc_label')}
                      placeholder={t('behavior_modal_desc_placeholder')}
                      multiline
                      numberOfLines={3}
                      value={desc}
                      onChangeText={setDesc}
                    />
                  </View>
                </ScrollView>

                {/* Footer */}
                <View style={styles.footer}>
                  <Button
                    title={t('behavior_modal_cancel')}
                    onPress={onClose}
                    variant="ghost"
                    style={{ flex: 1 }}
                  />
                  <Button
                    title={t('behavior_modal_submit')}
                    onPress={handleSubmit}
                    variant="primary"
                    style={{ flex: 2 }}
                    disabled={!desc.trim()}
                  />
                </View>
              </View>
            </View>
          </TouchableWithoutFeedback>
        </KeyboardAvoidingView>
      </Modal>
    );

    function setNewTypeSafely(nextType: 'merit' | 'violation') {
      setType(nextType);
    }
  },
);

BehaviorLogModal.displayName = 'BehaviorLogModal';

const styles = StyleSheet.create({
  overlay: {
    flex: 1,
    backgroundColor: 'rgba(0, 0, 0, 0.5)',
    justifyContent: 'flex-end',
  },
  overlayTouchable: {
    flex: 1,
    justifyContent: 'flex-end',
  },
  container: {
    borderTopLeftRadius: Radii.xl,
    borderTopRightRadius: Radii.xl,
    padding: Spacing.four,
    gap: Spacing.four,
  },
  header: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  title: {
    ...Typography.h3,
    fontSize: 18,
  },
  fields: {
    gap: Spacing.three,
    paddingBottom: Spacing.two,
  },
  footer: {
    flexDirection: 'row',
    gap: Spacing.two,
    paddingTop: Spacing.two,
  },
});
