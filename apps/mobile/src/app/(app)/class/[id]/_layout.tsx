/**
 * @module @orbis/mobile/app/(app)/class/[id]/_layout
 *
 * Nested Stack Navigator for Teacher / Class Workflows.
 */

import React from 'react';
import { Stack } from 'expo-router';

export default function ClassContextLayout() {
  return (
    <Stack
      screenOptions={{
        headerShown: false,
        animation: 'slide_from_right',
      }}>
      <Stack.Screen name="index" options={{ headerShown: false }} />
      <Stack.Screen name="attendance" options={{ headerShown: false }} />
      <Stack.Screen name="grades" options={{ headerShown: false }} />
      <Stack.Screen name="schedule" options={{ headerShown: false }} />
    </Stack>
  );
}
