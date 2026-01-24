CREATE TYPE "public"."assessment_type" AS ENUM('exam', 'quiz', 'assignment', 'project', 'participation', 'oral', 'practical', 'ministerial');--> statement-breakpoint
CREATE TYPE "public"."day" AS ENUM('monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday');--> statement-breakpoint
CREATE TYPE "public"."department" AS ENUM('sciences', 'literary', 'mathematics', 'languages', 'religious', 'social', 'other');--> statement-breakpoint
CREATE TYPE "public"."exam_type" AS ENUM('monthly', 'semester1', 'semester2', 'final_year', 'ministerial');--> statement-breakpoint
CREATE TYPE "public"."fee_type" AS ENUM('tuition', 'registration', 'transportation', 'books', 'other');--> statement-breakpoint
CREATE TYPE "public"."payment_status" AS ENUM('pending', 'paid', 'overdue');--> statement-breakpoint
CREATE TYPE "public"."promotion_status" AS ENUM('passed', 'failed', 'repeat', 'promoted', 'supplementary');--> statement-breakpoint
CREATE TYPE "public"."report_type" AS ENUM('ministerial_grade9', 'ministerial_grade12');--> statement-breakpoint
CREATE TYPE "public"."role" AS ENUM('admin', 'student', 'parent');--> statement-breakpoint
CREATE TYPE "public"."section" AS ENUM('male', 'female', 'mixed');--> statement-breakpoint
CREATE TYPE "public"."staff_role" AS ENUM('teacher', 'admin', 'accountant', 'nurse', 'security', 'other');--> statement-breakpoint
CREATE TYPE "public"."stage" AS ENUM('basic', 'secondary');--> statement-breakpoint
CREATE TYPE "public"."student_status" AS ENUM('active', 'graduated', 'transferred', 'suspended', 'dropped', 'repeating');--> statement-breakpoint
CREATE TYPE "public"."track" AS ENUM('scientific', 'literary', 'none');--> statement-breakpoint
CREATE TYPE "public"."user_sex" AS ENUM('male', 'female');--> statement-breakpoint
CREATE TABLE "academic_terms" (
	"id" serial PRIMARY KEY NOT NULL,
	"academic_year_id" integer NOT NULL,
	"name" varchar(100) NOT NULL,
	"start_date" timestamp NOT NULL,
	"end_date" timestamp,
	"order" integer NOT NULL,
	CONSTRAINT "academic_terms_academic_year_id_order_unique" UNIQUE("academic_year_id","order")
);
--> statement-breakpoint
CREATE TABLE "academic_years" (
	"id" serial PRIMARY KEY NOT NULL,
	"year" varchar(20) NOT NULL,
	"start_date" timestamp NOT NULL,
	"end_date" timestamp,
	"is_current" boolean DEFAULT false NOT NULL,
	"is_active" boolean DEFAULT true NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	CONSTRAINT "academic_years_year_unique" UNIQUE("year")
);
--> statement-breakpoint
CREATE TABLE "accounts" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"account_id" varchar(255) NOT NULL,
	"provider_id" varchar(255) NOT NULL,
	"user_id" uuid NOT NULL,
	"access_token" text,
	"refresh_token" text,
	"id_token" text,
	"scope" text,
	"expires_at" timestamp,
	"password" text,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	CONSTRAINT "accounts_provider_account_unique" UNIQUE("provider_id","account_id"),
	CONSTRAINT "accounts_provider_user_unique" UNIQUE("provider_id","user_id")
);
--> statement-breakpoint
CREATE TABLE "assessment_categories" (
	"id" serial PRIMARY KEY NOT NULL,
	"name" varchar(100) NOT NULL,
	"weight" numeric(5, 2) NOT NULL,
	"academic_year_id" integer,
	"curriculum_subject_id" integer,
	"order" integer,
	CONSTRAINT "assessment_categories_name_subject_unique" UNIQUE("name","curriculum_subject_id")
);
--> statement-breakpoint
CREATE TABLE "assessment_items" (
	"id" serial PRIMARY KEY NOT NULL,
	"title" varchar(255) NOT NULL,
	"type" "assessment_type" NOT NULL,
	"max_score" numeric(6, 2) DEFAULT '100' NOT NULL,
	"weight" numeric(5, 2),
	"due_date" timestamp,
	"assessment_category_id" integer,
	"curriculum_subject_id" integer,
	"term_id" integer,
	"is_ministerial" boolean DEFAULT false NOT NULL
);
--> statement-breakpoint
CREATE TABLE "attendances" (
	"id" serial PRIMARY KEY NOT NULL,
	"date" timestamp NOT NULL,
	"present" boolean NOT NULL,
	"student_id" uuid NOT NULL,
	"lesson_id" integer,
	"term_id" integer,
	"academic_year_id" integer
);
--> statement-breakpoint
CREATE TABLE "certificates" (
	"id" serial PRIMARY KEY NOT NULL,
	"title" varchar(255) NOT NULL,
	"issue_date" timestamp NOT NULL,
	"student_id" uuid,
	"details" text
);
--> statement-breakpoint
CREATE TABLE "class_subjects" (
	"id" serial PRIMARY KEY NOT NULL,
	"class_id" integer NOT NULL,
	"curriculum_subject_id" integer NOT NULL,
	CONSTRAINT "class_subjects_class_curriculum_unique" UNIQUE("class_id","curriculum_subject_id")
);
--> statement-breakpoint
CREATE TABLE "classes" (
	"id" serial PRIMARY KEY NOT NULL,
	"name" varchar(100) NOT NULL,
	"code" varchar(50),
	"division" varchar(10),
	"capacity" integer,
	"academic_year_id" integer NOT NULL,
	"level_id" integer NOT NULL,
	"section" "section",
	"track" "track",
	"supervisor_id" uuid,
	"current" boolean DEFAULT true NOT NULL,
	CONSTRAINT "classes_code_unique" UNIQUE("code")
);
--> statement-breakpoint
CREATE TABLE "curriculum_levels" (
	"id" serial PRIMARY KEY NOT NULL,
	"academic_year_id" integer NOT NULL,
	"level_id" integer NOT NULL,
	"is_active" boolean DEFAULT true NOT NULL,
	CONSTRAINT "curriculum_levels_year_level_unique" UNIQUE("academic_year_id","level_id")
);
--> statement-breakpoint
CREATE TABLE "curriculum_subjects" (
	"id" serial PRIMARY KEY NOT NULL,
	"academic_year_id" integer NOT NULL,
	"level_id" integer,
	"class_id" integer,
	"subject_id" integer NOT NULL,
	"is_mandatory" boolean DEFAULT true NOT NULL,
	"weekly_periods" numeric(4, 2),
	"order_in_report" integer,
	"max_score" numeric(6, 2) DEFAULT '100',
	CONSTRAINT "curriculum_subjects_year_level_subject_unique" UNIQUE("academic_year_id","level_id","subject_id")
);
--> statement-breakpoint
CREATE TABLE "employee_roles" (
	"employee_id" uuid NOT NULL,
	"role" "staff_role" NOT NULL,
	"start_date" timestamp,
	"end_date" timestamp,
	CONSTRAINT "employee_roles_employee_id_role_pk" PRIMARY KEY("employee_id","role")
);
--> statement-breakpoint
CREATE TABLE "employees" (
	"user_id" uuid PRIMARY KEY NOT NULL,
	"employee_number" varchar(50),
	"hire_date" timestamp,
	"phone" varchar(50),
	"address" text,
	"sex" "user_sex" NOT NULL,
	"birthday" timestamp,
	"nationality" varchar(100) DEFAULT 'Yemeni',
	CONSTRAINT "employees_employee_number_unique" UNIQUE("employee_number")
);
--> statement-breakpoint
CREATE TABLE "exam_committees" (
	"id" serial PRIMARY KEY NOT NULL,
	"name" varchar(255) NOT NULL,
	"exam_id" integer NOT NULL,
	"capacity" integer,
	"location" varchar(255),
	"class_id" integer
);
--> statement-breakpoint
CREATE TABLE "exam_seatings" (
	"id" serial PRIMARY KEY NOT NULL,
	"student_id" uuid NOT NULL,
	"committee_id" integer NOT NULL,
	"seating_number" varchar(50) NOT NULL,
	"assigned_at" timestamp DEFAULT now() NOT NULL,
	CONSTRAINT "exam_seatings_seating_number_unique" UNIQUE("seating_number")
);
--> statement-breakpoint
CREATE TABLE "exams" (
	"id" serial PRIMARY KEY NOT NULL,
	"title" varchar(255) NOT NULL,
	"start_time" timestamp NOT NULL,
	"end_time" timestamp NOT NULL,
	"type" "exam_type" NOT NULL,
	"term_id" integer,
	"academic_year_id" integer
);
--> statement-breakpoint
CREATE TABLE "extracurricular_students" (
	"extracurricular_id" integer NOT NULL,
	"student_id" uuid NOT NULL,
	CONSTRAINT "extracurricular_students_extracurricular_id_student_id_pk" PRIMARY KEY("extracurricular_id","student_id")
);
--> statement-breakpoint
CREATE TABLE "extracurriculars" (
	"id" serial PRIMARY KEY NOT NULL,
	"name" varchar(255) NOT NULL,
	"description" text,
	"start_date" timestamp NOT NULL,
	"end_date" timestamp
);
--> statement-breakpoint
CREATE TABLE "grade_entries" (
	"id" serial PRIMARY KEY NOT NULL,
	"student_id" uuid NOT NULL,
	"assessment_item_id" integer NOT NULL,
	"score" numeric(6, 2),
	"is_absent" boolean DEFAULT false NOT NULL,
	"comments" text,
	"graded_at" timestamp DEFAULT now() NOT NULL,
	"graded_by_id" uuid,
	"academic_year_id" integer NOT NULL,
	"term_id" integer NOT NULL,
	"is_final" boolean DEFAULT false NOT NULL,
	CONSTRAINT "grade_entries_student_assessment_unique" UNIQUE("student_id","assessment_item_id")
);
--> statement-breakpoint
CREATE TABLE "health_records" (
	"id" serial PRIMARY KEY NOT NULL,
	"student_id" uuid NOT NULL,
	"record" text NOT NULL,
	"date" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "holidays" (
	"id" serial PRIMARY KEY NOT NULL,
	"name" varchar(100) NOT NULL,
	"date" date NOT NULL,
	"academic_year_id" integer
);
--> statement-breakpoint
CREATE TABLE "lessons" (
	"id" serial PRIMARY KEY NOT NULL,
	"name" varchar(255),
	"day" "day" NOT NULL,
	"start_time" timestamp NOT NULL,
	"end_time" timestamp NOT NULL,
	"teacher_class_subject_id" integer NOT NULL,
	"academic_year_id" integer,
	"teacher_id" uuid,
	"class_id" integer
);
--> statement-breakpoint
CREATE TABLE "levels" (
	"id" serial PRIMARY KEY NOT NULL,
	"name" varchar(100) NOT NULL,
	"stage" "stage" NOT NULL,
	"order" integer NOT NULL,
	CONSTRAINT "levels_order_unique" UNIQUE("order")
);
--> statement-breakpoint
CREATE TABLE "ministerial_reports" (
	"id" serial PRIMARY KEY NOT NULL,
	"title" varchar(255) NOT NULL,
	"report_date" timestamp NOT NULL,
	"content" text NOT NULL,
	"generated_by" varchar(255) NOT NULL,
	"level_id" integer,
	"class_id" integer,
	"student_id" uuid,
	"total_score" numeric(7, 2),
	"average" numeric(5, 2),
	"pass_status" varchar(100),
	"report_type" "report_type"
);
--> statement-breakpoint
CREATE TABLE "parents" (
	"user_id" uuid PRIMARY KEY NOT NULL,
	"surname" varchar(255) NOT NULL,
	"phone" varchar(50) NOT NULL,
	"address" text,
	CONSTRAINT "parents_phone_unique" UNIQUE("phone")
);
--> statement-breakpoint
CREATE TABLE "payments" (
	"id" serial PRIMARY KEY NOT NULL,
	"student_id" uuid NOT NULL,
	"fee_type" "fee_type" NOT NULL,
	"amount" numeric(10, 2) NOT NULL,
	"due_date" timestamp NOT NULL,
	"paid_date" timestamp,
	"status" "payment_status" NOT NULL,
	"description" text,
	"academic_year_id" integer
);
--> statement-breakpoint
CREATE TABLE "sessions" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"token" varchar(255) NOT NULL,
	"user_id" uuid NOT NULL,
	"expires_at" timestamp NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	"ip_address" varchar(45),
	"user_agent" text,
	CONSTRAINT "sessions_token_unique" UNIQUE("token")
);
--> statement-breakpoint
CREATE TABLE "student_enrollments" (
	"id" serial PRIMARY KEY NOT NULL,
	"student_id" uuid NOT NULL,
	"academic_year_id" integer NOT NULL,
	"class_id" integer NOT NULL,
	"status" varchar(100),
	"final_average" numeric(5, 2),
	"ministerial_total_score" numeric(7, 2),
	"ministerial_average" numeric(5, 2),
	"promotion_decision" "promotion_status",
	CONSTRAINT "student_enrollments_student_year_unique" UNIQUE("student_id","academic_year_id")
);
--> statement-breakpoint
CREATE TABLE "students" (
	"user_id" uuid PRIMARY KEY NOT NULL,
	"student_number" varchar(50) NOT NULL,
	"current_class_id" integer,
	"current_academic_year_id" integer,
	"enrollment_date" timestamp,
	"status" "student_status" DEFAULT 'active' NOT NULL,
	"surname" varchar(255) NOT NULL,
	"phone" varchar(50),
	"address" text,
	"sex" "user_sex" NOT NULL,
	"birthday" timestamp NOT NULL,
	"nationality" varchar(100) DEFAULT 'Yemeni',
	"religion" varchar(50) DEFAULT 'Muslim',
	"parent_id" uuid,
	CONSTRAINT "students_student_number_unique" UNIQUE("student_number"),
	CONSTRAINT "students_phone_unique" UNIQUE("phone")
);
--> statement-breakpoint
CREATE TABLE "subjects" (
	"id" serial PRIMARY KEY NOT NULL,
	"code" varchar(50),
	"name" varchar(255) NOT NULL,
	"short_name" varchar(50),
	"department" "department",
	"description" text,
	CONSTRAINT "subjects_code_unique" UNIQUE("code")
);
--> statement-breakpoint
CREATE TABLE "teacher_class_subjects" (
	"id" serial PRIMARY KEY NOT NULL,
	"employee_id" uuid NOT NULL,
	"curriculum_subject_id" integer NOT NULL,
	"class_id" integer NOT NULL,
	"class_subject_id" integer,
	"teacher_subject_id" integer,
	"start_date" timestamp,
	"end_date" timestamp,
	CONSTRAINT "teacher_class_subjects_unique_assignment" UNIQUE("employee_id","curriculum_subject_id","class_id")
);
--> statement-breakpoint
CREATE TABLE "teacher_subjects" (
	"id" serial PRIMARY KEY NOT NULL,
	"employee_id" uuid NOT NULL,
	"subject_id" integer NOT NULL,
	"start_year" integer,
	"end_year" integer,
	CONSTRAINT "teacher_subjects_employee_subject_unique" UNIQUE("employee_id","subject_id")
);
--> statement-breakpoint
CREATE TABLE "users" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"name" varchar(255),
	"email" varchar(255),
	"email_verified" boolean DEFAULT false,
	"image" text,
	"role" "role" DEFAULT 'student' NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	CONSTRAINT "users_email_unique" UNIQUE("email")
);
--> statement-breakpoint
CREATE TABLE "verifications" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"identifier" varchar(255) NOT NULL,
	"value" varchar(255) NOT NULL,
	"expires_at" timestamp NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	CONSTRAINT "verifications_identifier_value_unique" UNIQUE("identifier","value")
);
--> statement-breakpoint
ALTER TABLE "academic_terms" ADD CONSTRAINT "academic_terms_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "accounts" ADD CONSTRAINT "accounts_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "assessment_categories" ADD CONSTRAINT "assessment_categories_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "assessment_categories" ADD CONSTRAINT "assessment_categories_curriculum_subject_id_curriculum_subjects_id_fk" FOREIGN KEY ("curriculum_subject_id") REFERENCES "public"."curriculum_subjects"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "assessment_items" ADD CONSTRAINT "assessment_items_assessment_category_id_assessment_categories_id_fk" FOREIGN KEY ("assessment_category_id") REFERENCES "public"."assessment_categories"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "assessment_items" ADD CONSTRAINT "assessment_items_curriculum_subject_id_curriculum_subjects_id_fk" FOREIGN KEY ("curriculum_subject_id") REFERENCES "public"."curriculum_subjects"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "assessment_items" ADD CONSTRAINT "assessment_items_term_id_academic_terms_id_fk" FOREIGN KEY ("term_id") REFERENCES "public"."academic_terms"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "attendances" ADD CONSTRAINT "attendances_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "attendances" ADD CONSTRAINT "attendances_lesson_id_lessons_id_fk" FOREIGN KEY ("lesson_id") REFERENCES "public"."lessons"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "attendances" ADD CONSTRAINT "attendances_term_id_academic_terms_id_fk" FOREIGN KEY ("term_id") REFERENCES "public"."academic_terms"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "attendances" ADD CONSTRAINT "attendances_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "certificates" ADD CONSTRAINT "certificates_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "class_subjects" ADD CONSTRAINT "class_subjects_class_id_classes_id_fk" FOREIGN KEY ("class_id") REFERENCES "public"."classes"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "class_subjects" ADD CONSTRAINT "class_subjects_curriculum_subject_id_curriculum_subjects_id_fk" FOREIGN KEY ("curriculum_subject_id") REFERENCES "public"."curriculum_subjects"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "classes" ADD CONSTRAINT "classes_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "classes" ADD CONSTRAINT "classes_level_id_levels_id_fk" FOREIGN KEY ("level_id") REFERENCES "public"."levels"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "classes" ADD CONSTRAINT "classes_supervisor_id_employees_user_id_fk" FOREIGN KEY ("supervisor_id") REFERENCES "public"."employees"("user_id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "curriculum_levels" ADD CONSTRAINT "curriculum_levels_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "curriculum_levels" ADD CONSTRAINT "curriculum_levels_level_id_levels_id_fk" FOREIGN KEY ("level_id") REFERENCES "public"."levels"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "curriculum_subjects" ADD CONSTRAINT "curriculum_subjects_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "curriculum_subjects" ADD CONSTRAINT "curriculum_subjects_level_id_levels_id_fk" FOREIGN KEY ("level_id") REFERENCES "public"."levels"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "curriculum_subjects" ADD CONSTRAINT "curriculum_subjects_class_id_classes_id_fk" FOREIGN KEY ("class_id") REFERENCES "public"."classes"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "curriculum_subjects" ADD CONSTRAINT "curriculum_subjects_subject_id_subjects_id_fk" FOREIGN KEY ("subject_id") REFERENCES "public"."subjects"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "employee_roles" ADD CONSTRAINT "employee_roles_employee_id_employees_user_id_fk" FOREIGN KEY ("employee_id") REFERENCES "public"."employees"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "employees" ADD CONSTRAINT "employees_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "exam_committees" ADD CONSTRAINT "exam_committees_exam_id_exams_id_fk" FOREIGN KEY ("exam_id") REFERENCES "public"."exams"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "exam_committees" ADD CONSTRAINT "exam_committees_class_id_classes_id_fk" FOREIGN KEY ("class_id") REFERENCES "public"."classes"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "exam_seatings" ADD CONSTRAINT "exam_seatings_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "exam_seatings" ADD CONSTRAINT "exam_seatings_committee_id_exam_committees_id_fk" FOREIGN KEY ("committee_id") REFERENCES "public"."exam_committees"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "exams" ADD CONSTRAINT "exams_term_id_academic_terms_id_fk" FOREIGN KEY ("term_id") REFERENCES "public"."academic_terms"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "exams" ADD CONSTRAINT "exams_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "extracurricular_students" ADD CONSTRAINT "extracurricular_students_extracurricular_id_extracurriculars_id_fk" FOREIGN KEY ("extracurricular_id") REFERENCES "public"."extracurriculars"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "extracurricular_students" ADD CONSTRAINT "extracurricular_students_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grade_entries" ADD CONSTRAINT "grade_entries_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grade_entries" ADD CONSTRAINT "grade_entries_assessment_item_id_assessment_items_id_fk" FOREIGN KEY ("assessment_item_id") REFERENCES "public"."assessment_items"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grade_entries" ADD CONSTRAINT "grade_entries_graded_by_id_employees_user_id_fk" FOREIGN KEY ("graded_by_id") REFERENCES "public"."employees"("user_id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grade_entries" ADD CONSTRAINT "grade_entries_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grade_entries" ADD CONSTRAINT "grade_entries_term_id_academic_terms_id_fk" FOREIGN KEY ("term_id") REFERENCES "public"."academic_terms"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "health_records" ADD CONSTRAINT "health_records_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "holidays" ADD CONSTRAINT "holidays_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "lessons" ADD CONSTRAINT "lessons_teacher_class_subject_id_teacher_class_subjects_id_fk" FOREIGN KEY ("teacher_class_subject_id") REFERENCES "public"."teacher_class_subjects"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "lessons" ADD CONSTRAINT "lessons_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "lessons" ADD CONSTRAINT "lessons_teacher_id_employees_user_id_fk" FOREIGN KEY ("teacher_id") REFERENCES "public"."employees"("user_id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "lessons" ADD CONSTRAINT "lessons_class_id_classes_id_fk" FOREIGN KEY ("class_id") REFERENCES "public"."classes"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ministerial_reports" ADD CONSTRAINT "ministerial_reports_level_id_levels_id_fk" FOREIGN KEY ("level_id") REFERENCES "public"."levels"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ministerial_reports" ADD CONSTRAINT "ministerial_reports_class_id_classes_id_fk" FOREIGN KEY ("class_id") REFERENCES "public"."classes"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "ministerial_reports" ADD CONSTRAINT "ministerial_reports_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "parents" ADD CONSTRAINT "parents_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "payments" ADD CONSTRAINT "payments_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "payments" ADD CONSTRAINT "payments_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "sessions" ADD CONSTRAINT "sessions_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_enrollments" ADD CONSTRAINT "student_enrollments_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_enrollments" ADD CONSTRAINT "student_enrollments_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_enrollments" ADD CONSTRAINT "student_enrollments_class_id_classes_id_fk" FOREIGN KEY ("class_id") REFERENCES "public"."classes"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "students" ADD CONSTRAINT "students_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "students" ADD CONSTRAINT "students_current_class_id_classes_id_fk" FOREIGN KEY ("current_class_id") REFERENCES "public"."classes"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "students" ADD CONSTRAINT "students_current_academic_year_id_academic_years_id_fk" FOREIGN KEY ("current_academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "students" ADD CONSTRAINT "students_parent_id_parents_user_id_fk" FOREIGN KEY ("parent_id") REFERENCES "public"."parents"("user_id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "teacher_class_subjects" ADD CONSTRAINT "teacher_class_subjects_employee_id_employees_user_id_fk" FOREIGN KEY ("employee_id") REFERENCES "public"."employees"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "teacher_class_subjects" ADD CONSTRAINT "teacher_class_subjects_curriculum_subject_id_curriculum_subjects_id_fk" FOREIGN KEY ("curriculum_subject_id") REFERENCES "public"."curriculum_subjects"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "teacher_class_subjects" ADD CONSTRAINT "teacher_class_subjects_class_id_classes_id_fk" FOREIGN KEY ("class_id") REFERENCES "public"."classes"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "teacher_class_subjects" ADD CONSTRAINT "teacher_class_subjects_class_subject_id_class_subjects_id_fk" FOREIGN KEY ("class_subject_id") REFERENCES "public"."class_subjects"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "teacher_class_subjects" ADD CONSTRAINT "teacher_class_subjects_teacher_subject_id_teacher_subjects_id_fk" FOREIGN KEY ("teacher_subject_id") REFERENCES "public"."teacher_subjects"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "teacher_subjects" ADD CONSTRAINT "teacher_subjects_employee_id_employees_user_id_fk" FOREIGN KEY ("employee_id") REFERENCES "public"."employees"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "teacher_subjects" ADD CONSTRAINT "teacher_subjects_subject_id_subjects_id_fk" FOREIGN KEY ("subject_id") REFERENCES "public"."subjects"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
CREATE INDEX "academic_years_is_current_idx" ON "academic_years" USING btree ("is_current");--> statement-breakpoint
CREATE INDEX "classes_academic_year_name_idx" ON "classes" USING btree ("academic_year_id","name");--> statement-breakpoint
CREATE INDEX "classes_level_track_idx" ON "classes" USING btree ("level_id","track");--> statement-breakpoint
CREATE INDEX "classes_division_idx" ON "classes" USING btree ("division");--> statement-breakpoint
CREATE INDEX "curriculum_subjects_year_idx" ON "curriculum_subjects" USING btree ("academic_year_id");--> statement-breakpoint
CREATE INDEX "grade_entries_student_year_idx" ON "grade_entries" USING btree ("student_id","academic_year_id");--> statement-breakpoint
CREATE INDEX "grade_entries_assessment_item_idx" ON "grade_entries" USING btree ("assessment_item_id");--> statement-breakpoint
CREATE INDEX "grade_entries_term_idx" ON "grade_entries" USING btree ("term_id");--> statement-breakpoint
CREATE INDEX "holidays_date_idx" ON "holidays" USING btree ("date");--> statement-breakpoint
CREATE INDEX "payments_student_year_idx" ON "payments" USING btree ("student_id","academic_year_id");--> statement-breakpoint
CREATE INDEX "students_current_class_idx" ON "students" USING btree ("current_class_id");--> statement-breakpoint
CREATE INDEX "students_current_year_idx" ON "students" USING btree ("current_academic_year_id");--> statement-breakpoint
CREATE INDEX "students_student_number_idx" ON "students" USING btree ("student_number");