b = ones(Float32, 2, 3)
ccall((:make_twice_array, "libmyext"), Cvoid, (Any,), b)
println(b)
