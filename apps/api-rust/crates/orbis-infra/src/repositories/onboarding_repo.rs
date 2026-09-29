//! Onboarding Repository — comprehensive persistence for institutional onboarding topology and configuration.

use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, TransactionTrait};
use orbis_domain::errors::DomainError;
use orbis_app::ports::onboarding::{OnboardingPort, ValidatedOnboardingClass, ValidatedOnboardingData};
use uuid::Uuid;
use chrono::Utc;
use async_trait::async_trait;
use tracing::info;

use crate::repositories::bootstrap_repo::seed_chart_of_accounts;

pub fn detect_grade_level(name: &str, name_ar: &str, level_str: &str) -> Option<u32> {
    if let Ok(n) = level_str.trim().parse::<u32>() {
        return Some(n);
    }
    let lower_name = name.to_lowercase();
    let lower_level = level_str.to_lowercase();
    let combined = format!("{lower_name} {name_ar} {lower_level}");

    if combined.contains("kg1") || combined.contains("روضة 1") || combined.contains("روضه 1") || combined.contains("تمهيدي 1") || combined.contains("kg 1") {
        return Some(0);
    }
    if combined.contains("kg2") || combined.contains("روضة 2") || combined.contains("روضه 2") || combined.contains("تمهيدي 2") || combined.contains("kg 2") || combined.contains("روضة") || combined.contains("روضه") || combined.contains("تمهيدي") {
        return Some(0);
    }

    if combined.contains("12") || combined.contains("ثالث ثانوي") || combined.contains("الثالث الثانوي") || combined.contains("grade 12") || combined.contains("g12") {
        return Some(12);
    }
    if combined.contains("11") || combined.contains("ثاني ثانوي") || combined.contains("الثاني الثانوي") || combined.contains("grade 11") || combined.contains("g11") {
        return Some(11);
    }
    if combined.contains("10") || combined.contains("أول ثانوي") || combined.contains("اول ثانوي") || combined.contains("الأول الثانوي") || combined.contains("grade 10") || combined.contains("g10") {
        return Some(10);
    }
    if combined.contains("9") || combined.contains("تاسع") || combined.contains("التاسع") || combined.contains("ثالث إعدادي") || combined.contains("ثالث متوسط") || combined.contains("grade 9") || combined.contains("g9") {
        return Some(9);
    }
    if combined.contains("8") || combined.contains("ثامن") || combined.contains("الثامن") || combined.contains("ثاني إعدادي") || combined.contains("ثاني متوسط") || combined.contains("grade 8") || combined.contains("g8") {
        return Some(8);
    }
    if combined.contains("7") || combined.contains("سابع") || combined.contains("السابع") || combined.contains("أول إعدادي") || combined.contains("اول اعدادي") || combined.contains("أول متوسط") || combined.contains("grade 7") || combined.contains("g7") {
        return Some(7);
    }
    if combined.contains("6") || combined.contains("سادس") || combined.contains("السادس") || combined.contains("grade 6") || combined.contains("g6") {
        return Some(6);
    }
    if combined.contains("5") || combined.contains("خامس") || combined.contains("الخامس") || combined.contains("grade 5") || combined.contains("g5") {
        return Some(5);
    }
    if combined.contains("4") || combined.contains("رابع") || combined.contains("الرابع") || combined.contains("grade 4") || combined.contains("g4") {
        return Some(4);
    }
    if combined.contains("3") || combined.contains("ثالث") || combined.contains("الثالث") || combined.contains("grade 3") || combined.contains("g3") {
        return Some(3);
    }
    if combined.contains("2") || combined.contains("ثاني") || combined.contains("الثاني") || combined.contains("grade 2") || combined.contains("g2") {
        return Some(2);
    }
    if combined.contains("1") || combined.contains("أول") || combined.contains("اول") || combined.contains("الأول") || combined.contains("grade 1") || combined.contains("g1") {
        return Some(1);
    }

    None
}

