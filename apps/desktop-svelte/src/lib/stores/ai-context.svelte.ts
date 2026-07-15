export interface AiPageContext {
  domain:
    | "academic"
    | "finance"
    | "exam"
    | "scheduling"
    | "dashboard"
    | "general";
  pageName: string;
  recordId?: string;
  metadata?: Record<string, any>;
}

class AiContextStore {
  context = $state<AiPageContext | null>(null);

  setContext(ctx: AiPageContext) {
    this.context = ctx;
  }

  clearContext() {
    this.context = null;
  }
}

export const aiContext = new AiContextStore();
