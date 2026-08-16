use std::path::PathBuf;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=swap-bss.x");

    // Only the real hardware build goes through the Xous loader. Hosted builds
    // link against the host toolchain, which would choke on this script.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("xous") {
        return;
    }

    // Work around a bug in the Xous loader's IniS (swap-resident process) reader.
    //
    // .bss is NOBITS: it occupies address space but contributes no bytes to the
    // image on disk (tools/src/elf.rs: "they don't exist in the file, they are
    // just zero'd on spec"). The loader's IniE reader honours that, but the IniS
    // reader advances its source pointer through NOCOPY sections anyway:
    //
    //     // loader/src/phase1.rs
    //     src_swap_img_addr += copyable;   // runs for no_copy sections too
    //
    // So the loader's source pointer overshoots the end of the real data by the
    // size of .bss. When that overshoot crosses a page boundary the loader reads
    // the page *after* the data region -- the MAC table -- and `signed_len -
    // hashed_count` borrows, producing a ~4-billion-iteration flash read loop.
    // The badge hangs with the boot progress bar frozen half way, with no output
    // on any interface. Whether a given build trips it depends on where .bss
    // happens to land relative to a page boundary, so it looks like a random
    // size ceiling: roughly a 1-in-4 chance per build, with the odds set by
    // sizeof(.bss) / 4096.
    //
    // Emitting .bss as PROGBITS costs sizeof(.bss) bytes of image and makes the
    // failure structurally impossible: with no NOBITS section left, the loader's
    // source pointer can no longer run past what is actually on disk. Runtime
    // behaviour is unchanged -- those pages were already being written to swap
    // as zeros, they just weren't being stored in the image.
    let script = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("swap-bss.x");
    println!("cargo::rustc-link-arg-bins=-T{}", script.display());
}
