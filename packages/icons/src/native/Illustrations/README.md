# Orbis Illustrations

Individual React Native SVG illustration components.

## Install
```bash
npx expo install react-native-svg
```

## Use
```tsx
import { Mathematics, StudentBoy, Attendance } from "@orbis/icons/native/Illustrations";

<Mathematics size={112} />
<StudentBoy size={96} />
<Attendance size={112} />
```

Files use `kebab-case` names while exported component names stay in `PascalCase`:
- `mathematics.tsx`
- `science.tsx`
- `english.tsx`
- `schedule.tsx`
- `assignments.tsx`
- `grades.tsx`
- `grade-7-math.tsx`
- `grade-8-science.tsx`
- `grade-10-math.tsx`
- `assignment-checklist.tsx`
- `student-boy.tsx`
- `student-girl.tsx`
- `attendance.tsx`
- `messages.tsx`
- `student-portrait.tsx` and `shared.tsx` are shared implementation helpers.
