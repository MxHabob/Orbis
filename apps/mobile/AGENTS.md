# Mobile Product Engineering Standards

## Project Stack

This project uses Expo, React Native, NativeWind, React Native Reanimated, and Expo Router.

An existing global.css and design system are present. Inspect the actual project before making changes.

## Mandatory Workflow

Before implementing a screen:
1. Inspect the relevant route, parent layouts, shared components, and theme tokens.
2. Understand the user's task and the screen's primary action.
3. Choose a layout appropriate to the content rather than reusing a generic template.
4. Identify relevant loading, empty, error, success, and disabled states.

After implementation:
1. Check types, imports, and project validation commands.
2. Verify safe areas, navigation chrome, keyboard behavior, and responsive layouts.
3. Verify Arabic and RTL behavior where applicable.
4. Inspect the running UI when preview or device tooling is available.
5. Report what was actually tested and what remains unverified.

## Design Principles

- Create elegant, restrained, production-quality mobile interfaces.
- Preserve the existing visual identity and shared components.
- Use consistent typography, spacing, alignment, and visual hierarchy.
- Choose screen composition according to its purpose.
- Avoid repetitive card layouts, unnecessary decoration, arbitrary colors, and oversized headings.
- Design empty, loading, error, and success states intentionally.
- Prioritize usability, readability, and accessibility.

## Architecture

- Expo Router is the project's navigation entry point.
- Preserve the current route structure and layout ownership.
- Do not recreate existing navigation headers or tab bars inside individual screens.
- Keep reusable UI components outside route directories where appropriate.
- Preserve existing NativeWind, Reanimated, and theme configuration.

## Responsive Mobile Layout

- Prefer flexible layouts and content-driven sizing.
- Respect safe areas and navigation-owned insets.
- Handle keyboard interactions for forms.
- Avoid arbitrary fixed screen dimensions and device-specific offsets.
- Support narrow screens, larger screens, text wrapping, and RTL where applicable.

## Dependencies

- Inspect installed versions before using APIs.
- Do not install a library without identifying a concrete need.
- Do not upgrade core dependencies or rewrite navigation as part of an unrelated task.
- Preserve existing behavior unless a change is explicitly required.

## Definition of Done

A screen is complete only when its design, important states, responsive behavior, and integration with the existing application have been checked.

Compilation alone does not establish visual correctness. Never claim to have run a test or visually inspected a screen unless that actually happened.