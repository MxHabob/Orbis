import React, { useState } from 'react';
import { Image, StyleSheet, Text, View } from 'react-native';
import { useTheme } from '@/hooks/use-theme';
import { Typography } from '@/constants/theme';

interface AvatarProps {
  name: string;
  imageUrl?: string | null;
  size?: number;
}

function getInitials(name: string) {
  const words = name.trim().split(/\s+/).filter(Boolean);
  if (words.length === 0) return '?';
  if (words.length === 1) return Array.from(words[0]).slice(0, 2).join('');
  return `${Array.from(words[0])[0]}${Array.from(words[1])[0]}`;
}

export const Avatar = React.memo<AvatarProps>(({ name, imageUrl, size = 64 }) => {
  const theme = useTheme();
  const [imageError, setImageError] = useState(false);
  const initials = getInitials(name);
  const showImage = Boolean(imageUrl) && !imageError;

  return (
    <View
      style={[
        styles.container,
        {
          width: size,
          height: size,
          borderRadius: size / 2,
          backgroundColor: theme.primaryLight,
        },
      ]}>
      {showImage ? (
        <Image
          source={{ uri: imageUrl! }}
          style={{ width: size, height: size, borderRadius: size / 2 }}
          resizeMode="cover"
          onError={() => setImageError(true)}
        />
      ) : (
        <Text style={[styles.initials, { color: theme.primary, fontSize: size * 0.3 }]}>
          {initials}
        </Text>
      )}
    </View>
  );
});

Avatar.displayName = 'Avatar';

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    justifyContent: 'center',
    overflow: 'hidden',
  },
  initials: {
    ...Typography.bodyBold,
    textAlign: 'center',
  },
});