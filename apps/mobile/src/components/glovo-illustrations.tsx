/**
 * @module @orbis/mobile/components/glovo-illustrations
 *
 * Pixel-perfect pure React Native illustrations and visual elements
 * for the Glovo-inspired Home Hub.
 * Built with 100% native views and vectors for maximum performance,
 * zero native binary linking issues, and universal cross-platform support.
 */

import React from 'react';
import { View, StyleSheet } from 'react-native';
import Svg, { Path } from 'react-native-svg';
import { Ionicons } from '@expo/vector-icons';
import { Radii } from '@/constants/theme';

export interface IllustrationProps {
  size?: number;
}

const RING_PATHS = [
  'M 50,2 C 79,-1 100,20 101,50 C 102,79 79,99 50,100 C 21,101 1,79 2,50 C 3,21 21,3 50,2 Z',
  'M 48,1 C 78,2 100,24 99,53 C 98,81 76,100 48,99 C 20,98 2,78 1,48 C 0,18 18,0 48,1 Z',
  'M 52,2 C 80,0 99,21 99,50 C 99,78 78,99 49,99 C 20,99 1,77 1,49 C 1,21 25,4 52,2 Z',
  'M 50,2 C 78,-1 100,21 99,52 C 98,81 79,100 50,101 C 21,102 1,79 2,50 C 3,21 23,5 50,2 Z',
  'M 49,0 C 78,-1 100,20 101,49 C 102,78 78,100 49,99 C 20,98 1,77 2,48 C 3,19 21,1 49,0 Z',
];

/**
 * Hand-drawn golden organic circle ring outline for Glovo-style bubbles
 */
export function OrganicBubbleRing({
  size = 118,
  variantIndex = 0,
}: {
  size?: number;
  variantIndex?: number;
}) {
  const pathD = RING_PATHS[variantIndex % RING_PATHS.length];

  return (
    <View style={{ width: size, height: size, position: 'absolute' }}>
      <Svg width={size} height={size} viewBox="0 0 102 102">
        <Path
          d={pathD}
          fill="none"
          stroke="rgba(255, 255, 255, 0.42)"
          strokeWidth="2.5"
          strokeDasharray="160 3"
        />
      </Svg>
    </View>
  );
}

/**
 * Student 1 / High School Senior (Cap & Books)
 */
export function StudentSeniorIllustration({ size = 68 }: IllustrationProps) {
  return (
    <View
      style={[
        styles.avatarContainer,
        {
          width: size,
          height: size,
          borderRadius: size / 2,
          backgroundColor: '#EDE9FE',
        },
      ]}>
      <Ionicons name="school" size={size * 0.55} color="#7C3AED" />
      <View style={styles.miniBadgeSenior}>
        <Ionicons name="star" size={10} color="#F59E0B" />
      </View>
    </View>
  );
}

/**
 * Student 2 / High School Sophomore (Books & Backpack)
 */
export function StudentSophomoreIllustration({ size = 68 }: IllustrationProps) {
  return (
    <View
      style={[
        styles.avatarContainer,
        {
          width: size,
          height: size,
          borderRadius: size / 2,
          backgroundColor: '#FCE7F3',
        },
      ]}>
      <Ionicons name="book" size={size * 0.55} color="#EC4899" />
      <View style={styles.miniBadgeSophomore}>
        <Ionicons name="sparkles" size={10} color="#EC4899" />
      </View>
    </View>
  );
}

/**
 * Student 3 / Middle School (Sport & Active)
 */
export function StudentMiddleIllustration({ size = 56 }: IllustrationProps) {
  return (
    <View
      style={[
        styles.avatarContainer,
        {
          width: size,
          height: size,
          borderRadius: size / 2,
          backgroundColor: '#E0E7FF',
        },
      ]}>
      <Ionicons name="person" size={size * 0.55} color="#4F46E5" />
    </View>
  );
}

/**
 * Student 4 / Elementary School (Art & Creativity)
 */
export function StudentElementaryIllustration({ size = 56 }: IllustrationProps) {
  return (
    <View
      style={[
        styles.avatarContainer,
        {
          width: size,
          height: size,
          borderRadius: size / 2,
          backgroundColor: '#FEF3C7',
        },
      ]}>
      <Ionicons name="color-palette" size={size * 0.55} color="#D97706" />
    </View>
  );
}

/**
 * Student 5 / Kindergarten (Joy & Smiles)
 */
