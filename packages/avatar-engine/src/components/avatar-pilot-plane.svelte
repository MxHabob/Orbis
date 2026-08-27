<script lang="ts">
  import Avatar from "./avatar.svelte";

  let {
    mood = "steady",
    width = "138px",
    height = "82px",
  }: {
    mood?: "steady" | "alert" | "working";
    width?: string;
    height?: string;
  } = $props();
</script>

<div class="pilot-flight {mood}" style:width style:height role="img" aria-label="Pilot assistant">
  <div class="flight-cloud cloud-back"></div>
  <div class="flight-cloud cloud-front"></div>
  <div class="pilot-plane">
    <div class="plane-tail"></div>
    <div class="plane-wing plane-wing-top"></div>
    <div class="plane-wing plane-wing-bottom"></div>
    <div class="plane-body"></div>
    <div class="plane-cockpit">
      <Avatar hat="pilot" width="42px" height="42px" />
    </div>
    <div class="plane-nose"></div>
    <div class="plane-window"></div>
  </div>
  <span class="flight-spark spark-one"></span>
  <span class="flight-spark spark-two"></span>
</div>

<style>
  .pilot-flight {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: visible;
    user-select: none;
    animation: flight-bob 3.2s ease-in-out infinite;
  }

  .pilot-plane {
    position: relative;
    z-index: 2;
    width: 112px;
    height: 58px;
    transform: rotate(-4deg);
    animation: plane-fly 3.8s ease-in-out infinite;
    filter: drop-shadow(0 7px 5px rgb(15 23 42 / 18%));
  }

  .plane-body,
  .plane-wing,
  .plane-tail,
  .plane-nose,
  .plane-window {
    position: absolute;
    display: block;
  }

  .plane-body {
    top: 24px;
    left: 12px;
    width: 88px;
    height: 22px;
    border: 2px solid #172554;
    border-radius: 45% 52% 48% 42%;
    background: linear-gradient(180deg, #60a5fa 0%, #2563eb 52%, #1d4ed8 100%);
  }

  .plane-nose {
    top: 27px;
    right: 2px;
    width: 25px;
    height: 16px;
    border-top: 2px solid #172554;
    border-right: 2px solid #172554;
    border-radius: 0 100% 35% 0;
    background: #93c5fd;
    transform: skewX(-24deg);
  }

  .plane-wing {
    z-index: -1;
    width: 54px;
    height: 15px;
    border: 2px solid #172554;
    background: #fbbf24;
  }

  .plane-wing-top {
    top: 8px;
    left: 38px;
    border-radius: 65% 15% 10% 15%;
    transform: skewX(-25deg) rotate(-8deg);
  }

  .plane-wing-bottom {
    top: 42px;
    left: 40px;
    border-radius: 15% 10% 65% 15%;
    transform: skewX(25deg) rotate(8deg);
  }

  .plane-tail {
    top: 9px;
    left: 17px;
    width: 19px;
    height: 31px;
    border: 2px solid #172554;
    border-radius: 70% 15% 10% 10%;
    background: #fbbf24;
    transform: rotate(16deg);
  }

  .plane-cockpit {
    position: absolute;
    top: 4px;
    left: 47px;
    z-index: 3;
    width: 43px;
    height: 40px;
    overflow: hidden;
    border: 2px solid #172554;
    border-radius: 50% 50% 42% 42%;
    background: linear-gradient(145deg, #bfdbfe 0%, #60a5fa 45%, #2563eb 100%);
    box-shadow: inset 3px 3px 0 rgb(255 255 255 / 35%);
  }

  .plane-cockpit :global(.avatar-svg) {
    transform: translateY(9px) scale(1.1);
  }

  .plane-window {
    top: 31px;
    left: 30px;
    width: 8px;
    height: 8px;
    border: 1px solid #172554;
    border-radius: 50%;
    background: #bfdbfe;
  }

  .flight-cloud {
    position: absolute;
    z-index: 1;
    width: 45px;
    height: 12px;
    border-radius: 999px;
    background: rgb(148 163 184 / 28%);
    filter: blur(0.2px);
  }

  .flight-cloud::before,
  .flight-cloud::after {
    position: absolute;
    border-radius: 50%;
    background: inherit;
    content: "";
  }

  .flight-cloud::before {
    top: -8px;
    left: 9px;
    width: 20px;
    height: 20px;
  }

  .flight-cloud::after {
    top: -5px;
    right: 7px;
    width: 15px;
    height: 15px;
  }

  .cloud-back {
    left: 0;
    bottom: 13px;
    transform: scale(0.7);
    opacity: 0.5;
    animation: cloud-drift 3.2s ease-in-out infinite;
  }

  .cloud-front {
    right: 2px;
    bottom: 5px;
    transform: scale(0.48);
    opacity: 0.35;
    animation: cloud-drift 3.2s 1.1s ease-in-out infinite reverse;
  }

  .flight-spark {
    position: absolute;
    z-index: 4;
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background: #a3e635;
    box-shadow: 0 0 8px rgb(163 230 53 / 85%);
    animation: spark-pop 1.8s ease-in-out infinite;
  }

  .spark-one {
    top: 12px;
    right: 12px;
  }

  .spark-two {
    right: 26px;
    bottom: 9px;
    width: 3px;
    height: 3px;
    animation-delay: 0.7s;
  }

  .alert .flight-spark {
    background: #fb7185;
    box-shadow: 0 0 8px rgb(251 113 133 / 85%);
  }

  @keyframes flight-bob {
    0%, 100% { transform: translateY(1px) rotate(-1deg); }
    50% { transform: translateY(-4px) rotate(1deg); }
  }

  @keyframes plane-fly {
    0%, 100% { transform: translateX(-7px) rotate(-5deg); }
    50% { transform: translateX(7px) rotate(2deg); }
  }

  @keyframes cloud-drift {
    0%, 100% { transform: translateX(0) scale(0.7); }
    50% { transform: translateX(7px) scale(0.76); }
  }

  @keyframes spark-pop {
    0%, 100% { opacity: 0.35; transform: scale(0.8); }
    50% { opacity: 1; transform: scale(1.25); }
  }

  @media (prefers-reduced-motion: reduce) {
    .pilot-flight,
    .pilot-plane,
    .flight-cloud,
    .flight-spark {
      animation: none;
    }
  }
</style>
