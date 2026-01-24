import type { Database } from "@db/client";
import {
  payments,
  students,
  users,
  academicYears,
} from "@db/schema";
import { and, asc, desc, eq, gte, lte, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetPaymentByIdParams = {
  id: number;
};

export const getPaymentById = async (
  db: Database,
  params: GetPaymentByIdParams,
) => {
  const [result] = await db
    .select({
      id: payments.id,
      studentId: payments.studentId,
      feeType: payments.feeType,
      amount: payments.amount,
      dueDate: payments.dueDate,
      paidDate: payments.paidDate,
      status: payments.status,
      description: payments.description,
      academicYearId: payments.academicYearId,
      student: {
        userId: students.userId,
        studentNumber: students.studentNumber,
        surname: students.surname,
        name: users.name,
        phone: students.phone,
      },
      academicYear: {
        id: academicYears.id,
        year: academicYears.year,
      },
    })
    .from(payments)
    .where(eq(payments.id, params.id))
    .leftJoin(students, eq(students.userId, payments.studentId))
    .leftJoin(users, eq(users.id, students.userId))
    .leftJoin(academicYears, eq(academicYears.id, payments.academicYearId))
    .limit(1);

  return result;
};

export type GetPaymentsParams = {
  cursor?: string | null;
  pageSize?: number;
  sort?: string[] | null;
  studentId?: string | null;
  academicYearId?: number | null;
  status?: string | null;
  feeType?: string | null;
  dueDateFrom?: Date | null;
  dueDateTo?: Date | null;
};

export const getPayments = async (
  db: Database,
  params: GetPaymentsParams,
) => {
  const {
    sort,
    cursor,
    pageSize = 25,
    studentId,
    academicYearId,
    status,
    feeType,
    dueDateFrom,
    dueDateTo,
  } = params;

  const whereConditions: SQL[] = [];

  // Apply filters
  if (studentId) {
    whereConditions.push(eq(payments.studentId, studentId));
  }

  if (academicYearId) {
    whereConditions.push(eq(payments.academicYearId, academicYearId));
  }

  if (status) {
    whereConditions.push(eq(payments.status, status as any));
  }

  if (feeType) {
    whereConditions.push(eq(payments.feeType, feeType as any));
  }

  if (dueDateFrom) {
    whereConditions.push(gte(payments.dueDate, dueDateFrom));
  }

  if (dueDateTo) {
    whereConditions.push(lte(payments.dueDate, dueDateTo));
  }

  const query = db
    .select({
      id: payments.id,
      studentId: payments.studentId,
      feeType: payments.feeType,
      amount: payments.amount,
      dueDate: payments.dueDate,
      paidDate: payments.paidDate,
      status: payments.status,
      description: payments.description,
      academicYearId: payments.academicYearId,
      student: {
        userId: students.userId,
        studentNumber: students.studentNumber,
        surname: students.surname,
        name: users.name,
      },
      academicYear: {
        id: academicYears.id,
        year: academicYears.year,
      },
    })
    .from(payments)
    .leftJoin(students, eq(students.userId, payments.studentId))
    .leftJoin(users, eq(users.id, students.userId))
    .leftJoin(academicYears, eq(academicYears.id, payments.academicYearId))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined);

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "amount") {
      isAscending
        ? query.orderBy(asc(payments.amount))
        : query.orderBy(desc(payments.amount));
    } else if (column === "due_date") {
      isAscending
        ? query.orderBy(asc(payments.dueDate))
        : query.orderBy(desc(payments.dueDate));
    } else if (column === "paid_date") {
      isAscending
        ? query.orderBy(asc(payments.paidDate))
        : query.orderBy(desc(payments.paidDate));
    } else if (column === "status") {
      isAscending
        ? query.orderBy(asc(payments.status))
        : query.orderBy(desc(payments.status));
    } else if (column === "student_number") {
      isAscending
        ? query.orderBy(asc(students.studentNumber))
        : query.orderBy(desc(students.studentNumber));
    }
  } else {
    // Default sort by due_date descending
    query.orderBy(desc(payments.dueDate));
  }

  // Apply pagination
  const offset = cursor ? Number.parseInt(cursor, 10) : 0;
  query.limit(pageSize).offset(offset);

  const data = await query;

  const nextCursor =
    data && data.length === pageSize
      ? (offset + pageSize).toString()
      : undefined;

  return {
    meta: {
      cursor: nextCursor ?? null,
      hasPreviousPage: offset > 0,
      hasNextPage: data && data.length === pageSize,
    },
    data,
  };
};

export type CreatePaymentParams = {
  studentId: string;
  feeType: string;
  amount: number;
  dueDate: Date;
  paidDate?: Date | null;
  status: string;
  description?: string | null;
  academicYearId?: number | null;
};

export const createPayment = async (
  db: Database,
  params: CreatePaymentParams,
) => {
  const [result] = await db
    .insert(payments)
    .values(params)
    .returning();

  return result;
};

export type UpdatePaymentParams = {
  id: number;
  feeType?: string;
  amount?: number;
  dueDate?: Date;
  paidDate?: Date | null;
  status?: string;
  description?: string | null;
};

export const updatePayment = async (
  db: Database,
  params: UpdatePaymentParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(payments)
    .set(rest)
    .where(eq(payments.id, id))
    .returning();

  return result;
};

export type DeletePaymentParams = {
  id: number;
};

export const deletePayment = async (
  db: Database,
  params: DeletePaymentParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(payments)
    .where(eq(payments.id, id))
    .returning();

  return result;
};

// Mark payment as paid
export type MarkPaymentAsPaidParams = {
  id: number;
  paidDate?: Date;
};

