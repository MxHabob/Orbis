import React from 'react';
import { Button, StyleSheet, Text, View, ActivityIndicator } from 'react-native';
import { useAuth } from '@/lib/api';

export default function HomeScreen() {
  const { user, isAuthenticated, isLoading, logout, error } = useAuth();

  if (isLoading) {
    return (
      <View style={styles.container}>
        <ActivityIndicator size="large" color="#4f46e5" />
        <Text style={styles.subtitle}>جاري تحميل الجلسة...</Text>
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Orbis Mobile Platform</Text>
      {isAuthenticated && user ? (
        <View style={styles.card}>
          <Text style={styles.name}>{user.full_name || user.email}</Text>
          <Text style={styles.subtitle}>{user.email}</Text>
          <Text style={styles.badge}>الحالة: {user.status}</Text>
          <Button title="تسجيل الخروج (Logout)" color="#ef4444" onPress={() => logout()} />
        </View>
      ) : (
        <View style={styles.card}>
          <Text style={styles.subtitle}>غير مسجل الدخول</Text>
          {error ? <Text style={styles.errorText}>{error}</Text> : null}
        </View>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 24,
    gap: 16,
  },
  card: {
    padding: 20,
    borderRadius: 12,
    backgroundColor: '#ffffff10',
    alignItems: 'center',
    gap: 12,
    width: '100%',
    maxWidth: 360,
  },
  title: {
    fontSize: 24,
    fontWeight: '700',
    marginBottom: 8,
  },
  name: {
    fontSize: 18,
    fontWeight: '600',
  },
  subtitle: {
    fontSize: 14,
    color: '#888888',
  },
  badge: {
    fontSize: 12,
    color: '#4f46e5',
    backgroundColor: '#4f46e515',
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 6,
  },
  errorText: {
    color: '#ef4444',
    fontSize: 13,
  },
});

