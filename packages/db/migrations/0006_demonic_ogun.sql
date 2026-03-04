CREATE TYPE "public"."promotion_decision" AS ENUM('auto', 'manual', 'override');--> statement-breakpoint
CREATE TYPE "public"."promotion_status" AS ENUM('promoted', 'retained', 'transferred', 'graduated', 'conditional');--> statement-breakpoint
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
ALTER TABLE "classes" DROP CONSTRAINT "classes_academic_year_id_academic_years_id_fk";
--> statement-breakpoint
DROP INDEX "class_year_idx";--> statement-breakpoint
ALTER TABLE "classes" ADD COLUMN "sort_order" integer DEFAULT 0 NOT NULL;--> statement-breakpoint
ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_student_id_students_user_id_fk" FOREIGN KEY ("student_id") REFERENCES "public"."students"("user_id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_from_class_id_classes_id_fk" FOREIGN KEY ("from_class_id") REFERENCES "public"."classes"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_to_class_id_classes_id_fk" FOREIGN KEY ("to_class_id") REFERENCES "public"."classes"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_from_academic_year_id_academic_years_id_fk" FOREIGN KEY ("from_academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_to_academic_year_id_academic_years_id_fk" FOREIGN KEY ("to_academic_year_id") REFERENCES "public"."academic_years"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "student_promotions" ADD CONSTRAINT "student_promotions_decision_by_id_users_id_fk" FOREIGN KEY ("decision_by_id") REFERENCES "public"."users"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
CREATE INDEX "promo_student_year_idx" ON "student_promotions" USING btree ("student_id","from_academic_year_id");--> statement-breakpoint
CREATE INDEX "promo_from_class_idx" ON "student_promotions" USING btree ("from_class_id","from_academic_year_id");--> statement-breakpoint
CREATE INDEX "promo_status_idx" ON "student_promotions" USING btree ("status");--> statement-breakpoint
CREATE INDEX "class_name_idx" ON "classes" USING btree ("name");--> statement-breakpoint
CREATE INDEX "classes_sort_order_idx" ON "classes" USING btree ("sort_order");--> statement-breakpoint
ALTER TABLE "classes" DROP COLUMN "academic_year_id";