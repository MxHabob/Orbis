# Orbis Mobile

## Styling

The app uses NativeWind 5 with Tailwind CSS 4. Metro processes `src/global.css`,
which is imported once by the Expo Router root layout. Keep utility class names
as complete static strings so Tailwind can discover them. Prefer `className`
for fixed layout and typography; retain style objects for runtime theme values,
animated values, or React Native APIs that require style objects.

`src/global.css` is the source of truth for the mobile design tokens and mirrors
the semantic palette, dark palette, typography, and radius scale from
`packages/ui/globals.css`. NativeWind classes should reference these tokens
directly; `useTheme` is reserved for native APIs and styles that require a
resolved color value.

The current NativeWind 5.0.0-rc.0 and react-native-css 3.1.0-rc.0 pair is
intentional: NativeWind's Expo 57 RC compatibility target matches this app's
React Native 0.86.3, React 19.2.3, Reanimated 4.5.1, and Worklets 0.10.1, and
requires Tailwind CSS newer than 4.1.11. The repository pins Tailwind CSS 4.3.1.
NativeWind 4.2.7 is the stable release, but its documented Tailwind 3 setup
does not match the app's Tailwind 4 constraint.

The Expo 57 NativeWind RC documents an upstream Android Reanimated limitation:
cancelling an animation can leave the final transform applied. See the
[NativeWind Expo 57 compatibility notes](https://github.com/nativewind/nativewind/blob/main/docs/rc-compatibility.md)
before relying on cancellation behavior.
