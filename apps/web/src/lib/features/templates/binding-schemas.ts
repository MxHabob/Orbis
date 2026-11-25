import * as m from "$lib/paraglide/messages";

export function templateBindingSchemas() {
  const organization = {
    type: "object",
    label: m.template_schema_organization_group(),
    properties: {
      country_name: {
        type: "string",
        label: m.template_schema_country_name(),
      },
      ministry_name: {
        type: "string",
        label: m.template_schema_ministry_name(),
      },
      region_name: {
        type: "string",
        label: m.template_schema_region_name(),
      },
      phone: {
        type: "string",
        label: m.settings_general_phone(),
      },
      website: {
        type: "string",
        label: m.settings_general_website(),
      },
      name_ar: {
        type: "string",
        label: m.template_schema_organization_name_ar(),
      },
      name: {
        type: "string",
        label: m.template_schema_organization_name_en(),
      },
      logo_url: {
        type: "string",
        label: m.template_schema_organization_logo(),
      },
      emblem_url: {
        type: "string",
        label: m.template_schema_ministry_emblem(),
      },
    },
  };
  return {
    student: {
      type: "object",
      label: m.template_schema_student_group(),
      properties: {
        name: {
          type: "string",
          label: m.template_schema_student_full_name(),
        },
        student_number: {
          type: "string",
          label: m.template_schema_student_number(),
        },
        studentNumber: {
          type: "string",
          label: m.template_schema_student_number(),
        },
        national_id: {
          type: "string",
          label: m.template_schema_student_national_id(),
        },
        nationalId: {
          type: "string",
          label: m.template_schema_student_national_id(),
        },
        seat_number: {
          type: "string",
          label: m.template_schema_student_seat_number(),
        },
        class_name: {
          type: "string",
          label: m.template_schema_student_class_section(),
        },
        className: {
          type: "string",
          label: m.template_schema_student_class_section(),
        },
        classNameAr: {
          type: "string",
          label: m.template_schema_student_class_section(),
        },
        class_name_ar: {
          type: "string",
          label: m.template_schema_student_class_section(),
        },
        grade_level: {
          type: "string",
          label: m.template_schema_student_grade_level(),
        },
        academic_calendar: {
          type: "string",
          label: m.template_schema_academic_year(),
        },
        birth_date: { type: "string", label: m.template_schema_birth_date() },
        birth_place: {
          type: "string",
          label: m.template_schema_birth_place(),
        },
        nationality: {
          type: "string",
          label: m.template_schema_nationality(),
        },
        guardian_name: {
          type: "string",
          label: m.template_schema_guardian_name(),
        },
        avatar_url: {
          type: "image",
          label: m.ui_copy_student_profile_photo_de522661(),
        },
        avatarUrl: {
          type: "image",
          label: m.ui_copy_student_profile_photo_de522661(),
        },
      },
    },
    academic: {
      type: "object",
      label: m.template_schema_academic_group(),
      properties: {
        subjects: {
          type: "array",
          label: m.template_schema_subjects_grades_table(),
          items: { type: "object" },
        },
        annual_total: {
          type: "number",
          label: m.template_schema_annual_total(),
        },
        max_score: {
          type: "number",
          label: m.template_schema_maximum_score(),
        },
        min_score: {
          type: "number",
          label: m.template_schema_minimum_score(),
        },
        percentage: {
          type: "string",
          label: m.template_schema_cumulative_percentage(),
        },
        appreciation: {
          type: "string",
          label: m.template_schema_overall_grade(),
        },
        status: { type: "string", label: m.template_schema_final_result() },
        rank_in_class: {
          type: "string",
          label: m.template_schema_class_rank(),
        },
      },
    },
    summary: {
      type: "object",
      label: m.template_schema_summary_group(),
      properties: {
        total_score: {
          type: "number",
          label: m.template_schema_total_score(),
        },
        percentage: {
          type: "string",
          label: m.template_schema_overall_percentage(),
        },
        status: {
          type: "string",
          label: m.template_schema_academic_status_decision(),
        },
        appreciation: {
          type: "string",
          label: m.template_schema_cumulative_grade(),
        },
        rank_in_class: {
          type: "string",
          label: m.template_schema_academic_rank(),
        },
      },
    },
    report: {
      type: "object",
      label: m.template_schema_report_group(),
      properties: {
        period_name: {
          type: "string",
          label: m.template_schema_report_period(),
        },
        academic_calendar: {
          type: "string",
          label: m.template_schema_academic_year(),
        },
        issued_date: {
          type: "string",
          label: m.template_schema_issued_date(),
        },
        principal_name: {
          type: "string",
          label: m.template_schema_principal_name(),
        },
        class_teacher: {
          type: "string",
          label: m.template_schema_class_teacher(),
        },
        verification_url: {
          type: "string",
          label: m.template_schema_verification_qr(),
        },
      },
    },
    exam: {
      type: "object",
      label: m.template_schema_exam_group(),
      properties: {
        session_name: {
          type: "string",
          label: m.template_schema_exam_session_name(),
        },
        hall_name: { type: "string", label: m.template_schema_exam_hall() },
        seating_number: {
          type: "string",
          label: m.template_schema_official_seat_number(),
        },
        desk_number: {
          type: "string",
          label: m.template_schema_desk_number(),
        },
        subject_name: {
          type: "string",
          label: m.template_schema_subject_name(),
        },
        exam_date: {
          type: "string",
          label: m.template_schema_exam_date_time(),
        },
        total_count: {
          type: "string",
          label: m.template_schema_committee_student_count(),
        },
      },
    },
    employee: {
      type: "object",
      label: m.template_schema_employee_group(),
      properties: {
        name: { type: "string", label: m.template_schema_employee_name() },
        job_title: { type: "string", label: m.template_schema_job_title() },
        employee_number: {
          type: "string",
          label: m.template_schema_employee_number(),
        },
        department: { type: "string", label: m.template_schema_department() },
      },
    },
    receipt: {
      type: "object",
      label: m.template_schema_receipt_group(),
      properties: {
        number: { type: "string", label: m.template_schema_receipt_number() },
        date: { type: "string", label: m.template_schema_receipt_date() },
        payer_name: {
          type: "string",
          label: m.template_schema_payer_recipient(),
        },
        student_name: {
          type: "string",
          label: m.template_schema_student_name(),
        },
        amount: { type: "number", label: m.template_schema_amount_numeric() },
        amount_words: {
          type: "string",
          label: m.template_schema_amount_words(),
        },
        description: {
          type: "string",
          label: m.template_schema_description(),
        },
        payment_method: {
          type: "string",
          label: m.template_schema_payment_method(),
        },
        cashier_name: {
          type: "string",
          label: m.template_schema_cashier_name(),
        },
      },
    },
    org: organization,
    $org: organization,
  };
}
