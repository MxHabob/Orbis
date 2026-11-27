use base64::{Engine as _, engine::general_purpose::STANDARD};
use orbis_domain::errors::DomainError;
use sea_orm::{ConnectionTrait, Statement};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct SyncDefaultsSummary {
    pub templates_inserted: u64,
    pub templates_updated: u64,
    pub templates_skipped_customized: u64,
}

/// Safely synchronizes stock templates for an organization:
/// - Inserts any missing stock templates.
/// - Updates only untouched stock templates (`is_system = TRUE`, `version = 1`,
///   `published_version = 1`, `updated_at = created_at`, `deleted_at IS NULL`).
/// - Never touches user-created templates or user-edited stock templates.
pub async fn sync_master_templates_for_org<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    admin_id: Uuid,
    now: &str,
) -> Result<SyncDefaultsSummary, DomainError> {
    let templates = get_master_template_definitions();
    let mut summary = SyncDefaultsSummary::default();

    for (code, name_en, name_ar, entity, r_type, page_sz, page_or, doc_json) in &templates {
        let doc_str = doc_json.to_string();
        let margins = doc_json["meta"]["pageSetup"]["margins"].to_string();

        let existing = db
            .query_one(Statement::from_sql_and_values(
                db.get_database_backend(),
                "SELECT id, is_system, status, version, published_version, (updated_at = created_at) AS is_untouched, (deleted_at IS NOT NULL) AS is_deleted \
                 FROM report_templates WHERE org_id = $1 AND code = $2 LIMIT 1",
                vec![org_id.into(), (*code).into()],
            ))
            .await
            .map_err(|error| DomainError::Internal(format!("Failed to check template {code}: {error}")))?;

        match existing {
            None => {
                let tid = Uuid::new_v4();
                db.execute(Statement::from_sql_and_values(
                    db.get_database_backend(),
                    "INSERT INTO report_templates (id, org_id, code, name, name_ar, description, description_ar, document_type, entity_type, master_template_id, status, version, published_version, published_at, published_by, page_size, orientation, margins_json, document_json, locale, tags, is_system, is_default, created_by, deleted_at, created_at, updated_at) \
                     SELECT $1, $2, $3, $4, $5, $4, $5, $6, $7, NULL, 'published', 1, 1, $8, $9, $10, $11, $12, $13, 'ar', '[\"templates\"]', TRUE, \
                     NOT EXISTS (SELECT 1 FROM report_templates WHERE org_id = $2 AND document_type = $6 AND entity_type = $7 AND is_default = TRUE AND deleted_at IS NULL), \
                     $9, NULL, $8, $8",
                    vec![
                        tid.into(), org_id.into(), (*code).into(),
                        (*name_en).into(), (*name_ar).into(),
                        (*r_type).into(), (*entity).into(),
                        now.to_string().into(), admin_id.into(),
                        (*page_sz).into(), (*page_or).into(),
                        margins.into(), doc_str.into(),
                    ],
                ))
                .await
                .map_err(|error| DomainError::Internal(format!("Failed to insert template {code}: {error}")))?;
                summary.templates_inserted += 1;
            }
            Some(row) => {
                let is_system: bool = row.try_get("", "is_system").unwrap_or(false);
                let status: String = row.try_get("", "status").unwrap_or_default();
                let version: i32 = row.try_get("", "version").unwrap_or(0);
                let published_version: Option<i32> =
                    row.try_get("", "published_version").ok().flatten();
                let is_untouched: bool = row.try_get("", "is_untouched").unwrap_or(false);
                let is_deleted: bool = row.try_get("", "is_deleted").unwrap_or(false);

                if is_system
                    && !is_deleted
                    && status == "published"
                    && version == 1
                    && published_version == Some(1)
                    && is_untouched
                {
                    db.execute(Statement::from_sql_and_values(
                        db.get_database_backend(),
                        "UPDATE report_templates \
                         SET name = $3, name_ar = $4, page_size = $5, orientation = $6, margins_json = $7, document_json = $8 \
                         WHERE org_id = $1 AND code = $2 AND is_system = TRUE AND status = 'published' AND version = 1 AND published_version = 1 AND updated_at = created_at AND deleted_at IS NULL",
                        vec![
                            org_id.into(), (*code).into(),
                            (*name_en).into(), (*name_ar).into(),
                            (*page_sz).into(), (*page_or).into(),
                            margins.into(), doc_str.into(),
                        ],
                    ))
                    .await
                    .map_err(|error| DomainError::Internal(format!("Failed to update template {code}: {error}")))?;
                    summary.templates_updated += 1;
                } else {
                    summary.templates_skipped_customized += 1;
                }
            }
        }
    }

    Ok(summary)
}

pub async fn provision_master_templates<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    admin_id: Uuid,
    now: &str,
) -> Result<(), DomainError> {
    info!(
        "Provisioning master visual AST document templates for org_id={}",
        org_id
    );

    let templates = get_master_template_definitions();

    for (code, name_en, name_ar, entity, r_type, page_sz, page_or, doc_json) in &templates {
        let tid = Uuid::new_v4();
        let doc_str = doc_json.to_string();
        let margins = doc_json["meta"]["pageSetup"]["margins"].to_string();

        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO report_templates (id, org_id, code, name, name_ar, description, description_ar, document_type, entity_type, master_template_id, status, version, published_version, published_at, published_by, page_size, orientation, margins_json, document_json, locale, tags, is_system, is_default, created_by, deleted_at, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $4, $5, $6, $7, NULL, 'published', 1, 1, $8, $9, $10, $11, $12, $13, 'ar', '[\"templates\"]', TRUE, \
             NOT EXISTS (SELECT 1 FROM report_templates WHERE org_id = $2 AND document_type = $6 AND entity_type = $7 AND is_default = TRUE AND deleted_at IS NULL), \
             $9, NULL, $8, $8 \
             WHERE NOT EXISTS (SELECT 1 FROM report_templates WHERE org_id = $2 AND code = $3)",
            vec![
                tid.into(), org_id.into(), (*code).into(),
                (*name_en).into(), (*name_ar).into(),
                (*r_type).into(), (*entity).into(),
                now.to_string().into(), admin_id.into(),
                (*page_sz).into(), (*page_or).into(),
                margins.into(), doc_str.into(),
            ]
        )).await.map_err(|error| {
            DomainError::Internal(format!("Failed to provision template {code}: {error}"))
        })?;
    }

    Ok(())
}

