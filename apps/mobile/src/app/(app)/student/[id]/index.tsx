import { StyleSheet, Text, View } from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { Typography, Fonts } from "@/constants/theme";

export default function StudentIndexScreen() {
  const theme = useTheme();

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Text style={[styles.title, { color: theme.text }]}>Student Details</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
  },
  title: {
    ...Typography.h1,
    fontFamily: Fonts.sans,
  },
});
