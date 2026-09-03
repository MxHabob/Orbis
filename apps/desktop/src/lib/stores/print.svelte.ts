// ═══════════════════════════════════════════════════════════════
// Global Print Store (Svelte 5)
// Provides a unified way to trigger the print dialog from anywhere.
// ═══════════════════════════════════════════════════════════════

export interface PrintDialogState {
  isOpen: boolean;
  entityType: string;
  entityIds: string[];
  entityLabels?: string[];
}

// Global reactive state
export const printState = $state<PrintDialogState>({
  isOpen: false,
  entityType: "",
  entityIds: [],
});

/**
 * Open the print dialog for specific entities.
 * @param entityType The entity type (e.g., 'invoice', 'user', 'enrollment')
 * @param entityIds Array of entity IDs to print
 * @param entityLabels Optional array of labels corresponding to IDs
 */
export function openPrintDialog(
  entityType: string,
  entityIds: string[],
  entityLabels?: string[],
) {
  printState.entityType = entityType;
  printState.entityIds = entityIds;
  printState.entityLabels = entityLabels;
  printState.isOpen = true;
}

/**
 * Close the print dialog
 */
export function closePrintDialog() {
  printState.isOpen = false;
}
