/**
 * Docforge command interfaces — document rendering and canvas actions.
 */
export interface DocforgeCommands {
  previewDocumentTemplate(
    templateJson: string,
    contextJson: string,
  ): Promise<number[]>;
}
