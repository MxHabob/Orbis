# Orbis Mobile Design System & Foundation Guide

## 1. Overview & Architecture

The Orbis Mobile foundation provides a unified, production-grade architecture built on **React Native 0.86**, **Expo 57**, **Expo Router 57**, **TypeScript**, and **Reanimated 4.5**.

All mobile screens are built from standardized primitives, providers, tokens, navigation patterns, and visual components ensuring consistency, accessibility, RTL support, and high performance.

### Provider Hierarchy (`src/app/_layout.tsx`)

The root application wraps components in the following strict order:

```tsx
<GestureHandlerRootView style={{ flex: 1 }}>
  <SafeAreaProvider>
    <I18nProvider>
      <AuthProvider>
        <AppQueryProvider>
          <EffectiveContextProvider>
            <ThemeProvider value={navigationTheme}>
              <ToastProvider>
                <AppErrorBoundary>
                  <ThemedStatusBar />
                  <Stack />
                </AppErrorBoundary>
              </ToastProvider>
            </ThemeProvider>
          </EffectiveContextProvider>
        </AppQueryProvider>
      </AuthProvider>
    </I18nProvider>
  </SafeAreaProvider>
</GestureHandlerRootView>
```

---

## 2. Design Tokens (`src/constants/theme.ts`)

### Color System & Palette
- **Light & Dark Mode**: Full dual-theme token definitions (`Colors.light`, `Colors.dark`).
- **Brand Colors**:
  - `primary`: `#7C3AED` (Light) / `#9061F9` (Dark)
  - `brandLime`: `#84CC16`
  - `brandPink`: `#EC4899`
  - `brandIndigo`: `#6366F1`
  - `brandRose`: `#F43F5E`
- **Semantic Surfaces**:
  - `background`: Canvas background
  - `card`, `cardElevated`, `cardBorder`: Card containers
  - `backgroundElement`, `backgroundSelected`: Interactive elements
  - `border`, `borderSubtle`: Hairlines & dividers
  - `text`, `textSecondary`, `textMuted`: Typographic hierarchy

### Typography Presets
All text presets include normalized `fontSize`, `lineHeight`, and `fontWeight`:
- `Typography.display` (32px / 38px / 800)
- `Typography.h1` (24px / 30px / 800)
- `Typography.h2` (18px / 24px / 700)
- `Typography.h3` (16px / 22px / 600)
- `Typography.title` (15px / 20px / 600)
- `Typography.bodyLarge` (16px / 24px / 400)
- `Typography.bodyMedium` (14px / 20px / 400)
- `Typography.bodySmall` (13px / 18px / 400)
- `Typography.caption` (12px / 16px / 400)
- `Typography.small` (11px / 14px / 500)
- `Typography.mono` (13px / 18px / 400)

### Spacing Scale
- `Spacing.half`: 2px
- `Spacing.one`: 4px
- `Spacing.two`: 8px
- `Spacing.three`: 12px
- `Spacing.four`: 16px
- `Spacing.five`: 20px
- `Spacing.six`: 24px
- `Spacing.eight`: 32px
- `Spacing.ten`: 40px
- `Spacing.twelve`: 48px
- `Spacing.sixteen`: 64px

### Radii & Shadows
- **Radii**: `none` (0), `xs` (2), `sm` (4), `md` (6), `lg` (8), `xl` (12), `2xl` (16), `3xl` (20), `full` (9999)
- **Shadows**: `Shadows.sm`, `Shadows.md`, `Shadows.lg` (with cross-platform iOS shadow and Android elevation support)
- **Control Heights**: `ControlHeights.sm` (36), `ControlHeights.md` (44), `ControlHeights.lg` (52)
- **Touch Targets**: Minimum 44×44px hitSlop compliance (`TouchTargets.min`)

---

## 3. Layout Primitives (`@/components/layout`)

| Component | Description | Key Props |
| :--- | :--- | :--- |
| `Screen` | Root page container with safe areas, status bar, and keyboard handling | `edges`, `header`, `footer`, `keyboardAvoiding`, `backgroundColor` |
| `ScreenScrollView` | Screen wrapper with embedded `ScrollView` and pull-to-refresh | `refreshing`, `onRefresh`, `contentPadding`, `horizontalPadding` |
| `Section` | Structured section container with title, subtitle, and action slot | `title`, `subtitle`, `action`, `spacing`, `headerSpacing` |
| `Stack` / `Row` / `Col` | Flex layout with theme gap support and direction controls | `gap`, `align`, `justify`, `wrap`, `fullWidth` |
| `Surface` | Elevation-aware container with radii and borders | `variant` (`flat`, `elevated`, `outlined`, `sunken`), `radius`, `padding` |
| `Divider` | Semantic hairline or styled separator | `orientation` (`horizontal`, `vertical`), `thickness`, `color`, `spacing` |

