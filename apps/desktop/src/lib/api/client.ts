import { invoke } from '@tauri-apps/api/core';

export interface ApiClient {
    iam: {
        login(email: string, password: string): Promise<any>;
        logout(token: string): Promise<void>;
        restoreSession(token: string): Promise<any>;
        listUsers(params: any): Promise<any[]>;
    };
    organization: {
        create(data: any): Promise<any>;
        createUnit(data: any): Promise<any>;
        checkSetup(): Promise<any>;
        listOrgUnits(): Promise<any[]>;
        selectOrgUnit(id: string): Promise<void>;
        getOrganization(): Promise<any>;
        updateOrganization(data: any): Promise<any>;
        updateSettings(data: any): Promise<any>;
        getDashboardStats(): Promise<any>;
        getMain(): Promise<any>;
        getPeriodTemplates(): Promise<any[]>;
        createOrgUnit(data: any): Promise<any>;
        deleteOrgUnit(id: string): Promise<void>;
        updateOrgUnit(id: string, data: any): Promise<any>;
    };
    calendar: {
        getCurrent(): Promise<any>;
        listAcademicPeriods(): Promise<any[]>;
        getAcademicPeriod(id: string): Promise<any>;
    };
    academic: {
        listClasses(): Promise<any[]>;
        listClassSubjects(classId: string, periodId: string): Promise<any[]>;
        listSubjects(): Promise<any[]>;
        listEnrollments(): Promise<any[]>;
    };
    assessment: {
        createScheme(data: any): Promise<any>;
        recordGrade(data: any): Promise<any>;
        calculateGrades(data: any): Promise<any>;
        listGradeScales(): Promise<any[]>;
        listGradeScaleItems(scaleId: string): Promise<any[]>;
        listCalculationPolicies(): Promise<any[]>;
    };
    finance: {
        listPayments(params: any, options: any): Promise<any[]>;
        listAccounts(): Promise<any[]>;
        listCurrencies(): Promise<any[]>;
        listStudentFeesByEnrollment(enrollmentId: string): Promise<any[]>;
        record(data: any): Promise<any>;
    };
    reports: {
        generate(data: any): Promise<any>;
        getJobStatus(jobId: string): Promise<{ jobId: string; status: string }>;
        downloadFile(jobId: string): Promise<Uint8Array>;
    };
    docforge: {
        listTemplates(params: any): Promise<any[]>;
        previewMultiUp(templateId: string, entityIds: string[], configJson: string): Promise<any>;
        startMultiUpPrint(templateId: string, entityIds: string[], configJson: string): Promise<any>;
        getMultiUpPrintStatus(jobId: string): Promise<any>;
    };
}

export class RemoteClient implements ApiClient {
    iam = { 
        login: async (e: string, p: string) => { throw new Error('Not implemented'); },
        logout: async (t: string) => { throw new Error('Not implemented'); },
        restoreSession: async (t: string) => { throw new Error('Not implemented'); },
        listUsers: async (params: any) => []
    };
    organization = {
        create: async (data: any) => { throw new Error('Not implemented'); },
        createUnit: async (data: any) => { throw new Error('Not implemented'); },
        checkSetup: async () => { throw new Error('Not implemented'); },
        listOrgUnits: async () => [],
        selectOrgUnit: async (id: string) => {},
        getOrganization: async () => ({}),
        updateOrganization: async (data: any) => ({}),
        updateSettings: async (data: any) => ({}),
        getDashboardStats: async () => ({}),
        getMain: async () => ({}),
        getPeriodTemplates: async () => [],
        createOrgUnit: async (data: any) => ({}),
        deleteOrgUnit: async (id: string) => {},
        updateOrgUnit: async (id: string, data: any) => ({})
    };
    calendar = {
        getCurrent: async () => { throw new Error('Not implemented'); },
        listAcademicPeriods: async () => [],
        getAcademicPeriod: async (id: string) => { throw new Error('Not implemented'); }
    };
    academic = {
        listClasses: async () => [],
        listClassSubjects: async () => [],
        listSubjects: async () => [],
        listEnrollments: async () => []
    };
    assessment = {
        createScheme: async (data: any) => { throw new Error('Not implemented'); },
        recordGrade: async (data: any) => { throw new Error('Not implemented'); },
        calculateGrades: async (data: any) => { throw new Error('Not implemented'); },
        listGradeScales: async () => [],
        listGradeScaleItems: async (scaleId: string) => [],
        listCalculationPolicies: async () => []
    };
    finance = {
        listPayments: async () => [],
        listAccounts: async () => [],
        listCurrencies: async () => [],
        listStudentFeesByEnrollment: async () => [],
        record: async () => {}
    };
    reports = {
        generate: async (data: any) => { throw new Error('Not implemented'); },
        getJobStatus: async (jobId: string) => { throw new Error('Not implemented'); },
        downloadFile: async (jobId: string) => { throw new Error('Not implemented'); },
    };
    docforge = {
        listTemplates: async () => [],
        previewMultiUp: async () => null,
        startMultiUpPrint: async () => null,
        getMultiUpPrintStatus: async () => null
    };
}

