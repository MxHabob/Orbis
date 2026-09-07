/**
 * @module @orbis/mobile/components/ui/dialog
 *
 * S.C.R.A.P.S. Modal Dialog Component for Orbis Mobile.
 * Synchronized with @orbis/ui design system tokens.
 */

import React, { createContext, useContext, useEffect, useState } from 'react';
import {
  Modal,
  Pressable,
  type StyleProp,
  StyleSheet,
  Text,
  type TextStyle,
  View,
  type ViewStyle,
} from 'react-native';
import Animated, { ZoomIn } from 'react-native-reanimated';
import { useTheme } from '@/hooks/use-theme';
import { useColorScheme } from '@/hooks/use-color-scheme';
import { Radii, Spacing, Typography } from '@/constants/theme';

// ─── Dialog Context ─────────────────────────────────────────────────────────

interface DialogContextValue {
  onClose: () => void;
  theme: ReturnType<typeof useTheme>;
  isDark: boolean;
}

const DialogContext = createContext<DialogContextValue | null>(null);

export function useDialog() {
  const context = useContext(DialogContext);
  if (!context) {
    throw new Error('Dialog compound components must be used within a <Dialog />');
  }
  return context;
}

// ─── Dialog Root ────────────────────────────────────────────────────────────

export interface DialogProps {
  visible: boolean;
  onClose: () => void;
  children: React.ReactNode;
}

export function Dialog({ visible, onClose, children }: DialogProps) {
  const theme = useTheme();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === 'dark';
  const [shouldRender, setShouldRender] = useState(visible);

  // Synchronize render status to prevent 1-frame React delay when mounting
  if (visible && !shouldRender) {
    setShouldRender(true);
  }

  useEffect(() => {
    if (!visible) {
      const timer = setTimeout(() => {
        setShouldRender(false);
      }, 200);
      return () => clearTimeout(timer);
    } else {
      setShouldRender(true);
    }
  }, [visible]);

  if (!shouldRender && !visible) return null;

  return (
    <DialogContext.Provider value={{ onClose, theme, isDark }}>
      <Modal
        visible={visible}
        transparent
        animationType="fade"
        statusBarTranslucent
        onRequestClose={onClose}
      >
        <View style={styles.backdrop}>
          <Pressable style={StyleSheet.absoluteFill} onPress={onClose} />
          {children}
        </View>
      </Modal>
    </DialogContext.Provider>
  );
}

// ─── Dialog Content ─────────────────────────────────────────────────────────

export interface DialogContentProps {
  children: React.ReactNode;
  style?: StyleProp<ViewStyle>;
}

export function DialogContent({ children, style }: DialogContentProps) {
  const { theme } = useDialog();

  return (
    <Animated.View
      entering={ZoomIn.duration(180)}
      style={[
        styles.container,
        {
          backgroundColor: theme.card,
          borderColor: theme.border,
        },
        style,
      ]}
    >
      {children}
    </Animated.View>
  );
}

// ─── Dialog Header ──────────────────────────────────────────────────────────

export interface DialogHeaderProps {
  children: React.ReactNode;
  style?: StyleProp<ViewStyle>;
}

export function DialogHeader({ children, style }: DialogHeaderProps) {
  return <View style={[styles.header, style]}>{children}</View>;
}

// ─── Dialog Footer ──────────────────────────────────────────────────────────

export interface DialogFooterProps {
  children: React.ReactNode;
  style?: StyleProp<ViewStyle>;
}

export function DialogFooter({ children, style }: DialogFooterProps) {
  return <View style={[styles.footer, style]}>{children}</View>;
}

// ─── Dialog Title ───────────────────────────────────────────────────────────

export interface DialogTitleProps {
  children: React.ReactNode;
  style?: StyleProp<TextStyle>;
}

export function DialogTitle({ children, style }: DialogTitleProps) {
  const { theme } = useDialog();

  return (
    <Text style={[styles.title, { color: theme.text }, style]}>
      {children}
    </Text>
  );
}

// ─── Dialog Description ─────────────────────────────────────────────────────

export interface DialogDescriptionProps {
  children: React.ReactNode;
  style?: StyleProp<TextStyle>;
}

export function DialogDescription({ children, style }: DialogDescriptionProps) {
  const { theme } = useDialog();

  return (
    <Text style={[styles.description, { color: theme.textSecondary }, style]}>
      {children}
    </Text>
  );
}

// ─── Dialog Close Wrapper ───────────────────────────────────────────────────

export interface DialogCloseProps {
  children: React.ReactElement<{ onPress?: () => void }>;
}

export function DialogClose({ children }: DialogCloseProps) {
  const { onClose } = useDialog();

  if (React.isValidElement(children)) {
    const child = children as React.ReactElement<{ onPress?: () => void }>;
    return React.cloneElement(child, {
      onPress: () => {
        if (child.props.onPress) {
          child.props.onPress();
        }
        onClose();
      },
    });
  }

  return children;
}

// ─── Styles ──────────────────────────────────────────────────────────────────

const styles = StyleSheet.create({
  backdrop: {
    flex: 1,
    backgroundColor: 'rgba(0, 0, 0, 0.52)',
    justifyContent: 'center',
    alignItems: 'center',
    padding: Spacing.six,
  },
  container: {
    width: '100%',
    maxWidth: 380,
    borderRadius: Radii['2xl'],
    borderWidth: 1,
    padding: Spacing.six,
    shadowColor: '#000000',
    shadowOffset: { width: 0, height: 8 },
    shadowOpacity: 0.16,
    shadowRadius: 18,
    elevation: 8,
  },
  header: {
    marginBottom: Spacing.four,
    gap: Spacing.one,
  },
  title: {
    ...Typography.h2,
    fontWeight: '700',
    letterSpacing: -0.3,
  },
  description: {
    ...Typography.body,
    lineHeight: 20,
  },
  footer: {
    flexDirection: 'row',
    justifyContent: 'flex-end',
    alignItems: 'center',
    gap: Spacing.three,
    marginTop: Spacing.six,
  },
});
