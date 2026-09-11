import { useMemo } from "react";
import { Image, StyleSheet, Text, View } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Rect, Svg } from "react-native-svg";

import Header from "@/components/header";
import { Fonts, Spacing, Typography } from "@/constants/theme";
import { useCurrentOrgQuery } from "@/hooks/use-api-queries";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import { useEffectiveContext } from "@/providers/effective-context";

export type IdCardInfo = {
  fullName: string;
  email?: string | null;
  orgName?: string;
  orgLogo?: string;
  role?: string;
  status?: string;
  userId?: string;
  photoUrl?: string;
};

function getInitials(name: string) {
  const words = name.trim().split(/\s+/).filter(Boolean);
  if (!words.length) return "OR";
  if (words.length === 1) return words[0].slice(0, 2).toUpperCase();

  return `${words[0][0]}${words[words.length - 1][0]}`.toUpperCase();
}

function generateEan13(value: string) {
  const raw = (value || "123456789012").replace(/\D/g, "").slice(0, 12).padStart(12, "0");
  let sum = 0;

  for (let i = 0; i < raw.length; i += 1) {
    const digit = Number(raw[i]);
    sum += digit * (i % 2 === 0 ? 1 : 3);
  }

  const checkDigit = (10 - (sum % 10)) % 10;
  const full = `${raw}${checkDigit}`;

  const leftPatterns = [
    "0001101",
    "0011001",
    "0010011",
    "0111101",
    "0100011",
    "0110001",
    "0101111",
    "0111011",
    "0110111",
    "0001011",
  ];

  const rightPatterns = [
    "1110010",
    "1100110",
    "1101100",
    "1000010",
    "1011100",
    "1001110",
    "1010000",
    "1000100",
    "1001000",
    "1110100",
  ];

  const bars: number[] = [];
  const addQuiet = (count: number) => {
    for (let i = 0; i < count; i += 1) bars.push(0);
  };
  const addPattern = (pattern: string) => {
    for (const bit of pattern) bars.push(bit === "1" ? 1 : 0);
  };

  addQuiet(10);
  addPattern("101");

  for (let i = 1; i <= 6; i += 1) addPattern(leftPatterns[Number(full[i])]);
  addPattern("01010");

  for (let i = 7; i <= 12; i += 1) addPattern(rightPatterns[Number(full[i])]);

  addPattern("101");
  addQuiet(10);

  return { full, bars };
}

function Barcode({ value }: { value?: string }) {
  const theme = useTheme();
  const { bars, full } = useMemo(() => generateEan13(value || "123456789012"), [value]);

  const barWidth = 2.1;
  const width = bars.length * barWidth;
  const rects: Array<{ x: number; width: number; height: number; fill: string }> = [];

  let x = 0;
  for (let i = 0; i < bars.length; i += 1) {
    if (bars[i] === 1) {
      rects.push({ x, width: barWidth, height: 72, fill: theme.text });
    }
    x += barWidth;
  }

  return (
    <View style={[styles.barcodeCard, { backgroundColor: theme.card, borderColor: theme.border }]}>
      <Svg width="100%" height={72} viewBox={`0 0 ${width} 72`}>
        {rects.map((rect, index) => (
          <Rect key={`${rect.x}-${index}`} x={rect.x} y={0} width={rect.width} height={rect.height} fill={rect.fill} />
        ))}
      </Svg>
      <Text style={[styles.barcodeNumber, { color: theme.textMuted }]}>{full}</Text>
    </View>
  );
}

export function IdentityCard({ data }: { data: IdCardInfo }) {
  const theme = useTheme();
  const initials = getInitials(data.fullName || "Orbis");
  const status = (data.status || "ACTIVE").toUpperCase();
  const isActive = status === "ACTIVE" || status === "VERIFIED" || status === "ENABLED";

  return (
    <View style={[styles.card, { backgroundColor: theme.card, borderColor: theme.border }]}>
      <View style={styles.cardTop}>
        <View
          style={[
            styles.status,
            {
              backgroundColor: isActive ? "#10B9811A" : "#EF44441A",
              borderColor: isActive ? "#10B98155" : "#EF444455",
            },
          ]}
        >
          <View style={[styles.dot, { backgroundColor: isActive ? "#10B981" : "#EF4444" }]} />
          <Text style={[styles.statusText, { color: isActive ? "#10B981" : "#EF4444" }]}>
            {isActive ? "ACTIVE" : status}
          </Text>
        </View>
      </View>

      <View style={styles.photoWrap}>
        <View style={[styles.photoRing, { borderColor: theme.primary + "44" }]}>
          {data.photoUrl ? (
            <Image source={{ uri: data.photoUrl }} style={styles.photo} resizeMode="cover" />
          ) : (
            <View style={[styles.photoFallback, { backgroundColor: theme.primary + "15" }]}>
              <Text style={[styles.photoInitials, { color: theme.primary }]}>{initials}</Text>
            </View>
          )}
        </View>
      </View>

      <Text style={[styles.name, { color: theme.text }]}>{data.fullName}</Text>
      <Text style={[styles.role, { color: theme.primary }]}>{data.role || "Member"}</Text>
    </View>
  );
}

