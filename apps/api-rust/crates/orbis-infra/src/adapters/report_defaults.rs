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
.receipt-box { max-width: 760px; margin: 0 auto; padding: 30px; border: 1px solid #d8e2eb; border-radius: 16px; background: #fff; color: #172033; font-family: 'Noto Sans Arabic', 'Segoe UI', sans-serif; }
.r-header { display: flex; align-items: flex-start; justify-content: space-between; gap: 24px; padding-bottom: 18px; border-bottom: 1px solid #e5ebf1; }
.r-org { color: #475569; font-size: 12px; font-weight: 600; }
.r-title { margin: 4px 0 0; color: {{ primary_color | default(value="#0f766e") }}; font-size: 22px; font-weight: 800; }
.r-number { padding: 8px 12px; border: 1px solid #d8e2eb; border-radius: 8px; color: #334155; font-size: 12px; white-space: nowrap; }
.r-amount { margin: 20px 0; padding: 18px; border: 1px solid #cce9e3; border-radius: 12px; background: #f0fdfa; color: #0f766e; text-align: center; font-size: 26px; font-weight: 800; }
.r-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 12px; }
.r-item { min-width: 0; padding: 12px 14px; border: 1px solid #e5ebf1; border-radius: 9px; overflow-wrap: anywhere; }
.r-item-wide { grid-column: 1 / -1; }
.r-item strong { display: block; margin-bottom: 4px; color: #64748b; font-size: 11px; font-weight: 600; }
.r-footer { display: flex; justify-content: space-between; gap: 16px; margin-top: 24px; padding-top: 16px; border-top: 1px dashed #cbd5e1; color: #64748b; font-size: 11px; }
@media print { .receipt-box { max-width: none; border-radius: 0; box-shadow: none; } }
</style>
<div class="receipt-box" dir="{% if is_rtl %}rtl{% else %}ltr{% endif %}">
    <div class="r-header">
        <div>
            <div class="r-org">{% if is_rtl %}{{ org_name_ar | default(value=org_name | default(value="")) }}{% else %}{{ org_name | default(value="") }}{% endif %}</div>
            <h2 class="r-title">{% if is_rtl %}سند قبض مالي{% else %}Payment Receipt{% endif %}</h2>
        </div>
        <div class="r-number">{% if is_rtl %}رقم السند{% else %}Receipt No.{% endif %} · {% if rows and rows | length > 0 %}{{ rows[0].receipt_number | default(value="—") }}{% endif %}</div>
    </div>

    {% if rows and rows | length > 0 %}
    <div class="r-amount">
        {{ rows[0].amount | default(value="0.00") }}
        {% if rows[0].currency %}<span>{{ rows[0].currency }}</span>{% endif %}
    </div>

    <div class="r-grid">
        <div class="r-item"><strong>{% if is_rtl %}التاريخ{% else %}Date{% endif %}</strong>{{ rows[0].payment_date | default(value=generated_at) }}</div>
        <div class="r-item"><strong>{% if is_rtl %}طريقة الدفع{% else %}Payment Method{% endif %}</strong>{{ rows[0].payment_method | default(value="—") }}</div>
        <div class="r-item"><strong>{% if is_rtl %}استلمنا من{% else %}Received From{% endif %}</strong>{{ rows[0].student_name | default(value="—") }}</div>
        <div class="r-item"><strong>{% if is_rtl %}نوع الرسم{% else %}Fee Type{% endif %}</strong>{{ rows[0].fee_name | default(value="—") }}</div>
        <div class="r-item r-item-wide"><strong>{% if is_rtl %}البيان{% else %}Description{% endif %}</strong>{{ rows[0].notes | default(value="—") }}</div>
    </div>
    {% endif %}

    <div class="r-footer">
        <span>{% if is_rtl %}تم إصدار هذا السند إلكترونياً{% else %}Electronically issued receipt{% endif %}</span>
        <span>{% if is_rtl %}المستلم{% else %}Received By{% endif %}: {{ printed_by }}</span>
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

pub const TRANSCRIPT_REPORT_HTML: &str = r##"
<style>
.transcript-container { font-family: 'Noto Sans Arabic', 'Segoe UI', sans-serif; font-size: 11px; }
.student-meta { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; background: #f8fafc; border: 1px solid #e2e8f0; padding: 12px; border-radius: 8px; margin-bottom: 20px; }
.meta-field { display: flex; flex-direction: column; }
.meta-field span { color: #64748b; font-size: 10px; }
.meta-field strong { color: #0f172a; font-size: 12px; }
.term-section { margin-bottom: 25px; }
.term-header { background: #f1f5f9; padding: 8px 12px; font-weight: bold; font-size: 13px; color: #1e293b; border-left: 4px solid {{ primary_color | default(value="#2563eb") }}; border-radius: 4px; margin-bottom: 8px; display: flex; justify-content: space-between; }
.summary-box { display: flex; justify-content: space-around; background: #f0fdf4; border: 1px solid #bbf7d0; padding: 12px; border-radius: 8px; margin-top: 20px; font-size: 13px; font-weight: bold; color: #166534; }
.seal-box { margin-top: 40px; display: flex; justify-content: space-between; text-align: center; font-size: 11px; }
.signature-line { margin-top: 40px; border-top: 1px dotted #94a3b8; width: 160px; }
</style>
<div class="transcript-container">
    <div class="student-meta">
        <div class="meta-field"><span>{% if is_rtl %}اسم الطالب{% else %}Student Name{% endif %}</span><strong>{{ student_name | default(value="---") }}</strong></div>
        <div class="meta-field"><span>{% if is_rtl %}الرقم الأكاديمي{% else %}Student ID{% endif %}</span><strong>{{ student_id | default(value="---") }}</strong></div>
        <div class="meta-field"><span>{% if is_rtl %}البرنامج / المرحلة{% else %}Program / Stage{% endif %}</span><strong>{{ program | default(value="---") }}</strong></div>
        <div class="meta-field"><span>{% if is_rtl %}تاريخ الالتحاق{% else %}Admission Date{% endif %}</span><strong>{{ admission_date | default(value="---") }}</strong></div>
    </div>

    {% for term in terms | default(value=[]) %}
    <div class="term-section">
        <div class="term-header">
            <span>{{ term.name }}</span>
            <span>{% if is_rtl %}المعدل الفصلي:{% else %}Term GPA:{% endif %} {{ term.gpa | default(value="---") }}</span>
        </div>
        <table>
            <thead>
                <tr>
                    <th>{% if is_rtl %}رمز المقرر{% else %}Course Code{% endif %}</th>
                    <th>{% if is_rtl %}اسم المقرر{% else %}Course Name{% endif %}</th>
                    <th>{% if is_rtl %}الساعات / الوزن{% else %}Credits / Weight{% endif %}</th>
                    <th>{% if is_rtl %}الدرجة{% else %}Score{% endif %}</th>
                    <th>{% if is_rtl %}التقدير{% else %}Grade{% endif %}</th>
                    <th>{% if is_rtl %}الحالة{% else %}Status{% endif %}</th>
                </tr>
            </thead>
            <tbody>
                {% for course in term.courses %}
                <tr>
                    <td>{{ course.code }}</td>
                    <td>{{ course.name }}</td>
                    <td>{{ course.credits }}</td>
                    <td>{{ course.score }}</td>
                    <td><strong>{{ course.grade }}</strong></td>
                    <td>{{ course.status }}</td>
                </tr>
                {% endfor %}
            </tbody>
        </table>
    </div>
    {% endfor %}

    <div class="summary-box">
        <div>{% if is_rtl %}إجمالي الساعات المكتسبة:{% else %}Total Credits Earned:{% endif %} {{ total_credits | default(value="0") }}</div>
        <div>{% if is_rtl %}المعدل التراكمي العام:{% else %}Cumulative GPA:{% endif %} {{ cumulative_gpa | default(value="0.00") }}</div>
        <div>{% if is_rtl %}التقدير العام:{% else %}Overall Standing:{% endif %} {{ overall_standing | default(value="---") }}</div>
    </div>

    <div class="seal-box">
        <div>
            <div>{% if is_rtl %}مسجل شؤون الطلاب{% else %}Registrar Officer{% endif %}</div>
            <div class="signature-line"></div>
        </div>
        <div>
            <div>{% if is_rtl %}عميد / مدير المؤسسة{% else %}Dean / Principal{% endif %}</div>
            <div class="signature-line"></div>
        </div>
        <div>
            <div>{% if is_rtl %}الختم الرسمي للمؤسسة{% else %}Official Seal{% endif %}</div>
            <div style="width: 80px; height: 80px; border: 2px dashed #cbd5e1; border-radius: 50%; margin: 10px auto 0; display: flex; align-items: center; justify-content: center; color: #94a3b8; font-size: 9px;">SEAL</div>
        </div>
    </div>
</div>
"##;

pub const STUDENT_INVOICE_HTML: &str = r##"
<style>
.invoice-box { font-family: 'Noto Sans Arabic', 'Segoe UI', sans-serif; font-size: 11px; }
.inv-header { display: flex; justify-content: space-between; margin-bottom: 20px; }
.inv-details { display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin-bottom: 20px; }
.inv-meta-card { background: #f8fafc; border: 1px solid #e2e8f0; padding: 12px; border-radius: 8px; font-size: 12px; }
.inv-meta-card strong { color: #1e293b; }
.total-table { width: 40%; margin-right: auto; margin-left: 0; }
.total-table td { padding: 6px; }
.total-table .grand-total { font-size: 14px; font-weight: bold; background: #f0fdf4; color: #15803d; }
.payment-status { display: inline-block; padding: 4px 10px; border-radius: 20px; font-size: 11px; font-weight: bold; }
.status-paid { background: #dcfce7; color: #166534; }
.status-partial { background: #fef9c3; color: #854d0e; }
.status-unpaid { background: #fee2e2; color: #991b1b; }
</style>
<div class="invoice-box">
    <div class="inv-details">
        <div class="inv-meta-card">
            <strong>{% if is_rtl %}بيانات الطالب{% else %}Bill To{% endif %}</strong><br><br>
            <div>{% if is_rtl %}الاسم:{% else %}Name:{% endif %} {{ student_name | default(value="---") }}</div>
            <div>{% if is_rtl %}الرقم الأكاديمي:{% else %}ID:{% endif %} {{ student_id | default(value="---") }}</div>
            <div>{% if is_rtl %}الصف / البرنامج:{% else %}Grade / Program:{% endif %} {{ grade_level | default(value="---") }}</div>
            <div>{% if is_rtl %}ولي الأمر:{% else %}Guardian:{% endif %} {{ guardian_name | default(value="---") }}</div>
        </div>
        <div class="inv-meta-card">
            <strong>{% if is_rtl %}تفاصيل الفاتورة{% else %}Invoice Details{% endif %}</strong><br><br>
            <div>{% if is_rtl %}رقم الفاتورة:{% else %}Invoice #:{% endif %} {{ invoice_number | default(value="INV-001") }}</div>
            <div>{% if is_rtl %}تاريخ الإصدار:{% else %}Issue Date:{% endif %} {{ issue_date | default(value="---") }}</div>
            <div>{% if is_rtl %}تاريخ الاستحقاق:{% else %}Due Date:{% endif %} {{ due_date | default(value="---") }}</div>
            <div>{% if is_rtl %}حالة السداد:{% else %}Status:{% endif %} <span class="payment-status status-{{ status_class | default(value='unpaid') }}">{{ payment_status | default(value='غير مسدد') }}</span></div>
        </div>
    </div>

    <table>
        <thead>
            <tr>
                <th>#</th>
                <th>{% if is_rtl %}البند / الرسم{% else %}Item / Fee Description{% endif %}</th>
                <th>{% if is_rtl %}المبلغ الأساسي{% else %}Base Amount{% endif %}</th>
                <th>{% if is_rtl %}الخصم{% else %}Discount{% endif %}</th>
                <th>{% if is_rtl %}الضريبة (15%){% else %}VAT (15%){% endif %}</th>
                <th>{% if is_rtl %}الصافي المستحق{% else %}Net Total{% endif %}</th>
            </tr>
        </thead>
        <tbody>
            {% for item in items | default(value=[]) %}
            <tr>
                <td>{{ loop.index }}</td>
                <td>{{ item.description }}</td>
                <td>{{ item.amount }} {{ currency | default(value="") }}</td>
                <td>{{ item.discount | default(value="0.00") }} {{ currency | default(value="") }}</td>
                <td>{{ item.tax | default(value="0.00") }} {{ currency | default(value="") }}</td>
                <td><strong>{{ item.net_amount }} {{ currency | default(value="") }}</strong></td>
            </tr>
            {% endfor %}
        </tbody>
    </table>

    <table class="total-table">
        <tr>
            <td>{% if is_rtl %}الإجمالي قبل الضريبة:{% else %}Subtotal:{% endif %}</td>
            <td style="text-align: end;">{{ subtotal | default(value="0.00") }} {{ currency | default(value="") }}</td>
        </tr>
        <tr>
            <td>{% if is_rtl %}إجمالي الخصومات:{% else %}Total Discounts:{% endif %}</td>
            <td style="text-align: end; color: #dc2626;">-{{ total_discounts | default(value="0.00") }} {{ currency | default(value="") }}</td>
        </tr>
        <tr>
            <td>{% if is_rtl %}ضريبة القيمة المضافة (15%):{% else %}VAT (15%):{% endif %}</td>
            <td style="text-align: end;">{{ total_vat | default(value="0.00") }} {{ currency | default(value="") }}</td>
        </tr>
        <tr>
            <td>{% if is_rtl %}المسدد سابقاً:{% else %}Paid Amount:{% endif %}</td>
            <td style="text-align: end; color: #16a34a;">{{ paid_amount | default(value="0.00") }} {{ currency | default(value="") }}</td>
        </tr>
        <tr class="grand-total">
            <td>{% if is_rtl %}المتبقي للاستحقاق:{% else %}Balance Due:{% endif %}</td>
            <td style="text-align: end;">{{ balance_due | default(value="0.00") }} {{ currency | default(value="") }}</td>
        </tr>
    </table>
</div>
"##;

pub const ENROLLMENT_CERTIFICATE_HTML: &str = r##"
<style>
.cert-container { font-family: 'Noto Sans Arabic', 'Segoe UI', sans-serif; text-align: center; padding: 30px; border: 3px double {{ primary_color | default(value="#2563eb") }}; border-radius: 12px; background: #fff; }
.cert-title { font-size: 24px; font-weight: bold; color: {{ primary_color | default(value="#2563eb") }}; margin-bottom: 25px; }
.cert-body { font-size: 15px; line-height: 2.2; text-align: justify; margin: 30px auto; max-width: 650px; }
.cert-footer { margin-top: 50px; display: flex; justify-content: space-between; font-size: 12px; }
</style>
<div class="cert-container">
    <div class="cert-title">
        {% if is_rtl %}شهادة قيد دراسي وتأكيد انتظام{% else %}Certificate of Enrollment{% endif %}
    </div>
    
    <div style="font-size: 14px; font-weight: bold; margin-bottom: 20px;">
        {% if is_rtl %}إلى من يهمه الأمر{% else %}To Whom It May Concern{% endif %}
    </div>

    <div class="cert-body">
        {% if is_rtl %}
        تشهد إدارة <strong>{{ org_name_ar | default(value=org_name | default(value="المؤسسة")) }}</strong> بأن الطالب/ة: <strong>{{ student_name | default(value="---") }}</strong>، والرقم الأكاديمي: <strong>{{ student_id | default(value="---") }}</strong>، والجنسية: <strong>{{ nationality | default(value="---") }}</strong>، مقيد/ة لدينا ومنتظم/ة بالدراسة في <strong>{{ grade_level | default(value="المرحلة الدراسية") }}</strong>، للعام الدراسي <strong>{{ academic_year | default(value="العام الحالي") }}</strong>.
        <br>
        وقد أُعطيت له هذه الشهادة بناءً على طلبه لتقديمها إلى الجهات المختصة دون أدنى مسؤولية على المؤسسة فيما يتعلق بالتزاماته الأخرى.
        {% else %}
        This is to certify that student: <strong>{{ student_name | default(value="---") }}</strong>, Student ID: <strong>{{ student_id | default(value="---") }}</strong>, Nationality: <strong>{{ nationality | default(value="---") }}</strong>, is officially enrolled and in good standing at <strong>{{ org_name | default(value="Institution") }}</strong> for the grade/program: <strong>{{ grade_level | default(value="---") }}</strong> during the academic year: <strong>{{ academic_year | default(value="---") }}</strong>.
        <br>
        This certificate is issued upon request to be submitted to the concerned authorities without any liability on the institution.
        {% endif %}
    </div>

    <div class="cert-footer">
        <div>
            <div>{% if is_rtl %}مسجل شؤون الطلاب{% else %}Admissions Officer{% endif %}</div>
            <div style="margin-top: 35px; border-top: 1px dotted #999; width: 140px;"></div>
        </div>
        <div>
            <div>{% if is_rtl %}مدير المؤسسة التعليمية{% else %}Principal / Director{% endif %}</div>
            <div style="margin-top: 35px; border-top: 1px dotted #999; width: 140px;"></div>
        </div>
    </div>
</div>
"##;

pub const STAFF_ID_CARD_HTML: &str = r##"
<style>
.id-card { width: 85.6mm; height: 53.98mm; border: 1px solid #ccc; border-radius: 8px; overflow: hidden; font-family: 'Noto Sans Arabic', sans-serif; position: relative; background: #fff; box-shadow: 0 4px 6px rgba(0,0,0,0.1); margin: 10px; display: inline-block; }
.id-header { background: #1e293b; color: white; padding: 5px; text-align: center; font-size: 10px; }
.id-body { padding: 10px; display: flex; gap: 10px; }
.id-photo { width: 60px; height: 75px; background: #eee; border: 1px solid #ccc; display: flex; align-items: center; justify-content: center; font-size: 10px; color: #999; }
.id-details { flex: 1; font-size: 9px; line-height: 1.4; }
.id-details strong { color: #1e293b; }
.id-footer { position: absolute; bottom: 0; width: 100%; text-align: center; font-size: 8px; background: #f8f9fa; border-top: 1px solid #eee; padding: 3px 0; }
.batch-container { display: flex; flex-wrap: wrap; justify-content: center; }
</style>
<div class="batch-container">
{% for row in rows %}
<div class="id-card">
    <div class="id-header">
        <strong>{% if is_rtl %}{{ org_name_ar | default(value=org_name | default(value="")) }}{% else %}{{ org_name | default(value="") }}{% endif %}</strong><br>
        {% if is_rtl %}بطاقة كادر تعليمي / موظف{% else %}Faculty & Staff ID Card{% endif %}
    </div>
    <div class="id-body">
        <div class="id-photo">{% if is_rtl %}صورة{% else %}Photo{% endif %}</div>
        <div class="id-details">
            <strong>{% if is_rtl %}الاسم:{% else %}Name:{% endif %}</strong> {{ row.staff_name | default(value="---") }}<br>
            <strong>{% if is_rtl %}الرقم الوظيفي:{% else %}Employee ID:{% endif %}</strong> {{ row.employee_id | default(value="---") }}<br>
            <strong>{% if is_rtl %}المسمى:{% else %}Title / Role:{% endif %}</strong> {{ row.job_title | default(value="---") }}<br>
            <strong>{% if is_rtl %}القسم:{% else %}Department:{% endif %}</strong> {{ row.department | default(value="---") }}
        </div>
    </div>
    <div class="id-footer">
        {% if is_rtl %}بطاقة رسمية معتمدة لمنسوبي المؤسسة{% else %}Official Institution Employee Badge{% endif %}
    </div>
</div>
{% endfor %}
</div>
"##;

#[cfg(test)]
mod tests {
    use super::*;
    use tera::{Context, Tera};

    #[test]
    fn test_master_template_compiles() {
        let mut tera = Tera::default();
        tera.add_raw_template("master", MASTER_TEMPLATE_HTML)
            .expect("Failed to parse master template");

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

        let result = tera
            .render("master", &ctx)
            .expect("Failed to render master template");
        assert!(result.contains("Test Org AR"));
    }

    #[test]
    fn test_tabular_report_compiles() {
        let mut tera = Tera::default();
        tera.add_raw_template("tabular", TABULAR_REPORT_HTML)
            .expect("Failed to parse tabular template");

        let mut ctx = Context::new();
        ctx.insert("is_rtl", &true);
        ctx.insert("columns", &Vec::<String>::new()); // dummy
        ctx.insert("rows", &Vec::<String>::new());

        let result = tera
            .render("tabular", &ctx)
            .expect("Failed to render tabular template");
        assert!(result.contains("<table>"));
    }

    #[test]
    fn test_payment_receipt_compiles() {
        let mut tera = Tera::default();
        tera.add_raw_template("receipt", PAYMENT_RECEIPT_HTML)
            .expect("Failed to parse payment receipt template");

        let mut ctx = Context::new();
        ctx.insert("is_rtl", &true);
        ctx.insert(
            "rows",
            &vec![serde_json::json!({
                "receipt_number": "R-2024-01",
                "amount": 125.0,
                "currency": "YER",
                "student_name": "طالب تجريبي",
                "fee_name": "رسوم دراسية",
                "payment_method": "نقداً",
                "payment_date": "2024-01-01",
                "notes": "رسوم الفصل"
            })],
        );
        ctx.insert("printed_by", &"System");
        ctx.insert("generated_at", &"2023-01-01 10:00");

        let result = tera
            .render("receipt", &ctx)
            .expect("Failed to render payment receipt template");
        assert!(result.contains("سند قبض"));
        assert!(result.contains("YER"));
        assert!(!result.contains("SAR"));
    }

    #[test]
    fn test_student_id_card_compiles() {
        let mut tera = Tera::default();
        tera.add_raw_template("id_card", STUDENT_ID_CARD_HTML)
            .expect("Failed to parse id card template");

        let mut ctx = Context::new();
        ctx.insert("is_rtl", &true);
        ctx.insert("rows", &Vec::<String>::new());

        let result = tera
            .render("id_card", &ctx)
            .expect("Failed to render id card template");
        assert!(result.contains("batch-container"));
    }

    #[test]
    fn test_class_timetable_compiles() {
        let mut tera = Tera::default();
        tera.add_raw_template("timetable", CLASS_TIMETABLE_HTML)
            .expect("Failed to parse timetable template");

        let mut ctx = Context::new();
        ctx.insert("is_rtl", &true);
        ctx.insert("rows", &Vec::<String>::new());
        ctx.insert(
            "parameters",
            &std::collections::HashMap::<String, String>::new(),
        );

        let result = tera
            .render("timetable", &ctx)
            .expect("Failed to render timetable template");
        assert!(result.contains("timetable"));
    }

    #[test]
    fn test_transcript_compiles() {
        let mut tera = Tera::default();
        tera.add_raw_template("transcript", TRANSCRIPT_REPORT_HTML)
            .expect("Failed to parse transcript template");
        let mut ctx = Context::new();
        ctx.insert("is_rtl", &true);
        ctx.insert("student_name", &"أحمد محمد");
        let result = tera
            .render("transcript", &ctx)
            .expect("Failed to render transcript template");
        assert!(result.contains("transcript-container"));
    }

    #[test]
    fn test_invoice_compiles() {
        let mut tera = Tera::default();
        tera.add_raw_template("invoice", STUDENT_INVOICE_HTML)
            .expect("Failed to parse invoice template");
        let mut ctx = Context::new();
        ctx.insert("is_rtl", &true);
        ctx.insert("student_name", &"أحمد محمد");
        let result = tera
            .render("invoice", &ctx)
            .expect("Failed to render invoice template");
        assert!(result.contains("invoice-box"));
    }
}
