import React from 'react';
import { View, Text, StyleSheet, ScrollView } from 'react-native';
import { router } from 'expo-router';
import { useTheme } from '@/hooks/use-theme';
import { Spacing, Typography, Radii } from '@/constants/theme';
import { useAuth } from '@/lib/api';
import { useCurrentOrgQuery } from '@/hooks/use-api-queries';
import { Button } from '@/components/ui/button';
import { OrganizationLogo } from '@/components/ui/organization-logo';

export default function ProfileScreen() {
  const theme = useTheme();
  const { user } = useAuth();
  const { data: orgData } = useCurrentOrgQuery();

  const current = new Date();
  const weekday = new Intl.DateTimeFormat(undefined, { weekday: 'long' }).format(current);
  const month = new Intl.DateTimeFormat(undefined, { month: 'long' }).format(current);
  const day = new Intl.DateTimeFormat(undefined, { day: 'numeric' }).format(current);
  const hour = current.getHours();
  const greeting = hour < 12 ? 'Good morning' : hour < 18 ? 'Good afternoon' : 'Good evening';

  const firstName = user?.full_name?.trim() || user?.email || 'alexsmithhh95';
  const cleanedHandle = firstName.includes('@')
    ? firstName
    : firstName.includes(' ') ? firstName.split(' ')[0] : firstName;

  const handle = `@${cleanedHandle.replace(/[^a-zA-Z0-9._]+/g, '').toLowerCase()}`;
  const organizationName = orgData?.name || 'Orbis';

  const actionGroups = [
    { title: 'Account', route: '/(app)/settings/account' },
    { title: 'Security', route: '/(app)/settings/security' },
    { title: 'Linked', route: '/(app)/settings/linked-accounts' },
    { title: 'Preferences', route: '/(app)/settings/preferences' },
  ];

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <ScrollView
        style={styles.scroll}
        contentContainerStyle={styles.scrollContent}
        showsVerticalScrollIndicator={false}>

        <Text style={[styles.brandTitle, { color: theme.foreground }]}>{organizationName}</Text>

        <View style={styles.greetingBlock}>
          <Text style={[styles.greeting, { color: theme.foreground }]}>
            {greeting}, <Text style={[styles.handle, { color: theme.primary }]}>{handle}</Text>. It's
            {` ${weekday}, ${month} ${day}. Today at a glance:`}
          </Text>

        </View>

        <View style={styles.avatarWrap}>
          <View style={[styles.avatarFrame, { backgroundColor: theme.border, borderColor: theme.border }]}>
            <View style={[styles.avatarInner, { backgroundColor: theme.foreground }]}>
              <View style={[styles.faceHighlight, { backgroundColor: theme.textMuted }]} />
              <View style={[styles.avatarShadeLeft, { backgroundColor: theme.backgroundSelected }]} />
              <View style={[styles.avatarShadeRight, { backgroundColor: theme.card }]} />
              <View style={[styles.avatarTopArc, { backgroundColor: theme.card, borderColor: theme.border }]} />
            </View>
          </View>
        </View>
        <View style={styles.twoColumnActions}>
          <View style={styles.columnActions}>
            {actionGroups.slice(0, 2).map((item) => (
              <Button
                key={item.title}
                title={item.title}
                variant="outline"
                size="sm"
                fullWidth
                onPress={() => router.push(item.route as any)}
                style={styles.actionButton}
              />
            ))}
          </View>

          <View style={styles.columnActions}>
            {actionGroups.slice(2).map((item) => (
              <Button
                key={item.title}
                title={item.title}
                variant="outline"
                size="sm"
                fullWidth
                onPress={() => router.push(item.route as any)}
                style={styles.actionButton}
              />
            ))}
          </View>
        </View>

        <Button
          title="DIVE DEEPER"
          variant="default"
          size="lg"
          onPress={() => undefined}
          style={styles.deepButton}
        />

        <View style={[styles.notification, { backgroundColor: theme.backgroundSelected }]}>
          <View style={styles.notificationLeft}>
            <Text style={[styles.notificationIcon, { color: theme.card }]}>!</Text>
            <Text style={[styles.notificationText, { color: theme.card }]}>NEW YEAR DETECTED</Text>
          </View>
          <Text style={[styles.closeText, { color: theme.card }]}>×</Text>
        </View>

        <View style={styles.navBar}>
          {['HOME', 'FRIENDS', 'VOID', 'SHOP', 'YOU'].map((item, index) => (
            <Text key={item} style={[styles.navItem, { color: index === 0 ? theme.foreground : theme.textMuted }]}>
              {item}
            </Text>
          ))}
        </View>

        <View style={[styles.navThumb, { backgroundColor: theme.foreground }]} />

        <View style={styles.footer}>
          <View style={styles.footerBrand}>
            <View style={[styles.logoDot, { backgroundColor: theme.foreground }]} />
            <Text style={[styles.footerBrandText, { color: theme.foreground }]}>{organizationName}</Text>
          </View>

          <View style={styles.footerCurated}>
            <Text style={[styles.curatedText, { color: theme.textMuted }]}>curated by</Text>
            <OrganizationLogo size={30} color={theme.foreground} dotColor={theme.primary} />
            <Text style={[styles.mobbinText, { color: theme.foreground }]}>Orbis</Text>
          </View>
        </View>
      </ScrollView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scroll: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: Spacing.six,
    paddingTop: Spacing.two,
    paddingBottom: Spacing.sixteen,
    alignItems: 'center',
  },
  brandTitle: {
    ...Typography.h1,
    fontFamily: 'serif',
    fontSize: 24,
    lineHeight: 42,
    marginTop: Spacing.eight,
    marginBottom: Spacing.six,
  },
  greetingBlock: {
    width: '100%',
    alignItems: 'center',
  },
  greeting: {
    ...Typography.body,
    fontSize: 16,
    lineHeight: 27,
    textAlign: 'center',
  },
  handle: {
    fontWeight: '700',
  },
  heroText: {
    ...Typography.h2,
    fontFamily: 'serif',
    fontSize: 28,
    lineHeight: 36,
    marginTop: Spacing.four,
    textAlign: 'center',
  },
  avatarWrap: {
    marginTop: Spacing.four,
    alignItems: 'center',
  },
  avatarFrame: {
    width: 126,
    height: 126,
    borderRadius: 63,
    padding: 2,
    alignItems: 'center',
    justifyContent: 'center',
    borderWidth: 1,
  },
  avatarInner: {
    width: '100%',
    height: '100%',
    borderRadius: 63,
    overflow: 'hidden',
    position: 'relative',
    borderWidth: 1,
  },
  faceHighlight: {
    position: 'absolute',
    width: 70,
    height: 95,
    left: 26,
    top: 10,
    borderRadius: 50,
    opacity: 0.9,
  },
  avatarShadeLeft: {
    position: 'absolute',
    left: 10,
    top: 26,
    width: 40,
    height: 88,
    borderRadius: 50,
    opacity: 0.8,
  },
  avatarShadeRight: {
    position: 'absolute',
    right: 15,
    top: 30,
    width: 34,
    height: 84,
    borderRadius: 50,
    opacity: 0.6,
  },
  avatarTopArc: {
    position: 'absolute',
    left: 16,
    top: -3,
    width: 92,
    height: 80,
    borderRadius: 60,
    opacity: 0.8,
    transform: [{ rotate: '16deg' }],
    borderWidth: 1,
  },
  twoColumnActions: {
    width: '84%',
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginTop: Spacing.four,
  },
  columnActions: {
    flex: 1,
    marginHorizontal: Spacing.one,
  },
  actionButton: {
    borderRadius: Radii.md,
    paddingVertical: Spacing.three,
    paddingHorizontal: Spacing.four,
    borderWidth: 1,
    alignItems: 'center',
    justifyContent: 'center',
  },
  actionButtonText: {
    ...Typography.bodyBold,
    fontSize: 14,
  },
  deepButton: {
    width: 260,
    height: 50,
    borderRadius: Radii.md,
    alignItems: 'center',
    justifyContent: 'center',
    marginTop: Spacing.eight,
  },
  deepButtonText: {
    ...Typography.bodyBold,
    fontSize: 16,
    letterSpacing: 0.8,
  },
  notification: {
    width: '100%',
    minHeight: 44,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingHorizontal: Spacing.four,
    borderRadius: Radii.xs,
    marginTop: Spacing.ten,
  },
  notificationLeft: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  notificationIcon: {
    fontSize: 22,
    fontWeight: '900',
    marginRight: Spacing.two,
  },
  notificationText: {
    fontSize: 14,
    fontWeight: '700',
    letterSpacing: 0.5,
  },
  closeText: {
    fontSize: 30,
    fontWeight: '300',
  },
  navBar: {
    width: '100%',
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginTop: Spacing.six,
    paddingHorizontal: Spacing.four,
  },
  navItem: {
    ...Typography.bodyBold,
    fontSize: 12,
  },
  navThumb: {
    width: 56,
    height: 4,
    borderRadius: 4,
    marginTop: Spacing.two,
  },
  footer: {
    width: '100%',
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    marginTop: Spacing.six,
    paddingHorizontal: Spacing.one,
  },
  footerBrand: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  logoDot: {
    width: 20,
    height: 20,
    borderRadius: 10,
    marginRight: Spacing.two,
  },
  footerBrandText: {
    ...Typography.h3,
    fontFamily: 'serif',
    fontSize: 14,
  },
  footerCurated: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.one,
  },
  curatedText: {
    ...Typography.body,
    fontSize: 11,
  },
  mobbinLogoWrap: {
    flexDirection: 'row',
    alignItems: 'center',
    marginLeft: Spacing.two,
  },
  mobbinLogo: {
    width: 34,
    height: 34,
    borderRadius: Radii.sm,
    alignItems: 'center',
    justifyContent: 'center',
    flexDirection: 'row',
  },
  mobbinLeft: {
    width: 10,
    height: 18,
    borderRadius: 10,
    transform: [{ rotate: '-30deg' }],
    marginRight: 2,
  },
  mobbinRight: {
    width: 10,
    height: 18,
    borderRadius: 10,
    transform: [{ rotate: '30deg' }],
    marginLeft: 2,
  },
  mobbinText: {
    ...Typography.bodyBold,
    fontSize: 16,
    marginLeft: Spacing.one,
  },
});
