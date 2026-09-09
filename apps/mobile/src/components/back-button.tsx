import React from 'react';

import { Button } from '@/components/ui/button';
import { useI18n } from '@/lib/i18n';
import { useTheme } from '@/hooks/use-theme';
import { MobileIcon } from '@orbis/icons/mobile';

export interface BackButtonProps {
  onPress: () => void;
  accessibilityLabel?: string;
}

export function BackButton({ onPress, accessibilityLabel = 'Back' }: BackButtonProps) {
  const theme = useTheme();
  const { isArabic } = useI18n();

  return (
    <Button
      variant="secondary"
      size="icon-sm"
      onPress={onPress}
      accessibilityLabel={accessibilityLabel}
    >
      <MobileIcon
        name="arrow-back"
        size={17}
        color={theme.foreground}
        style={{ transform: [{ scaleX: isArabic ? -1 : 1 }] }}
      />
    </Button>
  );
}