export default function IdScreen() {
  const theme = useTheme();
  const insets = useSafeAreaInsets();
  const { user } = useAuth();
  const { data: orgData } = useCurrentOrgQuery();
  const { role } = useEffectiveContext();

  const userData = user as
    | {
        id?: string;
        full_name?: string;
        email?: string | null;
        status?: string;
        role?: string;
        avatar_url?: string;
        photo_url?: string;
      }
    | undefined;

  const orgName =
    (orgData as { display_name?: string; name?: string } | undefined)?.display_name ||
    (orgData as { display_name?: string; name?: string } | undefined)?.name ||
    "Organization";

  const orgLogo =
    (orgData as { logo_url?: string; logo?: string } | undefined)?.logo_url ||
    (orgData as { logo_url?: string; logo?: string } | undefined)?.logo;

  const data = useMemo<IdCardInfo>(
    () => ({
      fullName: userData?.full_name?.trim() || userData?.email?.split("@")[0] || "Orbis User",
      orgName,
      orgLogo,
      role: role || userData?.role || "Member",
      status: userData?.status || "active",
      userId: userData?.id || "123456789012",
      photoUrl: userData?.avatar_url || userData?.photo_url || undefined,
    }),
    [orgName, orgLogo, role, userData],
  );

  return (
    <View
      style={[
        styles.screen,
        { backgroundColor: theme.background, paddingTop: insets.top, paddingBottom: insets.bottom },
      ]}
    >
      <View style={styles.container}>
        <Header title="Digital ID" showBack={true} showCrown={false} />

        <View style={styles.content}>
          <View style={styles.orgHeader}>
            {data.orgLogo ? (
              <Image source={{ uri: data.orgLogo }} style={styles.orgLogo} resizeMode="contain" />
            ) : null}
            <Text style={[styles.orgName, { color: theme.text }]}>{data.orgName}</Text>
          </View>

          <IdentityCard data={data} />
          <Barcode value={data.userId} />
        </View>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  screen: { flex: 1 },
  container: { flex: 1, paddingHorizontal: Spacing.five },
  content: { flex: 1, justifyContent: "center", paddingBottom: Spacing.five, gap: Spacing.three },
  orgHeader: { alignItems: "center", justifyContent: "center", gap: 8, paddingTop: Spacing.one },
  orgLogo: { width: 36, height: 36, borderRadius: 10 },
  orgName: {
    fontFamily: Fonts.sans,
    fontSize: Typography.h3.fontSize,
    fontWeight: "700",
    letterSpacing: 1.5,
    textTransform: "uppercase",
  },
  card: {
    width: "100%",
    borderWidth: 1,
    borderRadius: 26,
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.three,
    paddingBottom: Spacing.four,
    alignItems: "center",
  },
  cardTop: { width: "100%", flexDirection: "row", justifyContent: "flex-end" },
  status: {
    flexDirection: "row",
    alignItems: "center",
    gap: 6,
    paddingHorizontal: 10,
    paddingVertical: 5,
    borderRadius: 999,
    borderWidth: 1,
  },
  dot: { width: 6, height: 6, borderRadius: 3 },
  statusText: {
    fontFamily: Fonts.sans,
    fontSize: Typography.captionBold.fontSize,
    fontWeight: "800",
    letterSpacing: 0.8,
  },
  photoWrap: { marginTop: Spacing.two, alignItems: "center", justifyContent: "center" },
  photoRing: {
    width: 114,
    height: 114,
    borderRadius: 57,
    borderWidth: 2,
    padding: 4,
    alignItems: "center",
    justifyContent: "center",
  },
  photo: { width: "100%", height: "100%", borderRadius: 53 },
  photoFallback: { width: "100%", height: "100%", borderRadius: 53, alignItems: "center", justifyContent: "center" },
  photoInitials: { fontFamily: Fonts.sans, fontSize: 34, fontWeight: "800", letterSpacing: 1 },
  name: {
    marginTop: Spacing.three,
    fontFamily: Fonts.sans,
    fontSize: Typography.h1.fontSize,
    lineHeight: Typography.h1.lineHeight,
    fontWeight: Typography.h1.fontWeight,
    letterSpacing: -0.4,
    textAlign: "center",
  },
  role: {
    marginTop: 6,
    fontFamily: Fonts.sans,
    fontSize: Typography.captionBold.fontSize,
    fontWeight: "700",
    letterSpacing: 1.5,
    textTransform: "uppercase",
  },
  barcodeCard: {
    width: "100%",
    borderWidth: 1,
    borderRadius: 18,
    paddingHorizontal: Spacing.three,
    paddingVertical: Spacing.three,
    alignItems: "center",
    justifyContent: "center",
  },
  barcodeNumber: {
    marginTop: 8,
    fontFamily: Fonts.sans,
    fontSize: Typography.small.fontSize,
    fontWeight: "700",
    letterSpacing: 3,
  },
});