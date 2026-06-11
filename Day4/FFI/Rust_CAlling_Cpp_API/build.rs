fn main() {
    // Tell cargo to rebuild if the C++ source file changes
    println!("cargo:rerun-if-changed=cpp_src/math_lib.cpp");

    // Compile the C++ file
    cc::Build::new()
        .cpp(true) // Enable C++ compilation
        .file("cpp_src/math_lib.cpp")
        .compile("math_lib"); // This will create libmath_lib.a
}
