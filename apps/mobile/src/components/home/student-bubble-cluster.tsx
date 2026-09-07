/**
 * @module @orbis/mobile/components/home/student-bubble-cluster
 *
 * Staggered 5-bubble cluster presenting students, classes, or academic domains.
 */

import React from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import { OrganicBubbleRing } from '@/components/glovo-illustrations';
import { SkeletonBubble } from '@/components/ui/skeleton';
import { Radii, Spacing, Typography } from '@/constants/theme';

export interface BubbleItemData {
  id: string;
  title: string;
  sub?: string;
  type: 'child' | 'class' | 'student_domain';
  route?: string;
  Illustration: React.ComponentType<{ size: number }>;
}

interface StudentBubbleClusterProps {
  items: BubbleItemData[];
  isLoading?: boolean;
  onItemPress: (item: BubbleItemData) => void;
}

export const StudentBubbleCluster = React.memo<StudentBubbleClusterProps>(
  ({ items, isLoading = false, onItemPress }) => {
    if (isLoading) {
      return (
        <View style={styles.cluster}>
          <View style={styles.rowTop}>
            <SkeletonBubble size={118} />
            <SkeletonBubble size={118} />
          </View>
          <View style={styles.rowBottom}>
            <SkeletonBubble size={100} />
            <SkeletonBubble size={100} />
            <SkeletonBubble size={100} />
          </View>
        </View>
      );
    }

    const topItems = items.slice(0, 2);
    const bottomItems = items.slice(2, 5);

    return (
      <View style={styles.cluster}>
        {/* Top Row: 2 Large Bubbles */}
        <View style={styles.rowTop}>
          {topItems.map((item) => {
            const ItemIllustration = item.Illustration;
            return (
              <Pressable
                key={item.id}
                onPress={() => onItemPress(item)}
                style={({ pressed }) => [
                  styles.itemLarge,
                  pressed && { transform: [{ scale: 0.94 }] },
                ]}>
                <View style={styles.ringWrapperLarge}>
                  <OrganicBubbleRing size={118} />
                  <View style={styles.circleLarge}>
                    <ItemIllustration size={68} />
                  </View>
                  <View style={styles.pill}>
                    <Text style={styles.pillText} numberOfLines={1}>
                      {item.title}
                    </Text>
                  </View>
                </View>
              </Pressable>
            );
          })}
        </View>

        {/* Bottom Row: 3 Small Bubbles in an Arc */}
        {bottomItems.length > 0 && (
          <View style={styles.rowBottom}>
            {bottomItems.map((item, idx) => {
              const ItemIllustration = item.Illustration;
              const isMiddle = idx === 1;
              return (
                <Pressable
                  key={item.id}
                  onPress={() => onItemPress(item)}
                  style={({ pressed }) => [
                    styles.itemSmall,
                    isMiddle && styles.arcMiddle,
                    pressed && {
                      transform: isMiddle
                        ? [{ translateY: 6 }, { scale: 0.94 }]
                        : [{ scale: 0.94 }],
                    },
                  ]}>
                  <View style={styles.ringWrapperSmall}>
                    <OrganicBubbleRing size={100} />
                    <View style={styles.circleSmall}>
                      <ItemIllustration size={56} />
                    </View>
                    <View style={styles.pill}>
                      <Text style={styles.pillText} numberOfLines={1}>
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
    );
  },
);

StudentBubbleCluster.displayName = 'StudentBubbleCluster';

const styles = StyleSheet.create({
  cluster: {
    alignItems: 'center',
    justifyContent: 'center',
    gap: Spacing.four,
    paddingVertical: Spacing.two,
  },
  rowTop: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    gap: Spacing.six,
  },
  rowBottom: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    justifyContent: 'center',
    gap: Spacing.four,
    marginTop: -Spacing.one,
  },
  itemLarge: {
    alignItems: 'center',
    justifyContent: 'center',
  },
  itemSmall: {
    alignItems: 'center',
    justifyContent: 'center',
  },
  arcMiddle: {
    transform: [{ translateY: 6 }],
  },
  ringWrapperLarge: {
    width: 118,
    height: 118,
    alignItems: 'center',
    justifyContent: 'center',
    position: 'relative',
  },
  ringWrapperSmall: {
    width: 100,
    height: 100,
    alignItems: 'center',
    justifyContent: 'center',
    position: 'relative',
  },
  circleLarge: {
    position: 'absolute',
    width: 96,
    height: 96,
    borderRadius: 48,
    backgroundColor: '#FFFFFF',
    alignItems: 'center',
    justifyContent: 'center',
    shadowColor: '#000000',
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.1,
    shadowRadius: 8,
    elevation: 4,
  },
  circleSmall: {
    position: 'absolute',
    width: 80,
    height: 80,
    borderRadius: 40,
    backgroundColor: '#FFFFFF',
    alignItems: 'center',
    justifyContent: 'center',
    shadowColor: '#000000',
    shadowOffset: { width: 0, height: 3 },
    shadowOpacity: 0.1,
    shadowRadius: 6,
    elevation: 3,
  },
  pill: {
    position: 'absolute',
    bottom: -10,
    backgroundColor: '#FFFFFF',
    paddingVertical: 3,
    paddingHorizontal: 12,
    borderRadius: Radii.full,
    borderWidth: 1,
    borderColor: '#F4F4F5',
    shadowColor: '#000000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.08,
    shadowRadius: 4,
    elevation: 2,
    maxWidth: 105,
  },
  pillText: {
    ...Typography.small,
    fontWeight: '700',
    fontSize: 12,
    color: '#18181B',
    textAlign: 'center',
  },
});
