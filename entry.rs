#[link(name = "kernel", vers = "0.1")];
#[feature(globs)];
#[feature(macro_rules)];
#[feature(asm)];
#[allow(ctypes)];

#[no_std];

pub mod uefi;
pub mod util;
pub mod arch;
pub mod runtime;

#[no_mangle]
pub unsafe fn main ()
{
    use util::kprintln;

    kprintln("QUASAR");
    arch::idt::setup();
    loop {}
}
