import { StyleSheet, Text, View } from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { Typography, Fonts } from "@/constants/theme";

export default function StudentExamsScreen() {
  const theme = useTheme();

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Text style={[styles.title, { color: theme.text }]}>Student Exams</Text>
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
