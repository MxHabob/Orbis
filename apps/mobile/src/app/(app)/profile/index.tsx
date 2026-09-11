// app/(app)/(tabs)/profile.tsx

import { Ionicons } from "@expo/vector-icons";
import { router } from "expo-router";
import { useMemo } from "react";
import {
  Pressable,
  ScrollView,
  StyleSheet,
  Text,
  useWindowDimensions,
  View,
} from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { Header } from "@/components/header";
import { OrganizationLogo } from "@/components/ui/organization-logo";
import { Radii, Spacing, Typography } from "@/constants/theme";
import { useTheme } from "@/hooks/use-theme";
import { useCurrentOrgQuery } from "@/hooks/use-api-queries";
import { useAuth } from "@/lib/api";
import { Button } from "@/components/ui/button";

const DEFAULT_WEB_URL =
  process.env.EXPO_PUBLIC_ORBIS_WEB_URL?.trim() || "https://www.mulverse.com/";

const WEB_LINKS = {
  help: process.env.EXPO_PUBLIC_ORBIS_HELP_URL?.trim() || DEFAULT_WEB_URL,

  about: process.env.EXPO_PUBLIC_ORBIS_ABOUT_URL?.trim() || DEFAULT_WEB_URL,

  privacy: process.env.EXPO_PUBLIC_ORBIS_PRIVACY_URL?.trim() || DEFAULT_WEB_URL,

  terms: process.env.EXPO_PUBLIC_ORBIS_TERMS_URL?.trim() || DEFAULT_WEB_URL,
};

function getInitials(name: string) {
  const words = name.trim().split(/\s+/).filter(Boolean);

  if (words.length === 0) return "OR";

  if (words.length === 1) {
    return words[0].slice(0, 2).toUpperCase();
  }

  return `${words[0][0]}${words[words.length - 1][0]}`.toUpperCase();
}

function openWeb(url: string, title: string) {
  router.push({
    pathname: "/webviewer",
    params: {
      url,
      title,
    },
  });
}

