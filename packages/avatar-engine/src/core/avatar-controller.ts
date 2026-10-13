import {
  AvatarStateMachine,
  type AvatarState,
} from "./avatar-state-machine.js";

export interface AvatarControllerOptions {
  autoBlink?: boolean;
}

type StateListener = (state: AvatarState) => void;
type Timer = ReturnType<typeof setTimeout>;

export class AvatarController {
  private readonly machine: AvatarStateMachine;
  private readonly listeners = new Set<StateListener>();
  private readonly timers = new Set<Timer>();
  private autoBlink: boolean;
  private destroyed = false;

  constructor(
    initialState: AvatarState = "idle",
    { autoBlink = true }: AvatarControllerOptions = {},
  ) {
    this.machine = new AvatarStateMachine(initialState);
    this.autoBlink = autoBlink;
    this.machine.onTransition((from, to) => {
      this.handleStateChange(from, to);
      for (const listener of [...this.listeners]) listener(to);
    });
  }

  get state(): AvatarState {
    return this.machine.state;
  }

  getSnapshot = (): AvatarState => this.state;

  subscribe = (listener: StateListener): (() => void) => {
    if (this.destroyed) return () => {};
    this.listeners.add(listener);
    if (this.listeners.size === 1) this.startBlinkScheduler();
    return () => {
      this.listeners.delete(listener);
      if (this.listeners.size === 0) this.clearTimers();
    };
  };

  setAutoBlink(enabled: boolean): void {
    if (this.autoBlink === enabled || this.destroyed) return;
    this.autoBlink = enabled;
    if (enabled && this.listeners.size > 0 && this.state === "idle") {
      this.startBlinkScheduler();
    } else if (!enabled) {
      this.clearTimers();
    }
  }

  transitionTo(state: AvatarState): boolean {
    if (this.destroyed) return false;
    return this.machine.transitionTo(state);
  }

  blink(): void {
    this.transitionTo("blinking");
  }

  think(): void {
    this.transitionTo("thinking");
  }

  happy(): void {
    this.transitionTo("happy");
  }

  error(): void {
    this.transitionTo("error");
  }

  idle(): void {
    this.transitionTo("idle");
  }

  destroy(): void {
    if (this.destroyed) return;
    this.destroyed = true;
    this.clearTimers();
    this.listeners.clear();
  }

  private handleStateChange(from: AvatarState, to: AvatarState): void {
    this.clearTimers();

    if (to === "blinking") {
      const returnState =
        from !== "blinking" && from !== "error" ? from : "idle";
      this.schedule(() => {
        if (this.state === "blinking") this.transitionTo(returnState);
      }, 160);
    } else if (to === "happy") {
      this.schedule(() => {
        if (this.state === "happy") this.transitionTo("idle");
      }, 1800);
    } else if (to === "idle") {
      this.startBlinkScheduler();
    }
  }

  private startBlinkScheduler(): void {
    if (
      !this.autoBlink ||
      this.destroyed ||
      this.listeners.size === 0 ||
      this.state !== "idle" ||
      this.timers.size > 0
    ) {
      return;
    }

    const delay = 3000 + Math.random() * 4500;
    this.schedule(() => {
      if (this.state !== "idle") return;
      const doubleBlink = Math.random() < 0.25;
      this.blink();
      if (doubleBlink) {
        this.schedule(() => {
          if (this.state === "idle") this.blink();
        }, 320);
      }
    }, delay);
  }

  private schedule(callback: () => void, delay: number): void {
    const timer = setTimeout(() => {
      this.timers.delete(timer);
      callback();
    }, delay);
    this.timers.add(timer);
  }

  private clearTimers(): void {
    for (const timer of this.timers) clearTimeout(timer);
    this.timers.clear();
  }
}
