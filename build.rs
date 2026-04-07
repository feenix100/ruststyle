// build.rs compiles the root Slint file ahead of time.
// This keeps UI markup in external files and makes runtime startup simple.
fn main() {
    slint_build::compile("ui/app.slint").expect("Failed to compile Slint UI");
}