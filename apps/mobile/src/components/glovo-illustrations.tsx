/**
 * @module @orbis/mobile/components/glovo-illustrations
 *
 * Pixel-perfect pure React Native illustrations and visual elements
 * for the Glovo-inspired Home Hub.
 * Built with 100% native views and vectors for maximum performance,
 * zero native binary linking issues, and universal cross-platform support.
 */

import React from "react";
import { View, StyleSheet } from "react-native";
import Svg, {
  Circle,
  Defs,
  G,
  LinearGradient,
  Line,
  Path,
  Rect,
  Stop,
} from "react-native-svg";
import { useTheme } from "@/hooks/use-theme";

export interface IllustrationProps {
  size?: number;
}

const RING_PATHS = [
  "M 50,2 C 79,-1 100,20 101,50 C 102,79 79,99 50,100 C 21,101 1,79 2,50 C 3,21 21,3 50,2 Z",
  "M 48,1 C 78,2 100,24 99,53 C 98,81 76,100 48,99 C 20,98 2,78 1,48 C 0,18 18,0 48,1 Z",
  "M 52,2 C 80,0 99,21 99,50 C 99,78 78,99 49,99 C 20,99 1,77 1,49 C 1,21 25,4 52,2 Z",
  "M 50,2 C 78,-1 100,21 99,52 C 98,81 79,100 50,101 C 21,102 1,79 2,50 C 3,21 23,5 50,2 Z",
  "M 49,0 C 78,-1 100,20 101,49 C 102,78 78,100 49,99 C 20,98 1,77 2,48 C 3,19 21,1 49,0 Z",
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
    <View
      pointerEvents="none"
      style={{ width: size, height: size, position: "absolute" }}
    >
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
  return <StudentIllustration kind="senior" size={size} />;
}

/**
 * Student 2 / High School Sophomore (Books & Backpack)
 */
export function StudentSophomoreIllustration({ size = 68 }: IllustrationProps) {
  return <StudentIllustration kind="sophomore" size={size} />;
}

/**
 * Student 3 / Middle School (Sport & Active)
 */
export function StudentMiddleIllustration({ size = 56 }: IllustrationProps) {
  return <StudentIllustration kind="middle" size={size} />;
}

/**
 * Student 4 / Elementary School (Art & Creativity)
 */
export function StudentElementaryIllustration({
  size = 56,
}: IllustrationProps) {
  return <StudentIllustration kind="elementary" size={size} />;
}

/**
 * Student 5 / Kindergarten (Joy & Smiles)
 */
export function StudentKGIllustration({ size = 56 }: IllustrationProps) {
  return <StudentIllustration kind="kindergarten" size={size} />;
}

type StudentKind =
  | "senior"
  | "sophomore"
  | "middle"
  | "elementary"
  | "kindergarten";

function StudentIllustration({
  kind,
  size,
}: {
  kind: StudentKind;
  size: number;
}) {
  const theme = useTheme();
  const accent = theme.text;

  return (
    <Svg width={size} height={size} viewBox="0 0 64 64">
      <Circle cx="32" cy="32" r="27" fill={theme.backgroundElement} />
      <Circle
        cx="32"
        cy="27"
        r="9"
        fill={theme.card}
        stroke={accent}
        strokeWidth="2"
      />
      <Path d="M17 53c2-10 9-15 15-15s13 5 15 15" fill={accent} opacity={0.9} />
      {kind === "senior" && (
        <Path
          d="M19 18 32 12l13 6-13 6-13-6Zm13 6v6"
          fill="none"
          stroke={accent}
          strokeWidth="2.5"
          strokeLinejoin="round"
        />
      )}
      {kind === "sophomore" && (
        <Path
          d="M22 42h20v11H22zM22 42l10-5 10 5M27 42v11M37 42v11"
          fill="none"
          stroke={accent}
          strokeWidth="2"
          strokeLinejoin="round"
        />
      )}
      {kind === "middle" && (
        <Path
          d="M22 44c4 3 16 3 20 0M25 41l7 5 7-5"
          fill="none"
          stroke={accent}
          strokeWidth="2"
          strokeLinecap="round"
        />
      )}
      {kind === "elementary" && (
        <G>
          <Path
            d="M40 39c5 2 7 8 3 11-5 4-17-1-19-6-2-5 4-10 9-8l7 3Z"
            fill="none"
            stroke={accent}
            strokeWidth="2"
          />
          <Circle cx="29" cy="42" r="1.5" fill={accent} />
          <Circle cx="35" cy="45" r="1.5" fill={accent} />
        </G>
      )}
      {kind === "kindergarten" && (
        <G stroke={accent} strokeWidth="2" strokeLinecap="round">
          <Circle cx="28" cy="26" r="1" fill={accent} />
          <Circle cx="36" cy="26" r="1" fill={accent} />
          <Path d="M27 31c3 3 7 3 10 0" fill="none" />
        </G>
      )}
    </Svg>
  );
}

/**
 * Illustrated Feature Cards (Attendance, Grades, Behavior, Schedule)
 */
export function AttendanceFeatureIllustration({
  size = 44,
}: IllustrationProps) {
  const theme = useTheme();
  return (
    <Svg width={size} height={size} viewBox="0 0 44 44">
      <Defs>
        <LinearGradient id="attendance-fill" x1="0" y1="0" x2="1" y2="1">
          <Stop offset="0" stopColor={theme.backgroundElement} />
          <Stop offset="1" stopColor={theme.card} />
        </LinearGradient>
      </Defs>
      <Rect
        x="6"
        y="6"
        width="32"
        height="32"
        rx="11"
        fill="url(#attendance-fill)"
      />
      <Circle cx="22" cy="22" r="11" fill={theme.card} opacity="0.92" />
      <Path
        d="m15 22 5 5 10-11"
        fill="none"
        stroke={theme.text}
        strokeWidth="3"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <Circle cx="33" cy="10" r="2" fill={theme.textSecondary} />
    </Svg>
  );
}

