import { AvatarStateMachine, type AvatarState } from "./AvatarStateMachine.js";

export class AvatarController {
  private machine: AvatarStateMachine;
  #state = $state<AvatarState>("idle");
  private blinkTimer: any = null;
  private autoRecoveryTimer: any = null;

  constructor(initialState: AvatarState = "idle") {
    this.machine = new AvatarStateMachine(initialState);
    this.#state = initialState;

    this.machine.onTransition((from, to) => {
      this.#state = to;
      this.handleStateChange(from, to);
    });

    if (initialState === "idle") {
      this.startBlinkScheduler();
    }
  }

  get state(): AvatarState {
    return this.#state;
  }

  transitionTo(state: AvatarState): boolean {
    return this.machine.transitionTo(state);
  }

  blink() {
    this.transitionTo("blinking");
  }

  think() {
    this.transitionTo("thinking");
  }

  happy() {
    this.transitionTo("happy");
  }

  error() {
    this.transitionTo("error");
  }

  idle() {
    this.transitionTo("idle");
  }

  private handleStateChange(from: AvatarState, to: AvatarState) {
    if (this.autoRecoveryTimer) {
      clearTimeout(this.autoRecoveryTimer);
      this.autoRecoveryTimer = null;
    }

    if (to === "blinking") {
      const returnState =
        from !== "blinking" && from !== "error" ? from : "idle";
      this.autoRecoveryTimer = setTimeout(() => {
        this.transitionTo(returnState);
      }, 150);
    } else if (to === "happy") {
      this.autoRecoveryTimer = setTimeout(() => {
        this.transitionTo("idle");
      }, 1800);
    }

    if (to === "idle") {
      this.startBlinkScheduler();
    } else {
      this.stopBlinkScheduler();
    }
  }

  private startBlinkScheduler() {
    this.stopBlinkScheduler();

    const scheduleNext = () => {
      const delay = 3000 + Math.random() * 4500; // Random interval between 3s and 7.5s
      this.blinkTimer = setTimeout(() => {
        if (this.#state === "idle") {
          // 25% chance of a double blink for natural friendliness!
          if (Math.random() < 0.25) {
            this.doubleBlink();
          } else {
            this.blink();
          }
          scheduleNext();
        }
      }, delay);
    };

    scheduleNext();
  }

  private doubleBlink() {
    this.blink();
    // Wait for the first blink to recover (150ms + 150ms delay) then trigger the second blink
    setTimeout(() => {
      if (this.#state === "idle") {
        this.blink();
      }
    }, 300);
  }

  private stopBlinkScheduler() {
    if (this.blinkTimer) {
      clearTimeout(this.blinkTimer);
      this.blinkTimer = null;
    }
  }

  destroy() {
    this.stopBlinkScheduler();
    if (this.autoRecoveryTimer) {
      clearTimeout(this.autoRecoveryTimer);
    }
  }
}
