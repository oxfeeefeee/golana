package math2

var mathFfi ffiMath2

func init() {
	mathFfi = ffi(ffiMath2, "math2")
}

type ffiMath2 interface {
	u64_geometry_mean(x, y uint64) uint64

	u64_mul_div(x, m, d uint64) uint64
}

// Returns the geometric mean of x and y, i.e. sqrt(x*y) without overflowing
func U64GeometryMean(x, y uint64) uint64 {
	return mathFfi.u64_geometry_mean(x, y)
}

// Returns x*m/d without overflowing
func U64MulDiv(x, m, d uint64) uint64 {
	return mathFfi.u64_mul_div(x, m, d)
}
