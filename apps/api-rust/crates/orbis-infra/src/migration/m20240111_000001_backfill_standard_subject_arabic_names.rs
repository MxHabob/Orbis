use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE subjects
                 SET name_ar = CASE code
                     WHEN 'MATH-01' THEN 'الرياضيات'
                     WHEN 'ARAB-01' THEN 'اللغة العربية'
                     WHEN 'ENG-01' THEN 'اللغة الإنجليزية'
                     WHEN 'ISLM-01' THEN 'الدراسات الإسلامية'
                     WHEN 'SCI-01' THEN 'العلوم العامة'
                     WHEN 'SOC-01' THEN 'الدراسات الاجتماعية'
                     WHEN 'CS-01' THEN 'الحاسب الآلي والتقنية'
                     WHEN 'ART-01' THEN 'التربية الفنية'
                     WHEN 'PE-01' THEN 'التربية البدنية'
                 END
                 WHERE (name_ar IS NULL OR trim(name_ar) = '')
                   AND code IN (
                       'MATH-01', 'ARAB-01', 'ENG-01', 'ISLM-01', 'SCI-01',
                       'SOC-01', 'CS-01', 'ART-01', 'PE-01'
                   )",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
