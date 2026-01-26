import * as schema from './schema';
import { connectDb } from './client';

async function seed() {
  const db = await connectDb();

  try {
    await db.transaction(async (tx) => {
      console.log('Cleaning up old data...');
      await tx.delete(schema.holidays);
      await tx.delete(schema.employeeRoles);
      await tx.delete(schema.employees);
      await tx.delete(schema.curriculumSubjects);
      await tx.delete(schema.curriculumLevels);
      await tx.delete(schema.subjects);
      await tx.delete(schema.levels);
      await tx.delete(schema.academicTerms);
      await tx.delete(schema.academicYears);
      await tx.delete(schema.users);

      console.log('Starting the seeding process for the 2025-2026 academic year...');

      // 1. إنشاء السنة الدراسية
      const [academicYear] = await tx
        .insert(schema.academicYears)
        .values({
          year: '2025-2026',
          startDate: new Date('2025-09-15'),
          endDate: new Date('2026-06-20'),
          isCurrent: true,
          isActive: true,
        })
        .returning({ id: schema.academicYears.id });

      if (!academicYear) {
        throw new Error('Failed to create academic year');
      }

      const yearId = academicYear.id;

      // 2. الفصول الدراسية (bulk)
      await tx.insert(schema.academicTerms).values([
        { academicYearId: yearId, name: 'الفصل الأول', startDate: new Date('2025-09-15'), endDate: new Date('2026-01-05'), order: 1 },
        { academicYearId: yearId, name: 'الفصل الثاني', startDate: new Date('2026-01-18'), endDate: new Date('2026-06-20'), order: 2 },
      ]);

      // 3. المراحل الدراسية (bulk)
      const levelData: (typeof schema.levels.$inferInsert)[] = [
        { name: 'الصف الأول أساسي',   stage: 'basic' as const,     order: 1 },
        { name: 'الصف الثاني أساسي',   stage: 'basic' as const,     order: 2 },
        { name: 'الصف الثالث أساسي',   stage: 'basic' as const,     order: 3 },
        { name: 'الصف الرابع أساسي',   stage: 'basic' as const,     order: 4 },
        { name: 'الصف الخامس أساسي',   stage: 'basic' as const,     order: 5 },
        { name: 'الصف السادس أساسي',   stage: 'basic' as const,     order: 6 },
        { name: 'الصف السابع أساسي',   stage: 'basic' as const,     order: 7 },
        { name: 'الصف الثامن أساسي',   stage: 'basic' as const,     order: 8 },
        { name: 'الصف التاسع أساسي',   stage: 'basic' as const,     order: 9 },
        { name: 'الأول ثانوي علمي',     stage: 'secondary' as const, order: 10 },
        { name: 'الثاني ثانوي علمي',    stage: 'secondary' as const, order: 11 },
        { name: 'الثالث ثانوي علمي',    stage: 'secondary' as const, order: 12 },
      ];

      const levels = await tx
        .insert(schema.levels)
        .values(levelData)
        .returning({ id: schema.levels.id });

      // 4. تفعيل المناهج لكل مرحلة (bulk)
      await tx.insert(schema.curriculumLevels).values(
        levels.map(l => ({ academicYearId: yearId, levelId: l.id, isActive: true }))
      );

      // 5. المواد الدراسية (bulk)
      const subjectData: (typeof schema.subjects.$inferInsert)[] = [
        { code: 'ARB', name: 'اللغة العربية',     shortName: 'عربي',   department: 'languages' as const  },
        { code: 'ENG', name: 'اللغة الإنجليزية', shortName: 'إنج',   department: 'languages' as const  },
        { code: 'MATH',name: 'الرياضيات',         shortName: 'رياض',  department: 'mathematics' as const},
        { code: 'SCI', name: 'العلوم',            shortName: 'علوم',  department: 'sciences' as const   },
        { code: 'ISLM',name: 'التربية الإسلامية', shortName: 'إسلام', department: 'religious' as const  },
        { code: 'HIST',name: 'التاريخ والجغرافيا',shortName: 'تاريخ', department: 'social' as const     },
      ];

      const subjects = await tx
        .insert(schema.subjects)
        .values(subjectData)
        .returning({ id: schema.subjects.id, name: schema.subjects.name });

      // 6. ربط المواد بالمناهج لكل مرحلة (bulk – prepared data)
      const curriculumSubjectsData: (typeof schema.curriculumSubjects.$inferInsert)[] = [];
      for (const level of levels) {
        let levelOrder = 1;
        for (const subj of subjects) {
          curriculumSubjectsData.push({
            academicYearId: yearId,
            levelId: level.id,
            subjectId: subj.id,
            isMandatory: true,
            weeklyPeriods: (subj.name ?? '').includes('رياضيات') || (subj.name ?? '').includes('علوم') ? '5.00' : '4.00',
            orderInReport: levelOrder++,
            maxScore: '100.00',
          });
        }
      }

      await tx.insert(schema.curriculumSubjects).values(curriculumSubjectsData);

      // 7. الموظفون + الأدوار (bulk)
      const adminNames = ['محمد علي الشريف', 'أحمد صالح القاضي'];
      const teacherNames = [
        'فاطمة عبدالله الجابري',
        'علي حسن الدبعي',
        'مريم يحيى السنيني',
        'خالد عبدالكريم المطري',
      ];

      // إنشاء users (admins + teachers)
      const adminUsers = await tx
        .insert(schema.users)
        .values(adminNames.map(name => ({ 
          name, 
          email: `${name.replace(/\s+/g, '.').toLowerCase()}@admin.school.ye`, 
          role: 'admin' as const
        })))
        .returning({ id: schema.users.id });

      const teacherUsers = await tx
        .insert(schema.users)
        .values(teacherNames.map((name, i) => ({
          name,
          email: `t${i+1}@school.ye`,
          role: 'admin' as const, // حسب الـ enum الحالي – يمكن تعديله لاحقاً
        })))
        .returning({ id: schema.users.id });

      // Employees
      const employeeData: (typeof schema.employees.$inferInsert)[] = [
        ...adminUsers.map((u, i) => ({
          userId: u.id,
          employeeNumber: `ADM-${2025}-${String(i+1).padStart(3,'0')}`,
          hireDate: new Date('2020-08-01'),
          phone: `+967 777 12 ${30 + i}${30 + i}`,
          sex: 'male' as const,
          birthday: new Date(1982 + i*3, 4, 15),
          nationality: 'Yemeni',
        })),
        ...teacherUsers.map((u, i) => ({
          userId: u.id,
          employeeNumber: `TCH-${2025}-${String(i+1).padStart(3,'0')}`,
          hireDate: new Date('2022-09-01'),
          phone: `+967 771 00 ${10 + i}${10 + i}`,
          sex: i % 2 === 0 ? 'female' as const : 'male' as const,
          birthday: new Date(1988 + i*2, 6, 10),
          nationality: 'Yemeni',
        })),
      ];

      await tx.insert(schema.employees).values(employeeData);

      // Employee roles (bulk)
      const roleData: (typeof schema.employeeRoles.$inferInsert)[] = [
        ...adminUsers.map(u => ({ employeeId: u.id, role: 'admin' as const, startDate: new Date('2020-08-01') })),
        ...teacherUsers.map(u => ({ employeeId: u.id, role: 'teacher' as const, startDate: new Date('2022-09-01') })),
      ];

      await tx.insert(schema.employeeRoles).values(roleData);

      // ───────────────────────────────────────────────
      // باقي الجداول (يمكن توسيعها بنفس الأسلوب: جمع البيانات أولاً ثم bulk insert)
      // ───────────────────────────────────────────────

      // مثال: إجازات (realistic Yemeni & Islamic holidays approx 2025-2026)
      await tx.insert(schema.holidays).values([
        { name: 'عيد الفطر المبارك',        date: '2026-03-21', academicYearId: yearId },
        { name: 'عيد الأضحى المبارك',       date: '2026-05-28', academicYearId: yearId },
        { name: 'المولد النبوي الشريف',     date: '2026-09-04', academicYearId: yearId },
        { name: 'ثورة 26 سبتمبر',           date: '2025-09-26', academicYearId: yearId },
        { name: 'عيد الثورة 14 أكتوبر',     date: '2025-10-14', academicYearId: yearId },
      ]);
    });

    console.log('Seeding successful ✓');
    process.exit(0);
  } catch (err) {
    console.error('Error during seeding:', err);
    process.exit(1);
  }
}

seed();