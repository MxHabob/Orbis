# Camera integrations and live streaming

The app store lists Hikvision, Dahua, ONVIF, and RTSP as separate camera
integrations. Each uses the existing integration settings form. In the API,
all surveillance providers pass through one camera connection dispatcher,
which routes to the protocol-specific implementation.

Live playback uses a separately operated MediaMTX relay: it reads each
camera's RTSP stream and exposes its path through WebRTC/WHEP. Orbis does not
embed an RTSP server or transcode video.

## Configure the relay

Configure a MediaMTX path for each camera. For example:

```yaml
webrtc: yes
webrtcAddress: :8889
webrtcAllowOrigins:
  - https://orbis.example.com
webrtcAdditionalHosts:
  - stream.example.com

paths:
  campus-east:
    source: rtsp://CAMERA_USER:CAMERA_PASSWORD@192.168.1.100:554/Streaming/Channels/101
```

Replace the example host, path, and credentials. Protect the relay configuration
with filesystem permissions and a secret-management mechanism; do not commit
camera credentials. The relay host must be able to reach the camera over RTSP.
Put the WHEP listener behind HTTPS and restrict access with a private network
or firewall so only authorized Orbis clients can reach it. Configure MediaMTX
CORS to allow the Orbis application origin as an additional browser safeguard;
CORS is not authentication. Do not expose unauthenticated camera paths to the
public internet.

The matching Orbis endpoint is:

```text
https://stream.example.com/campus-east/whep
```

Enter this HTTPS WHEP URL in the camera integration. HTTP is accepted only for
localhost development. The WHEP URL must not contain credentials, query
parameters, or fragments.

Use an H.264 camera substream for the best browser compatibility and lower
bandwidth. MediaMTX relays packets without transcoding; choose a browser
compatible camera codec rather than enabling a CPU-intensive transcoding
pipeline. Configure a separate path and WHEP URL for every camera integration.

## Playback and resource limits

The 1×1 and 2×2 views start at most one or four streams respectively. The 3×3
view starts four streams; selecting another camera pauses the other feeds and
plays only the selected camera. Leaving the view closes peer connections and
deletes WHEP sessions when the relay advertises a session URL.

The browser connects directly to the configured HTTPS relay endpoint. It never
receives camera passwords or connects directly to the camera's RTSP endpoint.
WebRTC connectivity across NATs may require TURN servers configured on
MediaMTX. The reverse proxy and relay must allow the WebRTC ICE traffic as well
as HTTPS signaling.

The integrations currently verify device connectivity and display live video.
They do not provide motion events, recording, ONVIF discovery, PTZ control, or
attendance automation.
