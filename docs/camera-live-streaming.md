# Camera live streaming

Orbis Desktop bundles MediaMTX and manages it automatically. Users do not need
to install a separate relay, configure WHEP, or expose camera feeds to the
internet.

## What users configure

When adding a camera, enter its API address when the selected integration asks
for one, its RTSP stream URL, and its camera username and password. Keep the
credentials out of the RTSP URL; Orbis supplies them to the local relay
separately. For example, a Hikvision main stream commonly uses
`rtsp://192.168.1.100:554/Streaming/Channels/101`. Camera models may use
different paths or ports.

The camera must be reachable from the computer running Orbis Desktop. The RTSP
URL is required for live video even when the camera also supports Hikvision
ISAPI, Dahua CGI, or ONVIF.

## Resource use and privacy

- MediaMTX starts when the live camera view opens and is stopped when the view
  closes or Orbis exits.
- RTSP sources use on-demand mode and are pulled only while a viewer is
  connected; inactive sources close after a short timeout.
- The relay binds to loopback addresses on dynamically selected local ports.
  Camera credentials are passed directly to the relay and are not included in
  browser-visible settings or the temporary relay configuration.
- MediaMTX relays the stream without transcoding. H.264 without B-frames is
  recommended for broad WebRTC browser compatibility.
- The live view starts at most four streams at once. Selecting another camera
  pauses the other feeds.

Network firewalls can prevent a camera or a local UDP connection from working.
The user computer needs network access to the camera's RTSP port, and local
loopback UDP must be allowed for WebRTC. No inbound internet port forwarding is
required.

## Packaging

Tauri packages the platform-specific MediaMTX executable with Orbis Desktop.
`bun run prepare:mediamtx` downloads the pinned upstream release and verifies
its SHA-256 digest before extraction. It runs automatically before Tauri
development and production builds. MediaMTX is distributed under the MIT
license; the license is included in the desktop bundle.
