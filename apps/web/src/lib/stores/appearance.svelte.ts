export type FontSize = "default" | "large" | "larger";
export type MotionPreference = "system" | "reduce" | "full";
export type ToastPosition =
  | "top-left"
  | "top-center"
  | "top-right"
  | "bottom-left"
  | "bottom-center"
  | "bottom-right";

interface AppearancePreferences {
  fontSize: FontSize;
  highContrast: boolean;
  motion: MotionPreference;
  toastPosition: ToastPosition;
}

const STORAGE_KEY = "orbis_appearance_preferences";

const DEFAULTS: AppearancePreferences = {
  fontSize: "default",
  highContrast: false,
  motion: "system",
  toastPosition: "top-center",
};

function isFontSize(value: unknown): value is FontSize {
  return value === "default" || value === "large" || value === "larger";
}

function isMotionPreference(value: unknown): value is MotionPreference {
  return value === "system" || value === "reduce" || value === "full";
}

function isToastPosition(value: unknown): value is ToastPosition {
  return (
    value === "top-left" ||
    value === "top-center" ||
    value === "top-right" ||
    value === "bottom-left" ||
    value === "bottom-center" ||
    value === "bottom-right"
  );
}

class AppearanceStore {
  fontSize = $state<FontSize>(DEFAULTS.fontSize);
  highContrast = $state(DEFAULTS.highContrast);
  motion = $state<MotionPreference>(DEFAULTS.motion);
  toastPosition = $state<ToastPosition>(DEFAULTS.toastPosition);

  constructor() {
    this.restore();
  }

  setFontSize(value: FontSize) {
    this.fontSize = value;
    this.persist();
  }

  setHighContrast(value: boolean) {
    this.highContrast = value;
    this.persist();
  }

  setMotion(value: MotionPreference) {
    this.motion = value;
    this.persist();
  }

  setToastPosition(value: ToastPosition) {
    this.toastPosition = value;
    this.persist();
  }

  reset() {
    this.fontSize = DEFAULTS.fontSize;
    this.highContrast = DEFAULTS.highContrast;
    this.motion = DEFAULTS.motion;
    this.toastPosition = DEFAULTS.toastPosition;
    this.persist();
  }

  private restore() {
    if (typeof localStorage === "undefined") return;

    try {
      const saved = localStorage.getItem(STORAGE_KEY);
      if (saved) {
        const parsed: unknown = JSON.parse(saved);
        if (typeof parsed === "object" && parsed !== null) {
          const preferences = parsed as Partial<AppearancePreferences>;
          if (isFontSize(preferences.fontSize))
            this.fontSize = preferences.fontSize;
          if (typeof preferences.highContrast === "boolean") {
            this.highContrast = preferences.highContrast;
          }
          if (isMotionPreference(preferences.motion)) {
            this.motion = preferences.motion;
          }
          if (isToastPosition(preferences.toastPosition)) {
            this.toastPosition = preferences.toastPosition;
          }
        }
      }
    } catch (error) {
      console.error("Failed to restore appearance preferences:", error);
    }

    this.applyToDOM();
  }

  private persist() {
    this.applyToDOM();
    if (typeof localStorage === "undefined") return;

    try {
      localStorage.setItem(
        STORAGE_KEY,
        JSON.stringify({
          fontSize: this.fontSize,
          highContrast: this.highContrast,
          motion: this.motion,
          toastPosition: this.toastPosition,
        } satisfies AppearancePreferences),
      );
    } catch (error) {
      console.error("Failed to save appearance preferences:", error);
    }
  }

  applyToDOM() {
    if (typeof document === "undefined") return;

    const root = document.documentElement;
    root.dataset.fontSize = this.fontSize;
    root.dataset.contrast = this.highContrast ? "high" : "normal";
    root.dataset.motion = this.motion;
  }
}

export const appearanceStore = new AppearanceStore();
