#[allow(dead_code)]
fn add_lib(name:impl AsRef<str>, _static:bool)
{
    #[cfg(not(feature = "test"))]
    println!(
        "cargo:rustc-link-lib={}{}",
        if _static { "static=" } else { "" },
        name.as_ref()
    );
}

fn main()
{
    // Adds the libc++_shared.so for oboe.
    #[cfg(target_os = "android")]
    add_lib("c++_shared", false);
}