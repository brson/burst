use core::mem;

#[lang = "start"]
fn lang_start(main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        let main: fn() = mem::transmute(main);

        main();

        0
    }
}
