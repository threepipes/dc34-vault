//! Works around a hang in the Xous loader when this process is swap-resident.
//!
//! `.bss` is NOBITS: it takes up address space but contributes no bytes to the
//! image on disk. The loader's IniS reader ignores that and advances its source
//! pointer through NOCOPY sections anyway, so it overshoots the end of the real
//! data by `sizeof(.bss)`. When that overshoot crosses a page boundary the
//! loader reads the page after the data region -- the MAC table -- and hangs
//! with the boot progress bar frozen half way. See `build.rs` for the full
//! write-up.
//!
//! `swap-bss.x` merges `.bss` into an output section that starts with this
//! anchor. The anchor is PROGBITS, which is what makes the merged section
//! PROGBITS, which is what gets `.bss` written to disk as real zeros -- and a
//! `.bss` that exists on disk cannot be overshot.
//!
//! The value is arbitrary; only its section type matters. It must not be zero,
//! or the compiler would place it in `.bss` itself and we would be back where
//! we started.

#[used]
#[unsafe(link_section = ".swapfix_anchor")]
static SWAPFIX_ANCHOR: [u8; 4] = [0xa5; 4];
