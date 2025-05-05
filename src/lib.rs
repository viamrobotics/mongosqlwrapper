use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use mongosql::catalog::Catalog;
use mongosql::Namespace;
use mongosql::options::{ExcludeNamespacesOption, SqlOptions};
use mongosql::{schema::Schema, translate_sql};

#[no_mangle]
pub unsafe extern "C" fn compile_sql_cgo(
    sql: *const c_char,
    schema_json: *const c_char,
) -> *mut c_char {
    let sql_str = match CStr::from_ptr(sql).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let schema_str = match CStr::from_ptr(schema_json).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let json: BTreeMap<String, BTreeMap<String, mongosql::json_schema::Schema>> =
        match serde_json::from_str(schema_str) {
            Ok(j) => j,
            Err(_) => return std::ptr::null_mut(),
        };

    let mut schemas: BTreeMap<Namespace, Schema> = BTreeMap::new();
    for (db, collections) in json {
        for (coll, jschema) in collections {
            let mschema = match Schema::try_from(jschema) {
                Ok(s) => s,
                Err(_) => return std::ptr::null_mut(),
            };
            schemas.insert(Namespace { db: db.clone(), collection: coll }, mschema);
        }
    }

    let catalog = Catalog::new(schemas);
    let opts = SqlOptions::new(
        ExcludeNamespacesOption::IncludeNamespaces,
        mongosql::SchemaCheckingMode::Relaxed,
    );

    let translation = match translate_sql("default_db", sql_str, &catalog, opts) {
        Ok(t) => t,
        Err(_) => return std::ptr::null_mut(),
    };

    let pipeline_json = match serde_json::to_string(&translation.pipeline) {
        Ok(j) => j,
        Err(_) => return std::ptr::null_mut(),
    };

    match CString::new(pipeline_json) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}
