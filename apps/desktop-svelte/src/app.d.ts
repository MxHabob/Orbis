// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
import 'svelte/elements';

declare global {
  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }

  namespace svelteHTML {
    interface HTMLAttributes<T> {
      oncollapsesidebar?: (event: CustomEvent<any>) => void;
    }
  }
}

export {};

