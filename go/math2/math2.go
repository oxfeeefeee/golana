package math2

var nativeMath ffiMath2

func init() {
	nativeMath = ffi(ffiMath2, "math2")
}

type ffiMath2 interface {
	geometry_mean(a ...interface{})
}

func GeometryMean(a ...interface{}) {
	nativeMath.geometry_mean(a...)
}
