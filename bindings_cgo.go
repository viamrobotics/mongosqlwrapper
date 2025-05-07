package mongosqlwrapper

// #cgo LDFLAGS: -L${SRCDIR}/../target/release -lmongosqlwrapper
// #include <stdlib.h>
// #include <stdint.h>
// char* compile_sql_to_mql(const char* sql, const char* schema);
import "C"
import (
	"errors"
	"unsafe"
)

// CompileSQLToMQL compiles a SQL query to MongoDB Query Language (MQL) using the provided schema.
func CompileSQLToMQL(sql string, schema string) (string, error) {
	sqlC := C.CString(sql)
	defer C.free(unsafe.Pointer(sqlC))

	schemaC := C.CString(schema)
	defer C.free(unsafe.Pointer(schemaC))

	resultC := C.compile_sql_to_mql(sqlC, schemaC)
	if resultC == nil {
		return "", errors.New("failed to compile SQL to MQL")
	}
	defer C.free(unsafe.Pointer(resultC))

	return C.GoString(resultC), nil
}
