extern crate salsa;

#[derive(Clone, PartialEq, Eq, Debug)]
struct AmFmtDebug(bool);

#[derive(Clone, PartialEq, Eq)]
struct NotFmtDebug(bool);

#[salsa::query_group(NoFmtDebugStorage)]
trait NoFmtDebugDatabase: salsa::Database {
    fn am_fmt_debug_value(&self, key: bool) -> AmFmtDebug;
    fn no_fmt_debug_value(&self, key: bool) -> NotFmtDebug;
}

fn am_fmt_debug_value(_db: &dyn NoFmtDebugDatabase, key: bool) -> AmFmtDebug {
    AmFmtDebug(key)
}

fn no_fmt_debug_value(_db: &dyn NoFmtDebugDatabase, key: bool) -> NotFmtDebug {
    NotFmtDebug(key)
}

#[salsa::database(NoFmtDebugStorage)]
#[derive(Default)]
struct DatabaseImpl {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DatabaseImpl {}

#[test]
fn no_fmt_debug() {
    let db = DatabaseImpl::default();

    assert_eq!(db.am_fmt_debug_value(true), AmFmtDebug(true));
    assert!(db.no_fmt_debug_value(true) == NotFmtDebug(true));
}
