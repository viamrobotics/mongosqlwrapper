use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use mongosql::catalog::Catalog;
use mongosql::options::{ExcludeNamespacesOption, SqlOptions};
use mongosql::{schema::Schema, translate_sql};
use mongosql::json_schema::Schema as JsonSchema;
use agg_ast::definitions::Namespace;
use serde_json::Value;
use ffi_helpers::null_pointer_check;

#[repr(C)]
pub struct CompileResult {
    result: *mut c_char,
    error: *mut c_char,
}

impl CompileResult {
    fn new() -> Self {
        CompileResult {
            result: std::ptr::null_mut(),
            error: std::ptr::null_mut(),
        }
    }

    fn with_error(message: &str) -> Self {
        let mut result = Self::new();
        result.error = CString::new(message).unwrap().into_raw();
        result
    }
}

#[no_mangle]
pub unsafe extern "C" fn compile_sql_cgo(
    sql: *const c_char,
    schema_json: *const c_char,
) -> *mut CompileResult {
    let mut result = Box::new(CompileResult::new());

    if sql.is_null() {
        result.error = CString::new("SQL string is null").unwrap().into_raw();
        return Box::into_raw(result);
    }
    if schema_json.is_null() {
        result.error = CString::new("Schema JSON is null").unwrap().into_raw();
        return Box::into_raw(result);
    }

    let sql_str = match CStr::from_ptr(sql).to_str() {
        Ok(s) => s,
        Err(e) => {
            result.error = CString::new(format!("Failed to convert SQL string: {}", e)).unwrap().into_raw();
            return Box::into_raw(result);
        }
    };

    let schema_str = match CStr::from_ptr(schema_json).to_str() {
        Ok(s) => s,
        Err(e) => {
            result.error = CString::new(format!("Failed to convert schema string: {}", e)).unwrap().into_raw();
            return Box::into_raw(result);
        }
    };

    let schema_value: Value = match serde_json::from_str(schema_str) {
        Ok(v) => v,
        Err(e) => {
            result.error = CString::new(format!("Failed to parse schema JSON: {}", e)).unwrap().into_raw();
            return Box::into_raw(result);
        }
    };

    let mut schemas: BTreeMap<Namespace, Schema> = BTreeMap::new();
    let schema_obj = match schema_value.as_object() {
        Some(obj) => obj,
        None => {
            result.error = CString::new("Schema JSON is not an object").unwrap().into_raw();
            return Box::into_raw(result);
        }
    };

    for (db, collections) in schema_obj {
        let collections_obj = match collections.as_object() {
            Some(obj) => obj,
            None => {
                result.error = CString::new(format!("Collections for database {} is not an object", db)).unwrap().into_raw();
                return Box::into_raw(result);
            }
        };

        for (coll, schema_json) in collections_obj {
            let json_schema: JsonSchema = match serde_json::from_value(schema_json.clone()) {
                Ok(s) => s,
                Err(e) => {
                    result.error = CString::new(format!("Failed to parse schema for {}.{}: {}", db, coll, e)).unwrap().into_raw();
                    return Box::into_raw(result);
                }
            };
            let mschema = match Schema::try_from(json_schema) {
                Ok(s) => s,
                Err(e) => {
                    result.error = CString::new(format!("Failed to convert schema for {}.{}: {}", db, coll, e)).unwrap().into_raw();
                    return Box::into_raw(result);
                }
            };
            schemas.insert(Namespace { database: db.clone(), collection: coll.clone() }, mschema);
        }
    }

    let catalog = Catalog::new(schemas);
    let opts = SqlOptions::new(
        ExcludeNamespacesOption::IncludeNamespaces,
        mongosql::SchemaCheckingMode::Relaxed,
    );

    let translation = match translate_sql("default_db", sql_str, &catalog, opts) {
        Ok(t) => t,
        Err(e) => {
            result.error = CString::new(format!("Failed to translate SQL: {}", e)).unwrap().into_raw();
            return Box::into_raw(result);
        }
    };

    let pipeline_json = match serde_json::to_string(&translation.pipeline) {
        Ok(j) => j,
        Err(e) => {
            result.error = CString::new(format!("Failed to serialize pipeline: {}", e)).unwrap().into_raw();
            return Box::into_raw(result);
        }
    };

    match CString::new(pipeline_json) {
        Ok(cstr) => {
            result.result = cstr.into_raw();
            Box::into_raw(result)
        }
        Err(e) => {
            result.error = CString::new(format!("Failed to create C string: {}", e)).unwrap().into_raw();
            Box::into_raw(result)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}
#[no_mangle]
pub extern "C" fn free_compile_result(result: *mut CompileResult) {
    if result.is_null() {
        return;
    }
    unsafe {
        let result = &mut *result;
        if !result.result.is_null() {
            let _ = CString::from_raw(result.result);
        }
        if !result.error.is_null() {
            let _ = CString::from_raw(result.error);
        }
    }
}