export default function ProfileScreen() {
  const theme = useTheme();
  const insets = useSafeAreaInsets();
  const { width } = useWindowDimensions();

  const { user, logout } = useAuth();
  const { data: organization } = useCurrentOrgQuery();

  const userData = user as
    | {
        name?: string;
        full_name?: string;
        email?: string;
        role?: string;
      }
    | undefined;

  const orgData = organization as
    | {
        name?: string;
        display_name?: string;
        logo?: string;
        logo_url?: string;
        website?: string;
        website_url?: string;
      }
    | undefined;

  const name = useMemo(
    () => userData?.name?.trim() || userData?.full_name?.trim() || "Orbis User",
    [userData?.name, userData?.full_name],
  );

  const email = userData?.email?.trim() || "";
  const role = userData?.role?.trim();

  const organizationName =
    orgData?.display_name?.trim() || orgData?.name?.trim() || "Organization";

  const organizationLogo = orgData?.logo_url || orgData?.logo || undefined;

  const organizationWebsite =
    orgData?.website_url?.trim() || orgData?.website?.trim();

  const initials = getInitials(name);

  /*
   * Responsive ID card.
   * Compact on phones and capped on tablets.
   */
  const cardWidth = Math.min(width - Spacing.six, 520);

  const cardHeight = Math.min(cardWidth * 1.38, 610);

  const handleOrganizationPress = () => {
    if (!organizationWebsite) return;

    openWeb(organizationWebsite, organizationName);
  };

  const handleLogout = async () => {
    await logout();
    router.replace("/(auth)/sign-in");
  };

  return (
    <View
      style={[
        styles.container,
        {
          backgroundColor: theme.background,
        },
      ]}
    >
      <ScrollView
        showsVerticalScrollIndicator={false}
        contentContainerStyle={[
          styles.content,
          {
            paddingTop: Spacing.two,
            paddingBottom: insets.bottom + 120,
          },
        ]}
      >
        {/* ============================================================
            HEADER
            ============================================================ */}

        <Header
          title="Profile"
          showBack
          showCrown
          onBack={() => router.back()}
          onCrownPress={() => {
            // Add premium route/action here.
          }}
        />

        {/* ============================================================
            DIGITAL ID
            ============================================================ */}

        <View
          style={[
            styles.idCard,
            {
              width: cardWidth,
              height: cardHeight,
              backgroundColor: theme.cardElevated,
              borderColor: theme.cardBorder,
            },
          ]}
        >
          {/* Card header */}

          <View style={styles.cardHeader}>
            <Text
              style={[
                styles.orbisMark,
                {
                  color: theme.text,
                },
              ]}
            >
              ORBIS
            </Text>

            <View style={styles.cardNumber}>
              <View
                style={[
                  styles.statusDot,
                  {
                    backgroundColor: theme.primary,
                  },
                ]}
              />

              <Text
                style={[
                  styles.cardIndex,
                  {
                    color: theme.textMuted,
                  },
                ]}
              >
                01
              </Text>
            </View>
          </View>

          {/* Identity */}

          <View style={styles.identityArea}>
            <View
              style={[
                styles.avatar,
                {
                  backgroundColor: theme.background,
                  borderColor: theme.border,
                },
              ]}
            >
              <Text
                style={[
                  styles.avatarText,
                  {
                    color: theme.text,
                  },
                ]}
              >
                {initials}
              </Text>
            </View>

            <Text
              numberOfLines={2}
              style={[
                styles.name,
                {
                  color: theme.text,
                },
              ]}
            >
              {name}
            </Text>

            {!!email && (
              <Text
                numberOfLines={1}
                style={[
                  styles.email,
                  {
                    color: theme.textMuted,
                  },
                ]}
              >
                {email}
              </Text>
            )}

            {!!role && (
              <View
                style={[
                  styles.role,
                  {
                    borderColor: theme.border,
                  },
                ]}
              >
                <Text
                  style={[
                    styles.roleText,
                    {
                      color: theme.textSecondary,
                    },
                  ]}
                >
                  {role.toUpperCase()}
                </Text>
              </View>
            )}
          </View>

          {/* Divider */}

          <View
            style={[
              styles.divider,
              {
                backgroundColor: theme.border,
              },
            ]}
          />

          {/* Organization */}

          <Pressable
            disabled={!organizationWebsite}
            onPress={handleOrganizationPress}
            style={({ pressed }) => [
              styles.organization,
              {
                opacity: pressed && organizationWebsite ? 0.55 : 1,
              },
            ]}
          >
            <View
              style={[
                styles.orgLogo,
                {
                  backgroundColor: theme.background,
                },
              ]}
            >
              <OrganizationLogo logoUrl={organizationLogo} size={34} />
            </View>

            <View style={styles.orgInfo}>
              <Text
                style={[
                  styles.orgLabel,
                  {
                    color: theme.textMuted,
                  },
                ]}
              >
                CURRENT SPACE
              </Text>

              <Text
                numberOfLines={1}
                style={[
                  styles.orgName,
                  {
                    color: theme.text,
                  },
                ]}
              >
                {organizationName}
              </Text>
            </View>

            <View style={styles.orgArrow}>
              <Ionicons
                name={
                  organizationWebsite
                    ? "arrow-up-right-box-outline"
                    : "business-outline"
                }
                size={16}
                color={theme.textSecondary}
              />
            </View>
          </Pressable>

          {/* Card footer */}

          <View style={styles.cardFooter}>
            <View>
              <Text
                style={[
                  styles.footerLabel,
                  {
                    color: theme.textMuted,
                  },
                ]}
              >
                ORBIS ID
              </Text>

              <Text
                style={[
                  styles.footerValue,
                  {
                    color: theme.textSecondary,
                  },
                ]}
              >
                •••• •••• ••••
              </Text>
            </View>

            <Text
              style={[
                styles.footerYear,
                {
                  color: theme.textMuted,
                },
              ]}
            >
              DIGITAL
            </Text>
          </View>
        </View>

        {/* ============================================================
            ACCOUNT ACTIONS
            ============================================================ */}

        <View style={styles.actions}>
          <ActionRow
            label="Account"
            index="01"
            theme={theme}
            onPress={() => router.push("/profile/account")}
          />

          <ActionRow
            label="Preferences"
            index="02"
            theme={theme}
            onPress={() => router.push("/profile/preferences")}
          />

          <ActionRow
            label="Security"
            index="03"
            theme={theme}
            onPress={() => router.push("/profile/security")}
          />
        </View>

        {/* ============================================================
            ORBIS LINKS
            ============================================================ */}

        <View style={styles.links}>
          <Text
            style={[
              styles.linksLabel,
              {
                color: theme.textMuted,
              },
            ]}
          >
            ORBIS
          </Text>

          <WebRow
            label="Help Center"
            theme={theme}
            onPress={() => openWeb(WEB_LINKS.help, "Help Center")}
          />

          <WebRow
            label="About Orbis"
            theme={theme}
            onPress={() => openWeb(WEB_LINKS.about, "About Orbis")}
          />

          <WebRow
            label="Privacy"
            theme={theme}
            onPress={() => openWeb(WEB_LINKS.privacy, "Privacy")}
          />

          <WebRow
            label="Terms"
            theme={theme}
            onPress={() => openWeb(WEB_LINKS.terms, "Terms")}
          />
        </View>

        {/* ============================================================
            LOGOUT
            ============================================================ */}

        <Button variant="destructive" onPress={handleLogout}>
          <Text>Log out</Text>
        </Button>
      </ScrollView>
    </View>
  );
}

