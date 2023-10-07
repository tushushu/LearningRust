package main

func main() {
	// // 0. Nil pointer
	// var x *int
	// println(x)

	// // 1. Assigned by nil pointer
	// var x *int
	// y := Int(100)
	// println(y)

	// y = x
	// println(y)

	// // 2. Using *int type
	// var x *int
	// x = Int(100)
	// println(x)

	// x = nil
	// println(x)

	// *x += 3
	// println(x)

	// // 3. Using *Student type
	// // In this case there are 2 types in Go:
	// // - Student
	// // - *Student
	// type Student struct {
	// 	Age int
	// }
	// var s *Student
	// println(s.Age)
}

func Int(n int) *int {
	return &n
}
