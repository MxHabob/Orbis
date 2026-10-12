# Spec Delta

## Purpose

تحفظ هذه القدرة النتائج المحسوبة بوصفها سجلات دلالية ذات منشأ وإصدارات وحالات اعتماد، بما يحافظ على التاريخ الأكاديمي عند تغير الدرجات أو القواعد.

## ADDED Requirements

### Requirement: النتيجة تحفظ لقطة حساب قابلة لإعادة الإنتاج
The system SHALL persist each calculated result with its subject or learner scope, typed outputs, calculation-policy version, input revision references, and calculation timestamp.

#### Scenario: إعادة عرض نتيجة قديمة
- **WHEN** يطلب مستخدم نتيجة محسوبة قبل تحديث سياسة المؤسسة
- **THEN** يعرض النظام اللقطة المحفوظة وإصدار القاعدة الذي أنتجها دون إعادة حسابها بإصدار أحدث

### Requirement: حالات النتائج واعتمادها معرفة بالسياسة
The system SHALL expose configurable result lifecycle states and SHALL enforce the institution's required review and approval transitions before a result is considered official.

#### Scenario: نتيجة تتطلب اعتمادًا
- **WHEN** تكتمل الحسابات لنتيجة تتطلب اعتماد مراجع
- **THEN** تبقى النتيجة غير رسمية حتى ينفذ مستخدم مخول انتقال الاعتماد

### Requirement: الاعتماد يسجل المسؤولية والوقت
The system SHALL record the approving actor, approval time, and result revision for each official approval.

#### Scenario: اعتماد نتيجة
- **WHEN** يعتمد مستخدم مخول نتيجة جاهزة للمراجعة
- **THEN** يحفظ النظام هوية المعتمد وتوقيت الاعتماد على الإصدار المعتمد نفسه

### Requirement: التصحيح التاريخي ينشئ مراجعة جديدة
The system SHALL preserve every approved result revision and require a correction reason when a later revision supersedes an approved result.

#### Scenario: تصحيح بعد الاعتماد
- **WHEN** يغير مستخدم مخول مدخلًا أو قاعدة تؤثر في نتيجة معتمدة
- **THEN** ينشئ النظام مراجعة لاحقة مرتبطة بالأصل، ويحتفظ بالنتيجة السابقة وسبب التصحيح

### Requirement: الاعتماد يمنع التعديل الصامت
The system SHALL prevent direct mutation or deletion of approved result snapshots through ordinary grade-entry or calculation operations.

#### Scenario: محاولة حذف نتيجة رسمية
- **WHEN** يحاول مستخدم حذف لقطة نتيجة معتمدة عبر عملية عادية
- **THEN** يرفض النظام الحذف ويوجه العملية إلى مسار تصحيح موثق
