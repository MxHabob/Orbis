export interface BulkImportCommands {
  createStudents(orgId: string, userId: string, data: any): Promise<any>;
  createGrades(orgId: string, userId: string, cycleId: string | null, data: any): Promise<any>;
}
