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

Teachers can connect to a school server without internet access by joining the
school Wi-Fi and scanning its Orbis QR code on the institution selection screen.
Manual server entry is also available. The phone must be able to reach the
server over the local network. HTTPS with a certificate trusted by the device
is required for internet-facing servers. For a trusted local network, HTTP is
also accepted when the endpoint uses a private IPv4 address (`10.x.x.x`,
`172.16.x.x`–`172.31.x.x`, or `192.168.x.x`). Do not use this HTTP option on a
public or untrusted Wi-Fi network: traffic, including sign-in credentials, is
not encrypted. The app checks `/api/v1/health` before saving the institution.
Android cleartext networking is enabled at the native app level for this
feature only when `ORBIS_DEV_ALLOW_LOCAL_HTTP=true` is set while resolving the
Expo config and building the Android app. The app only accepts HTTP institution
endpoints in the private IPv4 ranges above. Rebuild the native Android app after
changing this setting; a JavaScript reload is not enough.

The QR code contains a JSON document in this shape:

```json
{
  "type": "orbis-institution",
  "version": 1,
  "institution": {
    "organization_id": "organization-id",
    "name": "Example School",
    "name_ar": null,
    "slug": "example-school",
    "deployment_id": "school-deployment-id",
    "deployment_name": "Example School",
    "endpoint_url": "https://orbis.school.local"
  }
}
```

`logo_url` and coordinates are optional. Distribute QR codes through an
official school channel; the app validates their structure and verifies the
server connection over HTTPS, but QR payloads are not cryptographically signed.
When a Wi-Fi network has no internet, the app still treats its local network
connection as available for API requests; requests to internet-only services
will remain unavailable.

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
Gradle runs offline, so required Android SDK/NDK and build dependencies must
already be installed locally.

The build uses the app version in `app.json`. Before publishing a new release,
update `expo.version` and increase `expo.android.versionCode` so Android and the
app's `mobile-vMAJOR.MINOR.PATCH` release tag identify the new version.