/* -------------------------------------------------------------------------- */
/* Action Row                                                                 */
/* -------------------------------------------------------------------------- */

function ActionRow({
  label,
  index,
  theme,
  onPress,
}: {
  label: string;
  index: string;
  theme: ReturnType<typeof useTheme>;
  onPress: () => void;
}) {
  return (
    <Pressable
      onPress={onPress}
      style={({ pressed }) => [
        styles.actionRow,
        {
          opacity: pressed ? 0.45 : 1,
        },
      ]}
    >
      <Text
        style={[
          styles.actionLabel,
          {
            color: theme.text,
          },
        ]}
      >
        {label}
      </Text>

      <View style={styles.actionMeta}>
        <Text
          style={[
            styles.actionIndex,
            {
              color: theme.textMuted,
            },
          ]}
        ></Text>

        <Ionicons name="arrow-forward" size={15} color={theme.textMuted} />
      </View>
    </Pressable>
  );
}

/* -------------------------------------------------------------------------- */
/* Web Row                                                                    */
/* -------------------------------------------------------------------------- */

function WebRow({
  label,
  theme,
  onPress,
}: {
  label: string;
  theme: ReturnType<typeof useTheme>;
  onPress: () => void;
}) {
  return (
    <Pressable
      onPress={onPress}
      style={({ pressed }) => [
        styles.webRow,
        {
          opacity: pressed ? 0.45 : 1,
        },
      ]}
    >
      <Text
        style={[
          styles.webLabel,
          {
            color: theme.text,
          },
        ]}
      >
        {label}
      </Text>

      <Ionicons name="open-outline" size={15} color={theme.textMuted} />
    </Pressable>
  );
}

