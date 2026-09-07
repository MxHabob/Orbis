/**
 * @module @orbis/mobile/components/home/prime-ai-banner
 *
 * Glovo-style Prime promotional banner highlighting unlimited AI analytics.
 */

import React from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { PrimeScooterMascot } from '@/components/glovo-illustrations';
import { useI18n } from '@/lib/i18n';
import { Radii, Spacing, Typography } from '@/constants/theme';

interface PrimeAiBannerProps {
  onPress: () => void;
}

export const PrimeAiBanner = React.memo<PrimeAiBannerProps>(({ onPress }) => {
  const { t } = useI18n();

  return (
    <Pressable
      onPress={onPress}
      style={({ pressed }) => [
        styles.banner,
        pressed && { opacity: 0.92, transform: [{ scale: 0.99 }] },
      ]}>
      {/* Left Content */}
      <View style={styles.left}>
        <View style={styles.badge}>
          <Ionicons name="sparkles" size={13} color="#FFFFFF" />
          <Text style={styles.badgeText}>{t('prime_badge')}</Text>
        </View>

        <Text style={styles.title}>{t('prime_title')}</Text>
        <Text style={styles.subtitle}>{t('prime_subtitle')}</Text>
      </View>

      {/* Right Mascot Artwork & Button */}
      <View style={styles.right}>
        <PrimeScooterMascot size={78} />
        <View style={styles.chevronButton}>
          <Ionicons name="chevron-forward" size={18} color="#18181B" />
        </View>
      </View>
    </Pressable>
  );
});

PrimeAiBanner.displayName = 'PrimeAiBanner';

const styles = StyleSheet.create({
  banner: {
    backgroundColor: '#FFE380',
    borderRadius: 24,
    paddingVertical: Spacing.four,
    paddingHorizontal: Spacing.four,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    shadowColor: '#000000',
    shadowOffset: { width: 0, height: 6 },
    shadowOpacity: 0.08,
    shadowRadius: 12,
    elevation: 4,
    marginTop: Spacing.two,
  },
  left: {
    flex: 1,
    gap: 4,
    paddingRight: Spacing.two,
  },
  badge: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#18181B',
    paddingVertical: 3,
    paddingHorizontal: 8,
    borderRadius: Radii.full,
    alignSelf: 'flex-start',
    gap: 4,
    marginBottom: 4,
  },
  badgeText: {
    ...Typography.small,
    fontSize: 10,
    fontWeight: '800',
    color: '#FFFFFF',
    letterSpacing: 0.8,
  },
  title: {
    ...Typography.bodyBold,
    fontSize: 15,
    fontWeight: '800',
    color: '#18181B',
  },
  subtitle: {
    ...Typography.small,
    fontSize: 12,
    color: '#52525B',
  },
  right: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
  },
  chevronButton: {
    width: 34,
    height: 34,
    borderRadius: 17,
    backgroundColor: '#FFFFFF',
    alignItems: 'center',
    justifyContent: 'center',
    shadowColor: '#000000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 2,
  },
});
