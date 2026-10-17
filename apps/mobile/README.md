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
server over the local network, and the server must use HTTPS with a certificate
trusted by the device. The app checks `/api/v1/health` before saving the
institution; it does not disable TLS certificate validation to support
self-signed certificates.

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

## Over-the-air updates

The version row in Preferences uses `expo-updates` to check and download an
EAS Update for the installed app runtime. Set `EAS_PROJECT_ID` to the Expo
project UUID when resolving the app config for native builds and update
publishes. This enables the `https://u.expo.dev/<project-id>` update URL and
the `appVersion` runtime policy. Without that ID, OTA updates remain disabled
and the manual check reports that setup is required. A new native build is
required after enabling the Expo Updates native module.
