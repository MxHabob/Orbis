export interface BulkImportCommands {
  createStudents(orgId: string, userId: string, data: any): Promise<any>;
  createGrades(orgId: string, userId: string, periodId: string | null, data: any): Promise<any>;
}
