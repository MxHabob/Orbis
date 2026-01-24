# Database Queries - School Management System

This directory contains all database queries for the school management system, following the Midday style architecture.

## Structure

All queries are organized by entity:

### Core Entities
- **academic-year.ts** - Academic years and terms management
- **student.ts** - Student records and enrollments
- **class.ts** - Class management and subjects
- **teacher.ts** - Teacher records and assignments
- **level.ts** - Education levels (grades)
- **parent.ts** - Parent/guardian records
- **subject.ts** - Subjects and curriculum

### Academic Operations
- **grade.ts** - Grade entries and student performance
- **assessment.ts** - Assessment categories and items
- **attendance.ts** - Attendance tracking
- **lesson.ts** - Schedule and lessons
- **exam.ts** - Exams, committees, and seatings

### Administrative
- **payment.ts** - Fee payments and tracking
- **report.ts** - Ministerial and performance reports

## Usage with Cache

All queries should be used with the cache layer for optimal performance. Here's the recommended pattern:

### Example: Get Student by ID

```typescript
import { getStudentById } from "@db/queries";
import { studentCache } from "@cache";
import { db } from "@db/client";

async function fetchStudent(studentId: string) {
  // Try cache first
  const cached = await studentCache.getById(studentId);
  if (cached) {
    return cached;
  }

  // Query database
  const student = await getStudentById(db, { userId: studentId });
  
  // Cache the result
  if (student) {
    await studentCache.setById(studentId, student);
  }

  return student;
}
```

### Example: List with Pagination

```typescript
import { getStudents } from "@db/queries";
import { db } from "@db/client";

async function listStudents(params: {
  classId?: number;
  pageSize?: number;
  cursor?: string;
}) {
  const result = await getStudents(db, {
    classId: params.classId,
    pageSize: params.pageSize || 25,
    cursor: params.cursor,
    sort: ["student_number", "asc"],
  });

  return result;
}
```

### Example: Create with Cache Invalidation

```typescript
import { createStudent } from "@db/queries";
import { studentCache, classCache } from "@cache";
import { db } from "@db/client";

async function addStudent(data: CreateStudentParams) {
  const student = await createStudent(db, data);

  // Invalidate related caches
  if (data.currentClassId) {
    await classCache.invalidate(data.currentClassId);
  }

  return student;
}
```

## Best Practices

### 1. Always Use Transactions for Related Operations

```typescript
await db.transaction(async (tx) => {
  const student = await createStudent(tx, studentData);
  await createStudentEnrollment(tx, {
    studentId: student.userId,
    academicYearId: currentYear.id,
    classId: classId,
  });
});
```

### 2. Cache Invalidation Strategy

When modifying data, invalidate related caches:

```typescript
// After updating a student
await studentCache.invalidate(studentId);

// After adding a student to a class
await classCache.invalidate(classId);
```

### 3. Use Filters and Sorting

All list queries support filtering and sorting:

```typescript
const students = await getStudents(db, {
  classId: 1,
  status: "active",
  sex: "male",
  sort: ["name", "asc"],
  pageSize: 50,
});
```

### 4. Bulk Operations

Use bulk operations for better performance:

```typescript
// Bulk create attendance
await bulkCreateAttendances(db, {
  attendances: studentsInClass.map(student => ({
    date: new Date(),
    present: true,
    studentId: student.userId,
    lessonId: lessonId,
    academicYearId: currentYear.id,
    termId: currentTerm.id,
  })),
});
```

## Query Patterns

### Get by ID
Returns a single record with related data.

```typescript
const student = await getStudentById(db, { userId: "..." });
```

### List with Pagination
Returns paginated results with metadata.

```typescript
const { data, meta } = await getStudents(db, {
  cursor: "0",
  pageSize: 25,
});
// meta: { cursor, hasPreviousPage, hasNextPage }
```

### Create
Creates a new record and returns it.

```typescript
const student = await createStudent(db, { ... });
```

### Update
Updates a record and returns the updated version.

```typescript
const updated = await updateStudent(db, {
  userId: "...",
  status: "graduated",
});
```

### Delete
Deletes a record and returns the deleted data.

```typescript
const deleted = await deleteStudent(db, { userId: "..." });
```

## Analytics and Reports

### Student Performance Summary

```typescript
const summary = await getStudentGradesSummary(db, {
  studentId: "...",
  academicYearId: 1,
  termId: 1,
});
```

### Class Performance

```typescript
const performance = await getClassPerformanceSummary(db, {
  classId: 1,
  academicYearId: 1,
});
```

### Attendance Statistics

```typescript
const stats = await getClassAttendanceStats(db, {
  classId: 1,
  academicYearId: 1,
  dateFrom: new Date("2024-01-01"),
  dateTo: new Date("2024-12-31"),
});
```

## Type Safety

All queries are fully typed with TypeScript:

```typescript
import type { GetStudentsParams, CreateStudentParams } from "@db/queries";

const params: GetStudentsParams = {
  classId: 1,
  status: "active", // Type-checked against enum
  sort: ["name", "asc"], // Type-checked
};
```

## Error Handling

Always wrap queries in try-catch blocks:

```typescript
try {
  const student = await getStudentById(db, { userId: "..." });
  if (!student) {
    throw new Error("Student not found");
  }
  return student;
} catch (error) {
  console.error("Failed to fetch student:", error);
  throw error;
}
```

## Performance Tips

1. **Use indexes**: All queries are optimized with proper indexes
2. **Limit results**: Always use pagination for large datasets
3. **Cache frequently accessed data**: Use Redis cache for hot data
4. **Batch operations**: Use bulk operations when possible
5. **Select only needed fields**: Queries already optimize field selection

## Cache TTL Guidelines

- **Students**: 30 minutes
- **Classes**: 20 minutes
- **Academic Years**: 60 minutes (rarely changes)
- **Grades**: 15 minutes
- **Attendance**: 10 minutes
- **Schedules**: 30 minutes

## Related Documentation

- [Schema Documentation](../schema.ts)
- [Cache Layer](../../cache/README.md)
- [Database Client](../client.ts)
