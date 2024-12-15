use uuid::Uuid;

pub fn sqlite_test_document(id: Uuid) -> String {
    format!("sqlite://tests/sqlite/tests-{id}.sqlite?mode=rwc")
}
