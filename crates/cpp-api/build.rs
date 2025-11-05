use std::path::Path;

fn main() {
    cc::Build::new()
        .cpp(true)
        .include(Path::new("src-cpp"))
        .file(Path::new("src-cpp/adjacency_list.cpp"))
        .opt_level(3)
        .flag_if_supported("-march=native")
        .flag_if_supported("-flto")
        .compile("graph_cpp");
}
