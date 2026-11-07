use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "UPDATE folders
             SET parent_id = (
                 SELECT canonical.id
                 FROM folders AS duplicate
                 JOIN folders AS canonical
                   ON canonical.org_id = duplicate.org_id
                 WHERE duplicate.id = folders.parent_id
                   AND duplicate.path = '/Students'
                   AND duplicate.folder_type = 'system'
                   AND duplicate.deleted_at IS NULL
                   AND canonical.path = '/Students'
                   AND canonical.folder_type = 'system'
                   AND canonical.deleted_at IS NULL
                 ORDER BY canonical.created_at, canonical.id
                 LIMIT 1
             )
             WHERE parent_id IN (
                 SELECT duplicate.id
                 FROM folders AS duplicate
                 WHERE duplicate.path = '/Students'
                   AND duplicate.folder_type = 'system'
                   AND duplicate.deleted_at IS NULL
                   AND duplicate.id <> (
                       SELECT canonical.id
                       FROM folders AS canonical
                       WHERE canonical.org_id = duplicate.org_id
                         AND canonical.path = '/Students'
                         AND canonical.folder_type = 'system'
                         AND canonical.deleted_at IS NULL
                       ORDER BY canonical.created_at, canonical.id
                       LIMIT 1
                   )
             )",
        )
        .await?;

        db.execute_unprepared(
            "UPDATE documents
             SET folder_id = (
                 SELECT canonical.id
                 FROM folders AS duplicate
                 JOIN folders AS canonical
                   ON canonical.org_id = duplicate.org_id
                 WHERE duplicate.id = documents.folder_id
                   AND duplicate.path = '/Students'
                   AND duplicate.folder_type = 'system'
                   AND duplicate.deleted_at IS NULL
                   AND canonical.path = '/Students'
                   AND canonical.folder_type = 'system'
                   AND canonical.deleted_at IS NULL
                 ORDER BY canonical.created_at, canonical.id
                 LIMIT 1
             )
             WHERE folder_id IN (
                 SELECT duplicate.id
                 FROM folders AS duplicate
                 WHERE duplicate.path = '/Students'
                   AND duplicate.folder_type = 'system'
                   AND duplicate.deleted_at IS NULL
                   AND duplicate.id <> (
                       SELECT canonical.id
                       FROM folders AS canonical
                       WHERE canonical.org_id = duplicate.org_id
                         AND canonical.path = '/Students'
                         AND canonical.folder_type = 'system'
                         AND canonical.deleted_at IS NULL
                       ORDER BY canonical.created_at, canonical.id
                       LIMIT 1
                   )
             )",
        )
        .await?;

        db.execute_unprepared(
            "DELETE FROM folders
             WHERE path = '/Students'
               AND folder_type = 'system'
               AND deleted_at IS NULL
               AND id <> (
                   SELECT canonical.id
                   FROM folders AS canonical
                   WHERE canonical.org_id = folders.org_id
                     AND canonical.path = '/Students'
                     AND canonical.folder_type = 'system'
                     AND canonical.deleted_at IS NULL
                   ORDER BY canonical.created_at, canonical.id
                   LIMIT 1
               )",
        )
        .await?;

        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_folders_unique_students_root
             ON folders (org_id)
             WHERE path = '/Students'
               AND folder_type = 'system'
               AND deleted_at IS NULL",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP INDEX IF EXISTS idx_folders_unique_students_root")
            .await
            .map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{ConnectionTrait, Database, DatabaseBackend, Statement};

    #[tokio::test]
    async fn migration_merges_duplicate_student_roots_and_prevents_recreation() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.execute_unprepared(
            "CREATE TABLE folders (
                id TEXT PRIMARY KEY,
                org_id TEXT NOT NULL,
                parent_id TEXT,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                folder_type TEXT NOT NULL,
                created_at TEXT NOT NULL,
                deleted_at TEXT
             );
             CREATE TABLE documents (id TEXT PRIMARY KEY, folder_id TEXT);
             INSERT INTO folders
                 (id, org_id, parent_id, name, path, folder_type, created_at, deleted_at)
             VALUES
                 ('root-a', 'org-a', NULL, 'Students', '/Students', 'system', '2025-01-01', NULL),
                 ('root-b', 'org-a', NULL, 'Students', '/Students', 'system', '2025-01-02', NULL),
                 ('student-b', 'org-a', 'root-b', 'Student B', '/Students/Student B', 'student', '2025-01-03', NULL);
             INSERT INTO documents (id, folder_id) VALUES ('document-b', 'root-b');",
        )
        .await
        .unwrap();

        Migration.up(&SchemaManager::new(&db)).await.unwrap();

        let root_count = db
            .query_one(Statement::from_string(
                DatabaseBackend::Sqlite,
                "SELECT COUNT(*) AS count FROM folders WHERE path = '/Students' AND deleted_at IS NULL"
                    .to_owned(),
            ))
            .await
            .unwrap()
            .unwrap()
            .try_get::<i64>("", "count")
            .unwrap();
        assert_eq!(root_count, 1);

        let child_parent = db
            .query_one(Statement::from_string(
                DatabaseBackend::Sqlite,
                "SELECT parent_id FROM folders WHERE id = 'student-b'".to_owned(),
            ))
            .await
            .unwrap()
            .unwrap()
            .try_get::<String>("", "parent_id")
            .unwrap();
        assert_eq!(child_parent, "root-a");

        let document_folder = db
            .query_one(Statement::from_string(
                DatabaseBackend::Sqlite,
                "SELECT folder_id FROM documents WHERE id = 'document-b'".to_owned(),
            ))
            .await
            .unwrap()
            .unwrap()
            .try_get::<String>("", "folder_id")
            .unwrap();
        assert_eq!(document_folder, "root-a");

        let duplicate_root = db
            .execute_unprepared(
                "INSERT INTO folders
                    (id, org_id, parent_id, name, path, folder_type, created_at, deleted_at)
                 VALUES
                    ('root-c', 'org-a', NULL, 'Students', '/Students', 'system', '2025-01-04', NULL)",
            )
            .await;
        assert!(duplicate_root.is_err());
    }
}
