pub fn cpuid_get() -> [u32; 4] {
    // see RA6M3 group reference manual 55.3.4

    // The FMIFRT is a read-only register that stores a base address
    // of the Unique ID register, Part Numbering register and MCU Version register.
    const FMIFRT: *const u32 = 0x407FB19C as *const u32;

    let base = unsafe { FMIFRT.read_volatile() };

    let uidr: *const u32 = (base + 0x14) as *const u32;
    // let pnr: *const u32 = (base + 0x24) as *const u32;
    // let mcuver: *const u32 = (base + 0x44) as *const u32;

    let mut cpuid = [0u32; 4];
    for i in 0..4 {
        cpuid[i] = unsafe { uidr.offset(i as isize).read_volatile() };
    }

    cpuid
}
