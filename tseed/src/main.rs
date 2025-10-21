use libloading::{Library, Symbol};

fn main() {
    let libpath = unsafe { Library::new("/data/adb/modules/ts_enhancer_extreme/lib/libsealer.so") }.unwrap();
    let verify: Symbol<unsafe fn() -> bool> = unsafe { libpath.get(b"rsealer\0") }.unwrap();

    if !unsafe { verify() } {
        std::process::exit(1);
    }

    println!("Hello World!");
}