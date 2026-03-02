-- Promotion System Migration
-- Adds student promotion tracking for end-of-year grade advancement

-- 1. Create enums
DO $$ BEGIN
  CREATE TYPE "public"."promotion_status" AS ENUM('promoted', 'retained', 'transferred', 'graduated', 'conditional');
EXCEPTION
  WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
  CREATE TYPE "public"."promotion_decision" AS ENUM('auto', 'manual', 'override');
EXCEPTION
  WHEN duplicate_object THEN null;
END $$;

-- 2. Create student_promotions table
CREATE TABLE IF NOT EXISTS "student_promotions" (
  "id" serial PRIMARY KEY NOT NULL,
  "student_id" uuid NOT NULL,
  "from_class_id" integer NOT NULL,
  "to_class_id" integer,
  "from_academic_year_id" integer NOT NULL,
  "to_academic_year_id" integer NOT NULL,
  "final_score" numeric(6, 2),
  "status" "promotion_status" NOT NULL,
  "decision" "promotion_decision" DEFAULT 'auto' NOT NULL,
  "decision_by_id" uuid,
  "notes" text,
  "promoted_at" timestamp DEFAULT now() NOT NULL,
  "created_at" timestamp DEFAULT now() NOT NULL
);

-- 3. Add foreign keys
DO $$ BEGIN
  ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;
EXCEPTION
  WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
  ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_from_class_id_classes_id_fk" FOREIGN KEY ("from_class_id") REFERENCES "public"."classes"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
  WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
  ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_to_class_id_classes_id_fk" FOREIGN KEY ("to_class_id") REFERENCES "public"."classes"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
  WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
  ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_from_academic_year_id_academic_years_id_fk" FOREIGN KEY ("from_academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
  WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
  ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_to_academic_year_id_academic_years_id_fk" FOREIGN KEY ("to_academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
  WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
  ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_decision_by_id_users_id_fk" FOREIGN KEY ("decision_by_id") REFERENCES "public"."users"("id") ON DELETE set null ON UPDATE no action;
EXCEPTION
  WHEN duplicate_object THEN null;
END $$;

-- 4. Add indexes
CREATE INDEX IF NOT EXISTS "promo_student_year_idx" ON "student_promotions" USING btree ("student_id","from_academic_year_id");
CREATE INDEX IF NOT EXISTS "promo_from_class_idx" ON "student_promotions" USING btree ("from_class_id","from_academic_year_id");
CREATE INDEX IF NOT EXISTS "promo_status_idx" ON "student_promotions" USING btree ("status");

-- 5. Add unique constraint (one promotion per student per year)
ALTER TABLE "student_promotions" ADD CONSTRAINT "unique_student_promotion_per_year" UNIQUE("student_id","from_academic_year_id");
