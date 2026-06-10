CREATE TYPE "public"."assessment_type" AS ENUM('exam', 'quiz', 'assignment', 'participation', 'oral', 'attendance', 'behavior', 'homework');--> statement-breakpoint
CREATE TYPE "public"."attendance_status" AS ENUM('present', 'absent', 'late', 'excused', 'sick', 'authorized_leave', 'field_trip');--> statement-breakpoint
CREATE TYPE "public"."class_type" AS ENUM('classroom', 'laboratory', 'library', 'computer_lab', 'music_room', 'exam_hall', 'workshop', 'multi-purpose_room');--> statement-breakpoint
CREATE TYPE "public"."document_type" AS ENUM('file', 'folder');--> statement-breakpoint
CREATE TYPE "public"."notification_type" AS ENUM('grade_published', 'new_assessment', 'attendance_alert', 'announcement', 'message', 'parent_meeting', 'fee_reminder', 'account_action', 'behavior_alert', 'system');--> statement-breakpoint
CREATE TYPE "public"."promotion_decision" AS ENUM('auto', 'manual', 'override');--> statement-breakpoint
CREATE TYPE "public"."promotion_status" AS ENUM('promoted', 'retained', 'transferred', 'graduated', 'conditional');--> statement-breakpoint
CREATE TYPE "public"."role" AS ENUM('admin', 'student', 'parent', 'teacher', 'staff');--> statement-breakpoint
CREATE TYPE "public"."student_status" AS ENUM('active', 'graduated', 'transferred', 'suspended');--> statement-breakpoint
CREATE TYPE "public"."user_sex" AS ENUM('unspecified', 'male', 'female');--> statement-breakpoint
CREATE TABLE "academic_terms" (
	"id" serial PRIMARY KEY NOT NULL,
	"name" varchar(50) NOT NULL,
	"academic_year_id" integer,
	"start_date" date,
	"end_date" date,
	"is_current" boolean DEFAULT false,
	"deleted_at" timestamp
);
--> statement-breakpoint
CREATE TABLE "academic_years" (
	"id" serial PRIMARY KEY NOT NULL,
	"year_name" varchar(20) NOT NULL,
	"start_date" date,
	"end_date" date,
	"is_current" boolean DEFAULT false NOT NULL,
	"deleted_at" timestamp,
	CONSTRAINT "academic_years_year_name_unique" UNIQUE("year_name")
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
CREATE TABLE "assessment_items" (
	"id" serial PRIMARY KEY NOT NULL,
	"title" varchar(255) NOT NULL,
	"type" "assessment_type" NOT NULL,
	"max_score" numeric(6, 2) NOT NULL,
	"weight" numeric(5, 2) DEFAULT '1.0',
	"subject_id" integer,
	"class_id" integer,
	"month" integer NOT NULL,
	"term_id" integer,
	"is_published" boolean DEFAULT false NOT NULL,
	"deleted_at" timestamp
);
--> statement-breakpoint
CREATE TABLE "attendance_records" (
	"id" serial PRIMARY KEY NOT NULL,
	"student_id" uuid NOT NULL,
	"scheduled_lesson_id" integer NOT NULL,
	"date" date NOT NULL,
	"status" "attendance_status" DEFAULT 'absent' NOT NULL,
	"arrival_time" time,
	"notes" text,
	"marked_by_id" uuid,
	"is_verified" boolean DEFAULT false NOT NULL,
	"deleted_at" timestamp,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	CONSTRAINT "unique_attendance_student_lesson" UNIQUE("student_id","scheduled_lesson_id")
);
--> statement-breakpoint
CREATE TABLE "audit_logs" (
	"id" serial PRIMARY KEY NOT NULL,
	"action" varchar(50) NOT NULL,
	"table_name" varchar(50) NOT NULL,
	"record_id" varchar(100) NOT NULL,
	"old_data" jsonb,
	"new_data" jsonb,
	"user_id" uuid,
	"timestamp" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "certificate_templates" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"name" varchar(100) NOT NULL,
	"is_default" boolean DEFAULT false NOT NULL,
	"orientation" varchar(20) DEFAULT 'portrait' NOT NULL,
	"size" varchar(20) DEFAULT 'a4' NOT NULL,
	"primary_color" varchar(20) DEFAULT '#000000',
	"background_image_url" text,
	"content" jsonb,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "classes" (
	"id" serial PRIMARY KEY NOT NULL,
	"name" varchar(100) NOT NULL,
	"type" "class_type" DEFAULT 'classroom' NOT NULL,
	"division" varchar(10),
	"parent_class_id" integer,
	"color" text DEFAULT '#e6e6e6' NOT NULL,
	"sort_order" integer DEFAULT 0 NOT NULL,
	"deleted_at" timestamp
);
--> statement-breakpoint
CREATE TABLE "documents" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"name" varchar(255) NOT NULL,
	"title" varchar(255),
	"size" integer,
	"type" "document_type" DEFAULT 'file' NOT NULL,
	"mimetype" varchar(100),
	"extension" varchar(20),
	"key" text,
	"path_tokens" jsonb,
	"user_id" uuid NOT NULL,
	"parent_folder_id" uuid,
	"metadata" jsonb,
	"processing_status" varchar(20) DEFAULT 'completed',
	"is_pinned" boolean DEFAULT false NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	"deleted_at" timestamp
);
--> statement-breakpoint
CREATE TABLE "enrollments" (
	"id" serial PRIMARY KEY NOT NULL,
	"student_id" uuid,
	"class_id" integer,
	"academic_year_id" integer,
	"status" "student_status" DEFAULT 'active',
	"enrolled_at" timestamp DEFAULT now() NOT NULL,
	"deleted_at" timestamp,
	CONSTRAINT "student_class_year_unique" UNIQUE("student_id","class_id","academic_year_id")
);
--> statement-breakpoint
CREATE TABLE "grade_entries" (
	"id" serial PRIMARY KEY NOT NULL,
	"student_id" uuid,
	"assessment_item_id" integer,
	"score" numeric(6, 2),
	"is_absent" boolean DEFAULT false,
	"comments" text,
	"metadata" jsonb,
	"graded_by_id" uuid,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	"deleted_at" timestamp,
	CONSTRAINT "student_assessment_unique" UNIQUE("student_id","assessment_item_id")
);
--> statement-breakpoint
CREATE TABLE "notifications" (
	"id" serial PRIMARY KEY NOT NULL,
	"sender_id" uuid,
	"recipient_id" uuid NOT NULL,
	"class_id" integer,
	"academic_term_id" integer,
	"type" "notification_type" NOT NULL,
	"title" varchar(255) NOT NULL,
	"message" text NOT NULL,
	"related_orbis_type" varchar(50),
	"related_orbis_id" varchar(100),
	"is_read" boolean DEFAULT false NOT NULL,
	"is_archived" boolean DEFAULT false NOT NULL,
	"is_high_priority" boolean DEFAULT false NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"delivered_at" timestamp,
	"metadata" jsonb
);
--> statement-breakpoint
CREATE TABLE "parent_students" (
	"id" serial PRIMARY KEY NOT NULL,
	"parent_id" uuid NOT NULL,
	"student_id" uuid NOT NULL,
	"relationship" varchar(50) DEFAULT 'parent' NOT NULL,
	"is_primary" boolean DEFAULT false NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	CONSTRAINT "parent_student_unique" UNIQUE("parent_id","student_id")
);
--> statement-breakpoint
CREATE TABLE "scheduled_lessons" (
	"id" serial PRIMARY KEY NOT NULL,
	"class_id" integer NOT NULL,
	"subject_id" integer NOT NULL,
	"teacher_id" uuid NOT NULL,
	"time_slot_id" integer NOT NULL,
	"day_of_week" integer NOT NULL,
	"academic_year_id" integer NOT NULL,
	"academic_term_id" integer,
	"room_id" integer,
	"is_active" boolean DEFAULT true,
	"notes" text,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	CONSTRAINT "unique_lesson_per_class_slot_day" UNIQUE("class_id","time_slot_id","day_of_week","academic_year_id")
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
CREATE TABLE "short_links" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"code" varchar(255) NOT NULL,
	"url" text NOT NULL,
	"user_id" uuid NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"expires_at" timestamp,
	CONSTRAINT "short_links_code_unique" UNIQUE("code")
);
--> statement-breakpoint
CREATE TABLE "student_promotions" (
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
	"created_at" timestamp DEFAULT now() NOT NULL,
	CONSTRAINT "unique_student_promotion_per_year" UNIQUE("student_id","from_academic_year_id")
);
--> statement-breakpoint
CREATE TABLE "students" (
	"user_id" uuid PRIMARY KEY NOT NULL,
	"student_number" varchar(20) NOT NULL,
	"secret_number" varchar(20),
	"current_class_id" integer,
	"status" "student_status" DEFAULT 'active',
	"stats" jsonb,
	"deleted_at" timestamp,
	CONSTRAINT "students_student_number_unique" UNIQUE("student_number")
);
--> statement-breakpoint
CREATE TABLE "subjects" (
	"id" serial PRIMARY KEY NOT NULL,
	"name" varchar(255) NOT NULL,
	"code" varchar(50),
	"color" text DEFAULT '#FFF3E0' NOT NULL,
	"deleted_at" timestamp,
	CONSTRAINT "subjects_code_unique" UNIQUE("code")
);
--> statement-breakpoint
CREATE TABLE "time_slots" (
	"id" serial PRIMARY KEY NOT NULL,
	"slot_number" integer NOT NULL,
	"start_time" time NOT NULL,
	"end_time" time NOT NULL,
	"duration_minutes" integer DEFAULT 40 NOT NULL,
	"is_break" boolean DEFAULT false,
	"academic_year_id" integer NOT NULL,
	"deleted_at" timestamp,
	CONSTRAINT "unique_slot_per_year" UNIQUE("academic_year_id","slot_number")
);
--> statement-breakpoint
CREATE TABLE "users" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"username" varchar(255),
	"name" varchar(255),
	"phone" varchar(255),
	"email" varchar(255),
	"email_verified" boolean DEFAULT false NOT NULL,
	"role" "role" DEFAULT 'student' NOT NULL,
	"sex" "user_sex" DEFAULT 'unspecified' NOT NULL,
	"image" varchar(255),
	"locale" varchar(255) DEFAULT 'en',
	"timezone" varchar(255) DEFAULT 'Asia/Aden',
	"time_format" integer DEFAULT 24,
	"week_starts_on_monday" boolean DEFAULT false,
	"metadata" jsonb,
	"deleted_at" timestamp,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	CONSTRAINT "users_username_unique" UNIQUE("username"),
	CONSTRAINT "users_phone_unique" UNIQUE("phone"),
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
ALTER TABLE "academic_terms" ADD CONSTRAINT "academic_terms_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "accounts" ADD CONSTRAINT "accounts_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "assessment_items" ADD CONSTRAINT "assessment_items_subject_id_subjects_id_fk" FOREIGN KEY ("subject_id") REFERENCES "public"."subjects"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "assessment_items" ADD CONSTRAINT "assessment_items_class_id_classes_id_fk" FOREIGN KEY ("class_id") REFERENCES "public"."classes"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "assessment_items" ADD CONSTRAINT "assessment_items_term_id_academic_terms_id_fk" FOREIGN KEY ("term_id") REFERENCES "public"."academic_terms"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "attendance_records" ADD CONSTRAINT "attendance_records_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "attendance_records" ADD CONSTRAINT "attendance_records_scheduled_lesson_id_scheduled_lessons_id_fk" FOREIGN KEY ("scheduled_lesson_id") REFERENCES "public"."scheduled_lessons"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "attendance_records" ADD CONSTRAINT "attendance_records_marked_by_id_users_id_fk" FOREIGN KEY ("marked_by_id") REFERENCES "public"."users"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "audit_logs" ADD CONSTRAINT "audit_logs_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "classes" ADD CONSTRAINT "classes_parent_class_id_classes_id_fk" FOREIGN KEY ("parent_class_id") REFERENCES "public"."classes"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "documents" ADD CONSTRAINT "documents_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "documents" ADD CONSTRAINT "documents_parent_folder_id_documents_id_fk" FOREIGN KEY ("parent_folder_id") REFERENCES "public"."documents"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "enrollments" ADD CONSTRAINT "enrollments_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "enrollments" ADD CONSTRAINT "enrollments_class_id_classes_id_fk" FOREIGN KEY ("class_id") REFERENCES "public"."classes"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "enrollments" ADD CONSTRAINT "enrollments_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grade_entries" ADD CONSTRAINT "grade_entries_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grade_entries" ADD CONSTRAINT "grade_entries_assessment_item_id_assessment_items_id_fk" FOREIGN KEY ("assessment_item_id") REFERENCES "public"."assessment_items"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "grade_entries" ADD CONSTRAINT "grade_entries_graded_by_id_users_id_fk" FOREIGN KEY ("graded_by_id") REFERENCES "public"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "notifications_sender_id_users_id_fk" FOREIGN KEY ("sender_id") REFERENCES "public"."users"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "notifications_recipient_id_users_id_fk" FOREIGN KEY ("recipient_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "notifications_class_id_classes_id_fk" FOREIGN KEY ("class_id") REFERENCES "public"."classes"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "notifications" ADD CONSTRAINT "notifications_academic_term_id_academic_terms_id_fk" FOREIGN KEY ("academic_term_id") REFERENCES "public"."academic_terms"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "parent_students" ADD CONSTRAINT "parent_students_parent_id_users_id_fk" FOREIGN KEY ("parent_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "parent_students" ADD CONSTRAINT "parent_students_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "scheduled_lessons" ADD CONSTRAINT "scheduled_lessons_class_id_classes_id_fk" FOREIGN KEY ("class_id") REFERENCES "public"."classes"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "scheduled_lessons" ADD CONSTRAINT "scheduled_lessons_subject_id_subjects_id_fk" FOREIGN KEY ("subject_id") REFERENCES "public"."subjects"("id") ON DELETE restrict ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "scheduled_lessons" ADD CONSTRAINT "scheduled_lessons_teacher_id_users_id_fk" FOREIGN KEY ("teacher_id") REFERENCES "public"."users"("id") ON DELETE restrict ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "scheduled_lessons" ADD CONSTRAINT "scheduled_lessons_time_slot_id_time_slots_id_fk" FOREIGN KEY ("time_slot_id") REFERENCES "public"."time_slots"("id") ON DELETE restrict ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "scheduled_lessons" ADD CONSTRAINT "scheduled_lessons_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "scheduled_lessons" ADD CONSTRAINT "scheduled_lessons_academic_term_id_academic_terms_id_fk" FOREIGN KEY ("academic_term_id") REFERENCES "public"."academic_terms"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "scheduled_lessons" ADD CONSTRAINT "scheduled_lessons_room_id_classes_id_fk" FOREIGN KEY ("room_id") REFERENCES "public"."classes"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "sessions" ADD CONSTRAINT "sessions_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "short_links" ADD CONSTRAINT "short_links_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_from_class_id_classes_id_fk" FOREIGN KEY ("from_class_id") REFERENCES "public"."classes"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_to_class_id_classes_id_fk" FOREIGN KEY ("to_class_id") REFERENCES "public"."classes"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_from_academic_year_id_academic_years_id_fk" FOREIGN KEY ("from_academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_to_academic_year_id_academic_years_id_fk" FOREIGN KEY ("to_academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_decision_by_id_users_id_fk" FOREIGN KEY ("decision_by_id") REFERENCES "public"."users"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "students" ADD CONSTRAINT "students_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "students" ADD CONSTRAINT "students_current_class_id_classes_id_fk" FOREIGN KEY ("current_class_id") REFERENCES "public"."classes"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "time_slots" ADD CONSTRAINT "time_slots_academic_year_id_academic_years_id_fk" FOREIGN KEY ("academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
CREATE INDEX "item_lookup_idx" ON "assessment_items" USING btree ("class_id","subject_id","month");--> statement-breakpoint
CREATE INDEX "attendance_student_date_idx" ON "attendance_records" USING btree ("student_id","date");--> statement-breakpoint
CREATE INDEX "attendance_lesson_date_idx" ON "attendance_records" USING btree ("scheduled_lesson_id","date");--> statement-breakpoint
CREATE INDEX "attendance_status_idx" ON "attendance_records" USING btree ("status");--> statement-breakpoint
CREATE INDEX "attendance_student_lesson_date_idx" ON "attendance_records" USING btree ("student_id","scheduled_lesson_id","date");--> statement-breakpoint
CREATE INDEX "class_name_idx" ON "classes" USING btree ("name");--> statement-breakpoint
CREATE INDEX "classes_parent_idx" ON "classes" USING btree ("parent_class_id");--> statement-breakpoint
CREATE INDEX "classes_sort_order_idx" ON "classes" USING btree ("sort_order");--> statement-breakpoint
CREATE INDEX "user_document_idx" ON "documents" USING btree ("user_id","created_at" DESC NULLS LAST);--> statement-breakpoint
CREATE INDEX "enrollments_class_idx" ON "enrollments" USING btree ("class_id");--> statement-breakpoint
CREATE INDEX "fast_grade_lookup_idx" ON "grade_entries" USING btree ("assessment_item_id","score");--> statement-breakpoint
CREATE INDEX "notif_recipient_created_idx" ON "notifications" USING btree ("recipient_id","created_at" DESC NULLS LAST);--> statement-breakpoint
CREATE INDEX "notif_recipient_unread_idx" ON "notifications" USING btree ("recipient_id") WHERE "notifications"."is_read" = false;--> statement-breakpoint
CREATE INDEX "notif_type_recipient_idx" ON "notifications" USING btree ("type","recipient_id");--> statement-breakpoint
CREATE INDEX "ps_parent_idx" ON "parent_students" USING btree ("parent_id");--> statement-breakpoint
CREATE INDEX "ps_student_idx" ON "parent_students" USING btree ("student_id");--> statement-breakpoint
CREATE INDEX "sched_teacher_day_slot_idx" ON "scheduled_lessons" USING btree ("teacher_id","day_of_week","time_slot_id","academic_year_id");--> statement-breakpoint
CREATE INDEX "sched_class_day_slot_idx" ON "scheduled_lessons" USING btree ("class_id","day_of_week","time_slot_id","academic_year_id");--> statement-breakpoint
CREATE INDEX "short_links_code_idx" ON "short_links" USING btree ("code");--> statement-breakpoint
CREATE INDEX "promo_student_year_idx" ON "student_promotions" USING btree ("student_id","from_academic_year_id");--> statement-breakpoint
CREATE INDEX "promo_from_class_idx" ON "student_promotions" USING btree ("from_class_id","from_academic_year_id");--> statement-breakpoint
CREATE INDEX "promo_status_idx" ON "student_promotions" USING btree ("status");--> statement-breakpoint
CREATE INDEX "student_num_idx" ON "students" USING btree ("student_number");--> statement-breakpoint
CREATE INDEX "secret_num_idx" ON "students" USING btree ("secret_number");--> statement-breakpoint
CREATE INDEX "time_slots_year_slot_idx" ON "time_slots" USING btree ("academic_year_id","slot_number");