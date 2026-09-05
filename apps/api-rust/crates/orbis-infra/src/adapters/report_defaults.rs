//! Default Built-in Templates for Orbis Reporting Engine.
//! These templates are seeded into the database on organization creation
//! and are rendered using Tera.

pub const MASTER_TEMPLATE_HTML: &str = r##"
<!DOCTYPE html>
<html dir="{% if is_rtl %}rtl{% else %}ltr{% endif %}" lang="{% if is_rtl %}ar{% else %}en{% endif %}">
<head>
<meta charset="UTF-8">
<title>{{ report_name }}</title>
<style>
@page { size: A4; margin: 15mm; }
body { font-family: 'Noto Sans Arabic', 'Segoe UI', sans-serif; margin: 0; direction: {% if is_rtl %}rtl{% else %}ltr{% endif %}; }
.master-header { display: flex; justify-content: space-between; align-items: center; border-bottom: 2px solid {{ primary_color | default(value="#6366f1") }}; padding-bottom: 10px; margin-bottom: 20px; }
.master-header .org-info h1 { margin: 0; font-size: 18px; color: {{ primary_color | default(value="#6366f1") }}; }
.master-header .org-info h2 { margin: 5px 0 0 0; font-size: 14px; color: #555; }
.master-header .report-title { text-align: center; font-size: 20px; font-weight: bold; }
.master-header .meta { font-size: 10px; color: #888; text-align: end; }
.master-footer { margin-top: 30px; padding-top: 10px; border-top: 1px solid #ddd; font-size: 10px; color: #888; text-align: center; display: flex; justify-content: space-between; }
.content-area { min-height: 500px; }
/* Common Table Styles */
table { width: 100%; border-collapse: collapse; font-size: 11px; margin-bottom: 20px; }
th { background: {{ primary_color | default(value="#6366f1") }}; color: white; padding: 8px; text-align: start; font-weight: 600; border: 1px solid #ddd; }
td { padding: 6px 8px; border: 1px solid #e0e0e0; }
tr:nth-child(even) { background: #f9fafb; }
.params-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; background: #f3f4f6; padding: 10px; border-radius: 6px; margin-bottom: 20px; font-size: 11px; }
.param-item strong { color: #4b5563; }
</style>
</head>
<body>
    <div class="master-header">
        <div class="org-info">
            <h1>{% if is_rtl %}{{ org_name_ar }}{% else %}{{ org_name }}{% endif %}</h1>
            <h2>{% if is_rtl %}{{ org_type_ar }}{% else %}{{ org_type }}{% endif %}</h2>
        </div>
        <div class="report-title">
            {% if is_rtl %}{{ report_name_ar }}{% else %}{{ report_name }}{% endif %}
        </div>
        <div class="meta">
            <div>{{ generated_at }}</div>
            <div>{{ printed_by }}</div>
        </div>
    </div>
    
    {% if has_parameters %}
    <div class="params-grid">
        {% for key, val in parameters %}
        <div class="param-item"><strong>{{ key }}:</strong> {{ val }}</div>
        {% endfor %}
    </div>
    {% endif %}

    <div class="content-area">
        {{ inner_content | safe }}
    </div>

    <div class="master-footer">
        <div>{{ org_website | default(value="www.orbis.system") }}</div>
        <div>{% if is_rtl %}نظام أوربس التعليمي{% else %}Orbis Educational System{% endif %}</div>
        <div>{{ current_page | default(value="1") }} / {{ total_pages | default(value="1") }}</div>
    </div>
</body>
</html>
"##;

pub const TABULAR_REPORT_HTML: &str = r##"
<table>
    <thead>
        <tr>
            {% for col in columns %}
                {% if col.visible %}
                    <th>{% if is_rtl %}{{ col.header_ar }}{% else %}{{ col.header }}{% endif %}</th>
                {% endif %}
            {% endfor %}
        </tr>
    </thead>
    <tbody>
        {% for row in rows %}
        <tr>
            {% for col in columns %}
                {% if col.visible %}
                    <td>
                        {{ row[col.key] | default(value="") }}
                    </td>
                {% endif %}
            {% endfor %}
        </tr>
        {% endfor %}
    </tbody>
</table>
"##;

pub const PAYMENT_RECEIPT_HTML: &str = r##"
<style>
.receipt-box { border: 2px dashed #ccc; padding: 20px; border-radius: 12px; max-width: 600px; margin: 0 auto; background: #fff; }
.r-header { text-align: center; margin-bottom: 20px; }
.r-header h2 { color: {{ primary_color | default(value="#10b981") }}; margin: 0; font-size: 24px; }
.r-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin-bottom: 20px; font-size: 13px; }
.r-item { border-bottom: 1px solid #eee; padding-bottom: 5px; }
.r-item strong { color: #555; display: block; font-size: 11px; margin-bottom: 2px; }
.r-amount { text-align: center; background: #f0fdf4; color: #166534; padding: 15px; border-radius: 8px; font-size: 24px; font-weight: bold; margin-bottom: 20px; }
.r-footer { text-align: center; font-size: 11px; color: #888; margin-top: 20px; border-top: 1px dashed #ccc; padding-top: 15px; }
</style>
<div class="receipt-box">
    <div class="r-header">
        <h2>{% if is_rtl %}سند قبض{% else %}Payment Receipt{% endif %}</h2>
        <div>#{% if rows and rows | length > 0 %}{{ rows[0].receipt_number | default(value="N/A") }}{% endif %}</div>
    </div>
    
    <div class="r-amount">
        {% if rows and rows | length > 0 %}{{ rows[0].amount | default(value="0.00") }}{% endif %} 
        {% if rows and rows | length > 0 %}{{ rows[0].currency | default(value="SAR") }}{% endif %}
    </div>

    <div class="r-grid">
        <div class="r-item"><strong>{% if is_rtl %}التاريخ{% else %}Date{% endif %}</strong> {% if rows and rows | length > 0 %}{{ rows[0].date | default(value=generated_at) }}{% endif %}</div>
        <div class="r-item"><strong>{% if is_rtl %}طريقة الدفع{% else %}Payment Method{% endif %}</strong> {% if rows and rows | length > 0 %}{{ rows[0].payment_method | default(value="Cash") }}{% endif %}</div>
        <div class="r-item"><strong>{% if is_rtl %}استلمنا من السيد/ة{% else %}Received From{% endif %}</strong> {% if rows and rows | length > 0 %}{{ rows[0].student_name | default(value="Student Name") }}{% endif %}</div>
        <div class="r-item"><strong>{% if is_rtl %}الرقم الأكاديمي{% else %}Student ID{% endif %}</strong> {% if rows and rows | length > 0 %}{{ rows[0].student_id | default(value="---") }}{% endif %}</div>
    </div>

    <div class="r-item" style="grid-column: 1 / -1;">
        <strong>{% if is_rtl %}وذلك عن{% else %}For (Description){% endif %}</strong> 
        {% if rows and rows | length > 0 %}{{ rows[0].description | default(value="Tuition Fees") }}{% endif %}
    </div>

    <div class="r-footer">
        {% if is_rtl %}المستلم{% else %}Received By{% endif %}: {{ printed_by }}<br><br>
        <em>{% if is_rtl %}هذا السند إلكتروني ولا يحتاج لتوقيع{% else %}This is an electronic receipt and does not require a signature{% endif %}</em>
    </div>
</div>
"##;

pub const STUDENT_ID_CARD_HTML: &str = r##"
<style>
.id-card { width: 85.6mm; height: 53.98mm; border: 1px solid #ccc; border-radius: 8px; overflow: hidden; font-family: 'Noto Sans Arabic', sans-serif; position: relative; background: #fff; box-shadow: 0 4px 6px rgba(0,0,0,0.1); margin: 10px; display: inline-block; }
.id-header { background: {{ primary_color | default(value="#2563eb") }}; color: white; padding: 5px; text-align: center; font-size: 10px; }
.id-body { padding: 10px; display: flex; gap: 10px; }
.id-photo { width: 60px; height: 75px; background: #eee; border: 1px solid #ccc; display: flex; align-items: center; justify-content: center; font-size: 10px; color: #999; }
.id-details { flex: 1; font-size: 9px; line-height: 1.4; }
.id-details strong { color: {{ primary_color | default(value="#2563eb") }}; }
.id-footer { position: absolute; bottom: 0; width: 100%; text-align: center; font-size: 8px; background: #f8f9fa; border-top: 1px solid #eee; padding: 3px 0; }
.batch-container { display: flex; flex-wrap: wrap; justify-content: center; }
</style>
<div class="batch-container">
{% for row in rows %}
<div class="id-card">
    <div class="id-header">
        <strong>{% if is_rtl %}{{ org_name_ar }}{% else %}{{ org_name }}{% endif %}</strong><br>
        {% if is_rtl %}بطاقة طالب{% else %}Student ID Card{% endif %}
    </div>
    <div class="id-body">
        <div class="id-photo">{% if is_rtl %}صورة{% else %}Photo{% endif %}</div>
        <div class="id-details">
            <strong>{% if is_rtl %}الاسم:{% else %}Name:{% endif %}</strong> {{ row.student_name | default(value="---") }}<br>
            <strong>{% if is_rtl %}الرقم:{% else %}ID:{% endif %}</strong> {{ row.student_id | default(value="---") }}<br>
            <strong>{% if is_rtl %}الصف:{% else %}Grade:{% endif %}</strong> {{ row.grade | default(value="---") }}<br>
            <strong>{% if is_rtl %}تاريخ الميلاد:{% else %}DOB:{% endif %}</strong> {{ row.dob | default(value="---") }}
        </div>
    </div>
    <div class="id-footer">
        {% if is_rtl %}هذه البطاقة ملك للمدرسة، الرجاء إعادتها عند العثور عليها.{% else %}Property of the school. Please return if found.{% endif %}
    </div>
</div>
{% endfor %}
</div>
"##;

pub const CLASS_TIMETABLE_HTML: &str = r##"
<style>
.timetable { width: 100%; border-collapse: collapse; text-align: center; margin-top: 10px; }
.timetable th, .timetable td { border: 1px solid #ccc; padding: 8px; }
.timetable th { background: {{ primary_color | default(value="#4f46e5") }}; color: white; font-size: 12px; }
.timetable td { font-size: 11px; height: 50px; vertical-align: middle; }
.subject { font-weight: bold; color: #333; }
.teacher { font-size: 9px; color: #666; margin-top: 4px; }
.period-time { font-size: 9px; color: #eed; display: block; }
</style>
<h3>{% if is_rtl %}الجدول الدراسي - {% else %}Class Timetable - {% endif %} {{ parameters.class_name | default(value="---") }}</h3>
<table class="timetable">
    <thead>
        <tr>
            <th>{% if is_rtl %}اليوم \ الحصة{% else %}Day \ Period{% endif %}</th>
            {% for period in parameters.periods | default(value=["1","2","3","4","5","6","7"]) %}
            <th>
                {% if is_rtl %}الحصة{% else %}Period{% endif %} {{ period }}
            </th>
            {% endfor %}
        </tr>
    </thead>
    <tbody>
        {% for day in parameters.days | default(value=["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday"]) %}
        <tr>
            <td style="background: #f8f9fa; font-weight: bold;">
                {{ day }}
            </td>
            {% for period in parameters.periods | default(value=["1","2","3","4","5","6","7"]) %}
            <td>
                <!-- We simulate finding a matching row for (day, period) from the dataset rows -->
                {% set found = false %}
                {% for row in rows %}
                    {% if row.day == day and row.period == period %}
                        <div class="subject">{{ row.subject | default(value="---") }}</div>
                        <div class="teacher">{{ row.teacher | default(value="---") }}</div>
                        {% set_global found = true %}
                    {% endif %}
                {% endfor %}
                {% if not found %}
                    <span style="color:#ccc">-</span>
                {% endif %}
            </td>
            {% endfor %}
        </tr>
        {% endfor %}
    </tbody>
</table>
"##;

#[cfg(test)]
mod tests {
    use super::*;
    use tera::{Tera, Context};

    #[test]
    fn test_master_template_compiles() {
        let mut tera = Tera::default();
        tera.add_raw_template("master", MASTER_TEMPLATE_HTML).expect("Failed to parse master template");
        
        let mut ctx = Context::new();
        ctx.insert("is_rtl", &true);
        ctx.insert("org_name", &"Test Org");
        ctx.insert("org_name_ar", &"Test Org AR");
        ctx.insert("org_type", &"School");
        ctx.insert("org_type_ar", &"مدرسة");
        ctx.insert("report_name", &"Test Report");
        ctx.insert("report_name_ar", &"Test Report AR");
        ctx.insert("generated_at", &"2023-01-01 10:00");
        ctx.insert("printed_by", &"System");
        ctx.insert("has_parameters", &false);
        ctx.insert("inner_content", &"<p>Inner</p>");
        
        let result = tera.render("master", &ctx).expect("Failed to render master template");
        assert!(result.contains("Test Org AR"));
    }

    #[test]
    fn test_tabular_report_compiles() {
        let mut tera = Tera::default();
        tera.add_raw_template("tabular", TABULAR_REPORT_HTML).expect("Failed to parse tabular template");
        
        let mut ctx = Context::new();
        ctx.insert("is_rtl", &true);
        ctx.insert("columns", &Vec::<String>::new()); // dummy
        ctx.insert("rows", &Vec::<String>::new());
        
        let result = tera.render("tabular", &ctx).expect("Failed to render tabular template");
        assert!(result.contains("<table>"));
    }

    #[test]
    fn test_payment_receipt_compiles() {
        let mut tera = Tera::default();
        tera.add_raw_template("receipt", PAYMENT_RECEIPT_HTML).expect("Failed to parse payment receipt template");
        
        let mut ctx = Context::new();
        ctx.insert("is_rtl", &true);
        ctx.insert("rows", &Vec::<String>::new());
        ctx.insert("printed_by", &"System");
        ctx.insert("generated_at", &"2023-01-01 10:00");
        
        let result = tera.render("receipt", &ctx).expect("Failed to render payment receipt template");
        assert!(result.contains("Payment Receipt") || result.contains("سند قبض"));
    }

    #[test]
    fn test_student_id_card_compiles() {
        let mut tera = Tera::default();
        tera.add_raw_template("id_card", STUDENT_ID_CARD_HTML).expect("Failed to parse id card template");
        
        let mut ctx = Context::new();
        ctx.insert("is_rtl", &true);
        ctx.insert("rows", &Vec::<String>::new());
        
        let result = tera.render("id_card", &ctx).expect("Failed to render id card template");
        assert!(result.contains("batch-container"));
    }

    #[test]
    fn test_class_timetable_compiles() {
        let mut tera = Tera::default();
        tera.add_raw_template("timetable", CLASS_TIMETABLE_HTML).expect("Failed to parse timetable template");
        
        let mut ctx = Context::new();
        ctx.insert("is_rtl", &true);
        ctx.insert("rows", &Vec::<String>::new());
        ctx.insert("parameters", &std::collections::HashMap::<String, String>::new());
        
        let result = tera.render("timetable", &ctx).expect("Failed to render timetable template");
        assert!(result.contains("timetable"));
    }
}
