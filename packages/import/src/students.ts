import { users, students, parentStudents, enrollments } from "@midday/db/schema";

// Map Arabic/English standard headers to our internal keys
// This can be expanded based on UI requirements
export const STANDARD_COLUMNS = [
  "name",
  "email",
  "phone",
  "studentNumber",
  "secretNumber",
  "fatherName",
  "fatherPhone",
  "motherName",
  "motherPhone",
];

export const ARABIC_MAPPING: Record<string, string> = {
  "اسم الطالب": "name",
  "البريد الالكتروني": "email",
  "رقم الهاتف": "phone",
  "رقم الطالب": "studentNumber",
  "الرقم السري": "secretNumber",
  "اسم الاب": "fatherName",
  "رقم الاب": "fatherPhone",
  "اسم الام": "motherName",
  "رقم الام": "motherPhone",
};

export type StudentImportRow = {
  [key: string]: any;
};

export async function processStudentsImport(
  db: any, // Using any for db to be agnostic, ideally should be PostgresJsDatabase<typeof schema>
  data: StudentImportRow[],
  options: { classId: number; academicYearId: number }
) {
  return await db.transaction(async (tx: any) => {
    const results = {
      importedCount: 0,
      errors: [] as string[],
    };

    for (let i = 0; i < data.length; i++) {
      const row = data[i];
      if (!row) continue;

      try {
        // 1. Normalize Header Keys and Identify Dynamic Fields
        const normalizedRow: any = {};
        const metadata: Record<string, any> = {};

        for (const [key, value] of Object.entries(row)) {
          // Check if it's a known english key or known arabic key
          const mappedKey = ARABIC_MAPPING[key] || key;
          
          if (STANDARD_COLUMNS.includes(mappedKey)) {
            normalizedRow[mappedKey] = value;
          } else {
            // It's a dynamic additional field (like Job, Allergies, etc.)
            metadata[key] = value;
          }
        }

        // Validate bare minimum
        if (!normalizedRow.name) {
          results.errors.push(`Row ${i + 2}: Missing student name`);
          continue;
        }

        // 2. Insert Student Core User
        const [studentUser] = await tx
          .insert(users)
          .values({
            name: normalizedRow.name,
            email: normalizedRow.email || null, // Ensure unique or null
            phone: normalizedRow.phone || null,
            role: "student",
          })
          .returning({ id: users.id });

        // 3. Insert Student Profile + Metadata
        const studentNum = normalizedRow.studentNumber || `STU-${Date.now()}-${i}`;
        await tx.insert(students).values({
          userId: studentUser.id,
          studentNumber: studentNum,
          secretNumber: normalizedRow.secretNumber || null,
          currentClassId: options.classId,
          metadata: metadata, // Dynamic JSONB Columns stored here
        });

        // 4. Enroll Student in the Class for current Academic Year
        await tx.insert(enrollments).values({
          studentId: studentUser.id,
          classId: options.classId,
          academicYearId: options.academicYearId,
          status: "active",
        });

        // 5. Handle Father Mapping
        if (normalizedRow.fatherName) {
           const [fatherUser] = await tx
              .insert(users)
              .values({
                name: normalizedRow.fatherName,
                phone: normalizedRow.fatherPhone || null,
                role: "parent",
              })
              .returning({ id: users.id });
              
           await tx.insert(parentStudents).values({
              parentId: fatherUser.id,
              studentId: studentUser.id,
              relationship: "father",
              isPrimary: true,
           });
        }

        // 6. Handle Mother Mapping
        if (normalizedRow.motherName) {
           const [motherUser] = await tx
              .insert(users)
              .values({
                name: normalizedRow.motherName,
                phone: normalizedRow.motherPhone || null,
                role: "parent",
              })
              .returning({ id: users.id });
              
           await tx.insert(parentStudents).values({
              parentId: motherUser.id,
              studentId: studentUser.id,
              relationship: "mother",
              isPrimary: !normalizedRow.fatherName, // Primary if no father
           });
        }

        results.importedCount++;
      } catch (error: any) {
        console.error(`Error processing row ${i + 2}:`, error);
        results.errors.push(`Row ${i + 2}: ${error.message}`);
        // For strict integrity, one failure aborts everything. 
        // If you want to skip failures, handling it per row requires nested savepoints or just accepting some rows to fail silently while continuing the transaction (depends on DB capability).
        // Since we are throwing, it rolls back the entire transaction.
        throw error;
      }
    }

    return results;
  });
}