export function StudentKGIllustration({ size = 56 }: IllustrationProps) {
  return (
    <View
      style={[
        styles.avatarContainer,
        {
          width: size,
          height: size,
          borderRadius: size / 2,
          backgroundColor: '#CCFBF1',
        },
      ]}>
      <Ionicons name="happy" size={size * 0.55} color="#0D9488" />
    </View>
  );
}

/**
 * Organic curved wave transitioning between golden hero and white sheet
 */
export function WaveCurveDivider({
  width,
  height = 36,
  primaryColor = '#FFC244',
  sheetColor = '#FFFFFF',
}: {
  width: number;
  height?: number;
  primaryColor?: string;
  sheetColor?: string;
}) {
  return (
    <View style={[styles.waveContainer, { width, height, backgroundColor: primaryColor }]}>
      <View
        style={[
          styles.waveSheetArc,
          {
            width: width * 1.08,
            height: height * 2.2,
            backgroundColor: sheetColor,
          },
        ]}
      />
    </View>
  );
}

/**
 * Illustrated Feature Cards (Attendance, Grades, Behavior, Schedule)
 */
export function AttendanceFeatureIllustration({ size = 44 }: IllustrationProps) {
  return (
    <View
      style={[
        styles.featureBox,
        { width: size, height: size, backgroundColor: '#D1FAE5', borderRadius: Radii.lg },
      ]}>
      <Ionicons name="checkmark-done-circle" size={size * 0.65} color="#059669" />
    </View>
  );
}

export function GradesFeatureIllustration({ size = 44 }: IllustrationProps) {
  return (
    <View
      style={[
        styles.featureBox,
        { width: size, height: size, backgroundColor: '#EDE9FE', borderRadius: Radii.lg },
      ]}>
      <Ionicons name="ribbon" size={size * 0.65} color="#7C3AED" />
    </View>
  );
}

export function BehaviorFeatureIllustration({ size = 44 }: IllustrationProps) {
  return (
    <View
      style={[
        styles.featureBox,
        { width: size, height: size, backgroundColor: '#FEF3C7', borderRadius: Radii.lg },
      ]}>
      <Ionicons name="sparkles" size={size * 0.65} color="#D97706" />
    </View>
  );
}

export function ScheduleFeatureIllustration({ size = 44 }: IllustrationProps) {
  return (
    <View
      style={[
        styles.featureBox,
        { width: size, height: size, backgroundColor: '#CCFBF1', borderRadius: Radii.lg },
      ]}>
      <Ionicons name="calendar" size={size * 0.65} color="#0D9488" />
    </View>
  );
}

/**
 * PRIME Banner Right Side Mascot Artwork
 */
export function PrimeScooterMascot({ size = 78 }: IllustrationProps) {
  return (
    <View style={[styles.mascotContainer, { width: size, height: size }]}>
      {/* Royal Purple Backdrop Circle */}
      <View style={styles.mascotBackdrop} />
      {/* Scooter / Educational Delivery Icon */}
      <View style={styles.mascotIconBox}>
        <Ionicons name="bicycle" size={38} color="#FFC244" />
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  ringBase: {
    position: 'absolute',
    borderWidth: 3.5,
    borderColor: '#E5A100',
    backgroundColor: '#FFFFFF',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.12,
    shadowRadius: 8,
    elevation: 4,
  },
  avatarContainer: {
    alignItems: 'center',
    justifyContent: 'center',
    position: 'relative',
  },
  miniBadgeSenior: {
    position: 'absolute',
    top: 2,
    right: 2,
    backgroundColor: '#FEF3C7',
    borderRadius: 8,
    padding: 2,
    borderWidth: 1,
    borderColor: '#F59E0B',
  },
  miniBadgeSophomore: {
    position: 'absolute',
    top: 2,
    right: 2,
    backgroundColor: '#FFFFFF',
    borderRadius: 8,
    padding: 2,
    borderWidth: 1,
    borderColor: '#FCE7F3',
  },
  waveContainer: {
    overflow: 'hidden',
    alignItems: 'center',
    justifyContent: 'flex-end',
  },
  waveSheetArc: {
    borderTopLeftRadius: 40,
    borderTopRightRadius: 40,
    marginBottom: -4,
  },
  featureBox: {
    alignItems: 'center',
    justifyContent: 'center',
  },
  mascotContainer: {
    position: 'relative',
    alignItems: 'center',
    justifyContent: 'center',
  },
  mascotBackdrop: {
    position: 'absolute',
    right: -10,
    width: 64,
    height: 64,
    borderRadius: 32,
    backgroundColor: '#4338CA',
  },
  mascotIconBox: {
    position: 'absolute',
    right: 0,
    top: 14,
    transform: [{ rotate: '-6deg' }],
  },
});
