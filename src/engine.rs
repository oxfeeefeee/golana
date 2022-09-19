use crate::go_lib_fmt;
use goscript_engine::run_map::*;
use solana_program::msg;
use std::collections::HashMap;
use std::path::Path;

pub fn run(s: &str) {
    let fmt2 = r#"
    package fmt2

    type ffiFmt2 interface {
        println(a ...interface{})
        printf(a ...interface{})
    }

    func Println(a ...interface{})  {
        var f = ffi(ffiFmt2, "fmt2")
        f.println(a...)
    }

    func Printf(a ...interface{})  {
        var f = ffi(ffiFmt2, "fmt2")
        f.printf(a...)
    }
"#;

    let mut cfg = Config::default();
    cfg.base_dir = Some("std/");
    cfg.extensions = Some(vec![Box::new(go_lib_fmt::Fmt2Ffi::register)]);

    let mut map = HashMap::new();
    map.insert(Path::new("std/fmt2/fmt2.go"), fmt2.to_owned());
    if let Err(el) = run_string(&map, cfg, s) {
        msg!(&el.to_string())
    }
}
