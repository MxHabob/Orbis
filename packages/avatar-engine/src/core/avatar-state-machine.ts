export type AvatarState = "idle" | "blinking" | "thinking" | "happy" | "error";

type StateCallback = (state: AvatarState) => void;
type TransitionCallback = (from: AvatarState, to: AvatarState) => void;

export class AvatarStateMachine {
  private currentState: AvatarState = "idle";
  private onTransitionCallbacks: TransitionCallback[] = [];
  private onEnterCallbacks: Map<AvatarState, StateCallback[]> = new Map();
  private onExitCallbacks: Map<AvatarState, StateCallback[]> = new Map();

  private allowedTransitions: Record<AvatarState, AvatarState[]> = {
    idle: ["blinking", "thinking", "happy", "error"],
    blinking: ["idle", "thinking", "happy", "error"],
    thinking: ["idle", "blinking", "happy", "error"],
    happy: ["idle", "blinking", "thinking", "error"],
    error: ["idle", "blinking", "thinking", "happy"],
  };

  constructor(initialState: AvatarState = "idle") {
    this.currentState = initialState;
  }

  get state(): AvatarState {
    return this.currentState;
  }

  transitionTo(to: AvatarState): boolean {
    if (this.currentState === to) return false;

    const allowed = this.allowedTransitions[this.currentState];
    if (!allowed.includes(to)) {
      return false;
    }

    const from = this.currentState;

    // Trigger exit callbacks for current state
    const exitCallbacks = this.onExitCallbacks.get(from);
    if (exitCallbacks) {
      for (const cb of [...exitCallbacks]) {
        cb(from);
      }
    }

    this.currentState = to;

    // Trigger enter callbacks for new state
    const enterCallbacks = this.onEnterCallbacks.get(to);
    if (enterCallbacks) {
      for (const cb of [...enterCallbacks]) {
        cb(to);
      }
    }

    // Trigger global transition callbacks
    for (const cb of [...this.onTransitionCallbacks]) {
      cb(from, to);
    }

    return true;
  }

  onTransition(cb: TransitionCallback): () => void {
    this.onTransitionCallbacks.push(cb);
    return () => {
      this.onTransitionCallbacks = this.onTransitionCallbacks.filter(
        (c) => c !== cb,
      );
    };
  }

  onEnter(state: AvatarState, cb: StateCallback): () => void {
    if (!this.onEnterCallbacks.has(state)) {
      this.onEnterCallbacks.set(state, []);
    }
    this.onEnterCallbacks.get(state)!.push(cb);
    return () => {
      const list = this.onEnterCallbacks.get(state);
      if (list) {
        this.onEnterCallbacks.set(
          state,
          list.filter((c) => c !== cb),
        );
      }
    };
  }

  onExit(state: AvatarState, cb: StateCallback): () => void {
    if (!this.onExitCallbacks.has(state)) {
      this.onExitCallbacks.set(state, []);
    }
    this.onExitCallbacks.get(state)!.push(cb);
    return () => {
      const list = this.onExitCallbacks.get(state);
      if (list) {
        this.onExitCallbacks.set(
          state,
          list.filter((c) => c !== cb),
        );
      }
    };
  }
}
