fn main() {
    write_plugins();
    println!("cargo::rustc-link-arg=/ENTRY:DllMain")
}

fn write_plugins() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let path = format!("{out_dir}/combined_libs.rs");

    let mut file = std::fs::File::options()
        .truncate(true)
        .write(true)
        .create(true)
        .open(&path)
        .unwrap();

    let p1 = include_str!("../nsis-semvercompare/src/lib.rs");
    let p2 = include_str!("../nsis-process/src/lib.rs");
    for p in [p1, p2] {
        let lines = p
            .lines()
            .filter(|l| {
                !(l.contains("#![no_std]")
                    || l.contains("use nsis_plugin_api::*;")
                    || l.contains("nsis_plugin!();"))
            })
            .collect::<Vec<&str>>();

        let content = lines.join("\n");
        std::io::Write::write_all(&mut file, content.as_bytes()).unwrap();
    }
}
