# Spec Delta

## Purpose

تتيح هذه القدرة استهلاك نتائج المؤسسة في DocForge والتقارير عبر عقود بيانات أكاديمية typed مستقرة، منفصلة عن تخزينها وعن نوع المستند النهائي.

## ADDED Requirements

### Requirement: التقرير يستهلك عقدًا دلاليًا typed
The system SHALL expose published academic results through reusable typed data contracts whose fields describe meaning and value type rather than database table structure.

#### Scenario: قالب يطلب معدلًا وتقديرًا
- **WHEN** يختار مصمم قالب حقلي المعدل والتقدير من نتيجة منشورة
- **THEN** يعرض المحرر حقولًا typed مع تسميات ووصف دون الحاجة لمعرفة جداول التخزين

### Requirement: عقود التقرير تعكس سياسة المؤسسة
The system SHALL allow report projections to select configured result outputs and SHALL NOT require a fixed certificate, term report, annual summary, GPA, or rank field.

#### Scenario: معهد يصدر تقرير كفاءات
- **WHEN** تعرف سياسة المعهد مخرجات كفاءات عملية دون GPA أو ترتيب
- **THEN** يستطيع قالب التقرير استخدام تلك المخرجات دون اشتراط حقول جامعية أو مدرسية

### Requirement: تحقق القالب يتحقق من عقد البيانات
The system SHALL validate template field references against the selected academic data contract and report unknown or type-incompatible fields before publication.

#### Scenario: حقل غير موجود في العقد
- **WHEN** يشير القالب إلى حقل غير مقدم من عقد النتيجة المختار
- **THEN** يظهر التحقق خطأ الحقل بدل استبداله بقيمة فارغة بصمت

### Requirement: استهلاك التقرير لا يغير النتيجة الرسمية
The system SHALL render reports from an identified published result revision and SHALL preserve that revision's values for the duration of report generation.

#### Scenario: تصحيح نتيجة أثناء طباعة دفعة
- **WHEN** تبدأ طباعة مجموعة تقارير لإصدار نتائج معتمد ثم تنشأ مراجعة أحدث
- **THEN** تستخدم المجموعة إصدار النتيجة الذي حددته عند البدء ولا تمزج بين المراجعتين
