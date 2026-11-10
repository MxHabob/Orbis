# Orbis Mobile

## Styling

The app uses NativeWind 4.2.7 with Tailwind CSS 3.4. Metro processes
`src/global.css` through `nativewind/metro`; the file is imported once by the
Expo Router root layout. Keep utility class names as complete static strings so
Tailwind can discover them. Prefer `className` for fixed layout and typography;
retain style objects for runtime theme values, animated values, or React Native
APIs that require style objects.

`src/global.css` is the source of truth for mobile design tokens and mirrors the
semantic and dark palettes from `packages/ui/globals.css`. Color variables use
RGB channels so Tailwind's opacity utilities and NativeWind resolve the same
tokens in light and dark mode. `tailwind.config.js` maps utility colors,
spacing, typography, and radii to those tokens. `useTheme` is for native APIs
and styles that require a resolved color value.

The app currently pairs NativeWind 4.2.7 and Tailwind CSS 3.4.17 with Expo SDK
58 and React Native 0.88.1. NativeWind's v4.2.7 installation notes explicitly
mention Expo SDK 57 support; verify the Android native build and theme behavior
when upgrading this SDK combination.

## Offline cache and keyboard

The native offline query cache is encrypted with SQLCipher. The
`expo-sqlite` config plugin enables SQLCipher, but this is a native build-time
setting: after changing it or adding a native dependency, rebuild and reinstall
the app with `bun run android:device` from `apps/mobile`. Expo Go does not
include SQLCipher and cannot restore the encrypted cache.

`KeyboardProvider` is mounted globally in `AppProviders`, so keyboard
controller hooks and components are available throughout every route.
`ScreenScrollView` and `KeyboardAwareView` use the controller's keyboard-aware
scrolling, and form drawers should use `DrawerScrollView` with `DrawerFooter`
to keep focused fields and actions reachable while typing. Android uses the
`resize` soft-input layout mode; rebuild the native app after changing this
setting.

## Local institution connections

The institution picker automatically discovers Orbis API servers advertising
`_orbis-api._tcp.local.` over mDNS/Bonjour, places them above the public
institution directory, and connects when a server is selected. The API server
advertises its institution name and ID while listening on a non-loopback
address. The phone and server must be on the same local network.

Automatic local discovery requires an Orbis development/native build; Expo Go
does not include the native mDNS module. iOS declares the Bonjour service and
local-network usage description in `app.json`. Android includes the local
network permission required by Android 17. Rebuild and reinstall after adding
or changing native permissions or dependencies.

Use the Orbis development build for local UI work as well, so Android status
bar, safe-area, and window-layout behavior match the standalone app. Install it
with `bun run mobile:android`, then start Metro with `bun run dev:mobile` (or
`bun run dev:mobile:tunnel`). These commands use the development client; Expo
Go does not apply Orbis's native config plugins and can render system insets
differently.

HTTPS with a certificate trusted by the device is required for internet-facing
servers. For a trusted local network, HTTP is also accepted when the server
resolves to a private IPv4 address (`10.x.x.x`, `172.16.x.x`–`172.31.x.x`, or
`192.168.x.x`). Do not use HTTP on a public or untrusted Wi-Fi network: traffic,
including sign-in credentials, is not encrypted. Android cleartext networking
is enabled for local development only when `ORBIS_DEV_ALLOW_LOCAL_HTTP=true`
is set while resolving Expo config and building the app. The app verifies
`/api/v1/health` before saving the institution.

## Mobile app version checks

The installed app version is read from the native app metadata
(`Constants.nativeAppVersion`, falling back to `Constants.expoConfig.version`).
It is compared with the highest-version published, stable GitHub release tagged
`mobile-vMAJOR.MINOR.PATCH`.
Desktop releases and EAS/Expo SDK versions are deliberately excluded. When a
newer mobile release is found, the app opens its GitHub release page so the user
can download the appropriate artifact.

Publish mobile app releases with a dedicated tag such as `mobile-v1.2.3`.
Keep the tag in strict `MAJOR.MINOR.PATCH` format and publish the release (not
as a draft or prerelease) after its installable artifacts are uploaded. The
mobile checker only reports an update when such a release exists; it does not
install native app binaries automatically. EAS Update remains a separate
over-the-air JavaScript update mechanism.

## Android release build

From the repository root, run:

```sh
bun run build:mobile:android
```

This generates the Android native project from the Expo config and builds a
local production APK with R8 code shrinking, resource shrinking, and PNG
crunching enabled. It builds only the `arm64-v8a` native architecture to keep
the APK smaller; it is suitable for 64-bit ARM Android phones and will not run
on x86 devices or 32-bit-only phones. The APK is
`apps/mobile/android/app/build/outputs/apk/release/app-release.apk`.
Android builds use SDK CMake 3.31.6. Its bundled Ninja supports the long native
source paths generated by React Native on Windows; the Ninja bundled with
CMake 3.22.1 does not.
Gradle runs offline by default, so required Android SDK/NDK and build
dependencies must already be installed locally. The GitHub Actions release
build runs Gradle online so uncached plugins and dependencies can be resolved.
For other CI builds that use the local build script, set
`ORBIS_GRADLE_OFFLINE=false`.

The build uses the app version in `app.json`. Before publishing a new release,
update `expo.version` and increase `expo.android.versionCode` so Android and the
app's `mobile-vMAJOR.MINOR.PATCH` release tag identify the new version.
