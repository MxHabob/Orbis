import { useEffect } from 'react';
import { Button, StyleSheet, Text, View } from 'react-native';


export default function HomeScreen() {



  return (
    <View style={styles.container}>
      <Text style={styles.title}>Authenticated</Text>
      <Text>Test</Text>
      <Button title="Logout" onPress={() => {}} />
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
