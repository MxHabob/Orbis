use orbis_domain::errors::DomainError;
use sea_orm::{ConnectionTrait, Statement};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

pub async fn seed_master_templates<C: ConnectionTrait>(
    db: &C,
    org_id: Uuid,
    admin_id: Uuid,
    now: &str,
) -> Result<(), DomainError> {
    info!("Seeding master visual AST document templates for org_id={}", org_id);

    let templates = get_master_template_definitions();

    for (code, name_en, name_ar, entity, r_type, page_sz, page_or, doc_json) in templates {
        let tid = Uuid::new_v4();
        let doc_str = doc_json.to_string();

        db.execute(Statement::from_sql_and_values(
            db.get_database_backend(),
            "INSERT INTO report_templates (id, org_id, code, name, name_ar, description, description_ar, document_type, entity_type, master_template_id, status, version, published_version, published_at, published_by, page_size, orientation, margins_json, document_json, locale, tags, is_system, is_default, created_by, deleted_at, created_at, updated_at) \
             SELECT $1, $2, $3, $4, $5, $4, $5, $6, $7, NULL, 'published', 1, 1, $8, $9, $10, $11, '{\"top\":10,\"bottom\":10,\"left\":10,\"right\":10}', $12, 'ar', '[]', 1, 1, $9, NULL, $8, $8 \
             WHERE NOT EXISTS (SELECT 1 FROM report_templates WHERE org_id = $2 AND code = $3)",
            vec![
                tid.into(), org_id.into(), code.into(),
                name_en.into(), name_ar.into(),
                r_type.into(), entity.into(),
                now.to_string().into(), admin_id.into(),
                page_sz.into(), page_or.into(),
                doc_str.into(),
            ]
        )).await.ok();
    }

    Ok(())
}

