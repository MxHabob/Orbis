import type { UiContext } from "$lib/api/types";

export interface AiPageContext extends UiContext {
  domain?:
    | "academic"
    | "finance"
    | "exam"
    | "scheduling"
    | "dashboard"
    | "general"
    | string;
  pageName?: string;
  recordId?: string;
  metadata?: Record<string, unknown>;
}

class AiContextStore {
  context = $state<AiPageContext | null>(null);

  setContext(ctx: AiPageContext) {
    this.context = {
      ...ctx,
      screen: ctx.screen || ctx.pageName,
      entity_id: ctx.entity_id || ctx.recordId,
    };
  }

  clearContext() {
    this.context = null;
  }
}

export const aiContext = new AiContextStore();