### Layout Example
```tsx
import { ScreenScrollView, Section, Row, Surface, Divider } from "@/components/layout";
import { Header } from "@/components/header";
import { Button } from "@/components/ui";

export default function ExampleScreen() {
  return (
    <ScreenScrollView
      header={<Header title="My Screen" showBack />}
      horizontalPadding="four"
    >
      <Section title="Overview" subtitle="Key metrics and status">
        <Surface variant="elevated" padding="four">
          <Row justify="space-between">
            {/* Content */}
          </Row>
        </Surface>
      </Section>
    </ScreenScrollView>
  );
}
```

---

## 4. Universal Header & Tab Navigation

### `Header` (`@/components/header`)
The Orbis header provides:
- Floating pill structure with dynamic marquee for long titles.
- Automatic RTL back-button mirroring (`isArabic ? <ChevronRight /> : <ChevronLeft />`).
- Profile trigger, action slots (`leftAction`, `rightAction`), and `largeTitle` support.

### `FloatingTabBar` (`@/components/floating-tab-bar`)
The signature Orbis 3-pill floating tab bar:
- `[ Apps ]` (Route index + New badge)
- `[ Digital ID ]` (Central quick action modal trigger)
- `[ Analytics ]` (Behavior / AI route)
- Auto-hides smoothly with spring animations when the keyboard appears.
- Fully safe-area aware on all devices.

---

## 5. UI Primitives (`@/components/ui`)

All primitives are exported from `@/components/ui`:

- **Buttons & Controls**:
  - `Button`: Tactile 3D button with variants (`default`, `secondary`, `outline`, `destructive`, `lime`, `ghost`, `link`), sizes (`xs`, `sm`, `md`, `lg`, `icon`), and loading states.
  - `IconButton`: Accessible circular/square icon button with automatic hitSlop expansion.
  - `FloatingActionButton`: Floating action button with corner/center anchors and label extension.
  - `Switch`, `Checkbox`, `Radio`, `RadioGroup`: Form selection controls.
  - `SegmentedControl`: Segmented tab switch with tactile elevation.
- **Inputs & Search**:
  - `Input`: Text input with error messaging, label, and tactile borders.
  - `SearchInput`: Search bar with clear button, debounce hooks, and `@orbis/icons/native` search glyph.
- **Lists & Data Display**:
  - `ListItem`: Row component with left icon/avatar, title, subtitle, badge, and trailing action/chevron.
  - `Card`: Surface card for grouped information.
  - `Chip`: Selectable and deletable filter chip / tag.
  - `Badge`: Status badges with semantic variants.
- **Feedback & Overlays**:
  - `Skeleton`: Shimmering placeholder skeletons.
  - `LoadingState`: Fullscreen or inline loading indicators.
  - `EmptyState`: Empty state container with icon, title, description, and primary CTA.
  - `Toast`: Global notification toast via `useToast()`.
  - `Dialog` & `Drawer`: Accessible modal dialogs and bottom sheets.

---

## 6. Icons & RTL Internationalization

### Custom Icon System (`@orbis/icons/native`)
Always import native icons from `@orbis/icons/native`. Do **NOT** use external icon packs (e.g. Lucide, FontAwesome).
```tsx
import {
  Grid,
  Document,
  ChartBar,
  ChevronLeft,
  ChevronRight,
  User,
  AppNotification,
  Search,
  CheckCircle,
  InfoCircle,
} from "@orbis/icons/native";
```

### RTL & Arabic Localization (`@/lib/i18n`)
- Use `useI18n()` hook for translations:
```tsx
const { t, isArabic, dir } = useI18n();
```
- Direction-aware chevrons and alignments automatically adapt based on `isArabic`.

---

## 7. Guidelines for Screen Development

1. **Root Screen Wrapping**: Always wrap screens using `Screen` or `ScreenScrollView`. Do not manually calculate raw `SafeAreaView` or hardcode notch heights.
2. **Spacing Tokens**: Avoid magic numbers for layout spacing; always use `Spacing.*` tokens.
3. **Typography**: Always reference `Typography.*` presets to maintain font consistency and scaling.
4. **Theme Awareness**: Use `useTheme()` to retrieve the dynamic color palette. Never hardcode hex codes directly inside screen styles.
5. **No Mock Data / Fallbacks**: Expose loading, error, and empty states explicitly using `LoadingState`, `EmptyState`, and `AppErrorBoundary`.
