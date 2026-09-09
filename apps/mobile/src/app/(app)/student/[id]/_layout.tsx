/**
 * @module @orbis/mobile/app/(app)/student/[id]/_layout
 *
 * Nested Stack Navigator for Student & Child Context Workflows.
 */

import { Stack } from 'expo-router';

export default function StudentContextLayout() {
  return (
    <Stack
      screenOptions={{
        headerShown: false,
        animation: 'slide_from_right',
      }}>
      <Stack.Screen name="index" options={{ headerShown: false }} />
      <Stack.Screen name="grades" options={{ headerShown: false }} />
      <Stack.Screen name="attendance" options={{ headerShown: false }} />
      <Stack.Screen name="schedule" options={{ headerShown: false }} />
      <Stack.Screen name="exams" options={{ headerShown: false }} />
      <Stack.Screen name="fees" options={{ headerShown: false }} />
    </Stack>
  );
}
