use pgq::commands::is_read_only_sql;

#[test]
fn accepts_select_style_queries() {
    assert!(is_read_only_sql("select * from users"));
    assert!(is_read_only_sql("  with t as (select 1) select * from t"));
    assert!(is_read_only_sql("explain select * from users"));
}

#[test]
fn rejects_mutating_queries() {
    assert!(!is_read_only_sql("delete from users"));
    assert!(!is_read_only_sql("update users set active = false"));
    assert!(!is_read_only_sql("insert into users(id) values (1)"));
    assert!(!is_read_only_sql("select * from users; delete from users"));
}
