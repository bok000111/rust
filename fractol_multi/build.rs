fn main() {
	println!("cargo:rustc-link-lib=mlx");
	println!("cargo:rustc-link-search=../minilibx")
}
