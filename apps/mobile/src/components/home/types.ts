import type { ReactNode } from "react";

export type HomeTileData = {
  id: string;
  title: string;
  subtitle?: string;
  icon: ReactNode;
  onPress: () => void;
};
