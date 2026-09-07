/**
 * @module @orbis/mobile/components/home/student-bubble-cluster
 *
 * Staggered 5-bubble cluster presenting students, classes, or academic domains.
 * Designed with Glovo's signature organic hand-drawn pebble curvature.
 */

import React from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import Svg, { Path } from 'react-native-svg';
import { OrganicBubbleRing } from '@/components/glovo-illustrations';
import { SkeletonBubble } from '@/components/ui/skeleton';
import { Avatar } from '@/components/ui/avatar';
import { useTheme } from '@/hooks/use-theme';
import { Radii, Spacing, Typography } from '@/constants/theme';

export interface BubbleItemData {
  id: string;
  title: string;
  sub?: string;
  avatarUrl?: string | null;
  type: 'child' | 'class' | 'student_domain';
  route?: string;
  Illustration: React.ComponentType<{ size: number }>;
}

interface StudentBubbleClusterProps {
  items: BubbleItemData[];
  isLoading?: boolean;
  onItemPress: (item: BubbleItemData) => void;
}

// 5 unique organic hand-drawn pebble contours for a lively, non-perfect circle aesthetic
const PEBBLE_PATHS = [
  'M 50,4 C 77,2 96,22 97,50 C 98,77 77,96 50,97 C 23,98 3,77 4,50 C 5,23 23,6 50,4 Z',
  'M 48,3 C 76,4 97,25 96,52 C 95,78 74,97 48,96 C 22,95 4,76 3,48 C 2,21 20,2 48,3 Z',
  'M 52,5 C 77,3 96,23 96,49 C 96,75 76,96 49,96 C 23,96 4,75 4,49 C 4,23 27,7 52,5 Z',
  'M 50,6 C 75,3 97,23 96,51 C 95,78 77,96 50,97 C 23,98 3,77 4,50 C 5,23 25,9 50,6 Z',
  'M 49,3 C 75,2 96,22 97,49 C 98,76 76,97 49,96 C 22,95 3,75 4,48 C 5,21 23,4 49,3 Z',
];

const PEBBLE_TILTS = [-2, 2.5, 1.5, -1, -3];

interface OrganicPebbleProps {
  size: number;
  variantIndex: number;
  children: React.ReactNode;
}

const OrganicPebble = React.memo<OrganicPebbleProps>(({ size, variantIndex, children }) => {
  const theme = useTheme();
  const pathD = PEBBLE_PATHS[variantIndex % PEBBLE_PATHS.length];
  const tilt = PEBBLE_TILTS[variantIndex % PEBBLE_TILTS.length];

  return (
    <View
      style={[
        styles.pebbleContainer,
        {
          width: size,
          height: size,
          transform: [{ rotate: `${tilt}deg` }],
        },
      ]}>
      {/* Organic Hand-drawn Pebble Background */}
      <View pointerEvents="none" style={[StyleSheet.absoluteFill, { width: size, height: size }]}>
        <Svg width={size} height={size} viewBox="0 0 100 100">
          <Path d={pathD} fill={theme.card} />
        </Svg>
      </View>

      {/* Centered Content with Counter-Rotation for Visual Stability */}
      <View
        style={[
          styles.pebbleInner,
          {
            transform: [{ rotate: `${-tilt}deg` }],
          },
        ]}>
        {children}
      </View>
    </View>
  );
});

OrganicPebble.displayName = 'OrganicPebble';

export const StudentBubbleCluster = React.memo<StudentBubbleClusterProps>(
  ({ items, isLoading = false, onItemPress }) => {
    const theme = useTheme();

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
        {/* Top Row: 2 Large Organic Bubbles */}
        <View style={styles.rowTop}>
          {topItems.map((item, idx) => {
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
                  <OrganicBubbleRing size={118} variantIndex={idx} />
                  <OrganicPebble size={96} variantIndex={idx}>
                    {item.type === 'child' ? (
                      <Avatar name={item.title} imageUrl={item.avatarUrl} size={68} />
                    ) : (
                      <ItemIllustration size={68} />
                    )}
                  </OrganicPebble>
                  <View
                    style={[
                      styles.pill,
                      {
                        backgroundColor: theme.card,
                        borderColor: theme.border,
                      },
                    ]}>
                    <Text
                      style={[styles.pillText, { color: theme.text }]}
                      numberOfLines={1}>
                      {item.title}
                    </Text>
                  </View>
                </View>
              </Pressable>
            );
          })}
        </View>

        {/* Bottom Row: 3 Small Organic Bubbles in an Arc */}
        {bottomItems.length > 0 && (
          <View style={styles.rowBottom}>
            {bottomItems.map((item, idx) => {
              const ItemIllustration = item.Illustration;
              const globalIdx = idx + 2;
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
                    <OrganicBubbleRing size={100} variantIndex={globalIdx} />
                    <OrganicPebble size={80} variantIndex={globalIdx}>
                      {item.type === 'child' ? (
                        <Avatar name={item.title} imageUrl={item.avatarUrl} size={56} />
                      ) : (
                        <ItemIllustration size={56} />
                      )}
                    </OrganicPebble>
                    <View
                      style={[
                        styles.pill,
                        {
                          backgroundColor: theme.card,
                          borderColor: theme.border,
                        },
                      ]}>
                      <Text
                        style={[styles.pillText, { color: theme.text }]}
                        numberOfLines={1}>
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
  pebbleContainer: {
    alignItems: 'center',
    justifyContent: 'center',
    shadowColor: '#000000',
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.12,
    shadowRadius: 10,
    elevation: 4,
  },
  pebbleInner: {
    alignItems: 'center',
    justifyContent: 'center',
  },
  pill: {
    position: 'absolute',
    bottom: -10,
    paddingVertical: 3,
    paddingHorizontal: 12,
    borderRadius: Radii.full,
    borderWidth: 1,
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
    textAlign: 'center',
  },
});