pub async fn seed_document_requirements<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    org_type: &str,
    classes: &[ValidatedOnboardingClass],
    now: &str,
) -> Result<(), DomainError> {
    // 1. Organization-wide Global Document Requirements
    let mut doc_entries: Vec<(Option<Uuid>, &str, &str, &str, &str, bool, bool, &str)> = Vec::new();

    match org_type {
        "university" => {
            doc_entries.push((None, "global", "DOC_UNI_HIGH_SCHOOL_CERT", "Original High School Certificate", "أصل شهادة الثانوية العامة / كشف الدرجات المعتمد", true, true, "all_students"));
            doc_entries.push((None, "global", "DOC_UNI_STUDENT_ID", "Student National ID Card / Passport", "أصل البطاقة الشخصية / جواز السفر للطالب", true, true, "all_students"));
            doc_entries.push((None, "global", "DOC_UNI_PHOTOS", "Personal Photos (6 Photos 4x6)", "عدد 6 صور شخصية حديثة بخلفية بيضاء", true, false, "all_students"));
            doc_entries.push((None, "global", "DOC_UNI_CONDUCT", "Good Conduct Certificate", "شهادة حسن السيرة والسلوك", true, false, "all_students"));
            doc_entries.push((None, "global", "DOC_UNI_MILITARY", "Military Service Form", "استمارة الخدمة العسكرية / وثيقة الموقف التجنيدي (للذكور)", true, false, "all_students"));
            doc_entries.push((None, "global", "DOC_UNI_TRANSFER", "University Transfer & Equivalency Form", "استمارة ونموذج المقاصة والتحويل الأكاديمي", true, true, "transfer_students_only"));
            doc_entries.push((None, "global", "DOC_UNI_INT_PASSPORT", "Valid Passport & Student Visa/Iqama", "جواز السفر وتأشيرة الإقامة الدراسية للوافدين", true, true, "non_national_only"));

            // Dynamic per-academic level transcripts for universities
            for c in classes {
                let name_ar = c.name_ar.as_deref().unwrap_or(&c.name);
                let level_num = detect_grade_level(&c.name, name_ar, &c.level).unwrap_or(1);
                if level_num >= 2 {
                    doc_entries.push((
                        Some(c.id),
                        "grade_level",
                        "DOC_UNI_TRANSCRIPT_Y1",
                        "Year 1 Official Academic Transcript",
                        "كشف درجات السنة الأولى الجامعية المعتمد",
                        true, true, "all_students"
                    ));
                }
                if level_num >= 3 {
                    doc_entries.push((
                        Some(c.id),
                        "grade_level",
                        "DOC_UNI_TRANSCRIPT_Y2",
                        "Year 2 Official Academic Transcript",
                        "كشف درجات السنة الثانية الجامعية المعتمد",
                        true, true, "all_students"
                    ));
                }
                if level_num >= 4 {
                    doc_entries.push((
                        Some(c.id),
                        "grade_level",
                        "DOC_UNI_TRANSCRIPT_Y3",
                        "Year 3 Official Academic Transcript",
                        "كشف درجات السنة الثالثة الجامعية المعتمد",
                        true, true, "all_students"
                    ));
                }
                if level_num >= 5 {
                    doc_entries.push((
                        Some(c.id),
                        "grade_level",
                        "DOC_UNI_TRANSCRIPT_Y4",
                        "Year 4 Official Academic Transcript",
                        "كشف درجات السنة الرابعة الجامعية المعتمد",
                        true, true, "all_students"
                    ));
                }
            }
        },
        "institute" | "training_center" => {
            doc_entries.push((None, "global", "DOC_INST_NATIONAL_ID", "National ID Card / Valid Passport", "البطاقة الشخصية / الهوية الوطنية / جواز السفر", true, true, "all_students"));
            doc_entries.push((None, "global", "DOC_INST_QUALIFICATION", "Last Academic Qualification Certificate", "أصل أو صورة مصدقة لآخر مؤهل دراسي / شهادة دراسية", true, false, "all_students"));
            doc_entries.push((None, "global", "DOC_INST_PHOTOS", "Personal Photos (4x6)", "عدد 4 صور شخصية حديثة", true, false, "all_students"));
            doc_entries.push((None, "global", "DOC_INST_GUARDIAN_CONSENT", "Guardian Approval & Consent Letter", "موافقة خطية من ولي الأمر (للمتدربين دون سن الرشد)", false, false, "all_students"));
            doc_entries.push((None, "global", "DOC_INST_NON_NAT", "Valid Residency / Iqama Copy", "صورة الإقامة السارية للوافدين والمقيمين", true, true, "non_national_only"));
        },
        _ => {
            // General school global requirements
            doc_entries.push((None, "global", "DOC_SCH_PHOTO", "Personal Photos (4x6)", "صور شخصية حديثة (4×6)", true, false, "all_students"));
            doc_entries.push((None, "global", "DOC_SCH_TRANSFER", "Official School Transfer Certificate", "استمارة التحويل المدرسي المعتمدة من الإدارة التعليمية", true, true, "transfer_students_only"));
            doc_entries.push((None, "global", "DOC_NON_NAT_PASSPORT", "Valid Passport & Iqama / Residence Permit", "جواز السفر والإقامة السارية للطالب وولي الأمر", true, true, "non_national_only"));

            // Exact cumulative grade certificates per class
            for c in classes {
                let name_ar = c.name_ar.as_deref().unwrap_or(&c.name);
                let grade = detect_grade_level(&c.name, name_ar, &c.level).unwrap_or(1);

                match grade {
                    0 => {
                        // Kindergarten (روضة / تمهيدي)
                        doc_entries.push((Some(c.id), "grade_level", "DOC_KG_BIRTH_CERT", "Child Official Birth Certificate", "شهادة ميلاد الطفل الرسمية المميكنة", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_KG_FATHER_ID", "Father / Guardian National ID Card", "بطاقة الرقم القومي / الهوية الوطنية لولي الأمر", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_KG_VACCINE", "Child Health Record & Vaccination Card", "كرت التطعيمات والسجل الصحي للطفل", true, false, "all_students"));
                    },
                    1 => {
                        // Grade 1 (الصف الأول الابتدائي)
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G1_BIRTH_CERT", "Original Birth Certificate", "أصل شهادة ميلاد الطفل المعتمدة", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G1_FATHER_ID", "Guardian National ID Copy", "صورة بطاقة الهوية الوطنية / الإقامة لولي الأمر", true, false, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G1_MEDICAL", "School Health Examination", "استمارة الفحص الطبي والتطعيمات", true, false, "all_students"));
                    },
                    2 => {
                        // Grade 2: Birth Cert + Grade 1
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G2_BIRTH_CERT", "Child Birth Certificate Copy", "صورة شهادة الميلاد للطفل", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G2_PREV_G1", "Grade 1 Pass Certificate / Report Card", "شهادة اجتياز الصف الأول الابتدائي / بيان الدرجات", true, true, "all_students"));
                    },
                    3 => {
                        // Grade 3: Birth Cert + Grade 1 + Grade 2
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G3_BIRTH_CERT", "Child Birth Certificate Copy", "صورة شهادة الميلاد للطفل", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G3_PREV_G1", "Grade 1 Pass Certificate", "شهادة اجتياز الصف الأول الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G3_PREV_G2", "Grade 2 Pass Certificate", "شهادة اجتياز الصف الثاني الابتدائي", true, true, "all_students"));
                    },
                    4 => {
                        // Grade 4: Birth Cert + Grade 1 + Grade 2 + Grade 3
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G4_BIRTH_CERT", "Child Birth Certificate Copy", "صورة شهادة الميلاد للطفل", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G4_PREV_G1", "Grade 1 Pass Certificate", "شهادة اجتياز الصف الأول الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G4_PREV_G2", "Grade 2 Pass Certificate", "شهادة اجتياز الصف الثاني الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G4_PREV_G3", "Grade 3 Pass Certificate", "شهادة اجتياز الصف الثالث الابتدائي", true, true, "all_students"));
                    },
                    5 => {
                        // Grade 5: Birth Cert + Grades 1-4
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G5_BIRTH_CERT", "Child Birth Certificate Copy", "صورة شهادة الميلاد للطفل", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G5_PREV_G1", "Grade 1 Pass Certificate", "شهادة اجتياز الصف الأول الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G5_PREV_G2", "Grade 2 Pass Certificate", "شهادة اجتياز الصف الثاني الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G5_PREV_G3", "Grade 3 Pass Certificate", "شهادة اجتياز الصف الثالث الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G5_PREV_G4", "Grade 4 Pass Certificate", "شهادة اجتياز الصف الرابع الابتدائي", true, true, "all_students"));
                    },
                    6 => {
                        // Grade 6: Birth Cert + Grades 1-5
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G6_BIRTH_CERT", "Child Birth Certificate Copy", "صورة شهادة الميلاد للطفل", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G6_PREV_G1", "Grade 1 Pass Certificate", "شهادة اجتياز الصف الأول الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G6_PREV_G2", "Grade 2 Pass Certificate", "شهادة اجتياز الصف الثاني الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G6_PREV_G3", "Grade 3 Pass Certificate", "شهادة اجتياز الصف الثالث الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G6_PREV_G4", "Grade 4 Pass Certificate", "شهادة اجتياز الصف الرابع الابتدائي", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G6_PREV_G5", "Grade 5 Pass Certificate", "شهادة اجتياز الصف الخامس الابتدائي", true, true, "all_students"));
                    },
                    7 => {
                        // Grade 7 / 1st Prep: Primary Completion Cert + Birth Cert
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G7_BIRTH_CERT", "Child Birth Certificate Copy", "صورة شهادة الميلاد للطفل", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G7_PRI_COMPLETION", "Primary Stage Completion Certificate", "أصل شهادة إتمام المرحلة الابتدائية (الصف السادس)", true, true, "all_students"));
                    },
                    8 => {
                        // Grade 8 / 2nd Prep: Primary Completion + Grade 7 Cert
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G8_PRI_COMPLETION", "Primary Stage Completion Certificate", "أصل شهادة إتمام المرحلة الابتدائية (الصف السادس)", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G8_PREV_G7", "Grade 7 Pass Certificate", "شهادة اجتياز الصف الأول الإعدادي (السابع)", true, true, "all_students"));
                    },
                    9 => {
                        // Grade 9 / 3rd Prep: Primary Completion + Grade 7 + Grade 8
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G9_PRI_COMPLETION", "Primary Stage Completion Certificate", "أصل شهادة إتمام المرحلة الابتدائية (الصف السادس)", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G9_PREV_G7", "Grade 7 Pass Certificate", "شهادة اجتياز الصف الأول الإعدادي (السابع)", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G9_PREV_G8", "Grade 8 Pass Certificate", "شهادة اجتياز الصف الثاني الإعدادي (الثامن)", true, true, "all_students"));
                    },
                    10 => {
                        // Grade 10 / 1st Sec: Prep Completion + Student ID / Birth Cert
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G10_ID_OR_BIRTH", "National ID Card or Birth Certificate", "أصل بطاقة الهوية الوطنية أو شهادة الميلاد", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G10_MID_COMPLETION", "Preparatory Stage Completion Certificate", "أصل شهادة إتمام مرحلة التعليم الأساسي / الإعدادية (الصف التاسع)", true, true, "all_students"));
                    },
                    11 => {
                        // Grade 11 / 2nd Sec: Prep Completion + Grade 10 Cert
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G11_MID_COMPLETION", "Preparatory Stage Completion Certificate", "أصل شهادة إتمام مرحلة التعليم الأساسي / الإعدادية (الصف التاسع)", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G11_PREV_G10", "Grade 10 Pass Certificate", "شهادة اجتياز الصف الأول الثانوي (العاشر)", true, true, "all_students"));
                    },
                    12 => {
                        // Grade 12 / 3rd Sec: Prep Completion + Grade 10 + Grade 11
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G12_MID_COMPLETION", "Preparatory Stage Completion Certificate", "أصل شهادة إتمام مرحلة التعليم الأساسي / الإعدادية (الصف التاسع)", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G12_PREV_G10", "Grade 10 Pass Certificate", "شهادة اجتياز الصف الأول الثانوي (العاشر)", true, true, "all_students"));
                        doc_entries.push((Some(c.id), "grade_level", "DOC_G12_PREV_G11", "Grade 11 Pass Certificate", "شهادة اجتياز الصف الثاني الثانوي (الحادي عشر)", true, true, "all_students"));
                    },
                    _ => {
                        // General grade fallback
                        doc_entries.push((Some(c.id), "grade_level", "DOC_GEN_PREV_CERT", "Previous Grade Pass Certificate", "شهادة اجتياز الصف السابق المعتمدة", true, true, "all_students"));
                    }
                }
            }

            // Fallback stage templates if classes list was empty during minimal seed
            if classes.is_empty() {
                doc_entries.push((None, "stage", "DOC_PRI_PREV_CERT", "Previous Grade Certificate", "شهادة الصف السابق / بيان الدرجات المعتمد", true, true, "all_students"));
                doc_entries.push((None, "stage", "DOC_MID_PRIMARY_CERT", "Primary Stage Completion Certificate", "شهادة إتمام المرحلة الابتدائية (الصف السادس)", true, true, "all_students"));
                doc_entries.push((None, "stage", "DOC_SEC_MIDDLE_CERT", "Preparatory / Middle School Certificate", "أصل شهادة إتمام مرحلة التعليم الأساسي / الإعدادية (الصف التاسع)", true, true, "all_students"));
            }
        }
    }

    for (target_id, target_type, base_code, name_en, name_ar, is_mandatory, blocks_enrollment, condition) in doc_entries {
        let did = Uuid::new_v4();
        let code = match target_id {
            Some(tid) => format!("{}_{}", base_code, &tid.to_string()[..8]),
            None => base_code.to_string(),
        };

        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO document_requirements (id, org_id, target_type, target_id, code, name_ar, name_en, is_mandatory, blocks_enrollment, condition, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12 \
             WHERE NOT EXISTS (SELECT 1 FROM document_requirements WHERE org_id = $2 AND code = $5)",
            vec![
                did.into(),
                org_id.into(),
                target_type.into(),
                target_id.into(),
                code.into(),
                name_ar.into(),
                name_en.into(),
                (if is_mandatory { 1i32 } else { 0i32 }).into(),
                (if blocks_enrollment { 1i32 } else { 0i32 }).into(),
                condition.into(),
                now.into(),
                now.into(),
            ]
        )).await.ok();
    }

    Ok(())
}