export function GradesFeatureIllustration({ size = 44 }: IllustrationProps) {
  const theme = useTheme();
  return (
    <Svg width={size} height={size} viewBox="0 0 44 44">
      <Defs>
        <LinearGradient id="grades-fill" x1="0" y1="0" x2="1" y2="1">
          <Stop offset="0" stopColor={theme.foreground} />
          <Stop offset="1" stopColor={theme.textSecondary} />
        </LinearGradient>
      </Defs>
      <Rect
        x="8"
        y="6"
        width="28"
        height="32"
        rx="8"
        fill={theme.backgroundElement}
      />
      <Path
        d="M14 13h16M14 19h10M14 25h7"
        stroke={theme.textSecondary}
        strokeWidth="2"
        strokeLinecap="round"
      />
      <Circle cx="30" cy="29" r="7" fill="url(#grades-fill)" />
      <Path
        d="m27 29 2 2 4-5"
        fill="none"
        stroke={theme.card}
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <Path
        d="M14 34h8"
        stroke={theme.text}
        strokeWidth="2"
        strokeLinecap="round"
      />
    </Svg>
  );
}

export function BehaviorFeatureIllustration({ size = 44 }: IllustrationProps) {
  const theme = useTheme();
  return (
    <Svg width={size} height={size} viewBox="0 0 44 44">
      <Defs>
        <LinearGradient id="behavior-fill" x1="0" y1="0" x2="1" y2="1">
          <Stop offset="0" stopColor={theme.text} />
          <Stop offset="1" stopColor={theme.textSecondary} />
        </LinearGradient>
      </Defs>
      <Circle cx="22" cy="22" r="16" fill={theme.backgroundElement} />
      <Path
        d="m22 7 3 11 11 3-11 3-3 11-3-11-11-3 11-3 3-11Z"
        fill="url(#behavior-fill)"
      />
      <Circle cx="22" cy="21" r="3" fill={theme.card} opacity="0.9" />
      <Path
        d="m35 8 1 4 4 1-4 1-1 4-1-4-4-1 4-1 1-4Z"
        fill={theme.textSecondary}
      />
    </Svg>
  );
}

export function ScheduleFeatureIllustration({ size = 44 }: IllustrationProps) {
  const theme = useTheme();
  return (
    <Svg width={size} height={size} viewBox="0 0 44 44">
      <Defs>
        <LinearGradient id="schedule-fill" x1="0" y1="0" x2="1" y2="1">
          <Stop offset="0" stopColor={theme.backgroundElement} />
          <Stop offset="1" stopColor={theme.card} />
        </LinearGradient>
      </Defs>
      <Rect
        x="7"
        y="9"
        width="30"
        height="28"
        rx="8"
        fill="url(#schedule-fill)"
      />
      <Line
        x1="7"
        y1="18"
        x2="37"
        y2="18"
        stroke={theme.text}
        strokeWidth="2"
      />
      <Line
        x1="15"
        y1="6"
        x2="15"
        y2="12"
        stroke={theme.text}
        strokeWidth="3"
        strokeLinecap="round"
      />
      <Line
        x1="29"
        y1="6"
        x2="29"
        y2="12"
        stroke={theme.text}
        strokeWidth="3"
        strokeLinecap="round"
      />
      <Circle cx="15" cy="25" r="2.5" fill={theme.text} />
      <Circle cx="22" cy="25" r="2.5" fill={theme.textSecondary} />
      <Circle cx="29" cy="25" r="2.5" fill={theme.foreground} />
      <Line
        x1="15"
        y1="31"
        x2="26"
        y2="31"
        stroke={theme.textSecondary}
        strokeWidth="2"
        strokeLinecap="round"
      />
    </Svg>
  );
}

/**
 * PRIME Banner Right Side Mascot Artwork
 */
export function PrimeScooterMascot({
  size = 54,
  iconColor,
}: IllustrationProps & { iconColor?: string }) {
  const theme = useTheme();
  const color = iconColor || theme.text;
  return (
    <Svg width={size} height={size} viewBox="0 0 54 54">
      <Defs>
        <LinearGradient id="prime-fill" x1="0" y1="0" x2="1" y2="1">
          <Stop offset="0" stopColor={theme.backgroundElement} />
          <Stop offset="1" stopColor={theme.card} />
        </LinearGradient>
      </Defs>
      <Circle cx="27" cy="28" r="17" fill="url(#prime-fill)" />
      <Path
        d="M18 30h18l4 7H14l4-7Zm4-4 5-8 8 8"
        fill="none"
        stroke={color}
        strokeWidth="2.5"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <Circle
        cx="20"
        cy="40"
        r="3.5"
        fill={theme.card}
        stroke={color}
        strokeWidth="2"
      />
      <Circle
        cx="36"
        cy="40"
        r="3.5"
        fill={theme.card}
        stroke={color}
        strokeWidth="2"
      />
      <Path
        d="m11 9 1.5 4L16 14.5 12.5 16 11 20l-1.5-4L6 14.5 9.5 13 11 9Z"
        fill={theme.textSecondary}
      />
      <Circle cx="43" cy="15" r="2" fill={theme.text} />
    </Svg>
  );
}

const styles = StyleSheet.create({
  avatarContainer: {
    alignItems: "center",
    justifyContent: "center",
    backgroundColor: "transparent",
  },
  featureBox: {
    alignItems: "center",
    justifyContent: "center",
    backgroundColor: "transparent",
  },
  mascotContainer: {
    alignItems: "center",
    justifyContent: "center",
    backgroundColor: "transparent",
  },
});
