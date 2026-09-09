/**
 * @module @orbis/mobile/components/home/student-bubble-cluster
 *
 * Staggered 5-bubble cluster presenting students, classes, or academic domains.
 * Designed with Glovo's signature organic hand-drawn pebble curvature.
 */

import React from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import Svg, { Path } from 'react-native-svg';
import { SkeletonBubble } from '@/components/ui/skeleton';
import { Avatar } from '@/components/ui/avatar';
import { useTheme } from '@/hooks/use-theme';
import { Radii, Typography } from '@/constants/theme';

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

interface BubbleSurfaceProps {
  size: number;
  variantIndex: number;
  children: React.ReactNode;
}

const BUBBLE_PATHS = [
  'M 50,3 C 77,1 98,22 97,50 C 96,78 77,97 50,97 C 22,97 3,78 4,50 C 5,23 23,5 50,3 Z',
  'M 47,4 C 75,2 97,24 96,52 C 95,79 74,97 47,96 C 20,95 3,75 4,48 C 5,21 20,5 47,4 Z',
  'M 53,4 C 79,3 97,23 96,49 C 96,76 76,97 49,96 C 22,96 3,75 4,49 C 5,23 27,7 53,4 Z',
  'M 50,5 C 76,2 98,23 96,52 C 94,79 76,97 49,96 C 22,95 3,77 4,50 C 5,24 25,8 50,5 Z',
  'M 49,3 C 76,2 97,22 97,49 C 98,76 76,97 49,96 C 22,95 3,75 4,48 C 5,21 23,5 49,3 Z',
];

const BUBBLE_TILTS = [-1.5, 1.5, 0.8, -1, -2];

const BubbleSurface = React.memo<BubbleSurfaceProps>(({ size, variantIndex, children }) => {
  const theme = useTheme();
  const path = BUBBLE_PATHS[variantIndex % BUBBLE_PATHS.length];
  const tilt = BUBBLE_TILTS[variantIndex % BUBBLE_TILTS.length];

  return (
    <View
      style={[
        styles.surfaceHalo,
        {
          width: size + 18,
          height: size + 18,
          transform: [{ rotate: `${tilt}deg` }],
        },
      ]}
    >
      <Svg width={size + 18} height={size + 18} viewBox="0 0 100 100">
        <Path d={path} fill={theme.primaryLight} />
      </Svg>
      <View style={[styles.surface, { width: size, height: size, transform: [{ rotate: `${-tilt}deg` }] }]}> 
        <Svg pointerEvents="none" width={size} height={size} viewBox="0 0 100 100">
          <Path
            d={path}
            fill={theme.card}
            stroke={theme.brandPurpleLight}
            strokeWidth="2.25"
            strokeOpacity="0.9"
          />
        </Svg>
        <View style={styles.surfaceContent}>{children}</View>
      </View>
    </View>
  );
});

BubbleSurface.displayName = 'BubbleSurface';

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
                  <BubbleSurface size={132} variantIndex={idx}>
                    {item.type === 'child' ? (
                      <Avatar name={item.title} imageUrl={item.avatarUrl} size={82} />
                    ) : (
                      <ItemIllustration size={76} />
                    )}
                  </BubbleSurface>
                  <View
                    style={[
                      styles.pill,
                      {
                        backgroundColor: theme.card,
                        borderColor: theme.brandPurpleLight,
                        shadowColor: theme.foreground,
                      },
                    ]}>
                    <Text style={[styles.pillText, { color: theme.text }]} numberOfLines={1}>
                      {item.title}
                    </Text>
                  </View>
                </View>
              </Pressable>
            );
          })}
        </View>

        {/* Three smaller bubbles sit on a shallow arc below the top row. */}
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
                    <BubbleSurface size={104} variantIndex={globalIdx}>
                      {item.type === 'child' ? (
                        <Avatar name={item.title} imageUrl={item.avatarUrl} size={64} />
                      ) : (
                        <ItemIllustration size={62} />
                      )}
                    </BubbleSurface>
                    <View
                      style={[
                        styles.pill,
                        {
                          backgroundColor: theme.card,
                          borderColor: theme.brandPurpleLight,
                          shadowColor: theme.foreground,
                        },
                      ]}>
                      <Text style={[styles.pillText, { color: theme.text }]} numberOfLines={1}>
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
    alignSelf: 'stretch',
    gap: 22,
    paddingVertical: 18,
  },
  rowTop: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    gap: 16,
  },
  rowBottom: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    justifyContent: 'center',
    width: '100%',
    gap: 4,
    marginTop: -4,
    paddingHorizontal: 8,
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
    transform: [{ translateY: 8 }],
  },
  ringWrapperLarge: {
    width: 140,
    height: 140,
    alignItems: 'center',
    justifyContent: 'center',
    position: 'relative',
  },
  ringWrapperSmall: {
    flex: 1,
    maxWidth: 118,
    height: 112,
    alignItems: 'center',
    justifyContent: 'center',
    position: 'relative',
  },
  surfaceHalo: {
    alignItems: 'center',
    justifyContent: 'center',
    flexShrink: 0,
    position: 'relative',
  },
  surface: {
    alignItems: 'center',
    justifyContent: 'center',
    position: 'absolute',
    flexShrink: 0,
    overflow: 'hidden',
  },
  surfaceContent: {
    position: 'absolute',
    alignItems: 'center',
    justifyContent: 'center',
    width: '100%',
    height: '100%',
  },
  pill: {
    position: 'absolute',
    bottom: -12,
    paddingVertical: 5,
    paddingHorizontal: 13,
    borderRadius: Radii.full,
    borderWidth: 2,
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.14,
    shadowRadius: 2,
    elevation: 2,
    maxWidth: 150,
  },
  pillText: {
    ...Typography.small,
    fontWeight: '500',
    fontSize: 15,
    textAlign: 'center',
  },
});