export class LocalClient implements ApiClient {
    iam = {
        login: async (email: string, password: string) => {
            return await invoke<any>('login_user', { email, password });
        },
        logout: async (token: string) => { return; },
        restoreSession: async (token: string) => {
            return await invoke<any>('restore_session', { token }).catch(() => {
                return { access_token: token, user: JSON.parse(localStorage.getItem('orbis_user') || '{}') };
            });
        },
        listUsers: async (params: any) => invoke<any[]>('list_users', { params })
    };
    organization = {
        create: async (data: any) => {
            const id = await invoke<string>('create_organization', data);
            return { id };
        },
        createUnit: async (data: any) => {
            const id = await invoke<string>('create_org_unit', data);
            return { id };
        },
        checkSetup: async () => { return true; },
        listOrgUnits: async () => [],
        selectOrgUnit: async (id: string) => {},
        getOrganization: async () => ({}),
        updateOrganization: async (data: any) => ({}),
        updateSettings: async (data: any) => ({}),
        getDashboardStats: async () => ({}),
        getMain: async () => ({}),
        getPeriodTemplates: async () => [],
        createOrgUnit: async (data: any) => {
            const id = await invoke<string>('create_org_unit', data);
            return { id };
        },
        deleteOrgUnit: async (id: string) => {},
        updateOrgUnit: async (id: string, data: any) => ({})
    };
    calendar = {
        getCurrent: async () => { return { id: "1", name: "2024-2025", status: "active" }; },
        listAcademicPeriods: async () => [],
        getAcademicPeriod: async (id: string) => { return { id }; }
    };
    academic = {
        listClasses: async () => invoke<any[]>('list_classes'),
        listClassSubjects: async (class_id: string, period_id: string) => invoke<any[]>('list_class_subjects', { class_id, period_id }),
        listSubjects: async () => invoke<any[]>('list_subjects'),
        listEnrollments: async () => invoke<any[]>('list_enrollments')
    };
    assessment = {
        createScheme: async (data: any) => {
            const id = await invoke<string>('create_assessment_scheme', data);
            return { id };
        },
        recordGrade: async (data: any) => {
            const id = await invoke<string>('record_grade', data);
            return { id };
        },
        calculateGrades: async (data: any) => {
            const finalScore = await invoke<number>('calculate_grades', data);
            return { finalScore };
        },
        listGradeScales: async () => [],
        listGradeScaleItems: async (scaleId: string) => [],
        listCalculationPolicies: async () => []
    };
    finance = {
        listPayments: async (params: any, options: any) => invoke<any[]>('list_payments', { params, options }),
        listAccounts: async () => invoke<any[]>('list_accounts'),
        listCurrencies: async () => invoke<any[]>('list_currencies'),
        listStudentFeesByEnrollment: async (enrollment_id: string) => invoke<any[]>('list_student_fees_by_enrollment', { enrollment_id }),
        record: async (data: any) => invoke<any>('record_payment', { data })
    };
    reports = {
        generate: async (data: any) => {
            const jobId = await invoke<string>('generate_report', data);
            return { jobId };
        },
        getJobStatus: async (jobId: string) => {
            return await invoke<{ job_id: string; status: string }>('get_report_job_status', { jobId });
        },
        downloadFile: async (jobId: string) => {
            const bytesArray = await invoke<number[]>('download_report_file', { jobId });
            return new Uint8Array(bytesArray);
        }
    };
    docforge = {
        listTemplates: async (params: any) => invoke<any[]>('list_templates', { params }),
        previewMultiUp: async (templateId: string, entityIds: string[], configJson: string) => invoke<any>('preview_multi_up', { templateId, entityIds, configJson }),
        startMultiUpPrint: async (templateId: string, entityIds: string[], configJson: string) => invoke<any>('start_multi_up_print', { templateId, entityIds, configJson }),
        getMultiUpPrintStatus: async (jobId: string) => invoke<any>('get_multi_up_print_status', { jobId })
    };
}

export const createApiClient = (): ApiClient => {
    const isLocalMode = !!(window as any).__TAURI_INTERNALS__;
    if (isLocalMode) {
        return new LocalClient();
    }
    return new RemoteClient();
};

export const api = createApiClient();
