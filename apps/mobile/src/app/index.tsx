import { useEffect } from 'react';
import { Button, StyleSheet, Text, View } from 'react-native';

import { useAuth } from '@/lib/auth-context';

export default function HomeScreen() {
  const { authState, login, restore, logout } = useAuth();

  useEffect(() => {
    void restore();
  }, [restore]);

  if (authState.status === 'loading') {
    return <View style={styles.container}><Text>Loading session...</Text></View>;
  }

  if (authState.status === 'anonymous') {
    return (
      <View style={styles.container}>
        <Text style={styles.title}>Sign in</Text>
        <Button title="Login demo" onPress={() => void login('admin@example.com', 'password')} />
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Authenticated</Text>
      <Text>{JSON.stringify(authState.user)}</Text>
      <Button title="Logout" onPress={() => void logout()} />
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
  title: {
    fontSize: 24,
    fontWeight: '700',
  },
});