pub struct OnboardingRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl OnboardingPort for OnboardingRepoImpl {
    async fn persist_onboarding(
        &self,
        data: ValidatedOnboardingData,
    ) -> Result<(), DomainError> {
        let txn = self.db.begin().await
            .map_err(|e| DomainError::Internal(format!("Failed to begin onboarding transaction: {e}")))?;

        let now = Utc::now().to_rfc3339();
        let org_id = data.org_id;

        // Lookup admin user id for audit metadata
        let admin_user_row = txn.query_one(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "SELECT id FROM users WHERE org_id = $1 LIMIT 1",
            vec![org_id.into()]
        )).await.ok().flatten();
        let admin_user_id = admin_user_row.and_then(|r| r.try_get::<Uuid>("", "id").ok()).unwrap_or_else(Uuid::new_v4);

        // 1. Update Organization Profile & Settings
        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "UPDATE organizations SET base_currency_code = $1, timezone = $2, locale = $3, settings = $4, updated_by = $5, version = version + 1, updated_at = $6 WHERE id = $7",
            vec![
                data.base_currency_code.clone().into(),
                data.timezone.into(),
                data.locale.into(),
                data.settings.to_string().into(),
                admin_user_id.into(),
                now.clone().into(),
                org_id.into(),
            ]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to update organization settings: {e}")))?;

