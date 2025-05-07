//go:build cgo
// +build cgo

// Package mongosqlwrapper provides CGO bindings to a Rust library that compiles SQL to MQL.
package mongosqlwrapper

/*
#cgo LDFLAGS: -L${SRCDIR}/../lib -L/usr/local/lib -lmongosqlwrapper
#include <stdlib.h>

typedef struct CompileResult {
    char* result;
    char* error;
} CompileResult;

CompileResult* compile_sql_cgo(const char* sql, const char* schema_json);
void free_compile_result(CompileResult* r);
void free_string(char* s);
*/
import "C"

import (
	"errors"
	"unsafe"
)

// CompileSQLToMQL compiles a SQL query to MongoDB Query Language (MQL) using the provided schema.
// The schema should be a JSON string representing the collection schema.
func CompileSQLToMQL(sql string, schemaJSON string) (string, error) {
	cSQL := C.CString(sql)
	defer C.free(unsafe.Pointer(cSQL))

	cSchema := C.CString(schemaJSON)
	defer C.free(unsafe.Pointer(cSchema))

	result := C.compile_sql_cgo(cSQL, cSchema)
	defer C.free_compile_result(result)

	if result.error != nil {
		return "", errors.New(C.GoString(result.error))
	}
	if result.result == nil {
		return "", errors.New("failed to compile SQL to MQL: unknown error")
	}
	return C.GoString(result.result), nil
}
