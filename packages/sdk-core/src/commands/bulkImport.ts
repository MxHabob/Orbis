export interface BulkImportCommands {
  createStudents(orgId: string, data: any): Promise<any>;
  createGrades(orgId: string, data: any): Promise<any>;
}
