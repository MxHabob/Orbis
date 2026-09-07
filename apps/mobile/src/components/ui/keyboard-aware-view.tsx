/**
 * @module @orbis/mobile/components/ui/keyboard-aware-view
 *
 * Universal keyboard protection wrapper for React Native forms and input screens.
 * Handles:
 * - Automatic keyboard dismissal on background taps
 * - KeyboardAvoidingView with platform-calibrated offsets
 * - ScrollView tap persistence without swallowing button events
 */

import React from 'react';
import {
  KeyboardAvoidingView,
  ScrollView,
  TouchableWithoutFeedback,
  Keyboard,
  Platform,
  StyleSheet,
  ViewStyle,
  StyleProp,
  View,
} from 'react-native';

interface KeyboardAwareViewProps {
  children: React.ReactNode;
  style?: StyleProp<ViewStyle>;
  contentContainerStyle?: StyleProp<ViewStyle>;
  keyboardVerticalOffset?: number;
  scrollable?: boolean;
}

export const KeyboardAwareView = React.memo<KeyboardAwareViewProps>(
  ({
    children,
    style,
    contentContainerStyle,
    keyboardVerticalOffset = 0,
    scrollable = true,
  }) => {
    const defaultOffset = Platform.OS === 'ios' ? (keyboardVerticalOffset || 20) : 0;

    if (!scrollable) {
      return (
        <KeyboardAvoidingView
          behavior={Platform.OS === 'ios' ? 'padding' : undefined}
          keyboardVerticalOffset={defaultOffset}
          style={[styles.container, style]}>
          <TouchableWithoutFeedback onPress={Keyboard.dismiss} accessible={false}>
            <View style={[styles.inner, contentContainerStyle]}>{children}</View>
          </TouchableWithoutFeedback>
        </KeyboardAvoidingView>
      );
    }

    return (
      <KeyboardAvoidingView
        behavior={Platform.OS === 'ios' ? 'padding' : undefined}
        keyboardVerticalOffset={defaultOffset}
        style={[styles.container, style]}>
        <TouchableWithoutFeedback onPress={Keyboard.dismiss} accessible={false}>
          <ScrollView
            style={styles.scrollView}
            contentContainerStyle={contentContainerStyle}
            keyboardShouldPersistTaps="handled"
            keyboardDismissMode="on-drag"
            showsVerticalScrollIndicator={false}>
            {children}
          </ScrollView>
        </TouchableWithoutFeedback>
      </KeyboardAvoidingView>
    );
  },
);

KeyboardAwareView.displayName = 'KeyboardAwareView';

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  inner: {
    flex: 1,
  },
  scrollView: {
    flex: 1,
  },
});
