package math2

var mathFfi ffiMath2

func init() {
	mathFfi = ffi(ffiMath2, "math2")
}

type ffiMath2 interface {
	geometry_mean(x, y uint64) uint64
}

func GeometryMean(x, y uint64) uint64 {
	return mathFfi.geometry_mean(x, y)
}
