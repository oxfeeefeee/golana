package math2

var mathFfi ffiMath2

func init() {
	mathFfi = ffi(ffiMath2, "math2")
}

type ffiMath2 interface {
	u64_geometry_mean(x, y uint64) uint64

	u64_mul_div(x, m, d uint64) uint64
}

func U64GeometryMean(x, y uint64) uint64 {
	return mathFfi.u64_geometry_mean(x, y)
}

func U64MulDiv(x, m, d uint64) uint64 {
	return mathFfi.u64_mul_div(x, m, d)
}