fn get_master_template_definitions() -> Vec<(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, serde_json::Value)> {
    vec![
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
                        // Background Card Base
                        {
                            "type": "Shape",
                            "id": "bg-base",
                            "props": { "shape": "rectangle", "fillColor": "#ffffff", "strokeColor": "#e2e8f0", "strokeWidth": 0.5, "borderRadius": 4 },
                            "position": { "x": 1, "y": 1, "width": 83.6, "height": 52 }
                        },
                        // Header Banner Shape
                        {
                            "type": "Shape",
                            "id": "header-banner",
                            "props": { "shape": "rectangle", "fillColor": "#0f172a", "strokeColor": "#0f172a", "strokeWidth": 0, "borderRadius": 3 },
                            "position": { "x": 1.5, "y": 1.5, "width": 82.6, "height": 11 }
                        },
                        // Institution Name in Header
                        {
                            "type": "Text",
                            "id": "org-name-ar",
                            "props": { "content": "{{$org.name_ar}}", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#ffffff", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 15, "y": 2.5, "width": 55, "height": 4.5 }
                        },
                        {
                            "type": "Text",
                            "id": "card-title-sub",
                            "props": { "content": "بطاقة هوية طالب منتظم | STUDENT IDENTITY CARD", "fontSize": 5.5, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#94a3b8", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 10, "y": 7.5, "width": 65, "height": 3.5 }
                        },
                        // Student Photo Frame
                        {
                            "type": "Image",
                            "id": "student-photo",
                            "props": { "src": "{{student.avatarUrl}}", "fit": "cover" },
                            "position": { "x": 4, "y": 14, "width": 22, "height": 26 }
                        },
                        {
                            "type": "Shape",
                            "id": "photo-border",
                            "props": { "shape": "rectangle", "fillColor": "transparent", "strokeColor": "#cbd5e1", "strokeWidth": 0.5, "borderRadius": 2 },
                            "position": { "x": 4, "y": 14, "width": 22, "height": 26 }
                        },
                        // Student Name
                        {
                            "type": "Text",
                            "id": "student-name",
                            "props": { "content": "{{student.name}}", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 28, "y": 14, "width": 42, "height": 5 }
                        },
                        // Student Number / ID
                        {
                            "type": "Text",
                            "id": "student-num-lbl",
                            "props": { "content": "الرقم الأكاديمي: {{student.studentNumber}}", "fontSize": 7, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 28, "y": 20, "width": 42, "height": 4 }
                        },
                        // Grade / Class
                        {
                            "type": "Text",
                            "id": "student-class-lbl",
                            "props": { "content": "الصف / الشعبة: {{student.className}}", "fontSize": 7, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 28, "y": 24.5, "width": 42, "height": 4 }
                        },
                        // National ID
                        {
                            "type": "Text",
                            "id": "student-nat-lbl",
                            "props": { "content": "رقم الهوية: {{student.nationalId}}", "fontSize": 7, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 28, "y": 29, "width": 42, "height": 4 }
                        },
                        // QR Code Verification
                        {
                            "type": "QRCode",
                            "id": "student-qr",
                            "props": { "data": "{{student.studentNumber}}", "size": 11 },
                            "position": { "x": 71, "y": 14, "width": 11, "height": 11 }
                        },
                        // Bottom Barcode & Expiry
                        {
                            "type": "Barcode",
                            "id": "student-barcode",
                            "props": { "data": "{{student.studentNumber}}", "format": "CODE128" },
                            "position": { "x": 28, "y": 34.5, "width": 53, "height": 9.5 }
                        },
                        // Card Footer Accent
                        {
                            "type": "Shape",
                            "id": "footer-accent",
                            "props": { "shape": "rectangle", "fillColor": "#2563eb", "strokeColor": "#2563eb", "strokeWidth": 0, "borderRadius": 1 },
                            "position": { "x": 1.5, "y": 48.5, "width": 82.6, "height": 4.5 }
                        },
                        {
                            "type": "Text",
                            "id": "footer-text",
                            "props": { "content": "صالحة للعام الدراسي الحالي | VALID FOR CURRENT ACADEMIC YEAR", "fontSize": 4.8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#ffffff", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 5, "y": 49.5, "width": 75, "height": 3 }
                        }
                    ]
                }]
            })
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
                            "id": "bg-card",
                            "props": { "shape": "rectangle", "fillColor": "#ffffff", "strokeColor": "#cbd5e1", "strokeWidth": 0.5, "borderRadius": 4 },
                            "position": { "x": 1, "y": 1, "width": 83.6, "height": 52 }
                        },
                        {
                            "type": "Shape",
                            "id": "header-bg",
                            "props": { "shape": "rectangle", "fillColor": "#1e1b4b", "strokeColor": "#1e1b4b", "strokeWidth": 0, "borderRadius": 3 },
                            "position": { "x": 1.5, "y": 1.5, "width": 82.6, "height": 11 }
                        },
                        {
                            "type": "Text",
                            "id": "org-title",
                            "props": { "content": "{{$org.name_ar}}", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#ffffff", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 15, "y": 2.5, "width": 55, "height": 4.5 }
                        },
                        {
                            "type": "Text",
                            "id": "badge-subtitle",
                            "props": { "content": "بطاقة تعريف كادر تعليمي وإداري | FACULTY & STAFF BADGE", "fontSize": 5.5, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#c7d2fe", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 10, "y": 7.5, "width": 65, "height": 3.5 }
                        },
                        {
                            "type": "Image",
                            "id": "employee-photo",
                            "props": { "src": "{{employee.avatarUrl}}", "fit": "cover" },
                            "position": { "x": 4, "y": 14, "width": 22, "height": 26 }
                        },
                        {
                            "type": "Text",
                            "id": "emp-name",
                            "props": { "content": "{{employee.name}}", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 28, "y": 14, "width": 42, "height": 5 }
                        },
                        {
                            "type": "Text",
                            "id": "emp-title",
                            "props": { "content": "المسمى: {{employee.jobTitle}}", "fontSize": 7.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#4338ca", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 28, "y": 19.5, "width": 42, "height": 4 }
                        },
                        {
                            "type": "Text",
                            "id": "emp-dept",
                            "props": { "content": "القسم: {{employee.department}}", "fontSize": 7, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#475569", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 28, "y": 24, "width": 42, "height": 4 }
                        },
                        {
                            "type": "Text",
                            "id": "emp-no",
                            "props": { "content": "الرقم الوظيفي: {{employee.employeeNumber}}", "fontSize": 7, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#475569", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 28, "y": 28.5, "width": 42, "height": 4 }
                        },
                        {
                            "type": "QRCode",
                            "id": "emp-qr",
                            "props": { "data": "{{employee.employeeNumber}}", "size": 11 },
                            "position": { "x": 71, "y": 14, "width": 11, "height": 11 }
                        },
                        {
                            "type": "Barcode",
                            "id": "emp-barcode",
                            "props": { "data": "{{employee.employeeNumber}}", "format": "CODE128" },
                            "position": { "x": 28, "y": 34, "width": 53, "height": 10 }
                        },
                        {
                            "type": "Shape",
                            "id": "footer-strip",
                            "props": { "shape": "rectangle", "fillColor": "#4338ca", "strokeColor": "#4338ca", "strokeWidth": 0, "borderRadius": 1 },
                            "position": { "x": 1.5, "y": 48.5, "width": 82.6, "height": 4.5 }
                        },
                        {
                            "type": "Text",
                            "id": "footer-lbl",
                            "props": { "content": "بطاقة رسمية معتمدة | OFFICIAL AUTHORIZED BADGE", "fontSize": 5, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#ffffff", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 5, "y": 49.5, "width": 75, "height": 3 }
                        }
                    ]
                }]
            })
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
                    "academicYear.name": { "source": "academic_year.name", "type": "string", "label": "Academic Year", "labelAr": "العام الدراسي" }
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
                            "props": { "shape": "rectangle", "fillColor": "transparent", "strokeColor": "#0f172a", "strokeWidth": 1.5, "borderRadius": 4 },
                            "position": { "x": 10, "y": 10, "width": 190, "height": 277 }
                        },
                        {
                            "type": "Shape",
                            "id": "inner-border",
                            "props": { "shape": "rectangle", "fillColor": "transparent", "strokeColor": "#d97706", "strokeWidth": 0.5, "borderRadius": 2 },
                            "position": { "x": 13, "y": 13, "width": 184, "height": 271 }
                        },
                        // Header Institutional Details
                        {
                            "type": "Text",
                            "id": "ministry-header",
                            "props": { "content": "المملكة العربية السعودية\nوزارة التعليم\n{{$org.name_ar}}", "fontSize": 10, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl", "lineHeight": 1.5 },
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
                                "content": "تشهد إدارة {{$org.name_ar}} بأن الطالب/ة:\n\nالاسم الكامل: {{student.name}}\nرقم الهوية الوطنية / الإقامة: {{student.nationalId}}\nالرقم الأكاديمي: {{student.studentNumber}}\n\nمقيد/ة رسمياً ومنتظم/ة بالدراسة لدينا للعام الدراسي {{academicYear.name}}، في المرحلة الدراسية بالصف: {{student.className}}.\n\nوقد مُنحت له هذه الإفادة بناءً على طلبه لتقديمها إلى من يهمه الأمر، دون أي مسؤولية على المنظمة تجاه حقوق الغير.",
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
                            "props": { "shape": "circle", "fillColor": "transparent", "strokeColor": "#94a3b8", "strokeWidth": 1 },
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
                            "props": { "data": "https://verify.orbis.edu/cert/{{student.studentNumber}}", "size": 18 },
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
            })
        ),

        // ── 4. Student Term Report Card (A4 Portrait 210mm x 297mm) ───────────
        (
            "academic.student-term-report-card",
            "Student Term Report Card",
            "كشف درجات الطالب للفصل الدراسي",
            "student",
            "REPORT",
            "A4",
            "portrait",
            json!({
                "version": "2.0",
                "meta": {
                    "name": "Student Term Report Card",
                    "nameAr": "كشف درجات الطالب للفصل الدراسي",
                    "documentType": "REPORT",
                    "entityType": "student",
                    "locale": "ar",
                    "pageSetup": { "size": "A4", "orientation": "portrait", "margins": { "top": 12, "bottom": 12, "left": 12, "right": 12 }, "unit": "mm" }
                },
                "variables": {
                    "student.name": { "source": "student.name", "type": "string", "label": "Student Name", "labelAr": "اسم الطالب" },
                    "student.studentNumber": { "source": "student.student_number", "type": "string", "label": "Student Number", "labelAr": "الرقم الأكاديمي" },
                    "student.className": { "source": "student.class_name", "type": "string", "label": "Class", "labelAr": "الصف والشعبة" },
                    "period.name": { "source": "period.name", "type": "string", "label": "Academic Period", "labelAr": "الفصل الدراسي" },
                    "academicYear.name": { "source": "academic_year.name", "type": "string", "label": "Academic Year", "labelAr": "العام الدراسي" }
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
                            "props": { "shape": "rectangle", "fillColor": "#f8fafc", "strokeColor": "#e2e8f0", "strokeWidth": 1, "borderRadius": 3 },
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
                            "props": { "content": "العام الدراسي: {{academicYear.name}} | {{period.name}}", "fontSize": 8.5, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#475569", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 50, "y": 30, "width": 110, "height": 5 }
                        },
                        // Student Info Summary Grid
                        {
                            "type": "Shape",
                            "id": "info-box",
                            "props": { "shape": "rectangle", "fillColor": "#ffffff", "strokeColor": "#cbd5e1", "strokeWidth": 0.5, "borderRadius": 2 },
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
                            "props": { "shape": "rectangle", "fillColor": "#f1f5f9", "strokeColor": "#cbd5e1", "strokeWidth": 0.5, "borderRadius": 2 },
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
            })
        ),

        // ── 5. Payment Receipt Voucher (A4 Portrait 210mm x 297mm) ────────────
        (
            "finance.payment-receipt",
            "Official Electronic Payment Receipt",
            "سند قبض مالي إلكتروني رسمي مزود بـ QR",
            "finance",
            "RECEIPT",
            "A4",
            "portrait",
            json!({
                "version": "2.0",
                "meta": {
                    "name": "Official Electronic Payment Receipt",
                    "nameAr": "سند قبض مالي إلكتروني رسمي مزود بـ QR",
                    "documentType": "RECEIPT",
                    "entityType": "finance",
                    "locale": "ar",
                    "pageSetup": { "size": "A4", "orientation": "portrait", "margins": { "top": 12, "bottom": 12, "left": 12, "right": 12 }, "unit": "mm" }
                },
                "variables": {
                    "receipt.number": { "source": "receipt.number", "type": "string", "label": "Receipt No", "labelAr": "رقم السند" },
                    "receipt.date": { "source": "receipt.date", "type": "string", "label": "Date", "labelAr": "التاريخ" },
                    "receipt.payerName": { "source": "receipt.payer_name", "type": "string", "label": "Payer Name", "labelAr": "استلمنا من السيد/ة" },
                    "receipt.studentName": { "source": "receipt.student_name", "type": "string", "label": "Student Name", "labelAr": "اسم الطالب" },
                    "receipt.amount": { "source": "receipt.amount", "type": "number", "label": "Amount", "labelAr": "المبلغ رقماً" },
                    "receipt.amountWords": { "source": "receipt.amount_words", "type": "string", "label": "Amount in Words", "labelAr": "المبلغ كتابة" }
                },
                "assets": {},
                "pages": [{
                    "type": "Page",
                    "id": "p1",
                    "children": [
                        // Border Frame
                        {
                            "type": "Shape",
                            "id": "rcpt-border",
                            "props": { "shape": "rectangle", "fillColor": "transparent", "strokeColor": "#0f172a", "strokeWidth": 1.2, "borderRadius": 3 },
                            "position": { "x": 10, "y": 10, "width": 190, "height": 135 }
                        },
                        // Header
                        {
                            "type": "Text",
                            "id": "rcpt-org-title",
                            "props": { "content": "{{$org.name_ar}}\nالرقم الضريبي: {{$org.tax_number}}", "fontSize": 10, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl", "lineHeight": 1.4 },
                            "position": { "x": 115, "y": 14, "width": 80, "height": 12 }
                        },
                        {
                            "type": "Text",
                            "id": "rcpt-title",
                            "props": { "content": "سـنـد قـبـض مـالـي\nOFFICIAL PAYMENT RECEIPT", "fontSize": 14, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl", "lineHeight": 1.3 },
                            "position": { "x": 60, "y": 14, "width": 90, "height": 14 }
                        },
                        {
                            "type": "Text",
                            "id": "rcpt-meta",
                            "props": { "content": "رقم السند: {{receipt.number}}\nالتاريخ: {{receipt.date}}", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#2563eb", "textAlign": "end", "direction": "rtl", "lineHeight": 1.4 },
                            "position": { "x": 15, "y": 14, "width": 55, "height": 12 }
                        },
                        // Divider
                        {
                            "type": "Separator",
                            "id": "rcpt-sep-1",
                            "props": { "direction": "horizontal", "color": "#0f172a", "thickness": 0.8 },
                            "position": { "x": 10, "y": 30, "width": 190, "height": 1 }
                        },
                        // Body Fields
                        {
                            "type": "Text",
                            "id": "rcpt-body",
                            "props": {
                                "content": "استلمنا من السيد / السيدة: {{receipt.payerName}}\nعن الطالب / الطالبة: {{receipt.studentName}}\nمبلغاً وقدره: {{receipt.amount}} ريال سعودي\nفقط وقدره: {{receipt.amountWords}}\nوذلك مقابل: {{receipt.description}}",
                                "fontSize": 10.5,
                                "fontFamily": "Rubik",
                                "fontWeight": "normal",
                                "color": "#1e293b",
                                "textAlign": "start",
                                "direction": "rtl",
                                "lineHeight": 2.0
                            },
                            "position": { "x": 55, "y": 34, "width": 140, "height": 65 }
                        },
                        // QR Code (ZATCA compliant)
                        {
                            "type": "QRCode",
                            "id": "rcpt-qr",
                            "props": { "data": "{{receipt.number}}", "size": 28 },
                            "position": { "x": 18, "y": 42, "width": 28, "height": 28 }
                        },
                        {
                            "type": "Text",
                            "id": "qr-lbl",
                            "props": { "content": "رمز التحقق الإلكتروني", "fontSize": 7, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#64748b", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 15, "y": 72, "width": 34, "height": 4 }
                        },
                        // Payment Mode & Signatures
                        {
                            "type": "Separator",
                            "id": "rcpt-sep-2",
                            "props": { "direction": "horizontal", "color": "#e2e8f0", "thickness": 0.5 },
                            "position": { "x": 10, "y": 105, "width": 190, "height": 1 }
                        },
                        {
                            "type": "Text",
                            "id": "rcpt-method",
                            "props": { "content": "طريقة الدفع: {{receipt.paymentMethod}} | رقم المرجع: {{receipt.referenceNumber}}", "fontSize": 8.5, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#475569", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 110, "y": 110, "width": 85, "height": 5 }
                        },
                        {
                            "type": "Text",
                            "id": "sig-cashier",
                            "props": { "content": "أمين الصندوق / المحاسب: {{receipt.cashierName}}\nالتوقيع: ___________________", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl", "lineHeight": 1.5 },
                            "position": { "x": 15, "y": 110, "width": 85, "height": 15 }
                        }
                    ]
                }]
            })
        ),

        // ── 6. Student Exam Admission Card (A5 Portrait 148mm x 210mm) ───────
        (
            "examinations.student-exam-card",
            "Student Exam Admission & Seating Card",
            "بطاقة دخول الاختبار ورقم الجلوس المعتمد",
            "student",
            "EXAM_CARD",
            "A5",
            "portrait",
            json!({
                "version": "2.0",
                "meta": {
                    "name": "Student Exam Admission & Seating Card",
                    "nameAr": "بطاقة دخول الاختبار ورقم الجلوس المعتمد",
                    "documentType": "EXAM_CARD",
                    "entityType": "student",
                    "locale": "ar",
                    "pageSetup": { "size": "A5", "orientation": "portrait", "margins": { "top": 8, "bottom": 8, "left": 8, "right": 8 }, "unit": "mm" }
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
                            "id": "exam-card-bg",
                            "props": { "shape": "rectangle", "fillColor": "#ffffff", "strokeColor": "#0f172a", "strokeWidth": 1, "borderRadius": 3 },
                            "position": { "x": 6, "y": 6, "width": 136, "height": 198 }
                        },
                        {
                            "type": "Text",
                            "id": "exam-header",
                            "props": { "content": "{{$org.name_ar}}\nلجنة الكنترول والامتحانات العامة", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl", "lineHeight": 1.4 },
                            "position": { "x": 20, "y": 10, "width": 108, "height": 10 }
                        },
                        {
                            "type": "Text",
                            "id": "exam-sub",
                            "props": { "content": "بطاقة دخول لجان الاختبارات ورقم الجلوس", "fontSize": 11, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#dc2626", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 10, "y": 22, "width": 128, "height": 6 }
                        },
                        // Seating Number Box
                        {
                            "type": "Shape",
                            "id": "seat-box",
                            "props": { "shape": "rectangle", "fillColor": "#fee2e2", "strokeColor": "#dc2626", "strokeWidth": 0.8, "borderRadius": 2 },
                            "position": { "x": 12, "y": 30, "width": 124, "height": 18 }
                        },
                        {
                            "type": "Text",
                            "id": "seat-title",
                            "props": { "content": "رقم الجلوس الرسمي", "fontSize": 8, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#991b1b", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 15, "y": 32, "width": 118, "height": 4 }
                        },
                        {
                            "type": "Text",
                            "id": "seat-val",
                            "props": { "content": "{{exam.seatingNumber}}", "fontSize": 16, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#991b1b", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 15, "y": 37, "width": 118, "height": 9 }
                        },
                        // Details
                        {
                            "type": "Text",
                            "id": "exam-student-name",
                            "props": { "content": "اسم الطالب: {{student.name}}", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 12, "y": 52, "width": 124, "height": 5 }
                        },
                        {
                            "type": "Text",
                            "id": "exam-hall-info",
                            "props": { "content": "مقر اللجنة / القاعة: {{exam.hallName}} | المقعد: {{exam.deskNumber}}", "fontSize": 9, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#334155", "textAlign": "start", "direction": "rtl" },
                            "position": { "x": 12, "y": 58, "width": 124, "height": 5 }
                        },
                        // Timetable
                        {
                            "type": "Table",
                            "id": "exam-schedule-table",
                            "props": {
                                "dataSource": "examSchedules",
                                "columns": [
                                    { "header": "اليوم والتاريخ", "headerAr": "اليوم والتاريخ", "field": "examDate", "width": "35%" },
                                    { "header": "المادة", "headerAr": "المادة", "field": "subjectName", "width": "40%" },
                                    { "header": "الفترة", "headerAr": "الفترة", "field": "timeSlot", "width": "25%" }
                                ],
                                "showHeader": true,
                                "showGrid": true,
                                "direction": "rtl"
                            },
                            "position": { "x": 12, "y": 66, "width": 124, "height": 85 }
                        },
                        // Barcode & Warning
                        {
                            "type": "Barcode",
                            "id": "exam-barcode",
                            "props": { "data": "{{exam.seatingNumber}}", "format": "CODE128" },
                            "position": { "x": 25, "y": 155, "width": 98, "height": 14 }
                        },
                        {
                            "type": "Text",
                            "id": "exam-rules",
                            "props": { "content": "تنبيه: يُمنع اصطحاب الهواتف الذكية أو المذكرات داخل لجان الامتحانات، ويجب إبراز هذه البطاقة عند الدخول.", "fontSize": 7, "fontFamily": "Rubik", "fontWeight": "normal", "color": "#b91c1c", "textAlign": "center", "direction": "rtl", "lineHeight": 1.3 },
                            "position": { "x": 12, "y": 174, "width": 124, "height": 10 }
                        },
                        {
                            "type": "Text",
                            "id": "sig-control",
                            "props": { "content": "رئيس لجنة الكنترول\n_______________", "fontSize": 8.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "center", "direction": "rtl" },
                            "position": { "x": 40, "y": 186, "width": 68, "height": 12 }
                        }
                    ]
                }]
            })
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
                            "props": { "shape": "rectangle", "fillColor": "transparent", "strokeColor": "#0f172a", "strokeWidth": 1.2, "borderRadius": 2 },
                            "position": { "x": 10, "y": 10, "width": 190, "height": 277 }
                        },
                        {
                            "type": "Text",
                            "id": "transcript-header",
                            "props": { "content": "المملكة العربية السعودية\nوزارة التعليم\n{{$org.name_ar}}\nعمادة القبول والتسجيل", "fontSize": 9.5, "fontFamily": "Rubik", "fontWeight": "bold", "color": "#0f172a", "textAlign": "start", "direction": "rtl", "lineHeight": 1.4 },
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
                            "props": { "data": "{{student.studentNumber}}", "size": 16 },
                            "position": { "x": 18, "y": 15, "width": 16, "height": 16 }
                        },
                        // Student Info Bar
                        {
                            "type": "Shape",
                            "id": "st-info-bar",
                            "props": { "shape": "rectangle", "fillColor": "#f8fafc", "strokeColor": "#cbd5e1", "strokeWidth": 0.5, "borderRadius": 1 },
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
                                    { "header": "رمز المقرر", "headerAr": "رمز المقرر", "field": "courseCode", "width": "15%" },
                                    { "header": "اسم المقرر الدراسي", "headerAr": "اسم المقرر الدراسي", "field": "courseName", "width": "40%" },
                                    { "header": "الساعات", "headerAr": "الساعات", "field": "creditHours", "width": "15%" },
                                    { "header": "الدرجة", "headerAr": "الدرجة", "field": "gradeScore", "width": "15%" },
                                    { "header": "الرمز", "headerAr": "الرمز", "field": "gradeLetter", "width": "15%" }
                                ],
                                "showHeader": true,
                                "showGrid": true,
                                "direction": "rtl"
                            },
                            "position": { "x": 12, "y": 60, "width": 186, "height": 175 }
                        },
                        // Summary & Attestation
                        {
                            "type": "Shape",
                            "id": "t-summary-box",
                            "props": { "shape": "rectangle", "fillColor": "#f1f5f9", "strokeColor": "#cbd5e1", "strokeWidth": 0.5, "borderRadius": 1 },
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
            })
        )
    ]
}