pub fn get_master_template_definitions() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    serde_json::Value,
)> {
    let mut templates = vec![
        // ── 1. Smart Student ID Card (CR80 / ID_CARD Landscape 85.6mm x 54mm) ─
        (
            "administration.student-id-card",
            "Smart Student ID Card",
            "بطاقة الهوية الطلابية الذكية (مع باركود / QR)",
            "student",
            "ID_CARD",
            "ID_CARD",
            "landscape",
            json!({
                "version": "2.0",
                "meta": {
                    "name": "Smart Student ID Card",
                    "nameAr": "بطاقة الهوية الطلابية الذكية (مع باركود / QR)",
                    "documentType": "ID_CARD",
                    "entityType": "student",
                    "locale": "ar",
                    "pageSetup": { "size": "ID_CARD", "orientation": "landscape", "margins": { "top": 0, "bottom": 0, "left": 0, "right": 0 }, "unit": "mm" }
                },
                "variables": {
                    "student.name": { "source": "student.name", "type": "string", "label": "Student Name", "labelAr": "اسم الطالب" },
                    "student.studentNumber": { "source": "student.student_number", "type": "string", "label": "Student Number", "labelAr": "الرقم الأكاديمي" },
                    "student.className": { "source": "student.class_name", "type": "string", "label": "Class", "labelAr": "الصف والشعبة" },
                    "student.nationalId": { "source": "student.national_id", "type": "string", "label": "National ID", "labelAr": "رقم الهوية / الإقامة" }
                },
                "assets": {},
                "pages": [{
                    "type": "Page",
                    "id": "p1",
                    "children": [
                        {
                            "type": "Shape",
                            "id": "top-accent",
                            "props": { "shapeType": "rectangle", "fillColor": "#1e293b", "strokeColor": "#1e293b", "strokeWidth": 0, "borderRadius": 0 },
                            "position": { "x": 0, "y": 0, "width": 85.6, "height": 1.4 }
                        },
                        {
                            "type": "Image",
                            "id": "org-logo",
                            "props": { "src": "{{$org.logo_url}}", "fit": "contain" },
                            "position": { "x": 73, "y": 3.5, "width": 8, "height": 8 }
                        },
                        {
                            "type": "Text",
                            "id": "org-name-ar",
                            "props": { "content": "{{$org.name_ar}}", "fontSize": 8.2, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 14, "y": 3.8, "width": 57, "height": 4.5 }
                        },
                        {
                            "type": "Text",
                            "id": "card-title-sub",
                            "props": { "content": "بطاقة هوية طالب | STUDENT ID CARD", "fontSize": 5.2, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 14, "y": 8.5, "width": 57, "height": 3.2 }
                        },
                        {
                            "type": "Separator",
                            "id": "header-sep",
                            "props": { "direction": "horizontal", "color": "#e2e8f0", "thickness": 0.25 },
                            "position": { "x": 4.5, "y": 13, "width": 76.6, "height": 0.3 }
                        },
                        {
                            "type": "Image",
                            "id": "student-photo",
                            "props": { "src": "{{student.avatarUrl}}", "fit": "cover" },
                            "position": { "x": 5, "y": 16, "width": 21, "height": 25 }
                        },
                        {
                            "type": "Shape",
                            "id": "photo-border",
                            "props": { "shapeType": "rectangle", "fillColor": "transparent", "strokeColor": "#e2e8f0", "strokeWidth": 0.3, "borderRadius": 1.5 },
                            "position": { "x": 5, "y": 16, "width": 21, "height": 25 }
                        },
                        {
                            "type": "Text",
                            "id": "student-name",
                            "props": { "content": "{{student.name}}", "fontSize": 9.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 29, "y": 16, "width": 52, "height": 5 }
                        },
                        {
                            "type": "Text",
                            "id": "student-num-lbl",
                            "props": { "content": "الرقم الأكاديمي: {{student.studentNumber}}", "fontSize": 6.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 29, "y": 22.5, "width": 40, "height": 4 }
                        },
                        {
                            "type": "Text",
                            "id": "student-class-lbl",
                            "props": { "content": "الصف / الشعبة: {{student.className}}", "fontSize": 6.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 29, "y": 27.5, "width": 40, "height": 4 }
                        },
                        {
                            "type": "Text",
                            "id": "student-nat-lbl",
                            "props": { "content": "رقم الهوية: {{student.nationalId}}", "fontSize": 6.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#475569", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 29, "y": 32.5, "width": 40, "height": 4 }
                        },
                        {
                            "type": "QRCode",
                            "id": "student-qr",
                            "props": { "data": "{{student.studentNumber}}", "size": 11, "errorCorrection": "M" },
                            "position": { "x": 70, "y": 28, "width": 11, "height": 11 }
                        },
                        {
                            "type": "Separator",
                            "id": "footer-sep",
                            "props": { "direction": "horizontal", "color": "#f1f5f9", "thickness": 0.25 },
                            "position": { "x": 4.5, "y": 45, "width": 76.6, "height": 0.3 }
                        },
                        {
                            "type": "Text",
                            "id": "footer-text",
                            "props": { "content": "صالحة للعام الدراسي الحالي  |  VALID FOR CURRENT ACADEMIC YEAR", "fontSize": 4.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 5, "y": 47, "width": 75.6, "height": 3 }
                        }
                    ]
                }]
            }),
        ),
        // ── 2. Staff & Faculty Work Badge (CR80 / ID_CARD Landscape) ─────────
        (
            "administration.staff-id-card",
            "Employee Work ID Badge",
            "بطاقة العمل والتعريف للكادر الوظيفي",
            "employee",
            "ID_CARD",
            "ID_CARD",
            "landscape",
            json!({
                "version": "2.0",
                "meta": {
                    "name": "Employee Work ID Badge",
                    "nameAr": "بطاقة العمل والتعريف للكادر الوظيفي",
                    "documentType": "ID_CARD",
                    "entityType": "employee",
                    "locale": "ar",
                    "pageSetup": { "size": "ID_CARD", "orientation": "landscape", "margins": { "top": 0, "bottom": 0, "left": 0, "right": 0 }, "unit": "mm" }
                },
                "variables": {
                    "employee.name": { "source": "employee.name", "type": "string", "label": "Employee Name", "labelAr": "اسم الموظف" },
                    "employee.jobTitle": { "source": "employee.job_title", "type": "string", "label": "Job Title", "labelAr": "المسمى الوظيفي" },
                    "employee.employeeNumber": { "source": "employee.employee_number", "type": "string", "label": "Employee Number", "labelAr": "الرقم الوظيفي" },
                    "employee.department": { "source": "employee.department", "type": "string", "label": "Department", "labelAr": "القسم / الإدارة" }
                },
                "assets": {},
                "pages": [{
                    "type": "Page",
                    "id": "p1",
                    "children": [
                        {
                            "type": "Shape",
                            "id": "top-accent",
                            "props": { "shapeType": "rectangle", "fillColor": "#312e81", "strokeColor": "#312e81", "strokeWidth": 0, "borderRadius": 0 },
                            "position": { "x": 0, "y": 0, "width": 85.6, "height": 1.4 }
                        },
                        {
                            "type": "Image",
                            "id": "org-logo",
                            "props": { "src": "{{$org.logo_url}}", "fit": "contain" },
                            "position": { "x": 73, "y": 3.5, "width": 8, "height": 8 }
                        },
                        {
                            "type": "Text",
                            "id": "org-title",
                            "props": { "content": "{{$org.name_ar}}", "fontSize": 8.2, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 14, "y": 3.8, "width": 57, "height": 4.5 }
                        },
                        {
                            "type": "Text",
                            "id": "badge-subtitle",
                            "props": { "content": "بطاقة تعريف الكادر الوظيفي | STAFF ID BADGE", "fontSize": 5.2, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 14, "y": 8.5, "width": 57, "height": 3.2 }
                        },
                        {
                            "type": "Separator",
                            "id": "header-sep",
                            "props": { "direction": "horizontal", "color": "#e2e8f0", "thickness": 0.25 },
                            "position": { "x": 4.5, "y": 13, "width": 76.6, "height": 0.3 }
                        },
                        {
                            "type": "Image",
                            "id": "employee-photo",
                            "props": { "src": "{{employee.avatarUrl}}", "fit": "cover" },
                            "position": { "x": 5, "y": 16, "width": 21, "height": 25 }
                        },
                        {
                            "type": "Shape",
                            "id": "photo-border",
                            "props": { "shapeType": "rectangle", "fillColor": "transparent", "strokeColor": "#e2e8f0", "strokeWidth": 0.3, "borderRadius": 1.5 },
                            "position": { "x": 5, "y": 16, "width": 21, "height": 25 }
                        },
                        {
                            "type": "Text",
                            "id": "emp-name",
                            "props": { "content": "{{employee.name}}", "fontSize": 9.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 29, "y": 16, "width": 52, "height": 5 }
                        },
                        {
                            "type": "Text",
                            "id": "emp-title",
                            "props": { "content": "{{employee.jobTitle}}", "fontSize": 7.2, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#312e81", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 29, "y": 21.8, "width": 40, "height": 4 }
                        },
                        {
                            "type": "Text",
                            "id": "emp-dept",
                            "props": { "content": "القسم: {{employee.department}}", "fontSize": 6.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 29, "y": 27, "width": 40, "height": 4 }
                        },
                        {
                            "type": "Text",
                            "id": "emp-no",
                            "props": { "content": "الرقم الوظيفي: {{employee.employeeNumber}}", "fontSize": 6.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#475569", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 29, "y": 32, "width": 40, "height": 4 }
                        },
                        {
                            "type": "QRCode",
                            "id": "emp-qr",
                            "props": { "data": "{{employee.employeeNumber}}", "size": 11, "errorCorrection": "M" },
                            "position": { "x": 70, "y": 28, "width": 11, "height": 11 }
                        },
                        {
                            "type": "Separator",
                            "id": "footer-sep",
                            "props": { "direction": "horizontal", "color": "#f1f5f9", "thickness": 0.25 },
                            "position": { "x": 4.5, "y": 45, "width": 76.6, "height": 0.3 }
                        },
                        {
                            "type": "Text",
                            "id": "footer-lbl",
                            "props": { "content": "بطاقة تعريف رسمية معتمدة  |  OFFICIAL STAFF BADGE", "fontSize": 4.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 5, "y": 47, "width": 75.6, "height": 3 }
                        }
                    ]
                }]
            }),
        ),
        // ── 3. Certificate of Regular Enrollment (A4 Portrait 210mm x 297mm) ─
        (
            "administration.enrollment-certificate",
            "Certificate of Regular Enrollment",
            "إفادة وقيد طالب منتظم ومستمر",
            "student",
            "CERTIFICATE",
            "A4",
            "portrait",
            json!({
                "version": "2.0",
                "meta": {
                    "name": "Certificate of Regular Enrollment",
                    "nameAr": "إفادة وقيد طالب منتظم ومستمر",
                    "documentType": "CERTIFICATE",
                    "entityType": "student",
                    "locale": "ar",
                    "pageSetup": { "size": "A4", "orientation": "portrait", "margins": { "top": 15, "bottom": 15, "left": 15, "right": 15 }, "unit": "mm" }
                },
                "variables": {
                    "student.name": { "source": "student.name", "type": "string", "label": "Student Name", "labelAr": "اسم الطالب" },
                    "student.studentNumber": { "source": "student.student_number", "type": "string", "label": "Student Number", "labelAr": "الرقم الأكاديمي" },
                    "student.className": { "source": "student.class_name", "type": "string", "label": "Class", "labelAr": "الصف والشعبة" },
                    "student.nationalId": { "source": "student.national_id", "type": "string", "label": "National ID", "labelAr": "رقم الهوية / الإقامة" },
                    "AcademicCalendar.name": { "source": "academic_calendar.name", "type": "string", "label": "Academic Year", "labelAr": "العام الدراسي" }
                },
                "assets": {},
                "pages": [{
                    "type": "Page",
                    "id": "p1",
                    "children": [
                        // Ornate Outer Frame
                        {
                            "type": "Shape",
                            "id": "outer-border",
                            "props": { "shapeType": "rectangle", "fillColor": "transparent", "strokeColor": "#0f172a", "strokeWidth": 1.5, "borderRadius": 4 },
                            "position": { "x": 10, "y": 10, "width": 190, "height": 277 }
                        },
                        {
                            "type": "Shape",
                            "id": "inner-border",
                            "props": { "shapeType": "rectangle", "fillColor": "transparent", "strokeColor": "#d97706", "strokeWidth": 0.5, "borderRadius": 2 },
                            "position": { "x": 13, "y": 13, "width": 184, "height": 271 }
                        },
                        // Header Institutional Details
                        {
                            "type": "Text",
                            "id": "ministry-header",
                            "props": { "content": "{{$org.country_name}}\n{{$org.ministry_name}}\n{{$org.region_name}}\n{{$org.name_ar}}", "fontSize": 10, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl", "lineHeight": 1.5 },
                            "position": { "x": 125, "y": 20, "width": 65, "height": 20 }
                        },
                        {
                            "type": "Text",
                            "id": "doc-meta-right",
                            "props": { "content": "الرقم الإشاري: {{referenceNumber}}\nالتاريخ: {{issueDate}}\nالمرفقات: لا يوجد", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#475569", "textAlign": "end", "direction": "rtl", "lineHeight": 1.5 },
                            "position": { "x": 20, "y": 20, "width": 65, "height": 20 }
                        },
                        // Certificate Main Title
                        {
                            "type": "Text",
                            "id": "cert-title",
                            "props": { "content": "إفـــادة قـيــد واستمـرار طــالـب", "fontSize": 20, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 20, "y": 55, "width": 170, "height": 12 }
                        },
                        {
                            "type": "Separator",
                            "id": "title-sep",
                            "props": { "direction": "horizontal", "color": "#d97706", "thickness": 1 },
                            "position": { "x": 70, "y": 70, "width": 70, "height": 2 }
                        },
                        // Body Attestation Text
                        {
                            "type": "Text",
                            "id": "body-text",
                            "props": {
                                "content": "تشهد إدارة {{$org.name_ar}} بأن الطالب/ة:\n\nالاسم الكامل: {{student.name}}\nرقم الهوية الوطنية / الإقامة: {{student.nationalId}}\nالرقم الأكاديمي: {{student.studentNumber}}\n\nمقيد/ة رسمياً ومنتظم/ة بالدراسة لدينا للعام الدراسي {{AcademicCalendar.name}}، في المرحلة الدراسية بالصف: {{student.className}}.\n\nوقد مُنحت له هذه الإفادة بناءً على طلبه لتقديمها إلى من يهمه الأمر، دون أي مسؤولية على المنظمة تجاه حقوق الغير.",
                                "fontSize": 12,
                                "fontFamily": "Rubik",
                                "fontWeight": "normal",
                                "color": "#1e293b",
                                "textAlign": "start",
                                "direction": "rtl",
                                "lineHeight": 2.0
                            },
                            "position": { "x": 25, "y": 80, "width": 160, "height": 100 }
                        },
                        // Signatures & Stamp Block
                        {
                            "type": "Text",
                            "id": "sig-registrar",
                            "props": { "content": "مسؤول القبول والتسجيل\n\n___________________", "fontSize": 10, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl", "lineHeight": 1.6 },
                            "position": { "x": 130, "y": 205, "width": 55, "height": 25 }
                        },
                        {
                            "type": "Text",
                            "id": "sig-principal",
                            "props": { "content": "مدير المؤسسة التعليمية\n\n___________________", "fontSize": 10, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl", "lineHeight": 1.6 },
                            "position": { "x": 25, "y": 205, "width": 55, "height": 25 }
                        },
                        {
                            "type": "Shape",
                            "id": "stamp-circle",
                            "props": { "shapeType": "circle", "fillColor": "transparent", "strokeColor": "#94a3b8", "strokeWidth": 1 },
                            "position": { "x": 92, "y": 200, "width": 26, "height": 26 }
                        },
                        {
                            "type": "Text",
                            "id": "stamp-lbl",
                            "props": { "content": "الختم الرسمي", "fontSize": 7, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#94a3b8", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 92, "y": 211, "width": 26, "height": 5 }
                        },
                        // Verification Footer & QR
                        {
                            "type": "QRCode",
                            "id": "cert-qr",
                            "props": { "data": "https://verify.orbis.edu/cert/{{student.studentNumber}}", "size": 18, "errorCorrection": "M" },
                            "position": { "x": 25, "y": 250, "width": 18, "height": 18 }
                        },
                        {
                            "type": "Text",
                            "id": "verification-note",
                            "props": { "content": "يمكن التحقق من صحة ومصداقية هذه الوثيقة إلكترونياً عبر مسح الرمز المرفق أو عبر المنصة الإلكترونية.", "fontSize": 7.5, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 48, "y": 256, "width": 135, "height": 8 }
                        }
                    ]
                }]
            }),
        ),
        // ── 4. Student Term Report Card (A4 Portrait 210mm x 297mm) ───────────
        (
            "academic.student-term-report-card",
            "Student Term Report Card",
            "كشف درجات الطالب للفصل الدراسي",
            "student",
            "TERM_CERTIFICATE",
            "A4",
            "portrait",
            json!({
                "version": "2.0",
                "meta": {
                    "name": "Student Term Report Card",
                    "nameAr": "كشف درجات الطالب للفصل الدراسي",
                    "documentType": "TERM_CERTIFICATE",
                    "entityType": "student",
                    "locale": "ar",
                    "pageSetup": { "size": "A4", "orientation": "portrait", "margins": { "top": 12, "bottom": 12, "left": 12, "right": 12 }, "unit": "mm" }
                },
                "variables": {
                    "student.name": { "source": "student.name", "type": "string", "label": "Student Name", "labelAr": "اسم الطالب" },
                    "student.studentNumber": { "source": "student.student_number", "type": "string", "label": "Student Number", "labelAr": "الرقم الأكاديمي" },
                    "student.className": { "source": "student.class_name", "type": "string", "label": "Class", "labelAr": "الصف والشعبة" },
                    "period.name": { "source": "period.name", "type": "string", "label": "Academic Period", "labelAr": "الفصل الدراسي" },
                    "AcademicCalendar.name": { "source": "academic_calendar.name", "type": "string", "label": "Academic Year", "labelAr": "العام الدراسي" }
                },
                "assets": {},
                "pages": [{
                    "type": "Page",
                    "id": "p1",
                    "children": [
                        // Header Box
                        {
                            "type": "Shape",
                            "id": "header-card",
                            "props": { "shapeType": "rectangle", "fillColor": "#f8fafc", "strokeColor": "#e2e8f0", "strokeWidth": 1, "borderRadius": 3 },
                            "position": { "x": 10, "y": 10, "width": 190, "height": 34 }
                        },
                        {
                            "type": "Text",
                            "id": "rep-org-title",
                            "props": { "content": "{{$org.name_ar}}", "fontSize": 12, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 115, "y": 14, "width": 80, "height": 6 }
                        },
                        {
                            "type": "Text",
                            "id": "rep-main-title",
                            "props": { "content": "إشعار كشف درجات أعمال واختبارات الفصل الدراسي", "fontSize": 11, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#2563eb", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 50, "y": 22, "width": 110, "height": 6 }
                        },
                        {
                            "type": "Text",
                            "id": "rep-year-period",
                            "props": { "content": "العام الدراسي: {{AcademicCalendar.name}} | {{period.name}}", "fontSize": 8.5, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#475569", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 50, "y": 30, "width": 110, "height": 5 }
                        },
                        // Student Info Summary Grid
                        {
                            "type": "Shape",
                            "id": "info-box",
                            "props": { "shapeType": "rectangle", "fillColor": "#ffffff", "strokeColor": "#cbd5e1", "strokeWidth": 0.5, "borderRadius": 2 },
                            "position": { "x": 10, "y": 48, "width": 190, "height": 18 }
                        },
                        {
                            "type": "Text",
                            "id": "info-line-1",
                            "props": { "content": "اسم الطالب: {{student.name}}", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 115, "y": 51, "width": 80, "height": 5 }
                        },
                        {
                            "type": "Text",
                            "id": "info-line-2",
                            "props": { "content": "الرقم الأكاديمي: {{student.studentNumber}}", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 15, "y": 51, "width": 80, "height": 5 }
                        },
                        {
                            "type": "Text",
                            "id": "info-line-3",
                            "props": { "content": "الصف والشعبة: {{student.className}}", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 115, "y": 58, "width": 80, "height": 5 }
                        },
                        {
                            "type": "Text",
                            "id": "info-line-4",
                            "props": { "content": "تاريخ الإصدار: {{issueDate}}", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 15, "y": 58, "width": 80, "height": 5 }
                        },
                        // Academic Results Table
                        {
                            "type": "Table",
                            "id": "grades-table",
                            "props": {
                                "dataSource": "grades",
                                "columns": [
                                    { "header": "المادة الدراسية", "headerAr": "المادة الدراسية", "field": "subjectName", "width": "35%" },
                                    { "header": "أعمال الفصل", "headerAr": "أعمال الفصل (40)", "field": "courseworkScore", "width": "15%" },
                                    { "header": "اختبار الفصل", "headerAr": "اختبار الفصل (60)", "field": "examScore", "width": "15%" },
                                    { "header": "المجموع الكلي", "headerAr": "المجموع (100)", "field": "totalScore", "width": "15%" },
                                    { "header": "التقدير", "headerAr": "التقدير", "field": "gradeLetter", "width": "20%" }
                                ],
                                "showHeader": true,
                                "showGrid": true,
                                "direction": "rtl"
                            },
                            "position": { "x": 10, "y": 72, "width": 190, "height": 120 }
                        },
                        // Summary Statistics Box
                        {
                            "type": "Shape",
                            "id": "summary-box",
                            "props": { "shapeType": "rectangle", "fillColor": "#f1f5f9", "strokeColor": "#cbd5e1", "strokeWidth": 0.5, "borderRadius": 2 },
                            "position": { "x": 10, "y": 200, "width": 190, "height": 28 }
                        },
                        {
                            "type": "Text",
                            "id": "sum-gpa",
                            "props": { "content": "المعدل الفصلي: {{summary.termGpa}} %", "fontSize": 11, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 110, "y": 204, "width": 80, "height": 6 }
                        },
                        {
                            "type": "Text",
                            "id": "sum-rank",
                            "props": { "content": "الترتيب على الشعبة: {{summary.rank}}", "fontSize": 10, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#2563eb", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 15, "y": 204, "width": 80, "height": 6 }
                        },
                        {
                            "type": "Text",
                            "id": "sum-standing",
                            "props": { "content": "التقدير العام: {{summary.standing}}", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 110, "y": 214, "width": 80, "height": 5 }
                        },
                        {
                            "type": "Text",
                            "id": "sum-attendance",
                            "props": { "content": "نسبة الحضور والمواظبة: {{summary.attendanceRate}} %", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 15, "y": 214, "width": 80, "height": 5 }
                        },
                        // Signatures & Endorsements
                        {
                            "type": "Text",
                            "id": "sig-advisor",
                            "props": { "content": "رائد الفصل / المرشد الأكاديمي\n\n___________________", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl", "lineHeight": 1.6 },
                            "position": { "x": 135, "y": 240, "width": 55, "height": 22 }
                        },
                        {
                            "type": "Text",
                            "id": "sig-head",
                            "props": { "content": "مدير الشؤون التعليمية\n\n___________________", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl", "lineHeight": 1.6 },
                            "position": { "x": 20, "y": 240, "width": 55, "height": 22 }
                        }
                    ]
                }]
            }),
        ),
        // ── 5. Payment Receipt Voucher (A4 Portrait 210mm x 297mm) ────────────
        finance_document(
            "finance.payment-receipt",
            "Payment Receipt",
            "سند قبض",
            "finance",
            "RECEIPT",
        ),
        // ── 6. Student Exam Admission & Seating Card (ID_CARD Landscape 85.6mm x 54mm) ─
        (
            "examinations.student-exam-card",
            "Student Exam Admission & Seating Card",
            "بطاقة دخول الاختبار ورقم الجلوس المعتمد",
            "student",
            "EXAM_CARD",
            "ID_CARD",
            "landscape",
            json!({
                "version": "2.0",
                "meta": {
                    "name": "Student Exam Admission & Seating Card",
                    "nameAr": "بطاقة دخول الاختبار ورقم الجلوس المعتمد",
                    "documentType": "EXAM_CARD",
                    "entityType": "student",
                    "locale": "ar",
                    "pageSetup": { "size": "ID_CARD", "orientation": "landscape", "margins": { "top": 0, "bottom": 0, "left": 0, "right": 0 }, "unit": "mm" }
                },
                "variables": {
                    "student.name": { "source": "student.name", "type": "string", "label": "Student Name", "labelAr": "اسم الطالب" },
                    "exam.seatingNumber": { "source": "exam.seating_number", "type": "string", "label": "Seating Number", "labelAr": "رقم الجلوس" },
                    "exam.hallName": { "source": "exam.hall_name", "type": "string", "label": "Exam Hall", "labelAr": "قاعة الاختبار" },
                    "exam.sessionName": { "source": "exam.session_name", "type": "string", "label": "Session Name", "labelAr": "الدورة الاختبارية" }
                },
                "assets": {},
                "pages": [{
                    "type": "Page",
                    "id": "p1",
                    "children": [
                        {
                            "type": "Shape",
                            "id": "top-accent",
                            "props": { "shapeType": "rectangle", "fillColor": "#1e293b", "strokeColor": "#1e293b", "strokeWidth": 0, "borderRadius": 0 },
                            "position": { "x": 0, "y": 0, "width": 85.6, "height": 1.4 }
                        },
                        {
                            "type": "Image",
                            "id": "org-logo",
                            "props": { "src": "{{$org.logo_url}}", "fit": "contain" },
                            "position": { "x": 73, "y": 3.5, "width": 8, "height": 8 }
                        },
                        {
                            "type": "Text",
                            "id": "exam-header",
                            "props": { "content": "{{$org.name_ar | default:$org.name}}", "fontSize": 8.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 14, "y": 3.8, "width": 57, "height": 4.5 }
                        },
                        {
                            "type": "Text",
                            "id": "exam-sub",
                            "props": { "content": "بطاقة رقم الجلوس ودخول الاختبار", "fontSize": 5.2, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 14, "y": 8.5, "width": 57, "height": 3.2 }
                        },
                        {
                            "type": "Separator",
                            "id": "exam-header-sep",
                            "props": { "direction": "horizontal", "color": "#e2e8f0", "thickness": 0.25 },
                            "position": { "x": 4.5, "y": 13, "width": 76.6, "height": 0.3 }
                        },
                        {
                            "type": "Shape",
                            "id": "seat-box",
                            "props": { "shapeType": "rectangle", "fillColor": "#f8fafc", "strokeColor": "#cbd5e1", "strokeWidth": 0.3, "borderRadius": 2 },
                            "position": { "x": 5, "y": 16, "width": 23, "height": 22 }
                        },
                        {
                            "type": "Text",
                            "id": "seat-title",
                            "props": { "content": "رقم الجلوس", "fontSize": 5.5, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 6, "y": 18, "width": 21, "height": 3.5 }
                        },
                        {
                            "type": "Text",
                            "id": "seat-val",
                            "props": { "content": "{{exam.seatingNumber | default:'—'}}", "fontSize": 12.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "ltr" },
                            "position": { "x": 6, "y": 23.5, "width": 21, "height": 9 }
                        },
                        {
                            "type": "Text",
                            "id": "exam-student-name",
                            "props": { "content": "{{student.name | default:'—'}}", "fontSize": 8.8, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 31, "y": 16.5, "width": 50, "height": 5 }
                        },
                        {
                            "type": "Text",
                            "id": "exam-hall-info",
                            "props": { "content": "القاعة: {{exam.hallName | default:'—'}}", "fontSize": 6.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 31, "y": 23, "width": 37, "height": 4 }
                        },
                        {
                            "type": "Text",
                            "id": "exam-session-info",
                            "props": { "content": "الدورة: {{exam.sessionName | default:'—'}}", "fontSize": 6.2, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 31, "y": 28, "width": 37, "height": 4 }
                        },
                        {
                            "type": "QRCode",
                            "id": "exam-qr",
                            "props": { "data": "{{exam.seatingNumber}}", "size": 11, "errorCorrection": "M" },
                            "position": { "x": 70, "y": 28, "width": 11, "height": 11 }
                        },
                        {
                            "type": "Separator",
                            "id": "exam-sep",
                            "props": { "direction": "horizontal", "color": "#f1f5f9", "thickness": 0.25 },
                            "position": { "x": 4.5, "y": 45, "width": 76.6, "height": 0.3 }
                        },
                        {
                            "type": "Text",
                            "id": "exam-rules",
                            "props": { "content": "تُبرز البطاقة عند دخول اللجنة ويُمنع اصطحاب الهواتف", "fontSize": 4.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 5, "y": 47, "width": 75.6, "height": 3 }
                        }
                    ]
                }]
            }),
        ),
        // ── 7. Official Multi-Year Academic Transcript (A4 Portrait) ──────────
        (
            "academic.official-transcript",
            "Official Multi-Year Academic Transcript",
            "السجل الأكاديمي الشامل وكشف العلامات التراكمي",
            "student",
            "TRANSCRIPT",
            "A4",
            "portrait",
            json!({
                "version": "2.0",
                "meta": {
                    "name": "Official Multi-Year Academic Transcript",
                    "nameAr": "السجل الأكاديمي الشامل وكشف العلامات التراكمي",
                    "documentType": "TRANSCRIPT",
                    "entityType": "student",
                    "locale": "ar",
                    "pageSetup": { "size": "A4", "orientation": "portrait", "margins": { "top": 12, "bottom": 12, "left": 12, "right": 12 }, "unit": "mm" }
                },
                "variables": {
                    "student.name": { "source": "student.name", "type": "string", "label": "Student Name", "labelAr": "اسم الطالب" },
                    "student.studentNumber": { "source": "student.student_number", "type": "string", "label": "Student Number", "labelAr": "الرقم الأكاديمي" },
                    "student.major": { "source": "student.major", "type": "string", "label": "Major/Track", "labelAr": "التخصص / المسار" },
                    "transcript.cumulativeGpa": { "source": "transcript.cumulative_gpa", "type": "number", "label": "Cumulative GPA", "labelAr": "المعدل التراكمي العام" }
                },
                "assets": {},
                "pages": [{
                    "type": "Page",
                    "id": "p1",
                    "children": [
                        {
                            "type": "Shape",
                            "id": "outer-frame",
                            "props": { "shapeType": "rectangle", "fillColor": "transparent", "strokeColor": "#0f172a", "strokeWidth": 1.2, "borderRadius": 2 },
                            "position": { "x": 10, "y": 10, "width": 190, "height": 277 }
                        },
                        {
                            "type": "Text",
                            "id": "transcript-header",
                            "props": { "content": "{{$org.country_name}}\n{{$org.ministry_name}}\n{{$org.region_name}}\n{{$org.name_ar}}\nعمادة القبول والتسجيل", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl", "lineHeight": 1.4 },
                            "position": { "x": 120, "y": 15, "width": 70, "height": 22 }
                        },
                        {
                            "type": "Text",
                            "id": "transcript-title",
                            "props": { "content": "السجل الأكاديمي الرسمي\nOFFICIAL ACADEMIC TRANSCRIPT", "fontSize": 13, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl", "lineHeight": 1.3 },
                            "position": { "x": 55, "y": 16, "width": 100, "height": 14 }
                        },
                        {
                            "type": "QRCode",
                            "id": "transcript-qr",
                            "props": { "data": "{{student.studentNumber}}", "size": 16, "errorCorrection": "M" },
                            "position": { "x": 18, "y": 15, "width": 16, "height": 16 }
                        },
                        // Student Info Bar
                        {
                            "type": "Shape",
                            "id": "st-info-bar",
                            "props": { "shapeType": "rectangle", "fillColor": "#f8fafc", "strokeColor": "#cbd5e1", "strokeWidth": 0.5, "borderRadius": 1 },
                            "position": { "x": 12, "y": 40, "width": 186, "height": 16 }
                        },
                        {
                            "type": "Text",
                            "id": "st-meta-1",
                            "props": { "content": "اسم الطالب: {{student.name}}\nالمسار / البرنامج: {{student.major}}", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl", "lineHeight": 1.5 },
                            "position": { "x": 110, "y": 42, "width": 84, "height": 12 }
                        },
                        {
                            "type": "Text",
                            "id": "st-meta-2",
                            "props": { "content": "الرقم الأكاديمي: {{student.studentNumber}}\nالمعدل التراكمي: {{transcript.cumulativeGpa}}", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#2563eb", "textAlign": "start", "direction": "rtl", "lineHeight": 1.5 },
                            "position": { "x": 18, "y": 42, "width": 84, "height": 12 }
                        },
                        // Transcript Courses Table
                        {
                            "type": "Table",
                            "id": "transcript-table",
                            "props": {
                                "dataSource": "transcriptCourses",
                                "columns": [
                                    { "header": "Course Code", "headerAr": "رمز المقرر", "field": "courseCode", "width": "15%", "alignment": "center" },
                                    { "header": "Course Title", "headerAr": "اسم المقرر الدراسي", "field": "courseName", "width": "40%", "alignment": "right", "total": "label", "totalLabel": "إجمالي الساعات المعتمدة" },
                                    { "header": "Credits", "headerAr": "الساعات", "field": "creditHours", "width": "15%", "alignment": "center", "total": "sum" },
                                    { "header": "Grade", "headerAr": "الدرجة", "field": "gradeScore", "width": "15%", "alignment": "center" },
                                    { "header": "Letter", "headerAr": "الرمز", "field": "gradeLetter", "width": "15%", "alignment": "center" }
                                ],
                                "showHeader": true,
                                "showTotals": true,
                                "showGrid": true,
                                "showBorder": true,
                                "direction": "rtl",
                                "style": {
                                    "headerBgColor": "#0f172a",
                                    "headerTextColor": "#ffffff",
                                    "alternateRowBgColor": "#f8fafc",
                                    "borderColor": "#cbd5e1",
                                    "borderWidth": 0.75,
                                    "headerFontSize": 8.0,
                                    "fontSize": 8.0,
                                    "rowHeight": 18.0
                                }
                            },
                            "position": { "x": 12, "y": 60, "width": 186, "height": 175 }
                        },
                        // Summary & Attestation
                        {
                            "type": "Shape",
                            "id": "t-summary-box",
                            "props": { "shapeType": "rectangle", "fillColor": "#f1f5f9", "strokeColor": "#cbd5e1", "strokeWidth": 0.5, "borderRadius": 1 },
                            "position": { "x": 12, "y": 240, "width": 186, "height": 18 }
                        },
                        {
                            "type": "Text",
                            "id": "t-summary-text",
                            "props": { "content": "إجمالي الساعات المكتسبة: {{transcript.earnedHours}} | الساعات المسجلة: {{transcript.registeredHours}} | المعدل العام: {{transcript.cumulativeGpa}} | التقدير: {{transcript.standing}}", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 15, "y": 245, "width": 180, "height": 8 }
                        },
                        {
                            "type": "Text",
                            "id": "sig-dean",
                            "props": { "content": "عميد القبول والتسجيل\n___________________", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 20, "y": 262, "width": 60, "height": 18 }
                        }
                    ]
                }]
            }),
        ),
    ];

    templates.push(default_yemeni_academic_certificate_template());
    templates.push(default_exam_hall_seating_card_template());
    templates.push(default_exam_hall_register_template());
    templates.push(default_user_id_card_template());
    templates.push(default_letterhead_template());
    templates.push(finance_document(
        "finance.disbursement-voucher",
        "Disbursement Voucher",
        "سند صرف",
        "finance_disbursement",
        "VOUCHER",
    ));
    templates.push(finance_document(
        "finance.journal-voucher",
        "Journal Voucher",
        "سند قيد يومية",
        "finance_journal",
        "VOUCHER",
    ));
    templates.extend(default_grade_certificate_templates());
    templates.push(default_class_schedule_template());
    templates.push(default_account_statement_template());
    for (_, _, _, _, _, _, _, document) in &mut templates {
        if let Some(pages) = document
            .get_mut("pages")
            .and_then(serde_json::Value::as_array_mut)
        {
            pages.iter_mut().for_each(apply_stock_template_style);
        }
    }
    templates
}

fn apply_stock_template_style(node: &mut serde_json::Value) {
    let node_type = node["type"].as_str().unwrap_or_default().to_owned();
    let id = node["id"].as_str().unwrap_or_default().to_owned();
    if let Some(props) = node
        .get_mut("props")
        .and_then(serde_json::Value::as_object_mut)
    {
        if props.get("fontFamily").and_then(serde_json::Value::as_str) == Some("Rubik") {
            props.insert("fontFamily".into(), json!("Noto Sans Arabic"));
        }
        if node_type == "Table" {
            if let Some(style) = props
                .get_mut("style")
                .and_then(serde_json::Value::as_object_mut)
            {
                style.insert("fontFamily".into(), json!("Noto Sans Arabic"));
                style.insert("headerBgColor".into(), json!("#eaf0f5"));
                style.insert("headerTextColor".into(), json!("#0f172a"));
                style
                    .entry("altRowBgColor")
                    .or_insert_with(|| json!("#f8fafc"));
            }
        }
        if node_type == "Separator" {
            props.insert("color".into(), json!("org-primary"));
        }
        if node_type == "Shape" {
            if id.ends_with("-accent") || id.ends_with("-stripe") || id.ends_with("-strip") {
                props.insert("fillColor".into(), json!("org-primary"));
            } else if id.ends_with("-border") || id.ends_with("-frame") || id.starts_with("bg-") {
                props.insert("strokeColor".into(), json!("org-primary"));
            }
        }
    }
    if let Some(children) = node
        .get_mut("children")
        .and_then(serde_json::Value::as_array_mut)
    {
        children.iter_mut().for_each(apply_stock_template_style);
    }
}

fn finance_document(
    code: &'static str,
    name: &'static str,
    name_ar: &'static str,
    entity: &'static str,
    document_type: &'static str,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    serde_json::Value,
) {
    let receipt = document_type == "RECEIPT";
    let namespace = if receipt { "receipt" } else { "voucher" };
    let mut variables = serde_json::Map::new();
    let text_box = |id: &str,
                    content: &str,
                    x: f64,
                    y: f64,
                    width: f64,
                    height: f64,
                    size: f64,
                    bold: bool,
                    align: &str,
                    dir: &str,
                    color: &str| {
        json!({
            "type": "Text",
            "id": id,
            "props": {
                "content": content,
                "fontFamily": "Noto Sans Arabic",
                "fontSize": size,
                "fontWeight": if bold { "bold" } else { "normal" },
                "direction": dir,
                "textAlign": align,
                "lineHeight": 1.25,
                "color": color
            },
            "position": { "x": x, "y": y, "width": width, "height": height }
        })
    };
    let divider = |id: &str, y: f64| {
        json!({
            "type": "Separator",
            "id": id,
            "props": { "direction": "horizontal", "color": "#d8e0e7", "thickness": 0.6 },
            "position": { "x": 12, "y": y, "width": 186, "height": 1 }
        })
    };

    let mut children = vec![
        // Outer subtle frame & top accent bar (A5 Landscape: 210mm x 148mm)
        json!({
            "type": "Shape",
            "id": "bg-voucher",
            "props": { "shapeType": "rectangle", "fillColor": "#ffffff", "strokeColor": "#cbd5e1", "strokeWidth": 0.5, "borderRadius": 3 },
            "position": { "x": 6, "y": 6, "width": 198, "height": 136 }
        }),
        json!({
            "type": "Shape",
            "id": "voucher-accent",
            "props": { "shapeType": "rectangle", "fillColor": "#0f172a", "strokeWidth": 0, "borderRadius": 1.5 },
            "position": { "x": 6, "y": 6, "width": 198, "height": 2.5 }
        }),
        // Header Right: Organization + Voucher Title
        text_box(
            "organization",
            "{{$org.name_ar | default:$org.name}}",
            106.0,
            11.5,
            92.0,
            6.0,
            10.0,
            true,
            "start",
            "rtl",
            "#475569",
        ),
        text_box(
            "title-ar", name_ar, 106.0, 18.5, 92.0, 8.0, 15.0, true, "start", "rtl", "#0f172a",
        ),
        text_box(
            "title-en", name, 106.0, 27.5, 92.0, 5.0, 8.5, false, "start", "rtl", "#64748b",
        ),
        // Header Left: Metadata Pill (Number & Date)
        json!({
            "type": "Shape",
            "id": "meta-box",
            "props": { "shapeType": "rectangle", "fillColor": "#f8fafc", "strokeColor": "#e2e8f0", "strokeWidth": 0.45, "borderRadius": 2 },
            "position": { "x": 12, "y": 12, "width": 78, "height": 19 }
        }),
        divider("header-divider", 35.0),
    ];

    let fields: &[(&str, &str, &str)] = if receipt {
        &[
            ("number", "Receipt number", "رقم السند"),
            ("date", "Date", "التاريخ"),
            ("payerName", "Received from", "استلمنا من"),
            ("studentName", "Student", "الطالب"),
            ("paymentMethod", "Payment method", "طريقة الدفع"),
            ("referenceNumber", "Reference", "رقم المرجع"),
            ("description", "Description", "البيان"),
        ]
    } else {
        &[
            ("number", "Voucher number", "رقم السند"),
            ("date", "Date", "التاريخ"),
            (
                "primaryName",
                "Debit account",
                if entity == "finance_disbursement" {
                    "المستفيد"
                } else {
                    "الحساب المدين"
                },
            ),
            (
                "secondaryName",
                "Credit account",
                if entity == "finance_disbursement" {
                    "حساب الخزينة"
                } else {
                    "الحساب الدائن"
                },
            ),
            ("description", "Description", "البيان"),
        ]
    };

    for (index, &(field, label, label_ar)) in fields.iter().enumerate() {
        let key = format!("{namespace}.{field}");
        variables.insert(
            key.clone(),
            json!({ "source": key, "type": "string", "label": label, "labelAr": label_ar }),
        );
        let value_format = if field == "date" {
            " | date:'yyyy/MM/dd'"
        } else {
            ""
        };
        let content = format!("{label_ar}: {{{{{key}{value_format} | default:'—'}}}}");

        if index == 0 {
            children.push(text_box(
                field, &content, 15.0, 14.0, 72.0, 6.0, 9.5, true, "start", "rtl", "#0f172a",
            ));
        } else if index == 1 {
            children.push(text_box(
                field, &content, 15.0, 22.0, 72.0, 6.0, 8.8, false, "start", "rtl", "#475569",
            ));
        } else {
            let body_idx = index - 2;
            let is_last_desc = field == "description";
            if is_last_desc {
                let row = body_idx / 2;
                let y = 60.5 + row as f64 * 12.5;
                children.push(text_box(
                    field, &content, 12.0, y, 186.0, 11.0, 9.5, false, "start", "rtl", "#1e293b",
                ));
            } else {
                let row = body_idx / 2;
                let col = body_idx % 2;
                let x = if col == 0 { 108.0 } else { 12.0 };
                let y = 60.5 + row as f64 * 12.5;
                children.push(text_box(
                    field, &content, x, y, 90.0, 8.0, 9.5, false, "start", "rtl", "#1e293b",
                ));
            }
        }
    }

    let amount_key = format!("{namespace}.amount");
    let currency_key = format!("{namespace}.currency");
    variables.insert(
        amount_key.clone(),
        json!({ "source": amount_key, "type": "number", "label": "Amount", "labelAr": "المبلغ" }),
    );
    variables.insert(
        currency_key.clone(),
        json!({ "source": currency_key, "type": "string", "label": "Currency", "labelAr": "العملة" }),
    );

    children.extend([
        json!({
            "type": "Shape",
            "id": "amount-background",
            "props": { "shapeType": "rectangle", "fillColor": "#f1f5f9", "strokeColor": "#cbd5e1", "strokeWidth": 0.4, "borderRadius": 2.5 },
            "position": { "x": 12, "y": 39, "width": 186, "height": 16.5 }
        }),
        text_box(
            "amount-label",
            "المبلغ الإجمالي",
            152.0,
            44.0,
            40.0,
            6.0,
            9.5,
            false,
            "start",
            "rtl",
            "#475569",
        ),
        text_box(
            "amount",
            &format!("{{{{{amount_key} | default:'—'}}}}  {{{{{currency_key} | default:$org.currency}}}}"),
            16.0,
            42.5,
            132.0,
            9.5,
            15.5,
            true,
            "center",
            "ltr",
            "#0f172a",
        ),
        divider("footer-divider", 107.0),
    ]);

    if receipt {
        children.push(json!({
            "type": "QRCode",
            "id": "receipt-reference-code",
            "props": { "data": "{{receipt.number}}", "size": 20, "errorCorrection": "M" },
            "position": { "x": 14, "y": 112, "width": 20, "height": 20 }
        }));
        children.push(text_box(
            "electronic-note",
            "سند قبض مالي معتمد إلكترونياً",
            38.0,
            119.5,
            68.0,
            6.0,
            8.0,
            false,
            "start",
            "rtl",
            "#64748b",
        ));
        children.push(text_box(
            "signature",
            "توقيع أمين الصندوق:  _______________________",
            112.0,
            119.0,
            86.0,
            7.0,
            9.0,
            false,
            "start",
            "rtl",
            "#334155",
        ));
    } else {
        children.push(text_box(
            "signature",
            "المحاسب:  _______________________",
            112.0,
            119.0,
            86.0,
            7.0,
            9.0,
            false,
            "start",
            "rtl",
            "#334155",
        ));
        children.push(text_box(
            "signature-approval",
            "الاعتماد:  _______________________",
            14.0,
            119.0,
            86.0,
            7.0,
            9.0,
            false,
            "start",
            "rtl",
            "#334155",
        ));
    }

    (
        code,
        name,
        name_ar,
        entity,
        document_type,
        "A5",
        "landscape",
        json!({
            "version": "2.0",
            "meta": {
                "name": name,
                "nameAr": name_ar,
                "documentType": document_type,
                "entityType": entity,
                "locale": "ar",
                "pageSetup": {
                    "size": "A5",
                    "orientation": "landscape",
                    "unit": "mm",
                    "margins": { "top": 8, "bottom": 8, "left": 10, "right": 10 }
                }
            },
            "variables": variables,
            "pages": [{ "type": "Page", "id": "p1", "children": children }]
        }),
    )
}

#[cfg(test)]
#[test]
fn financial_master_templates_render_as_single_a5_pages() {
    use docforge_engine::{
        layout::engine::LayoutEngine,
        model::document::TemplateDocument,
        render::{
            font_cache::FontCache,
            types::{PositionedElement, TextAlign},
        },
    };

    let fonts = FontCache::with_system_fonts();
    let engine = LayoutEngine::new(&fonts);
    let context = json!({
        "$org": { "name": "Orbis School", "currency": "SAR", "primary_color": "#086f72" },
        "receipt": { "number": "R-125", "amount": 825, "date": "2026-11-25T02:00:00+03:00",
            "payerName": "أحمد محمد", "studentName": "محمد أحمد", "paymentMethod": "تحويل بنكي",
            "referenceNumber": "TX-451", "description": "رسوم الفصل الدراسي الأول" },
        "voucher": { "number": "V-125", "amount": 825, "date": "2026-11-25" }
    });
    let mut checked = 0;
    for (code, _, _, _, _, size, orientation, definition) in get_master_template_definitions() {
        let document: TemplateDocument =
            serde_json::from_value(definition).unwrap_or_else(|error| panic!("{code}: {error}"));
        if !matches!(
            code,
            "finance.payment-receipt" | "finance.disbursement-voucher" | "finance.journal-voucher"
        ) {
            continue;
        }
        assert_eq!(size, "A5");
        assert_eq!(orientation, "landscape");
        let layout = engine.layout(&document, &context).unwrap();
        assert_eq!(layout.pages.len(), 1);
        let page = &layout.pages[0];
        if code == "finance.payment-receipt" {
            if let Some(directory) = std::env::var_os("ORBIS_PRINT_PREVIEW") {
                let pdf = docforge_engine::render::pdf_renderer::render_to_pdf(
                    &layout,
                    &fonts,
                    &Default::default(),
                )
                .unwrap();
                std::fs::write(
                    std::path::Path::new(&directory).join("receipt.pdf"),
                    pdf.bytes,
                )
                .unwrap();
            }
        }
        assert!((page.width_pt - 595.28).abs() < 0.2);
        assert!((page.height_pt - 419.53).abs() < 0.2);
        assert!(page.elements.iter().any(
            |element| matches!(element, PositionedElement::Text(text) if text.content.contains("825") && text.content.contains("SAR"))
        ));
        assert!(page.elements.iter().any(
            |element| matches!(element, PositionedElement::Text(text) if text.content.contains("2026/11/25"))
        ));
        assert!(page.elements.iter().any(
            |element| matches!(element, PositionedElement::Text(text) if text.content == "Orbis School" && matches!(text.text_align, TextAlign::Start))
        ));
        assert!(page.elements.iter().any(
            |element| matches!(element, PositionedElement::Line(line) if (line.color.r, line.color.g, line.color.b) == (8, 111, 114))
        ));
        checked += 1;
    }
    assert_eq!(checked, 3);
}

fn default_grade_certificate_templates() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    serde_json::Value,
)> {
    let emblem = STANDARD.encode(include_bytes!(
        "../../../orbis-app/src/defaults/assest/The Public Slogan.png"
    ));
    let school_art = STANDARD.encode(include_bytes!(
        "../../../orbis-app/src/defaults/assest/The smiling face.png"
    ));
    let play_blocks = STANDARD.encode(include_bytes!(
        "../../../orbis-app/src/defaults/assest/cube.png"
    ));
    let emblem = format!("data:image/png;base64,{emblem}");
    let school_art = format!("data:image/png;base64,{school_art}");
    let play_blocks = format!("data:image/png;base64,{play_blocks}");

    [
        (
            "academic.primary-grade-certificate",
            "Primary School Grade Certificate",
            "شهادة درجات المرحلة الابتدائية",
            "student",
            "REPORT",
            "A5",
            "landscape",
            true,
        ),
        (
            "academic.general-grade-certificate",
            "General Grade Certificate",
            "شهادة درجات عامة",
            "student",
            "REPORT",
            "A5",
            "landscape",
            false,
        ),
    ]
    .into_iter()
    .map(
        |(code, name, name_ar, entity, document_type, page_size, orientation, elementary)| {
            let decorative_nodes = if elementary {
                vec![
                    {
                        json!({
                            "type": "Image",
                            "id": "certificate-school-art",
                            "props": { "src": school_art, "fit": "contain" },
                            "position": { "x": 180, "y": 7, "width": 25, "height": 30 }
                        })
                    },
                    json!({
                        "type": "Image",
                        "id": "certificate-play-blocks",
                        "props": { "src": play_blocks, "fit": "contain" },
                        "position": { "x": 5, "y": 108, "width": 32, "height": 36 }
                    }),
                ]
            } else {
                vec![json!({
                        "type": "Shape",
                        "id": "certificate-decoration",
                        "props": { "shapeType": "rectangle", "fillColor": "transparent", "strokeColor": "#d6b66a", "strokeWidth": 0.8, "borderRadius": 3 },
                        "position": { "x": 4, "y": 4, "width": 202, "height": 140 }
                    })]
            };

            let mut children = vec![
                json!({
                    "type": "Image",
                    "id": "organization-emblem",
                    "props": { "src": emblem, "fit": "contain" },
                    "position": { "x": 96, "y": 8, "width": 18, "height": 16 }
                }),
                json!({
                    "type": "Text",
                    "id": "bismillah",
                    "props": { "content": "بسم الله الرحمن الرحيم", "fontSize": 8, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#111827", "textAlign": "center", "direction": "rtl" },
                    "position": { "x": 65, "y": 3, "width": 80, "height": 5 }
                }),
                json!({
                    "type": "Text",
                    "id": "institution-header",
                    "props": { "content": "{{$org.country_name}}\n{{$org.ministry_name}}\n{{$org.region_name}}\n{{$org.name_ar}}", "fontSize": 7, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#111827", "textAlign": "center", "direction": "rtl", "lineHeight": 1.25 },
                    "position": { "x": 145, "y": 5, "width": 60, "height": 26 }
                }),
                json!({
                    "type": "Text",
                    "id": "certificate-title",
                    "props": { "content": "كشف درجات {{report.periodName}}", "fontSize": 13, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#111827", "textAlign": "center", "direction": "rtl" },
                    "position": { "x": 30, "y": 34, "width": 150, "height": 10 }
                }),
                json!({
                    "type": "Text",
                    "id": "student-details-right",
                    "props": { "content": "اسم الطالب: {{student.name}}\nالصف: {{student.className}}", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#111827", "textAlign": "start", "direction": "rtl", "lineHeight": 1.7 },
                    "position": { "x": 110, "y": 45, "width": 90, "height": 15 }
                }),
                json!({
                    "type": "Text",
                    "id": "student-details-left",
                    "props": { "content": "العام الدراسي: {{AcademicCalendar.name}}\nالتقرير: {{report.periodName}}", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#111827", "textAlign": "start", "direction": "rtl", "lineHeight": 1.7 },
                    "position": { "x": 10, "y": 45, "width": 90, "height": 15 }
                }),
                json!({
                    "type": "Table",
                    "id": "grades-table",
                    "props": {
                        "dataSource": "grades",
                        "columns": [
                            { "header": "المادة", "headerAr": "المادة", "field": "subjectName", "width": "32%", "alignment": "right", "total": "label", "totalLabel": "المجموع الكلي" },
                            { "header": "أعمال الفصل", "headerAr": "أعمال الفصل", "field": "courseworkScore", "width": "17%", "alignment": "center", "total": "sum" },
                            { "header": "الاختبار", "headerAr": "الاختبار", "field": "examScore", "width": "17%", "alignment": "center", "total": "sum" },
                            { "header": "المجموع", "headerAr": "المجموع", "field": "totalScore", "width": "17%", "alignment": "center", "total": "sum" },
                            { "header": "التقدير", "headerAr": "التقدير", "field": "gradeLetter", "width": "17%", "alignment": "center" }
                        ],
                        "showHeader": true,
                        "showTotals": true,
                        "showGrid": true,
                        "showBorder": true,
                        "direction": "rtl",
                        "style": {
                            "headerBgColor": "#eaf0f5",
                            "headerTextColor": "#0f172a",
                            "alternateRowBgColor": "#f8fafc",
                            "borderColor": "#94a3b8",
                            "borderWidth": 0.6,
                            "headerFontSize": 8.5,
                            "fontSize": 8.0,
                            "rowHeight": 19.0,
                            "padding": 4
                        }
                    },
                    "position": { "x": 45, "y": 64, "width": 155, "height": 62 }
                }),
                json!({
                    "type": "Text",
                    "id": "notes",
                    "props": { "content": "ملاحظات\n..........................................\n..........................................", "fontSize": 8, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#111827", "textAlign": "center", "direction": "rtl", "lineHeight": 1.7 },
                    "position": { "x": 5, "y": 77, "width": 38, "height": 32 }
                }),
                json!({
                    "type": "Text",
                    "id": "teacher-signature",
                    "props": { "content": "معلم الصف\n........................", "fontSize": 8, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#111827", "textAlign": "center", "direction": "rtl" },
                    "position": { "x": 145, "y": 130, "width": 50, "height": 14 }
                }),
                json!({
                    "type": "Text",
                    "id": "principal-signature",
                    "props": { "content": "مدير المدرسة\n........................", "fontSize": 8, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#111827", "textAlign": "center", "direction": "rtl" },
                    "position": { "x": 65, "y": 130, "width": 50, "height": 14 }
                }),
                json!({
                    "type": "Text",
                    "id": "teacher-name",
                    "props": { "content": "{{teacher.name}}", "fontSize": 7, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#111827", "textAlign": "center", "direction": "rtl" },
                    "position": { "x": 145, "y": 144, "width": 55, "height": 4 }
                }),
            ];
            children.extend(decorative_nodes);

            (
                code,
                name,
                name_ar,
                entity,
                document_type,
                page_size,
                orientation,
                json!({
                    "version": "2.0",
                    "meta": {
                        "name": name,
                        "nameAr": name_ar,
                        "documentType": document_type,
                        "entityType": entity,
                        "locale": "ar",
                        "pageSetup": {
                            "size": page_size,
                            "orientation": orientation,
                            "margins": { "top": 3, "bottom": 3, "left": 3, "right": 3 },
                            "unit": "mm"
                        }
                    },
                    "variables": {
                        "student.name": { "source": "student.name", "type": "string", "label": "Student Name", "labelAr": "اسم الطالب" },
                        "student.className": { "source": "student.class_name", "type": "string", "label": "Class", "labelAr": "الصف" },
                        "AcademicCalendar.name": { "source": "academic_calendar.name", "type": "string", "label": "Academic Year", "labelAr": "العام الدراسي" },
                        "report.periodName": { "source": "report.period_name", "type": "string", "label": "Report Period", "labelAr": "فترة التقرير" },
                        "teacher.name": { "source": "teacher.name", "type": "string", "label": "Teacher Name", "labelAr": "اسم المعلم" }
                    },
                    "assets": {},
                    "pages": [{
                        "type": "Page",
                        "id": "p1",
                        "children": children
                    }]
                }),
            )
        },
    )
    .collect()
}

fn default_letterhead_template() -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    serde_json::Value,
) {
    (
        "administration.organization-letterhead",
        "Organization Letterhead",
        "ترويسة المؤسسة",
        "organization",
        "CERTIFICATE",
        "A4",
        "portrait",
        json!({
            "version": "2.0",
            "meta": {
                "name": "Organization Letterhead",
                "nameAr": "ترويسة المؤسسة",
                "documentType": "CERTIFICATE",
                "entityType": "organization",
                "locale": "ar",
                "pageSetup": {
                    "size": "A4",
                    "orientation": "portrait",
                    "margins": { "top": 12, "bottom": 15, "left": 15, "right": 15 },
                    "unit": "mm"
                }
            },
            "variables": {},
            "assets": {},
            "pages": [{
                "type": "Page",
                "id": "p1",
                "children": [
                    {
                        "type": "Text",
                        "id": "header-right-identity",
                        "props": {
                            "content": "{{$org.country_name}}\n{{$org.ministry_name}}\n{{$org.region_name}}\n{{$org.name_ar | default:$org.name}}",
                            "fontSize": 9,
                            "fontFamily": "Noto Sans Arabic",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "center",
                            "direction": "rtl",
                            "lineHeight": 1.45
                        },
                        "position": { "x": 132, "y": 11, "width": 64, "height": 26 }
                    },
                    {
                        "type": "Text",
                        "id": "header-bismillah",
                        "props": {
                            "content": "بسم الله الرحمن الرحيم",
                            "fontSize": 8.5,
                            "fontFamily": "Noto Sans Arabic",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "center",
                            "direction": "rtl"
                        },
                        "position": { "x": 76, "y": 9, "width": 58, "height": 5 }
                    },
                    {
                        "type": "Image",
                        "id": "organization-logo",
                        "props": { "src": "{{$org.logo_url}}", "fit": "contain" },
                        "position": { "x": 94, "y": 16, "width": 22, "height": 20 }
                    },
                    {
                        "type": "Text",
                        "id": "header-left-meta",
                        "props": {
                            "content": "{{$org.name}}\nالتاريخ: {{issueDate}}\nالمرجع: {{referenceNumber}}",
                            "fontSize": 8.5,
                            "fontFamily": "Noto Sans Arabic",
                            "fontWeight": "normal",
                            "color": "#334155",
                            "textAlign": "center",
                            "direction": "rtl",
                            "lineHeight": 1.45
                        },
                        "position": { "x": 14, "y": 12, "width": 62, "height": 24 }
                    },
                    {
                        "type": "Separator",
                        "id": "header-divider",
                        "props": { "direction": "horizontal", "color": "#0f172a", "thickness": 1.1 },
                        "position": { "x": 14, "y": 39, "width": 182, "height": 1 }
                    },
                    {
                        "type": "Separator",
                        "id": "header-divider-thin",
                        "props": { "direction": "horizontal", "color": "#64748b", "thickness": 0.4 },
                        "position": { "x": 14, "y": 40.8, "width": 182, "height": 0.5 }
                    }
                ]
            }]
        }),
    )
}

fn default_yemeni_academic_certificate_template() -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    serde_json::Value,
) {
    (
        "academic.student-certificate-yemeni",
        "Official Academic Student Certificate",
        "الشهادة المدرسية الرسمية المعتمدة (شهادة نجاح ونقل)",
        "student",
        "REPORT",
        "A4",
        "portrait",
        json!({
            "version": "2.0",
            "meta": {
                "name": "Official Academic Student Certificate",
                "nameAr": "الشهادة المدرسية الرسمية المعتمدة (شهادة نجاح ونقل)",
                "documentType": "REPORT",
                "entityType": "student",
                "locale": "ar",
                "pageSetup": {
                    "size": "A4",
                    "orientation": "portrait",
                    "margins": { "top": 8, "bottom": 8, "left": 8, "right": 8 },
                    "unit": "mm"
                }
            },
            "variables": {
                "student.name": { "source": "student.name", "type": "string", "label": "Student Name", "labelAr": "اسم الطالب" },
                "student.className": { "source": "student.class_name", "type": "string", "label": "Class & Section", "labelAr": "الصف والشعبة" },
                "student.studentNumber": { "source": "student.student_number", "type": "string", "label": "Academic ID", "labelAr": "الرقم الأكاديمي" },
                "student.seatNumber": { "source": "student.seat_number", "type": "string", "label": "Seat Number", "labelAr": "رقم الجلوس" },
                "student.nationalId": { "source": "student.national_id", "type": "string", "label": "National ID", "labelAr": "رقم الهوية" },
                "student.nationality": { "source": "student.nationality", "type": "string", "label": "Nationality", "labelAr": "الجنسية" },
                "AcademicCalendar.name": { "source": "academic_calendar.name", "type": "string", "label": "Academic Year", "labelAr": "العام الدراسي" },
                "report.periodName": { "source": "report.period_name", "type": "string", "label": "Term / Period", "labelAr": "فترة التقرير" },
                "summary.totalScore": { "source": "summary.total_score", "type": "number", "label": "Total Score", "labelAr": "المجموع الكلي" },
                "summary.percentage": { "source": "summary.percentage", "type": "string", "label": "Percentage", "labelAr": "النسبة المئوية" },
                "summary.appreciation": { "source": "summary.appreciation", "type": "string", "label": "Appreciation", "labelAr": "التقدير العام" },
                "summary.status": { "source": "summary.status", "type": "string", "label": "Academic Decision", "labelAr": "القرار والنتيجة" },
                "summary.rankInClass": { "source": "summary.rank_in_class", "type": "string", "label": "Rank in Class", "labelAr": "الترتيب على الصف" },
                "report.verificationUrl": { "source": "report.verification_url", "type": "string", "label": "Verification QR URL", "labelAr": "رابط التحقق الرقمي" },
                "report.issuedDate": { "source": "report.issued_date", "type": "string", "label": "Issued Date", "labelAr": "تاريخ التحرير والاعتماد" }
            },
            "assets": {},
            "pages": [{
                "type": "Page",
                "id": "p1",
                "children": [
                    // Ornate Dual Border Frame
                    {
                        "type": "Shape",
                        "id": "cert-outer-border",
                        "props": { "shapeType": "rectangle", "fillColor": "transparent", "strokeColor": "#0f172a", "strokeWidth": 1.5, "borderRadius": 4 },
                        "position": { "x": 6, "y": 6, "width": 198, "height": 285 }
                    },
                    {
                        "type": "Shape",
                        "id": "cert-inner-border",
                        "props": { "shapeType": "rectangle", "fillColor": "transparent", "strokeColor": "#94a3b8", "strokeWidth": 0.5, "borderRadius": 2 },
                        "position": { "x": 8, "y": 8, "width": 194, "height": 281 }
                    },
                    // Institutional Header (Right: Arabic, Center: Logo/Emblem, Left: English)
                    {
                        "type": "Text",
                        "id": "cert-header-right",
                        "props": {
                            "content": "{{$org.country_name}}\n{{$org.ministry_name}}\nمكتب التربية والتعليم: {{$org.region_name}}\n{{$org.name_ar}}",
                            "fontSize": 8.5,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "center",
                            "direction": "rtl",
                            "lineHeight": 1.3
                        },
                        "position": { "x": 128, "y": 12, "width": 68, "height": 24 }
                    },
                    {
                        "type": "Image",
                        "id": "cert-institution-logo",
                        "props": { "src": "{{$org.logo_url}}", "fit": "contain" },
                        "position": { "x": 93, "y": 11, "width": 24, "height": 20 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-bismillah",
                        "props": {
                            "content": "بسم الله الرحمن الرحيم",
                            "fontSize": 8.5,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "center",
                            "direction": "rtl"
                        },
                        "position": { "x": 65, "y": 32, "width": 80, "height": 5 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-header-left",
                        "props": {
                            "content": "{{$org.name}}\nDepartment of Evaluation & Exams\nAcademic Achievement Certificate",
                            "fontSize": 8.0,
                            "fontFamily": "Rubik",
                            "fontWeight": "normal",
                            "color": "#334155",
                            "textAlign": "center",
                            "direction": "ltr",
                            "lineHeight": 1.3
                        },
                        "position": { "x": 14, "y": 12, "width": 68, "height": 24 }
                    },
                    // Certificate Title Banner
                    {
                        "type": "Shape",
                        "id": "cert-title-banner",
                        "props": { "shapeType": "rectangle", "fillColor": "#0f172a", "strokeColor": "#0f172a", "strokeWidth": 0, "borderRadius": 3 },
                        "position": { "x": 42, "y": 39, "width": 126, "height": 9 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-title-text",
                        "props": {
                            "content": "شهادة درجات ونجاح الطالب (نقل عام)",
                            "fontSize": 12.0,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#ffffff",
                            "textAlign": "center",
                            "direction": "rtl"
                        },
                        "position": { "x": 42, "y": 40.5, "width": 126, "height": 6.5 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-academic-period-text",
                        "props": {
                            "content": "للعام الدراسي: {{AcademicCalendar.name}} — {{report.periodName}}",
                            "fontSize": 8.5,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "center",
                            "direction": "rtl"
                        },
                        "position": { "x": 35, "y": 50, "width": 140, "height": 5.5 }
                    },
                    // Student Identity Card Container Box
                    {
                        "type": "Shape",
                        "id": "cert-student-box",
                        "props": { "shapeType": "rectangle", "fillColor": "#f8fafc", "strokeColor": "#cbd5e1", "strokeWidth": 0.8, "borderRadius": 2 },
                        "position": { "x": 14, "y": 57, "width": 182, "height": 22 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-student-info-col1",
                        "props": {
                            "content": "اسم الطالب: {{student.name}}\nالصف والشعبة: {{student.className}}",
                            "fontSize": 8.5,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "start",
                            "direction": "rtl",
                            "lineHeight": 1.6
                        },
                        "position": { "x": 120, "y": 59, "width": 72, "height": 18 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-student-info-col2",
                        "props": {
                            "content": "رقم الجلوس: {{student.seatNumber}}\nالرقم الأكاديمي: {{student.studentNumber}}",
                            "fontSize": 8.5,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "start",
                            "direction": "rtl",
                            "lineHeight": 1.6
                        },
                        "position": { "x": 68, "y": 59, "width": 50, "height": 18 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-student-info-col3",
                        "props": {
                            "content": "الجنسية: {{student.nationality}}\nالهوية: {{student.nationalId}}",
                            "fontSize": 8.5,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "start",
                            "direction": "rtl",
                            "lineHeight": 1.6
                        },
                        "position": { "x": 16, "y": 59, "width": 50, "height": 18 }
                    },
                    // Dynamic Subject Grades Table
                    {
                        "type": "Table",
                        "id": "cert-subjects-table",
                        "props": {
                            "dataSource": "subjects",
                            "columns": [
                                { "field": "seq", "header": "No", "headerAr": "م", "width": "5%", "alignment": "center" },
                                { "field": "subject_name", "header": "Subject", "headerAr": "المادة الدراسية", "width": "25%", "alignment": "right", "total": "label", "totalLabel": "المجموع العام / Total" },
                                { "field": "max_score", "header": "Max", "headerAr": "الكبرى", "width": "8.5%", "alignment": "center", "total": "sum" },
                                { "field": "min_score", "header": "Min", "headerAr": "الصغرى", "width": "8.5%", "alignment": "center", "total": "sum" },
                                { "field": "t1_coursework", "header": "S1 CW", "headerAr": "محصلة ف1", "width": "8.5%", "alignment": "center", "total": "sum" },
                                { "field": "t1_exam", "header": "S1 Ex", "headerAr": "اختبار ف1", "width": "8.5%", "alignment": "center", "total": "sum" },
                                { "field": "t2_coursework", "header": "S2 CW", "headerAr": "محصلة ف2", "width": "8.5%", "alignment": "center", "total": "sum" },
                                { "field": "t2_exam", "header": "S2 Ex", "headerAr": "اختبار ف2", "width": "8.5%", "alignment": "center", "total": "sum" },
                                { "field": "annual_total", "header": "Total", "headerAr": "المجموع", "width": "9%", "alignment": "center", "total": "sum" },
                                { "field": "appreciation", "header": "Grade", "headerAr": "التقدير", "width": "10%", "alignment": "center" }
                            ],
                            "showHeader": true,
                            "showTotals": true,
                            "showGrid": true,
                            "showBorder": true,
                            "direction": "rtl",
                            "style": {
                                "headerBgColor": "#eaf0f5",
                                "headerTextColor": "#0f172a",
                                "alternateRowBgColor": "#f8fafc",
                                "borderColor": "#94a3b8",
                                "borderWidth": 0.6,
                                "headerFontSize": 7.5,
                                "fontSize": 7.5,
                                "rowHeight": 19.0,
                                "padding": 3.5
                            }
                        },
                        "position": { "x": 14, "y": 82, "width": 182, "height": 120 }
                    },
                    // Result Summary & Academic Decision Banner
                    {
                        "type": "Shape",
                        "id": "cert-summary-box",
                        "props": { "shapeType": "rectangle", "fillColor": "#f1f5f9", "strokeColor": "#94a3b8", "strokeWidth": 0.8, "borderRadius": 3 },
                        "position": { "x": 14, "y": 206, "width": 182, "height": 22 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-summary-stats",
                        "props": {
                            "content": "المجموع العام: {{summary.totalScore}}   |   المعدل المئوي: {{summary.percentage}}   |   التقدير العام: {{summary.appreciation}}",
                            "fontSize": 9.0,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "center",
                            "direction": "rtl"
                        },
                        "position": { "x": 16, "y": 208, "width": 178, "height": 6 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-summary-decision",
                        "props": {
                            "content": "القرار الرسمي: تعتبر النتيجة أعلاه معتمدة وقد تقرر أن الطالب [ {{summary.status}} ] والترتيب على الصف: ( {{summary.rankInClass}} )",
                            "fontSize": 8.5,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "center",
                            "direction": "rtl"
                        },
                        "position": { "x": 16, "y": 216, "width": 178, "height": 8 }
                    },
                    // Official Stamp & Signatures
                    {
                        "type": "Text",
                        "id": "cert-committee-sig",
                        "props": { "content": "لجنة الكنترول والرصد\n........................", "fontSize": 8.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 146, "y": 234, "width": 46, "height": 16 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-teacher-sig",
                        "props": { "content": "مربّي الصف\n........................", "fontSize": 8.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 98, "y": 234, "width": 46, "height": 16 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-principal-sig",
                        "props": { "content": "مدير المدرسة\n........................", "fontSize": 8.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 50, "y": 234, "width": 46, "height": 16 }
                    },
                    {
                        "type": "Shape",
                        "id": "cert-stamp-circle",
                        "props": { "shapeType": "circle", "fillColor": "transparent", "strokeColor": "#94a3b8", "strokeWidth": 1.0 },
                        "position": { "x": 16, "y": 232, "width": 24, "height": 24 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-stamp-text",
                        "props": { "content": "ختم المنشأة\nالرسمي", "fontSize": 6.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#94a3b8", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 16, "y": 240, "width": 24, "height": 8 }
                    },
                    // QR Code & Issuance Footer
                    {
                        "type": "QRCode",
                        "id": "cert-verification-qr",
                        "props": { "data": "{{report.verificationUrl}}", "size": 18, "errorCorrection": "M" },
                        "position": { "x": 14, "y": 262, "width": 18, "height": 18 }
                    },
                    {
                        "type": "Text",
                        "id": "cert-footer-text",
                        "props": {
                            "content": "وثيقة رسمية صادرة عبر منظومة Orbis التعليمية — رمز التحقق الرقمي المعتمد\nتاريخ التحرير والاعتماد: {{report.issuedDate}}",
                            "fontSize": 7.0,
                            "fontFamily": "Rubik",
                            "fontWeight": "normal",
                            "color": "#64748b",
                            "textAlign": "start",
                            "direction": "rtl",
                            "lineHeight": 1.4
                        },
                        "position": { "x": 36, "y": 265, "width": 160, "height": 14 }
                    }
                ]
            }]
        }),
    )
}

fn default_exam_hall_register_template() -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    serde_json::Value,
) {
    (
        "examinations.hall-register",
        "Exam Hall Attendance & Proctor Register",
        "كشف مناداة وسجل حضور لجان الاختبارات",
        "exam",
        "EXAM_REGISTER",
        "A4",
        "portrait",
        json!({
            "version": "2.0",
            "meta": {
                "name": "Exam Hall Attendance & Proctor Register",
                "nameAr": "كشف مناداة وسجل حضور لجان الاختبارات",
                "documentType": "EXAM_REGISTER",
                "entityType": "exam",
                "locale": "ar",
                "pageSetup": {
                    "size": "A4",
                    "orientation": "portrait",
                    "margins": { "top": 8, "bottom": 8, "left": 8, "right": 8 },
                    "unit": "mm"
                }
            },
            "variables": {
                "exam.sessionName": { "source": "exam.session_name", "type": "string", "label": "Session Name", "labelAr": "الدورة الاختبارية" },
                "exam.hallName": { "source": "exam.hall_name", "type": "string", "label": "Exam Hall", "labelAr": "قاعة الاختبار" },
                "exam.subjectName": { "source": "exam.subject_name", "type": "string", "label": "Subject", "labelAr": "المادة الدراسية" },
                "exam.examDate": { "source": "exam.exam_date", "type": "string", "label": "Date & Time", "labelAr": "التاريخ والفترة" },
                "AcademicCalendar.name": { "source": "academic_calendar.name", "type": "string", "label": "Academic Year", "labelAr": "العام الدراسي" },
                "report.periodName": { "source": "report.period_name", "type": "string", "label": "Academic Period", "labelAr": "الفصل الدراسي" }
            },
            "assets": {},
            "pages": [{
                "type": "Page",
                "id": "p1",
                "children": [
                    {
                        "type": "Shape",
                        "id": "reg-outer-border",
                        "props": { "shapeType": "rectangle", "fillColor": "transparent", "strokeColor": "#0f172a", "strokeWidth": 1.2, "borderRadius": 2 },
                        "position": { "x": 8, "y": 8, "width": 194, "height": 281 }
                    },
                    {
                        "type": "Shape",
                        "id": "reg-inner-border",
                        "props": { "shapeType": "rectangle", "fillColor": "transparent", "strokeColor": "#94a3b8", "strokeWidth": 0.5, "borderRadius": 1 },
                        "position": { "x": 10, "y": 10, "width": 190, "height": 277 }
                    },
                    {
                        "type": "Text",
                        "id": "reg-org-header",
                        "props": {
                            "content": "{{$org.country_name}}\n{{$org.ministry_name}}\n{{$org.region_name}}\n{{$org.name_ar}}",
                            "fontSize": 8.5,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "start",
                            "direction": "rtl",
                            "lineHeight": 1.4
                        },
                        "position": { "x": 118, "y": 13, "width": 78, "height": 20 }
                    },
                    {
                        "type": "Text",
                        "id": "reg-committee-header",
                        "props": {
                            "content": "لجنة الكنترول والامتحانات العامة\nالعام الدراسي: {{AcademicCalendar.name}}\nالفترة: {{report.periodName}}\nتاريخ الاختبار: {{exam.examDate}}",
                            "fontSize": 8.0,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#334155",
                            "textAlign": "start",
                            "direction": "rtl",
                            "lineHeight": 1.4
                        },
                        "position": { "x": 14, "y": 13, "width": 78, "height": 20 }
                    },
                    {
                        "type": "Shape",
                        "id": "reg-title-bg",
                        "props": { "shapeType": "rectangle", "fillColor": "#0f172a", "strokeColor": "#0f172a", "strokeWidth": 0, "borderRadius": 2 },
                        "position": { "x": 14, "y": 35, "width": 182, "height": 8.5 }
                    },
                    {
                        "type": "Text",
                        "id": "reg-title-text",
                        "props": {
                            "content": "كشف مناداة وسجل حضور وتوقيع لجان الاختبارات | EXAM ATTENDANCE & PROCTOR SHEET",
                            "fontSize": 8.5,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#ffffff",
                            "textAlign": "center",
                            "direction": "rtl"
                        },
                        "position": { "x": 16, "y": 36.5, "width": 178, "height": 5.5 }
                    },
                    {
                        "type": "Shape",
                        "id": "reg-meta-box",
                        "props": { "shapeType": "rectangle", "fillColor": "#f8fafc", "strokeColor": "#cbd5e1", "strokeWidth": 0.6, "borderRadius": 2 },
                        "position": { "x": 14, "y": 46, "width": 182, "height": 13 }
                    },
                    {
                        "type": "Text",
                        "id": "reg-meta-col1",
                        "props": {
                            "content": "الدورة الاختبارية: {{exam.sessionName}}\nقاعة الاختبار: {{exam.hallName}}",
                            "fontSize": 8.0,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "start",
                            "direction": "rtl",
                            "lineHeight": 1.5
                        },
                        "position": { "x": 105, "y": 47.5, "width": 88, "height": 10 }
                    },
                    {
                        "type": "Text",
                        "id": "reg-meta-col2",
                        "props": {
                            "content": "المادة الدراسية: {{exam.subjectName}}\nالفترة / التوقيت: {{exam.examDate}}",
                            "fontSize": 8.0,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "start",
                            "direction": "rtl",
                            "lineHeight": 1.5
                        },
                        "position": { "x": 16, "y": 47.5, "width": 88, "height": 10 }
                    },
                    {
                        "type": "Table",
                        "id": "reg-attendance-table",
                        "props": {
                            "dataSource": "seatingList",
                            "columns": [
                                { "field": "seq", "header": "No", "headerAr": "م", "width": "6%", "alignment": "center" },
                                { "field": "seating_number", "header": "Seat No", "headerAr": "رقم الجلوس", "width": "15%", "alignment": "center" },
                                { "field": "student_name", "header": "Student Full Name", "headerAr": "اسم الطالب الرباعي", "width": "35%", "alignment": "right" },
                                { "field": "class_name", "header": "Class", "headerAr": "الصف / الشعبة", "width": "16%", "alignment": "right" },
                                { "field": "desk_number", "header": "Desk", "headerAr": "المقعد", "width": "10%", "alignment": "center" },
                                { "field": "signature", "header": "Signature", "headerAr": "توقيع الطالب", "width": "18%", "alignment": "center" }
                            ],
                            "showHeader": true,
                            "showGrid": true,
                            "showBorder": true,
                            "direction": "rtl",
                            "style": {
                                "headerBgColor": "#0f172a",
                                "headerTextColor": "#ffffff",
                                "alternateRowBgColor": "#f8fafc",
                                "borderColor": "#cbd5e1",
                                "borderWidth": 0.75,
                                "headerFontSize": 7.5,
                                "fontSize": 7.5,
                                "rowHeight": 17.0
                            }
                        },
                        "position": { "x": 14, "y": 62, "width": 182, "height": 188 }
                    },
                    {
                        "type": "Shape",
                        "id": "reg-footer-box",
                        "props": { "shapeType": "rectangle", "fillColor": "#f1f5f9", "strokeColor": "#cbd5e1", "strokeWidth": 0.6, "borderRadius": 2 },
                        "position": { "x": 14, "y": 253, "width": 182, "height": 28 }
                    },
                    {
                        "type": "Text",
                        "id": "reg-footer-summary",
                        "props": {
                            "content": "إجمالي المقيدين باللجنة: (      )  |  عدد الحاضرين: (      )  |  عدد الغائبين: (      )",
                            "fontSize": 8.0,
                            "fontFamily": "Rubik",
                            "fontWeight": "bold",
                            "color": "#0f172a",
                            "textAlign": "center",
                            "direction": "rtl"
                        },
                        "position": { "x": 18, "y": 255, "width": 174, "height": 5 }
                    },
                    {
                        "type": "Text",
                        "id": "reg-sig-proctor1",
                        "props": { "content": "المراقب الأول:\n___________________", "fontSize": 7.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl", "lineHeight": 1.4 },
                        "position": { "x": 136, "y": 262, "width": 56, "height": 16 }
                    },
                    {
                        "type": "Text",
                        "id": "reg-sig-proctor2",
                        "props": { "content": "المراقب الثاني:\n___________________", "fontSize": 7.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl", "lineHeight": 1.4 },
                        "position": { "x": 76, "y": 262, "width": 56, "height": 16 }
                    },
                    {
                        "type": "Text",
                        "id": "reg-sig-control",
                        "props": { "content": "رئيس الكنترول:\n___________________", "fontSize": 7.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl", "lineHeight": 1.4 },
                        "position": { "x": 16, "y": 262, "width": 56, "height": 16 }
                    }
                ]
            }]
        }),
    )
}

fn default_exam_hall_seating_card_template() -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    serde_json::Value,
) {
    (
        "examinations.hall-seating-card",
        "Exam Hall Seating Card",
        "بطاقة رقم الجلوس ودخول قاعات الاختبار",
        "exam",
        "EXAM_CARD",
        "ID_CARD",
        "landscape",
        json!({
            "version": "2.0",
            "meta": {
                "name": "Exam Hall Seating Card",
                "nameAr": "بطاقة رقم الجلوس ودخول قاعات الاختبار",
                "documentType": "EXAM_CARD",
                "entityType": "exam",
                "locale": "ar",
                "pageSetup": { "size": "ID_CARD", "orientation": "landscape", "margins": { "top": 0, "bottom": 0, "left": 0, "right": 0 }, "unit": "mm" }
            },
            "variables": {
                "student.name": { "source": "student.name", "type": "string", "label": "Student Name", "labelAr": "اسم الطالب" },
                "exam.seatingNumber": { "source": "exam.seating_number", "type": "string", "label": "Seating Number", "labelAr": "رقم الجلوس" },
                "exam.hallName": { "source": "exam.hall_name", "type": "string", "label": "Exam Hall", "labelAr": "قاعة الاختبار" },
                "exam.deskNumber": { "source": "exam.desk_number", "type": "string", "label": "Desk Number", "labelAr": "رقم المقعد" },
                "exam.sessionName": { "source": "exam.session_name", "type": "string", "label": "Session Name", "labelAr": "الدورة الاختبارية" }
            },
            "assets": {},
            "pages": [{
                "type": "Page",
                "id": "p1",
                "children": [
                    {
                        "type": "Shape",
                        "id": "top-accent",
                        "props": { "shapeType": "rectangle", "fillColor": "#1e293b", "strokeColor": "#1e293b", "strokeWidth": 0, "borderRadius": 0 },
                        "position": { "x": 0, "y": 0, "width": 85.6, "height": 1.4 }
                    },
                    {
                        "type": "Image",
                        "id": "org-logo",
                        "props": { "src": "{{$org.logo_url}}", "fit": "contain" },
                        "position": { "x": 73, "y": 3.5, "width": 8, "height": 8 }
                    },
                    {
                        "type": "Text",
                        "id": "exam-header",
                        "props": { "content": "{{$org.name_ar | default:$org.name}}", "fontSize": 8.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                        "position": { "x": 14, "y": 3.8, "width": 57, "height": 4.5 }
                    },
                    {
                        "type": "Text",
                        "id": "exam-sub",
                        "props": { "content": "بطاقة رقم الجلوس — {{exam.sessionName | default:'الاختبارات'}}", "fontSize": 5.3, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "start", "direction": "rtl" },
                        "position": { "x": 14, "y": 8.5, "width": 57, "height": 3.2 }
                    },
                    {
                        "type": "Separator",
                        "id": "seating-header-sep",
                        "props": { "direction": "horizontal", "color": "#e2e8f0", "thickness": 0.25 },
                        "position": { "x": 4.5, "y": 13, "width": 76.6, "height": 0.3 }
                    },
                    {
                        "type": "Shape",
                        "id": "seat-box",
                        "props": { "shapeType": "rectangle", "fillColor": "#f8fafc", "strokeColor": "#cbd5e1", "strokeWidth": 0.3, "borderRadius": 2 },
                        "position": { "x": 5, "y": 16, "width": 23, "height": 22 }
                    },
                    {
                        "type": "Text",
                        "id": "seat-title",
                        "props": { "content": "رقم الجلوس", "fontSize": 5.5, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 6, "y": 18, "width": 21, "height": 3.5 }
                    },
                    {
                        "type": "Text",
                        "id": "seat-val",
                        "props": { "content": "{{exam.seatingNumber | default:'—'}}", "fontSize": 12.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "ltr" },
                        "position": { "x": 6, "y": 23.5, "width": 21, "height": 9 }
                    },
                    {
                        "type": "Text",
                        "id": "exam-student-name",
                        "props": { "content": "{{student.name | default:'—'}}", "fontSize": 8.8, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                        "position": { "x": 31, "y": 16.5, "width": 50, "height": 5 }
                    },
                    {
                        "type": "Text",
                        "id": "exam-hall-info",
                        "props": { "content": "القاعة: {{exam.hallName | default:'—'}}", "fontSize": 6.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                        "position": { "x": 31, "y": 23, "width": 37, "height": 4 }
                    },
                    {
                        "type": "Text",
                        "id": "exam-desk-info",
                        "props": { "content": "المقعد: {{exam.deskNumber | default:'—'}}", "fontSize": 6.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                        "position": { "x": 31, "y": 28, "width": 37, "height": 4 }
                    },
                    {
                        "type": "QRCode",
                        "id": "exam-qr",
                        "props": { "data": "{{exam.seatingNumber}}", "size": 11, "errorCorrection": "M" },
                        "position": { "x": 70, "y": 28, "width": 11, "height": 11 }
                    },
                    {
                        "type": "Separator",
                        "id": "seating-sep",
                        "props": { "direction": "horizontal", "color": "#f1f5f9", "thickness": 0.25 },
                        "position": { "x": 4.5, "y": 45, "width": 76.6, "height": 0.3 }
                    },
                    {
                        "type": "Text",
                        "id": "exam-rules",
                        "props": { "content": "يُرجى إبراز البطاقة عند دخول قاعة الاختبار", "fontSize": 4.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 5, "y": 47, "width": 75.6, "height": 3 }
                    }
                ]
            }]
        }),
    )
}

fn default_user_id_card_template() -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    serde_json::Value,
) {
    (
        "administration.user-id-card",
        "Employee & User ID Badge",
        "بطاقة العمل والتعريف للكادر الوظيفي والمستخدمين",
        "user",
        "ID_CARD",
        "ID_CARD",
        "landscape",
        json!({
            "version": "2.0",
            "meta": {
                "name": "Employee & User ID Badge",
                "nameAr": "بطاقة العمل والتعريف للكادر الوظيفي والمستخدمين",
                "documentType": "ID_CARD",
                "entityType": "user",
                "locale": "ar",
                "pageSetup": { "size": "ID_CARD", "orientation": "landscape", "margins": { "top": 0, "bottom": 0, "left": 0, "right": 0 }, "unit": "mm" }
            },
            "variables": {
                "user.name": { "source": "user.name", "type": "string", "label": "User Name", "labelAr": "اسم المستخدم" },
                "user.jobTitle": { "source": "user.job_title", "type": "string", "label": "Role / Title", "labelAr": "المسمى الوظيفي" },
                "user.employeeNumber": { "source": "user.employee_number", "type": "string", "label": "Employee ID", "labelAr": "الرقم الوظيفي" },
                "user.department": { "source": "user.department", "type": "string", "label": "Department", "labelAr": "القسم / الإدارة" }
            },
            "assets": {},
            "pages": [{
                "type": "Page",
                "id": "p1",
                "children": [
                    {
                        "type": "Shape",
                        "id": "top-accent",
                        "props": { "shapeType": "rectangle", "fillColor": "#312e81", "strokeColor": "#312e81", "strokeWidth": 0, "borderRadius": 0 },
                        "position": { "x": 0, "y": 0, "width": 85.6, "height": 1.4 }
                    },
                    {
                        "type": "Image",
                        "id": "org-logo",
                        "props": { "src": "{{$org.logo_url}}", "fit": "contain" },
                        "position": { "x": 73, "y": 3.5, "width": 8, "height": 8 }
                    },
                    {
                        "type": "Text",
                        "id": "org-title",
                        "props": { "content": "{{$org.name_ar}}", "fontSize": 8.2, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                        "position": { "x": 14, "y": 3.8, "width": 57, "height": 4.5 }
                    },
                    {
                        "type": "Text",
                        "id": "badge-subtitle",
                        "props": { "content": "بطاقة تعريف الكادر الوظيفي | STAFF ID BADGE", "fontSize": 5.2, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "start", "direction": "rtl" },
                        "position": { "x": 14, "y": 8.5, "width": 57, "height": 3.2 }
                    },
                    {
                        "type": "Separator",
                        "id": "header-sep",
                        "props": { "direction": "horizontal", "color": "#e2e8f0", "thickness": 0.25 },
                        "position": { "x": 4.5, "y": 13, "width": 76.6, "height": 0.3 }
                    },
                    {
                        "type": "Image",
                        "id": "user-photo",
                        "props": { "src": "{{user.avatarUrl}}", "fit": "cover" },
                        "position": { "x": 5, "y": 16, "width": 21, "height": 25 }
                    },
                    {
                        "type": "Shape",
                        "id": "photo-border",
                        "props": { "shapeType": "rectangle", "fillColor": "transparent", "strokeColor": "#e2e8f0", "strokeWidth": 0.3, "borderRadius": 1.5 },
                        "position": { "x": 5, "y": 16, "width": 21, "height": 25 }
                    },
                    {
                        "type": "Text",
                        "id": "user-name",
                        "props": { "content": "{{user.name}}", "fontSize": 9.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                        "position": { "x": 29, "y": 16, "width": 52, "height": 5 }
                    },
                    {
                        "type": "Text",
                        "id": "user-title",
                        "props": { "content": "{{user.jobTitle}}", "fontSize": 7.2, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#312e81", "textAlign": "start", "direction": "rtl" },
                        "position": { "x": 29, "y": 21.8, "width": 40, "height": 4 }
                    },
                    {
                        "type": "Text",
                        "id": "user-dept",
                        "props": { "content": "القسم: {{user.department}}", "fontSize": 6.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                        "position": { "x": 29, "y": 27, "width": 40, "height": 4 }
                    },
                    {
                        "type": "Text",
                        "id": "user-no",
                        "props": { "content": "الرقم الوظيفي: {{user.employeeNumber}}", "fontSize": 6.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#475569", "textAlign": "start", "direction": "rtl" },
                        "position": { "x": 29, "y": 32, "width": 40, "height": 4 }
                    },
                    {
                        "type": "QRCode",
                        "id": "user-qr",
                        "props": { "data": "{{user.employeeNumber}}", "size": 11, "errorCorrection": "M" },
                        "position": { "x": 70, "y": 28, "width": 11, "height": 11 }
                    },
                    {
                        "type": "Separator",
                        "id": "footer-sep",
                        "props": { "direction": "horizontal", "color": "#f1f5f9", "thickness": 0.25 },
                        "position": { "x": 4.5, "y": 45, "width": 76.6, "height": 0.3 }
                    },
                    {
                        "type": "Text",
                        "id": "footer-lbl",
                        "props": { "content": "بطاقة تعريف رسمية معتمدة  |  OFFICIAL AUTHORIZED BADGE", "fontSize": 4.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 5, "y": 47, "width": 75.6, "height": 3 }
                    }
                ]
            }]
        }),
    )
}

fn default_class_schedule_template() -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    serde_json::Value,
) {
    (
        "academic.class-schedule",
        "Weekly Class Timetable Schedule",
        "جدول الحصص الأسبوعي الدراسي المعتمد",
        "schedule",
        "SCHEDULE",
        "A4",
        "landscape",
        json!({
            "version": "2.0",
            "meta": {
                "name": "Weekly Class Timetable Schedule",
                "nameAr": "جدول الحصص الأسبوعي الدراسي المعتمد",
                "documentType": "SCHEDULE",
                "entityType": "schedule",
                "locale": "ar",
                "pageSetup": {
                    "size": "A4",
                    "orientation": "landscape",
                    "margins": { "top": 8, "bottom": 8, "left": 8, "right": 8 },
                    "unit": "mm"
                }
            },
            "variables": {
                "schedule.title": { "source": "schedule.title", "type": "string", "label": "Schedule Title", "labelAr": "عنوان الجدول" },
                "schedule.periodName": { "source": "schedule.period_name", "type": "string", "label": "Academic Period", "labelAr": "الفصل الدراسي" },
                "schedule.date": { "source": "schedule.date", "type": "string", "label": "Date", "labelAr": "تاريخ الطباعة" }
            },
            "assets": {},
            "pages": [{
                "type": "Page",
                "id": "p1",
                "children": [
                    {
                        "type": "Shape",
                        "id": "sched-border",
                        "props": { "shapeType": "rectangle", "fillColor": "#ffffff", "strokeColor": "#0f766e", "strokeWidth": 0.8, "borderRadius": 3 },
                        "position": { "x": 6, "y": 6, "width": 285, "height": 198 }
                    },
                    {
                        "type": "Shape",
                        "id": "sched-header",
                        "props": { "shapeType": "rectangle", "fillColor": "#0f766e", "strokeColor": "#0f766e", "strokeWidth": 0, "borderRadius": 2 },
                        "position": { "x": 8, "y": 8, "width": 281, "height": 24 }
                    },
                    {
                        "type": "Text",
                        "id": "sched-org",
                        "props": { "content": "{{$org.name_ar}}\n{{$org.name}}", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#ffffff", "textAlign": "start", "direction": "rtl", "lineHeight": 1.3 },
                        "position": { "x": 190, "y": 10, "width": 95, "height": 18 }
                    },
                    {
                        "type": "Text",
                        "id": "sched-title",
                        "props": { "content": "جدول الحصص الدراسية الأسبوعي\n{{schedule.title}}", "fontSize": 12.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#ffffff", "textAlign": "center", "direction": "rtl", "lineHeight": 1.3 },
                        "position": { "x": 75, "y": 9, "width": 137, "height": 18 }
                    },
                    {
                        "type": "Text",
                        "id": "sched-meta",
                        "props": { "content": "الفصل: {{schedule.periodName}}\nالتاريخ: {{schedule.date}}", "fontSize": 8.0, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#ccfbf1", "textAlign": "start", "direction": "rtl", "lineHeight": 1.3 },
                        "position": { "x": 12, "y": 10, "width": 60, "height": 18 }
                    },
                    {
                        "type": "Table",
                        "id": "sched-matrix",
                        "props": {
                            "dataSource": "matrix",
                            "columns": [
                                { "header": "الحصة / التوقيت", "headerAr": "الحصة / التوقيت", "field": "slotName", "width": "16%", "alignment": "center" },
                                { "header": "السبت", "headerAr": "السبت", "field": "day_6", "width": "14%", "alignment": "center" },
                                { "header": "الأحد", "headerAr": "الأحد", "field": "day_0", "width": "14%", "alignment": "center" },
                                { "header": "الإثنين", "headerAr": "الإثنين", "field": "day_1", "width": "14%", "alignment": "center" },
                                { "header": "الثلاثاء", "headerAr": "الثلاثاء", "field": "day_2", "width": "14%", "alignment": "center" },
                                { "header": "الأربعاء", "headerAr": "الأربعاء", "field": "day_3", "width": "14%", "alignment": "center" },
                                { "header": "الخميس", "headerAr": "الخميس", "field": "day_4", "width": "14%", "alignment": "center" }
                            ],
                            "showHeader": true,
                            "showTotals": false,
                            "showGrid": true,
                            "showBorder": true,
                            "direction": "rtl",
                            "style": {
                                "headerBgColor": "#0f766e",
                                "headerTextColor": "#ffffff",
                                "alternateRowBgColor": "#f0fdfa",
                                "borderColor": "#cbd5e1",
                                "borderWidth": 0.6,
                                "headerFontSize": 8.5,
                                "fontSize": 8.0,
                                "rowHeight": 18.0
                            }
                        },
                        "position": { "x": 8, "y": 36, "width": 281, "height": 142 }
                    },
                    {
                        "type": "Text",
                        "id": "sched-sig-1",
                        "props": { "content": "وكيل الشؤون التعليمية\n...................................", "fontSize": 8.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 190, "y": 184, "width": 80, "height": 14 }
                    },
                    {
                        "type": "Text",
                        "id": "sched-sig-2",
                        "props": { "content": "المشرف الأكاديمي\n...................................", "fontSize": 8.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 105, "y": 184, "width": 80, "height": 14 }
                    },
                    {
                        "type": "Text",
                        "id": "sched-sig-3",
                        "props": { "content": "مدير المدرسة / الاعتماد\n...................................", "fontSize": 8.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 20, "y": 184, "width": 80, "height": 14 }
                    }
                ]
            }]
        }),
    )
}

fn default_account_statement_template() -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    serde_json::Value,
) {
    (
        "finance.account-statement",
        "Official General Ledger Account Statement",
        "كشف حساب مالي رسمي معتمد",
        "statement",
        "STATEMENT",
        "A4",
        "portrait",
        json!({
            "version": "2.0",
            "meta": {
                "name": "Official General Ledger Account Statement",
                "nameAr": "كشف حساب مالي رسمي معتمد",
                "documentType": "STATEMENT",
                "entityType": "statement",
                "locale": "ar",
                "pageSetup": {
                    "size": "A4",
                    "orientation": "portrait",
                    "margins": { "top": 8, "bottom": 8, "left": 8, "right": 8 },
                    "unit": "mm"
                }
            },
            "variables": {
                "account.code": { "source": "account.code", "type": "string", "label": "Account Code", "labelAr": "رقم الحساب" },
                "account.nameAr": { "source": "account.name_ar", "type": "string", "label": "Account Name", "labelAr": "اسم الحساب" },
                "account.currency": { "source": "account.currency", "type": "string", "label": "Currency", "labelAr": "العملة" },
                "summary.closingBalanceFormatted": { "source": "summary.closing_balance_formatted", "type": "string", "label": "Closing Balance", "labelAr": "الرصيد النهائي" }
            },
            "assets": {},
            "pages": [{
                "type": "Page",
                "id": "p1",
                "children": [
                    {
                        "type": "Shape",
                        "id": "stmt-border",
                        "props": { "shapeType": "rectangle", "fillColor": "#ffffff", "strokeColor": "#0f2942", "strokeWidth": 0.8, "borderRadius": 3 },
                        "position": { "x": 6, "y": 6, "width": 198, "height": 285 }
                    },
                    {
                        "type": "Shape",
                        "id": "stmt-header",
                        "props": { "shapeType": "rectangle", "fillColor": "#0f2942", "strokeColor": "#0f2942", "strokeWidth": 0, "borderRadius": 2 },
                        "position": { "x": 8, "y": 8, "width": 194, "height": 26 }
                    },
                    {
                        "type": "Text",
                        "id": "stmt-title",
                        "props": { "content": "{{$org.name_ar}}\nكشف حساب الأستاذ العام المعتمد | STATEMENT OF ACCOUNT", "fontSize": 10.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#ffffff", "textAlign": "center", "direction": "rtl", "lineHeight": 1.3 },
                        "position": { "x": 10, "y": 9, "width": 190, "height": 13 }
                    },
                    {
                        "type": "Text",
                        "id": "stmt-details",
                        "props": { "content": "الحساب: {{account.code}} - {{account.nameAr}} | العملة: {{account.currency}} | الفترة: {{period.startDateFormatted}} - {{period.endDateFormatted}}", "fontSize": 7.5, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#94a3b8", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 10, "y": 24, "width": 190, "height": 8 }
                    },
                    {
                        "type": "Shape",
                        "id": "box-deb",
                        "props": { "shapeType": "rectangle", "fillColor": "#f0fdf4", "strokeColor": "#16a34a", "strokeWidth": 0.5, "borderRadius": 2 },
                        "position": { "x": 138, "y": 37, "width": 64, "height": 13 }
                    },
                    {
                        "type": "Text",
                        "id": "txt-deb",
                        "props": { "content": "إجمالي المدين: {{summary.totalDebitFormatted}}", "fontSize": 7.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#166534", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 138, "y": 41, "width": 64, "height": 6 }
                    },
                    {
                        "type": "Shape",
                        "id": "box-crd",
                        "props": { "shapeType": "rectangle", "fillColor": "#fff1f2", "strokeColor": "#e11d48", "strokeWidth": 0.5, "borderRadius": 2 },
                        "position": { "x": 73, "y": 37, "width": 62, "height": 13 }
                    },
                    {
                        "type": "Text",
                        "id": "txt-crd",
                        "props": { "content": "إجمالي الدائن: {{summary.totalCreditFormatted}}", "fontSize": 7.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#9f1239", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 73, "y": 41, "width": 62, "height": 6 }
                    },
                    {
                        "type": "Shape",
                        "id": "box-bal",
                        "props": { "shapeType": "rectangle", "fillColor": "#f8fafc", "strokeColor": "#0f2942", "strokeWidth": 0.5, "borderRadius": 2 },
                        "position": { "x": 8, "y": 37, "width": 62, "height": 13 }
                    },
                    {
                        "type": "Text",
                        "id": "txt-bal",
                        "props": { "content": "الرصيد: {{summary.closingBalanceFormatted}}", "fontSize": 7.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f2942", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 8, "y": 41, "width": 62, "height": 6 }
                    },
                    {
                        "type": "Table",
                        "id": "stmt-table",
                        "props": {
                            "dataSource": "rows",
                            "columns": [
                                { "header": "#", "headerAr": "#", "field": "index", "width": "6%", "alignment": "center" },
                                { "header": "التاريخ", "headerAr": "التاريخ", "field": "date", "width": "15%", "alignment": "center" },
                                { "header": "رقم المرجع", "headerAr": "رقم المرجع", "field": "ref", "width": "16%", "alignment": "center" },
                                { "header": "البيان والتفاصيل", "headerAr": "البيان والتفاصيل", "field": "description", "width": "33%", "alignment": "right" },
                                { "header": "مدين", "headerAr": "مدين", "field": "debitFormatted", "width": "10%", "alignment": "center" },
                                { "header": "دائن", "headerAr": "دائن", "field": "creditFormatted", "width": "10%", "alignment": "center" },
                                { "header": "الرصيد", "headerAr": "الرصيد", "field": "balanceFormatted", "width": "10%", "alignment": "center" }
                            ],
                            "showHeader": true,
                            "showTotals": false,
                            "showGrid": true,
                            "showBorder": true,
                            "direction": "rtl",
                            "style": {
                                "headerBgColor": "#0f2942",
                                "headerTextColor": "#ffffff",
                                "alternateRowBgColor": "#f8fafc",
                                "borderColor": "#cbd5e1",
                                "borderWidth": 0.6,
                                "headerFontSize": 7.5,
                                "fontSize": 7.0,
                                "rowHeight": 15.0
                            }
                        },
                        "position": { "x": 8, "y": 53, "width": 194, "height": 208 }
                    },
                    {
                        "type": "Text",
                        "id": "stmt-sig-1",
                        "props": { "content": "المحاسب المالي\n...............................", "fontSize": 8.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 135, "y": 268, "width": 60, "height": 14 }
                    },
                    {
                        "type": "Text",
                        "id": "stmt-sig-2",
                        "props": { "content": "المراجعة والتدقيق\n...............................", "fontSize": 8.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 75, "y": 268, "width": 60, "height": 14 }
                    },
                    {
                        "type": "Text",
                        "id": "stmt-sig-3",
                        "props": { "content": "المدير المالي والختم\n...............................", "fontSize": 8.0, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                        "position": { "x": 12, "y": 268, "width": 60, "height": 14 }
                    }
                ]
            }]
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::get_master_template_definitions;

    #[test]
    fn provisions_a5_grade_certificates_and_an_organization_letterhead() {
        let templates = get_master_template_definitions();
        let primary = templates
            .iter()
            .find(|template| template.0 == "academic.primary-grade-certificate")
            .expect("primary certificate template");
        let general = templates
            .iter()
            .find(|template| template.0 == "academic.general-grade-certificate")
            .expect("general certificate template");
        let letterhead = templates
            .iter()
            .find(|template| template.0 == "administration.organization-letterhead")
            .expect("default letterhead template");

        for certificate in [primary, general] {
            assert_eq!(certificate.5, "A5");
            assert_eq!(
                certificate.7["meta"]["pageSetup"]["orientation"],
                "landscape"
            );
            let children = certificate.7["pages"][0]["children"]
                .as_array()
                .expect("template nodes");
            assert!(children.iter().any(|node| {
                node["type"] == "Image"
                    && node["props"]["src"]
                        .as_str()
                        .is_some_and(|src| src.starts_with("data:image/png;base64,"))
            }));
            assert!(children.iter().any(|node| {
                node["type"] == "Text"
                    && node["props"]["content"]
                        .as_str()
                        .is_some_and(|content| content.contains("{{$org.ministry_name}}"))
            }));
        }

        assert_eq!(letterhead.5, "A4");
        assert_eq!(letterhead.3, "organization");
    }

    #[test]
    fn default_catalog_does_not_embed_country_or_ministry_names() {
        let templates = get_master_template_definitions();
        for template in templates {
            let serialized = template.7.to_string();
            assert!(!serialized.contains("المملكة العربية السعودية"));
            assert!(!serialized.contains("وزارة التعليم"));
        }
    }

    #[test]
    fn payment_receipt_uses_organization_currency_and_spaced_layout() {
        let templates = get_master_template_definitions();
        let receipt = templates
            .iter()
            .find(|template| template.0 == "finance.payment-receipt")
            .expect("payment receipt template");
        let serialized = receipt.7.to_string();

        assert!(serialized.contains("default:$org.currency"));
        assert_eq!(receipt.5, "A5");
        assert!(!serialized.contains("ريال سعودي"));
        assert!(!serialized.contains("SAR"));
        assert!(serialized.contains("amount-background"));
        assert!(serialized.contains("header-divider"));
    }

    #[test]
    fn finance_voucher_defaults_are_distinct_and_use_matching_entities() {
        let templates = get_master_template_definitions();
        for (code, entity_type) in [
            ("finance.disbursement-voucher", "finance_disbursement"),
            ("finance.journal-voucher", "finance_journal"),
        ] {
            let template = templates
                .iter()
                .find(|template| template.0 == code)
                .expect("default finance voucher template");
            assert_eq!(template.3, entity_type);
            assert_eq!(template.4, "VOUCHER");
            assert_eq!(template.5, "A5");
            assert_eq!(template.7["meta"]["entityType"], entity_type);
            assert_eq!(template.7["meta"]["documentType"], "VOUCHER");
            assert!(template.7["variables"]["voucher.number"].is_object());
            assert!(template.7["variables"]["voucher.primaryName"].is_object());
        }
    }

    #[test]
    fn provisions_yemeni_academic_certificate_master_template() {
        let templates = get_master_template_definitions();
        let cert = templates
            .iter()
            .find(|template| template.0 == "academic.student-certificate-yemeni")
            .expect("yemeni certificate master template");

        assert_eq!(cert.3, "student");
        assert_eq!(cert.4, "REPORT");
        assert_eq!(cert.5, "A4");
        assert_eq!(cert.6, "portrait");

        let children = cert.7["pages"][0]["children"]
            .as_array()
            .expect("template nodes");

        // Asserts presence of Table node bound to subjects with totals
        let table_node = children
            .iter()
            .find(|n| n["type"] == "Table")
            .expect("Table node in certificate");
        assert_eq!(table_node["props"]["dataSource"], "subjects");
        assert_eq!(table_node["props"]["showTotals"], true);

        // Asserts presence of QR code for digital verification
        assert!(children.iter().any(|n| n["type"] == "QRCode"));

        // Asserts dynamic multi-tenant org expression
        assert!(children.iter().any(|n| {
            n["type"] == "Text"
                && n["props"]["content"]
                    .as_str()
                    .is_some_and(|c| c.contains("{{$org.ministry_name}}"))
        }));
    }

    #[test]
    fn provisions_exam_templates_with_distinct_entities_and_document_types() {
        let templates = get_master_template_definitions();
        let register = templates
            .iter()
            .find(|t| t.0 == "examinations.hall-register")
            .expect("exam hall register template");
        assert_eq!(register.3, "exam");
        assert_eq!(register.4, "EXAM_REGISTER");
        assert_eq!(register.5, "A4");
        assert_eq!(register.6, "portrait");

        let reg_children = register.7["pages"][0]["children"]
            .as_array()
            .expect("register nodes");
        let reg_table = reg_children
            .iter()
            .find(|n| n["type"] == "Table")
            .expect("attendance table in register");
        assert_eq!(reg_table["props"]["dataSource"], "seatingList");

        let seating_card = templates
            .iter()
            .find(|t| t.0 == "examinations.hall-seating-card")
            .expect("exam hall seating card template");
        assert_eq!(seating_card.3, "exam");
        assert_eq!(seating_card.4, "EXAM_CARD");
        assert_eq!(seating_card.5, "ID_CARD");
        assert_eq!(seating_card.6, "landscape");

        let user_card = templates
            .iter()
            .find(|t| t.0 == "administration.user-id-card")
            .expect("user id card template");
        assert_eq!(user_card.3, "user");
        assert_eq!(user_card.4, "ID_CARD");
        assert_eq!(user_card.5, "ID_CARD");
    }

    #[test]
    fn yemeni_certificate_is_authoritative_first_report_template() {
        let templates = get_master_template_definitions();
        let first_report = templates
            .iter()
            .find(|t| t.3 == "student" && t.4 == "REPORT")
            .expect("first student REPORT template");
        assert_eq!(
            first_report.0, "academic.student-certificate-yemeni",
            "Official Yemeni academic certificate must be the first (REPORT, student) template to receive is_default = 1"
        );
    }
}
