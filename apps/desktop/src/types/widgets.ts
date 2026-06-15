/** Widget dashboard preference items stored in user prefs. */
export type WidgetPreferenceItem = {
  id: string;
  visible: boolean;
  w: number;
  h: number;
  x?: number;
  y?: number;
  [key: string]: unknown;
};

export type WidgetPreferences = {
  primaryWidgets: WidgetPreferenceItem[];
  availableWidgets?: WidgetPreferenceItem[];
};