/* -------------------------------------------------------------------------- */
/* Styles                                                                     */
/* -------------------------------------------------------------------------- */

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },

  content: {
    alignItems: "center",
    paddingHorizontal: Spacing.three,
  },

  /* ID Card */

  idCard: {
    maxWidth: 520,
    minHeight: 430,

    borderRadius: Radii["3xl"],
    borderWidth: StyleSheet.hairlineWidth,

    padding: Spacing.six,

    overflow: "hidden",

    marginTop: Spacing.three,
  },

  cardHeader: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
  },

  orbisMark: {
    ...Typography.bodyBold,
    fontSize: 13,
    letterSpacing: 2.2,
  },

  cardNumber: {
    flexDirection: "row",
    alignItems: "center",
    gap: 7,
  },

  statusDot: {
    width: 6,
    height: 6,
    borderRadius: Radii.full,
  },

  cardIndex: {
    ...Typography.small,
    fontVariant: ["tabular-nums"],
  },

  /* Identity */

  identityArea: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",

    paddingVertical: Spacing.eight,
  },

  avatar: {
    width: 94,
    height: 94,

    borderRadius: Radii.full,
    borderWidth: StyleSheet.hairlineWidth,

    alignItems: "center",
    justifyContent: "center",

    marginBottom: Spacing.four,
  },

  avatarText: {
    ...Typography.h2,
    fontSize: 25,
    lineHeight: 30,
    letterSpacing: -0.5,
  },

  name: {
    ...Typography.h1,
    fontSize: 25,
    lineHeight: 30,
    fontWeight: "800",
    letterSpacing: -0.7,

    textAlign: "center",

    maxWidth: "90%",
  },

  email: {
    ...Typography.caption,

    marginTop: 5,

    maxWidth: "90%",
  },

  role: {
    marginTop: Spacing.three,

    minHeight: 24,

    paddingHorizontal: Spacing.three,

    borderRadius: Radii.full,
    borderWidth: StyleSheet.hairlineWidth,

    alignItems: "center",
    justifyContent: "center",
  },

  roleText: {
    ...Typography.small,
    fontSize: 9,
    letterSpacing: 1.2,
  },

  divider: {
    width: "100%",
    height: StyleSheet.hairlineWidth,
  },

  /* Organization */

  organization: {
    minHeight: 64,

    flexDirection: "row",
    alignItems: "center",

    gap: Spacing.three,

    paddingVertical: Spacing.three,
  },

  orgLogo: {
    width: 46,
    height: 46,

    borderRadius: Radii.lg,

    alignItems: "center",
    justifyContent: "center",

    overflow: "hidden",
  },

  orgInfo: {
    flex: 1,
    minWidth: 0,
  },

  orgLabel: {
    ...Typography.small,
    fontSize: 8,
    lineHeight: 11,
    letterSpacing: 1.25,

    marginBottom: 2,
  },

  orgName: {
    ...Typography.bodyBold,
    fontSize: 14,
  },

  orgArrow: {
    width: 32,
    height: 32,

    alignItems: "center",
    justifyContent: "center",
  },

  /* Card footer */

  cardFooter: {
    flexDirection: "row",
    alignItems: "flex-end",
    justifyContent: "space-between",

    paddingTop: Spacing.four,
  },

  footerLabel: {
    ...Typography.small,
    fontSize: 8,
    lineHeight: 11,
    letterSpacing: 1.4,
  },

  footerValue: {
    ...Typography.small,
    fontSize: 10,
    letterSpacing: 1.5,

    marginTop: 2,
  },

  footerYear: {
    ...Typography.small,
    fontSize: 8,
    letterSpacing: 1.4,
  },

  /* Actions */

  actions: {
    width: "100%",
    maxWidth: 520,

    marginTop: 44,
  },

  actionRow: {
    minHeight: 62,

    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
  },

  actionLabel: {
    ...Typography.body,
    fontSize: 15,
    letterSpacing: -0.15,
  },

  actionMeta: {
    flexDirection: "row",
    alignItems: "center",
    gap: Spacing.three,
  },

  actionIndex: {
    ...Typography.small,
    fontVariant: ["tabular-nums"],
  },

  /* Orbis links */

  links: {
    width: "100%",
    maxWidth: 520,

    marginTop: 42,
  },

  linksLabel: {
    ...Typography.small,
    fontSize: 9,
    letterSpacing: 1.6,

    marginBottom: Spacing.two,
  },

  webRow: {
    minHeight: 42,

    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
  },

  webLabel: {
    ...Typography.body,
    fontSize: 14,
  },

  /* Logout */

  logout: {
    alignSelf: "flex-start",

    width: "100%",
    maxWidth: 520,
    marginTop: 38,

    paddingVertical: Spacing.two,
  },
});