        // 2. OrgUnits Hierarchy (Campus + Buildings)
        let campus_id = Uuid::new_v4();
        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "INSERT INTO org_units (id, org_id, parent_id, path, depth, name, name_ar, code, unit_type, data_isolation, children_count, descendant_count, sort_order, is_active, created_by, updated_by, created_at, updated_at) \
             SELECT $1, $2, NULL, 'root', 0, 'Main Campus', 'المقر الرئيسي', 'HQ', 'campus', 'none', 0, 0, 1, 1, $3, $3, $4, $5 \
             WHERE NOT EXISTS (SELECT 1 FROM org_units WHERE org_id = $2 AND depth = 0)",
            vec![campus_id.into(), org_id.into(), admin_user_id.into(), now.clone().into(), now.clone().into()]
        )).await.map_err(|e| DomainError::Internal(format!("Failed to seed root campus: {e}")))?;

        let root_unit_row = txn.query_one(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "SELECT id FROM org_units WHERE org_id = $1 AND depth = 0 LIMIT 1",
            vec![org_id.into()]
        )).await.map_err(|e| DomainError::Internal(e.to_string()))?.ok_or_else(|| DomainError::NotFound("Root campus unit not found".into()))?;
        let real_campus_id: Uuid = root_unit_row.try_get("", "id").unwrap_or(campus_id);

        for b in &data.buildings {
            let parent_id = b.parent_unit_id.unwrap_or(real_campus_id);
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO org_units (id, org_id, parent_id, path, depth, name, name_ar, code, unit_type, data_isolation, children_count, descendant_count, sort_order, is_active, created_by, updated_by, created_at, updated_at) \
                 SELECT $1, $2, $3, 'root.' || $4, 1, $5, $6, $4, 'building', 'none', 0, 0, 1, 1, $7, $7, $8, $9 \
                 WHERE NOT EXISTS (SELECT 1 FROM org_units WHERE org_id = $2 AND code = $4)",
                vec![
                    b.id.into(), org_id.into(), parent_id.into(),
                    b.code.clone().into(),
                    b.name.clone().into(),
                    b.name_ar.clone().unwrap_or_else(|| b.name.clone()).into(),
                    admin_user_id.into(),
                    now.clone().into(), now.clone().into(),
                ]
            )).await.map_err(|e| DomainError::Internal(format!("Failed to create building unit: {e}")))?;
        }

        // 3. Rooms
        for rm in &data.rooms {
            let room_unit_id = rm.building_unit_id.unwrap_or(real_campus_id);
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO rooms (id, org_id, name, name_ar, room_type, capacity, floor, org_unit_id, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10 \
                 WHERE NOT EXISTS (SELECT 1 FROM rooms WHERE org_id = $2 AND name = $3)",
                vec![
                    rm.id.into(), org_id.into(), rm.name.clone().into(),
                    rm.name_ar.clone().into(),
                    rm.room_type.clone().into(), rm.capacity.into(),
                    rm.floor.into(),
                    room_unit_id.into(),
                    now.clone().into(), now.clone().into(),
                ]
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert room: {e}")))?;
        }

        // 4. Academic Year
        let normalize_start = |s: &str| -> String {
            if s.len() == 10 && !s.contains('T') { format!("{}T00:00:00Z", s) } else { s.to_string() }
        };
        let normalize_end = |s: &str| -> String {
            if s.len() == 10 && !s.contains('T') { format!("{}T23:59:59Z", s) } else { s.to_string() }
        };

        let year_start = normalize_start(&data.academic_year.start_date);
        let year_end = normalize_end(&data.academic_year.end_date);

        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "INSERT INTO academic_years (id, org_id, name, name_ar, start_date, end_date, status, is_locked, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $6, 'active', 0, $7, $8 \
             WHERE NOT EXISTS (SELECT 1 FROM academic_years WHERE org_id = $2 AND name = $3)",
            vec![
                data.academic_year.id.into(),
                org_id.into(),
                data.academic_year.name.clone().into(),
                data.academic_year.name_ar.clone().unwrap_or_else(|| data.academic_year.name.clone()).into(),
                year_start.into(),
                year_end.into(),
                now.clone().into(),
                now.clone().into(),
            ],
        )).await.map_err(|e| DomainError::Internal(format!("Failed to insert academic year: {e}")))?;

        // 5. Academic Periods
        for p in &data.periods {
            let p_start = normalize_start(&p.start_date);
            let p_end = normalize_end(&p.end_date);
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO academic_periods (id, org_id, year_id, parent_id, name, name_ar, period_type, start_date, end_date, status, is_locked, is_enrollment_open, is_grade_entry_open, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, 'active', 0, 1, 1, $10, $11 \
                 WHERE NOT EXISTS (SELECT 1 FROM academic_periods WHERE org_id = $2 AND year_id = $3 AND name = $5)",
                vec![
                    p.id.into(),
                    org_id.into(),
                    p.year_id.into(),
                    p.parent_id.into(),
                    p.name.clone().into(),
                    p.name_ar.clone().unwrap_or_else(|| p.name.clone()).into(),
                    p.period_type.clone().into(),
                    p_start.into(),
                    p_end.into(),
                    now.clone().into(),
                    now.clone().into(),
                ],
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert period: {e}")))?;
        }

        let first_period_id = data.periods.first().map(|p| p.id).unwrap_or(data.academic_year.id);

        // 6. Classes
        for c in &data.classes {
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO classes (id, org_id, unit_id, period_id, name, name_ar, level, division, is_active, is_deleted, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, $8, 1, 0, $9, $10 \
                 WHERE NOT EXISTS (SELECT 1 FROM classes WHERE org_id = $2 AND period_id = $4 AND name = $5)",
                vec![
                    c.id.into(),
                    org_id.into(),
                    real_campus_id.into(),
                    first_period_id.into(),
                    c.name.clone().into(),
                    c.name_ar.clone().unwrap_or_else(|| c.name.clone()).into(),
                    c.level.clone().into(),
                    c.division.clone().unwrap_or_default().into(),
                    now.clone().into(),
                    now.clone().into(),
                ],
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert class: {e}")))?;
        }

        // 7. Subjects
        for s in &data.subjects {
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO subjects (id, org_id, code, name, name_ar, is_core, is_elective, is_lab_required, is_active, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, 0, 0, 1, $7, $8 \
                 WHERE NOT EXISTS (SELECT 1 FROM subjects WHERE org_id = $2 AND code = $3)",
                vec![
                    s.id.into(),
                    org_id.into(),
                    s.code.clone().into(),
                    s.name.clone().into(),
                    s.name_ar.clone().unwrap_or_else(|| s.name.clone()).into(),
                    (if s.is_core { 1i32 } else { 0i32 }).into(),
                    now.clone().into(),
                    now.clone().into(),
                ],
            )).await.map_err(|e| DomainError::Internal(format!("Failed to insert subject: {e}")))?;
        }

        // 8. Time Slots
        for (slot_idx, ts) in data.time_slots.iter().enumerate() {
            let start_time_str = if ts.start_time.len() == 5 { format!("{}:00", ts.start_time) } else { ts.start_time.clone() };
            let end_time_str = if ts.end_time.len() == 5 { format!("{}:00", ts.end_time) } else { ts.end_time.clone() };

            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO time_slots (id, period_id, day_of_week, slot_index, name, name_ar, start_time, end_time, is_break, break_type, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12 \
                 WHERE NOT EXISTS (SELECT 1 FROM time_slots WHERE period_id = $2 AND start_time = $7 AND end_time = $8 AND day_of_week = $3)",
                vec![
                    ts.id.into(),
                    first_period_id.into(),
                    (ts.day_of_week as i16).into(),
                    (slot_idx as i32 + 1).into(),
                    ts.name.clone().into(),
                    ts.name_ar.clone().into(),
                    start_time_str.into(),
                    end_time_str.into(),
                    (if ts.is_break { 1i32 } else { 0i32 }).into(),
                    ts.break_type.clone().into(),
                    now.clone().into(), now.clone().into(),
                ]
            )).await.ok();
        }

        // 9. Chart of Accounts (COA) initialization if requested
        if data.initialize_chart_of_accounts {
            if let Err(e) = seed_chart_of_accounts(&txn, org_id, admin_user_id, &data.base_currency_code, &now).await {
                info!("COA onboarding initialization note: {}", e);
            }
        }

        // 10. Fee Templates (Only persist if explicitly provided by user in onboarding)
        if !data.fee_templates.is_empty() {
            let rec_row = txn.query_one(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "SELECT id FROM accounts WHERE org_id = $1 AND (code = '110301' OR code = '110401' OR account_type = 'asset') ORDER BY CASE WHEN code = '110301' THEN 0 WHEN code = '110401' THEN 1 ELSE 2 END LIMIT 1",
                vec![org_id.into()]
            )).await.ok().flatten();
            let rec_id: Uuid = match rec_row {
                Some(r) => r.try_get("", "id").unwrap_or_else(|_| Uuid::new_v4()),
                None => Uuid::new_v4(),
            };
            let rev_row = txn.query_one(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "SELECT id FROM accounts WHERE org_id = $1 AND (code = '410301' OR code = '410101' OR account_type = 'revenue') ORDER BY CASE WHEN code IN ('410301', '410101') THEN 0 ELSE 1 END LIMIT 1",
                vec![org_id.into()]
            )).await.ok().flatten();
            let rev_id: Uuid = match rev_row {
                Some(r) => r.try_get("", "id").unwrap_or(rec_id),
                None => rec_id,
            };

            for f in &data.fee_templates {
                let applicable_levels_json = f.applicable_levels.as_ref().map(|l| serde_json::to_string(l).unwrap_or_default());

                txn.execute(Statement::from_sql_and_values(
                    txn.get_database_backend(),
                    "INSERT INTO fee_templates (id, org_id, code, name, name_ar, category, default_amount, currency_code, revenue_account_id, receivable_account_id, applicable_levels, frequency, is_mandatory, is_active, created_at, updated_at) \
                     SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, 1, $14, $15 \
                     WHERE NOT EXISTS (SELECT 1 FROM fee_templates WHERE org_id = $2 AND code = $3)",
                    vec![
                        f.id.into(), org_id.into(), f.code.clone().into(),
                        f.name.clone().into(), f.name_ar.clone().into(),
                        f.category.clone().into(), f.default_amount.into(),
                        f.currency_code.clone().into(),
                        rev_id.into(), rec_id.into(),
                        applicable_levels_json.into(),
                        f.frequency.clone().into(),
                        (if f.is_mandatory { 1i32 } else { 0i32 }).into(),
                        now.clone().into(), now.clone().into(),
                    ]
                )).await.ok();
            }
        }

        // 11. Custom Discount Templates (from Onboarding)
        for d in &data.discount_templates {
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "INSERT INTO discount_templates (id, org_id, code, name, name_ar, discount_type, value, is_approval_required, is_auto_apply, is_stackable, priority, is_active, created_at, updated_at) \
                 SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 1, $12, $13 \
                 WHERE NOT EXISTS (SELECT 1 FROM discount_templates WHERE org_id = $2 AND code = $3)",
                vec![
                    d.id.into(), org_id.into(), d.code.clone().into(),
                    d.name.clone().into(), d.name_ar.clone().into(),
                    d.discount_type.clone().into(), d.value.into(),
                    (if d.is_approval_required { 1i32 } else { 0i32 }).into(),
                    (if d.is_auto_apply { 1i32 } else { 0i32 }).into(),
                    (if d.is_stackable { 1i32 } else { 0i32 }).into(),
                    d.priority.into(),
                    now.clone().into(), now.clone().into(),
                ]
            )).await.ok();
        }

        // 12. Document Requirements Seeding (Per Profile, Stage, & Grade)
        if let Err(e) = seed_document_requirements(&txn, org_id, &data.org_type, &data.classes, &now).await {
            info!("Document requirements onboarding seeding note: {}", e);
        }

        // 15. Record Onboarding Audit Log
        let ob_log_id = Uuid::new_v4();
        txn.execute(Statement::from_sql_and_values(
            txn.get_database_backend(),
            "INSERT INTO report_audit_log (id, org_id, actor_id, action, entity_type, entity_id, details, created_at) \
             VALUES ($1, $2, $3, 'onboarding.completed', 'Organization', $2, '{\"status\": \"operational\"}', $4)",
            vec![ob_log_id.into(), org_id.into(), admin_user_id.into(), now.clone().into()]
        )).await.ok();

        txn.commit().await
            .map_err(|e| DomainError::Internal(format!("Failed to commit onboarding transaction: {e}")))?;

        Ok(())
    }
}
