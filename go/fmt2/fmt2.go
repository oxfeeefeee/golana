package fmt2

var nativeFmt ffiFmt2

func init() {
	nativeFmt = ffi(ffiFmt2, "fmt2")
}

type ffiFmt2 interface {
	println(a ...interface{})
}

func Println(a ...interface{}) {
	nativeFmt.println(a...)
}