export const markPaymentAsPaid = async (
  db: Database,
  params: MarkPaymentAsPaidParams,
) => {
  const { id, paidDate = new Date() } = params;

  const [result] = await db
    .update(payments)
    .set({
      status: "paid",
      paidDate,
    })
    .where(eq(payments.id, id))
    .returning();

  return result;
};

// Get student payment summary
export type GetStudentPaymentSummaryParams = {
  studentId: string;
  academicYearId?: number | null;
};

export const getStudentPaymentSummary = async (
  db: Database,
  params: GetStudentPaymentSummaryParams,
) => {
  const { studentId, academicYearId } = params;

  const whereConditions: SQL[] = [eq(payments.studentId, studentId)];

  if (academicYearId) {
    whereConditions.push(eq(payments.academicYearId, academicYearId));
  }

  const [result] = await db
    .select({
      totalAmount: sql<number>`coalesce(sum(${payments.amount}), 0)`,
      paidAmount: sql<number>`coalesce(sum(case when ${payments.status} = 'paid' then ${payments.amount} else 0 end), 0)`,
      pendingAmount: sql<number>`coalesce(sum(case when ${payments.status} = 'pending' then ${payments.amount} else 0 end), 0)`,
      overdueAmount: sql<number>`coalesce(sum(case when ${payments.status} = 'overdue' then ${payments.amount} else 0 end), 0)`,
      totalPayments: sql<number>`cast(count(*) as int)`,
      paidPayments: sql<number>`cast(sum(case when ${payments.status} = 'paid' then 1 else 0 end) as int)`,
      pendingPayments: sql<number>`cast(sum(case when ${payments.status} = 'pending' then 1 else 0 end) as int)`,
      overduePayments: sql<number>`cast(sum(case when ${payments.status} = 'overdue' then 1 else 0 end) as int)`,
    })
    .from(payments)
    .where(and(...whereConditions));

  return result;
};

// Get overdue payments
export type GetOverduePaymentsParams = {
  academicYearId?: number | null;
  cursor?: string | null;
  pageSize?: number;
};

export const getOverduePayments = async (
  db: Database,
  params: GetOverduePaymentsParams,
) => {
  const { academicYearId, cursor, pageSize = 25 } = params;

  const whereConditions: SQL[] = [
    eq(payments.status, "overdue"),
  ];

  if (academicYearId) {
    whereConditions.push(eq(payments.academicYearId, academicYearId));
  }

  const query = db
    .select({
      id: payments.id,
      studentId: payments.studentId,
      feeType: payments.feeType,
      amount: payments.amount,
      dueDate: payments.dueDate,
      description: payments.description,
      student: {
        userId: students.userId,
        studentNumber: students.studentNumber,
        surname: students.surname,
        name: users.name,
        phone: students.phone,
      },
      daysOverdue: sql<number>`extract(day from now() - ${payments.dueDate})`,
    })
    .from(payments)
    .leftJoin(students, eq(students.userId, payments.studentId))
    .leftJoin(users, eq(users.id, students.userId))
    .where(and(...whereConditions))
    .orderBy(asc(payments.dueDate));

  // Apply pagination
  const offset = cursor ? Number.parseInt(cursor, 10) : 0;
  query.limit(pageSize).offset(offset);

  const data = await query;

  const nextCursor =
    data && data.length === pageSize
      ? (offset + pageSize).toString()
      : undefined;

  return {
    meta: {
      cursor: nextCursor ?? null,
      hasPreviousPage: offset > 0,
      hasNextPage: data && data.length === pageSize,
    },
    data,
  };
};

// Get payment statistics by academic year
export type GetPaymentStatsByYearParams = {
  academicYearId: number;
};

export const getPaymentStatsByYear = async (
  db: Database,
  params: GetPaymentStatsByYearParams,
) => {
  const { academicYearId } = params;

  const [result] = await db
    .select({
      totalAmount: sql<number>`coalesce(sum(${payments.amount}), 0)`,
      paidAmount: sql<number>`coalesce(sum(case when ${payments.status} = 'paid' then ${payments.amount} else 0 end), 0)`,
      pendingAmount: sql<number>`coalesce(sum(case when ${payments.status} = 'pending' then ${payments.amount} else 0 end), 0)`,
      overdueAmount: sql<number>`coalesce(sum(case when ${payments.status} = 'overdue' then ${payments.amount} else 0 end), 0)`,
      totalPayments: sql<number>`cast(count(*) as int)`,
      paidPayments: sql<number>`cast(sum(case when ${payments.status} = 'paid' then 1 else 0 end) as int)`,
      pendingPayments: sql<number>`cast(sum(case when ${payments.status} = 'pending' then 1 else 0 end) as int)`,
      overduePayments: sql<number>`cast(sum(case when ${payments.status} = 'overdue' then 1 else 0 end) as int)`,
      collectionRate: sql<number>`
        case 
          when sum(${payments.amount}) > 0 
          then (sum(case when ${payments.status} = 'paid' then ${payments.amount} else 0 end) / sum(${payments.amount})) * 100
          else 0
        end
      `,
    })
    .from(payments)
    .where(eq(payments.academicYearId, academicYearId));

  return result;
};

// Bulk create payments
export type BulkCreatePaymentsParams = {
  payments: Array<{
    studentId: string;
    feeType: string;
    amount: number;
    dueDate: Date;
    status: string;
    description?: string | null;
    academicYearId?: number | null;
  }>;
};

export const bulkCreatePayments = async (
  db: Database,
  params: BulkCreatePaymentsParams,
) => {
  const { payments: paymentList } = params;

  const results = await db
    .insert(payments)
    .values(paymentList)
    .returning();

  return results;
};
