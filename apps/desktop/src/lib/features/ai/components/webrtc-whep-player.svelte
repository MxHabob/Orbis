<script lang="ts">
import { onMount } from "svelte";

let {
  whepUrl,
  connectingLabel,
  unavailableLabel,
}: {
  whepUrl: string;
  connectingLabel: string;
  unavailableLabel: string;
} = $props();

let videoElement = $state<HTMLVideoElement>();
let isConnecting = $state(true);
let hasError = $state(false);

function waitForIceGathering(peerConnection: RTCPeerConnection): Promise<void> {
  if (peerConnection.iceGatheringState === "complete") return Promise.resolve();

  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => {
      peerConnection.removeEventListener(
        "icegatheringstatechange",
        handleChange,
      );
      reject(new Error("Timed out gathering WebRTC ICE candidates"));
    }, 8000);
    const handleChange = () => {
      if (peerConnection.iceGatheringState === "complete") {
        clearTimeout(timeout);
        peerConnection.removeEventListener(
          "icegatheringstatechange",
          handleChange,
        );
        resolve();
      }
    };
    peerConnection.addEventListener("icegatheringstatechange", handleChange);
  });
}

onMount(() => {
  const endpoint = whepUrl;
  const video = videoElement;
  if (!endpoint || !video) return;

  let disposed = false;
  let peerConnection: RTCPeerConnection | undefined;
  let sessionUrl: string | undefined;
  const abortController = new AbortController();
  isConnecting = true;
  hasError = false;

  const releaseStream = () => {
    peerConnection?.close();
    peerConnection = undefined;
    video.srcObject = null;
    if (sessionUrl) {
      void fetch(sessionUrl, { method: "DELETE", keepalive: true }).catch(
        (error: unknown) => {
          console.warn("Could not close the WHEP session", error);
        },
      );
      sessionUrl = undefined;
    }
  };

  const connect = async () => {
    peerConnection = new RTCPeerConnection();
    peerConnection.addTransceiver("video", { direction: "recvonly" });
    peerConnection.addTransceiver("audio", { direction: "recvonly" });
    peerConnection.ontrack = (event) => {
      if (disposed) return;
      video.srcObject = event.streams[0] ?? new MediaStream([event.track]);
      void video.play().catch((error: unknown) => {
        console.warn("Could not autoplay the camera stream", error);
      });
      isConnecting = false;
    };
    peerConnection.onconnectionstatechange = () => {
      if (!disposed && peerConnection?.connectionState === "failed") {
        hasError = true;
        isConnecting = false;
      }
    };

    const offer = await peerConnection.createOffer();
    await peerConnection.setLocalDescription(offer);
    await waitForIceGathering(peerConnection);
    if (!peerConnection.localDescription?.sdp) {
      throw new Error("WebRTC did not produce a local session description");
    }

    const response = await fetch(endpoint, {
      method: "POST",
      headers: {
        Accept: "application/sdp",
        "Content-Type": "application/sdp",
      },
      body: peerConnection.localDescription.sdp,
      signal: abortController.signal,
    });
    if (!response.ok) {
      throw new Error(`WHEP request failed with HTTP ${response.status}`);
    }
    const location = response.headers.get("Location");
    if (location) {
      const session = new URL(location, endpoint);
      if (session.origin !== new URL(endpoint).origin) {
        throw new Error("WHEP session endpoint changed origin");
      }
      sessionUrl = session.toString();
    }

    const answer = await response.text();
    if (!answer.trim())
      throw new Error("WHEP returned an empty session description");
    await peerConnection.setRemoteDescription({ type: "answer", sdp: answer });
  };

  void connect().catch((error: unknown) => {
    if (disposed || abortController.signal.aborted) return;
    console.error("Could not start the WHEP camera stream", error);
    releaseStream();
    hasError = true;
    isConnecting = false;
  });

  return () => {
    disposed = true;
    abortController.abort();
    releaseStream();
  };
});
</script>

<video
  bind:this={videoElement}
  autoplay
  playsinline
  muted
  class="absolute inset-0 h-full w-full object-contain"
></video>

{#if isConnecting || hasError}
  <div class="absolute inset-0 flex items-center justify-center bg-black/70 p-4 text-center">
    <p class="text-xs text-white/80">
      {hasError ? unavailableLabel : connectingLabel}
    </p>
  </div>
{/if}
