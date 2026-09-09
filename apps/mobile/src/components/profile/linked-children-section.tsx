/**
 * @module @orbis/mobile/components/profile/linked-children-section
 *
 * Section presenting linked children cards for guardian/parent accounts.
 */

import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { Card } from '@/components/ui/card';
import { Avatar } from '@/components/ui/avatar';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { Spacing, Typography } from '@/constants/theme';

export interface LinkedChildItem {
  id: string;
  name: string;
  className?: string;
  gradeLevel?: string;
}

interface LinkedChildrenSectionProps {
  childrenList: LinkedChildItem[];
  selectedChildId: string | null;
  onSelectChild: (id: string) => void;
}

export const LinkedChildrenSection = React.memo<LinkedChildrenSectionProps>(
  ({ childrenList, selectedChildId, onSelectChild }) => {
    const theme = useTheme();
    const { t } = useI18n();

    if (childrenList.length === 0) return null;

    return (
      <View style={styles.section}>
        <View style={styles.header}>
          <Text style={[styles.title, { color: theme.text }]}>
            {t('profile_linked_children')}
          </Text>
        </View>

        <View style={styles.grid}>
          {childrenList.map((child) => {
            const isSelected = child.id === selectedChildId;
            return (
              <Card
                key={child.id}
                onPress={() => onSelectChild(child.id)}
                variant={isSelected ? 'elevated' : 'flat'}
                style={[
                  styles.card,
                  isSelected && { borderColor: theme.primary, borderWidth: 1.5 },
                ]}>
                <View style={styles.row}>
                  <Avatar name={child.name} size={44} />

                  <View style={styles.info}>
                    <Text style={[styles.name, { color: theme.text }]}>{child.name}</Text>
                    <Text style={[styles.className, { color: theme.textSecondary }]}>
                      {child.className || child.gradeLevel}
                    </Text>
                  </View>

                  {isSelected && (
                    <Ionicons name="checkmark-circle" size={22} color={theme.primary} />
                  )}
                </View>
              </Card>
            );
          })}
        </View>
      </View>
    );
  },
);

LinkedChildrenSection.displayName = 'LinkedChildrenSection';

const styles = StyleSheet.create({
  section: {
    gap: Spacing.two,
  },
  header: {
    paddingHorizontal: Spacing.one,
  },
  title: {
    ...Typography.h3,
    fontSize: 16,
  },
  grid: {
    gap: Spacing.two,
  },
  card: {
    padding: Spacing.three,
  },
  row: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.three,
  },
  info: {
    flex: 1,
    gap: 2,
  },
  name: {
    ...Typography.bodyBold,
    fontSize: 15,
  },
  className: {
    ...Typography.small,
    fontSize: 12,
  },
});
