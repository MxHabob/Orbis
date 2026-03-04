-- Migration: Make classes year-independent
-- Classes are now permanent entities. academicYearId is removed.
-- The enrollment table handles the year context.

-- Step 1: Before removing the column, ensure there are no duplicate class names
-- This query helps identify potential duplicates
-- SELECT name, array_agg(id ORDER BY id) as ids, count(*) 
-- FROM classes 
-- WHERE deleted_at IS NULL AND parent_class_id IS NULL 
-- GROUP BY name 
-- HAVING count(*) > 1;

-- Step 2: Drop the old index
DROP INDEX IF EXISTS class_year_idx;

-- Step 3: Create the new name index
CREATE INDEX IF NOT EXISTS class_name_idx ON classes (name);

-- Step 4: Remove the foreign key constraint and column
ALTER TABLE classes DROP CONSTRAINT IF EXISTS classes_academic_year_id_academic_years_id_fk;
ALTER TABLE classes DROP COLUMN IF EXISTS academic_year_id;

-- Step 5: Verify enrollments cover all student-class-year relationships
-- (enrollments already has academic_year_id, class_id, student_id)
-- No changes needed to enrollments table.

-- IMPORTANT: If you have duplicate classes (same name, different years),
-- you need to merge them BEFORE running this migration.
-- Use the following approach:
-- 1. Pick the lowest-id class as the "canonical" class
-- 2. Update all enrollments, assessment_items, scheduled_lessons, etc. 
--    to point to the canonical class
-- 3. Soft-delete the duplicate classes
