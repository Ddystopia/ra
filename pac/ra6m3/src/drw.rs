#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved_0_status: [u8; 0x04],
    _reserved_1_control2: [u8; 0x04],
    _reserved2: [u8; 0x08],
    lstart: [LSTART; 6],
    lxadd: [LXADD; 6],
    lyadd: [LYADD; 6],
    lband: [LBAND; 2],
    _reserved6: [u8; 0x04],
    color1: COLOR1,
    color2: COLOR2,
    _reserved8: [u8; 0x08],
    pattern: PATTERN,
    size: SIZE,
    pitch: PITCH,
    origin: ORIGIN,
    _reserved12: [u8; 0x0c],
    lustart: LUSTART,
    luxadd: LUXADD,
    luyadd: LUYADD,
    lvstarti: LVSTARTI,
    lvstartf: LVSTARTF,
    lvxaddi: LVXADDI,
    lvyaddi: LVYADDI,
    lvyxaddf: LVYXADDF,
    _reserved20: [u8; 0x04],
    texpitch: TEXPITCH,
    texmask: TEXMASK,
    texorigin: TEXORIGIN,
    irqctl: IRQCTL,
    cachectl: CACHECTL,
    dliststart: DLISTSTART,
    perfcount: [PERFCOUNT; 2],
    perftrigger: PERFTRIGGER,
    _reserved28: [u8; 0x04],
    texcladdr: TEXCLADDR,
    texcldata: TEXCLDATA,
    texcloffset: TEXCLOFFSET,
    colkey: COLKEY,
}
impl RegisterBlock {
    ///0x00 - Status Control Register
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - Geometry Control Register
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - Hardware Version and Feature Set ID Register
    #[inline(always)]
    pub const fn hwrevision(&self) -> &HWREVISION {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - Surface Control Register
    #[inline(always)]
    pub const fn control2(&self) -> &CONTROL2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x10..0x28 - Limiter %s Start Value Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `L1START` register.</div>
    #[inline(always)]
    pub const fn lstart(&self, n: usize) -> &LSTART {
        &self.lstart[n]
    }
    ///Iterator for array of:
    ///0x10..0x28 - Limiter %s Start Value Register
    #[inline(always)]
    pub fn lstart_iter(&self) -> impl Iterator<Item = &LSTART> {
        self.lstart.iter()
    }
    ///0x10 - Limiter 1 Start Value Register
    #[inline(always)]
    pub const fn l1start(&self) -> &LSTART {
        self.lstart(0)
    }
    ///0x14 - Limiter 2 Start Value Register
    #[inline(always)]
    pub const fn l2start(&self) -> &LSTART {
        self.lstart(1)
    }
    ///0x18 - Limiter 3 Start Value Register
    #[inline(always)]
    pub const fn l3start(&self) -> &LSTART {
        self.lstart(2)
    }
    ///0x1c - Limiter 4 Start Value Register
    #[inline(always)]
    pub const fn l4start(&self) -> &LSTART {
        self.lstart(3)
    }
    ///0x20 - Limiter 5 Start Value Register
    #[inline(always)]
    pub const fn l5start(&self) -> &LSTART {
        self.lstart(4)
    }
    ///0x24 - Limiter 6 Start Value Register
    #[inline(always)]
    pub const fn l6start(&self) -> &LSTART {
        self.lstart(5)
    }
    ///0x28..0x40 - Limiter %s X-Axis Increment Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `L1XADD` register.</div>
    #[inline(always)]
    pub const fn lxadd(&self, n: usize) -> &LXADD {
        &self.lxadd[n]
    }
    ///Iterator for array of:
    ///0x28..0x40 - Limiter %s X-Axis Increment Register
    #[inline(always)]
    pub fn lxadd_iter(&self) -> impl Iterator<Item = &LXADD> {
        self.lxadd.iter()
    }
    ///0x28 - Limiter 1 X-Axis Increment Register
    #[inline(always)]
    pub const fn l1xadd(&self) -> &LXADD {
        self.lxadd(0)
    }
    ///0x2c - Limiter 2 X-Axis Increment Register
    #[inline(always)]
    pub const fn l2xadd(&self) -> &LXADD {
        self.lxadd(1)
    }
    ///0x30 - Limiter 3 X-Axis Increment Register
    #[inline(always)]
    pub const fn l3xadd(&self) -> &LXADD {
        self.lxadd(2)
    }
    ///0x34 - Limiter 4 X-Axis Increment Register
    #[inline(always)]
    pub const fn l4xadd(&self) -> &LXADD {
        self.lxadd(3)
    }
    ///0x38 - Limiter 5 X-Axis Increment Register
    #[inline(always)]
    pub const fn l5xadd(&self) -> &LXADD {
        self.lxadd(4)
    }
    ///0x3c - Limiter 6 X-Axis Increment Register
    #[inline(always)]
    pub const fn l6xadd(&self) -> &LXADD {
        self.lxadd(5)
    }
    ///0x40..0x58 - Limiter %s Y-Axis Increment Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `L1YADD` register.</div>
    #[inline(always)]
    pub const fn lyadd(&self, n: usize) -> &LYADD {
        &self.lyadd[n]
    }
    ///Iterator for array of:
    ///0x40..0x58 - Limiter %s Y-Axis Increment Register
    #[inline(always)]
    pub fn lyadd_iter(&self) -> impl Iterator<Item = &LYADD> {
        self.lyadd.iter()
    }
    ///0x40 - Limiter 1 Y-Axis Increment Register
    #[inline(always)]
    pub const fn l1yadd(&self) -> &LYADD {
        self.lyadd(0)
    }
    ///0x44 - Limiter 2 Y-Axis Increment Register
    #[inline(always)]
    pub const fn l2yadd(&self) -> &LYADD {
        self.lyadd(1)
    }
    ///0x48 - Limiter 3 Y-Axis Increment Register
    #[inline(always)]
    pub const fn l3yadd(&self) -> &LYADD {
        self.lyadd(2)
    }
    ///0x4c - Limiter 4 Y-Axis Increment Register
    #[inline(always)]
    pub const fn l4yadd(&self) -> &LYADD {
        self.lyadd(3)
    }
    ///0x50 - Limiter 5 Y-Axis Increment Register
    #[inline(always)]
    pub const fn l5yadd(&self) -> &LYADD {
        self.lyadd(4)
    }
    ///0x54 - Limiter 6 Y-Axis Increment Register
    #[inline(always)]
    pub const fn l6yadd(&self) -> &LYADD {
        self.lyadd(5)
    }
    ///0x58..0x60 - Limiter %s Band Width Parameter Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `L1BAND` register.</div>
    #[inline(always)]
    pub const fn lband(&self, n: usize) -> &LBAND {
        &self.lband[n]
    }
    ///Iterator for array of:
    ///0x58..0x60 - Limiter %s Band Width Parameter Register
    #[inline(always)]
    pub fn lband_iter(&self) -> impl Iterator<Item = &LBAND> {
        self.lband.iter()
    }
    ///0x58 - Limiter 1 Band Width Parameter Register
    #[inline(always)]
    pub const fn l1band(&self) -> &LBAND {
        self.lband(0)
    }
    ///0x5c - Limiter 2 Band Width Parameter Register
    #[inline(always)]
    pub const fn l2band(&self) -> &LBAND {
        self.lband(1)
    }
    ///0x64 - Base Color Register
    #[inline(always)]
    pub const fn color1(&self) -> &COLOR1 {
        &self.color1
    }
    ///0x68 - Secondary Color Register
    #[inline(always)]
    pub const fn color2(&self) -> &COLOR2 {
        &self.color2
    }
    ///0x74 - Pattern Register
    #[inline(always)]
    pub const fn pattern(&self) -> &PATTERN {
        &self.pattern
    }
    ///0x78 - Bounding Box Dimension Register
    #[inline(always)]
    pub const fn size(&self) -> &SIZE {
        &self.size
    }
    ///0x7c - Framebuffer Pitch And Spanstore Delay Register
    #[inline(always)]
    pub const fn pitch(&self) -> &PITCH {
        &self.pitch
    }
    ///0x80 - Framebuffer Base Address Register
    #[inline(always)]
    pub const fn origin(&self) -> &ORIGIN {
        &self.origin
    }
    ///0x90 - U Limiter Start Value Register
    #[inline(always)]
    pub const fn lustart(&self) -> &LUSTART {
        &self.lustart
    }
    ///0x94 - U Limiter X-Axis Increment Register
    #[inline(always)]
    pub const fn luxadd(&self) -> &LUXADD {
        &self.luxadd
    }
    ///0x98 - U Limiter Y-Axis Increment Register
    #[inline(always)]
    pub const fn luyadd(&self) -> &LUYADD {
        &self.luyadd
    }
    ///0x9c - V Limiter Start Value Integer Part Register
    #[inline(always)]
    pub const fn lvstarti(&self) -> &LVSTARTI {
        &self.lvstarti
    }
    ///0xa0 - V Limiter Start Value Fractional Part Register
    #[inline(always)]
    pub const fn lvstartf(&self) -> &LVSTARTF {
        &self.lvstartf
    }
    ///0xa4 - V Limiter X-Axis Increment Integer Part Register
    #[inline(always)]
    pub const fn lvxaddi(&self) -> &LVXADDI {
        &self.lvxaddi
    }
    ///0xa8 - V Limiter Y-Axis Increment Integer Part Register
    #[inline(always)]
    pub const fn lvyaddi(&self) -> &LVYADDI {
        &self.lvyaddi
    }
    ///0xac - V Limiter Increment Fractional Parts Register
    #[inline(always)]
    pub const fn lvyxaddf(&self) -> &LVYXADDF {
        &self.lvyxaddf
    }
    ///0xb4 - Texels Per Texture Line Register
    #[inline(always)]
    pub const fn texpitch(&self) -> &TEXPITCH {
        &self.texpitch
    }
    ///0xb8 - Texture Size or Texture Address Mask Register
    #[inline(always)]
    pub const fn texmask(&self) -> &TEXMASK {
        &self.texmask
    }
    ///0xbc - Texture Base Address Register
    #[inline(always)]
    pub const fn texorigin(&self) -> &TEXORIGIN {
        &self.texorigin
    }
    ///0xc0 - Interrupt Control Register
    #[inline(always)]
    pub const fn irqctl(&self) -> &IRQCTL {
        &self.irqctl
    }
    ///0xc4 - Cache Control Register
    #[inline(always)]
    pub const fn cachectl(&self) -> &CACHECTL {
        &self.cachectl
    }
    ///0xc8 - Display List Start Address Register
    #[inline(always)]
    pub const fn dliststart(&self) -> &DLISTSTART {
        &self.dliststart
    }
    ///0xcc..0xd4 - Performance Counter %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PERFCOUNT1` register.</div>
    #[inline(always)]
    pub const fn perfcount(&self, n: usize) -> &PERFCOUNT {
        &self.perfcount[n]
    }
    ///Iterator for array of:
    ///0xcc..0xd4 - Performance Counter %s
    #[inline(always)]
    pub fn perfcount_iter(&self) -> impl Iterator<Item = &PERFCOUNT> {
        self.perfcount.iter()
    }
    ///0xcc - Performance Counter 1
    #[inline(always)]
    pub const fn perfcount1(&self) -> &PERFCOUNT {
        self.perfcount(0)
    }
    ///0xd0 - Performance Counter 2
    #[inline(always)]
    pub const fn perfcount2(&self) -> &PERFCOUNT {
        self.perfcount(1)
    }
    ///0xd4 - Performance Counters Control Register
    #[inline(always)]
    pub const fn perftrigger(&self) -> &PERFTRIGGER {
        &self.perftrigger
    }
    ///0xdc - CLUT Start Address Register
    #[inline(always)]
    pub const fn texcladdr(&self) -> &TEXCLADDR {
        &self.texcladdr
    }
    ///0xe0 - CLUT Data Register
    #[inline(always)]
    pub const fn texcldata(&self) -> &TEXCLDATA {
        &self.texcldata
    }
    ///0xe4 - CLUT Offset Register
    #[inline(always)]
    pub const fn texcloffset(&self) -> &TEXCLOFFSET {
        &self.texcloffset
    }
    ///0xe8 - Color Key Register
    #[inline(always)]
    pub const fn colkey(&self) -> &COLKEY {
        &self.colkey
    }
}
/**CONTROL (w) register accessor: Geometry Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@control`] module*/
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
///Geometry Control Register
pub mod control;
/**CONTROL2 (w) register accessor: Surface Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@control2`] module*/
pub type CONTROL2 = crate::Reg<control2::CONTROL2_SPEC>;
///Surface Control Register
pub mod control2;
/**IRQCTL (w) register accessor: Interrupt Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@irqctl`] module*/
pub type IRQCTL = crate::Reg<irqctl::IRQCTL_SPEC>;
///Interrupt Control Register
pub mod irqctl;
/**CACHECTL (w) register accessor: Cache Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cachectl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cachectl`] module*/
pub type CACHECTL = crate::Reg<cachectl::CACHECTL_SPEC>;
///Cache Control Register
pub mod cachectl;
/**STATUS (r) register accessor: Status Control Register

You can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///Status Control Register
pub mod status;
/**HWREVISION (r) register accessor: Hardware Version and Feature Set ID Register

You can [`read`](crate::Reg::read) this register and get [`hwrevision::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hwrevision`] module*/
pub type HWREVISION = crate::Reg<hwrevision::HWREVISION_SPEC>;
///Hardware Version and Feature Set ID Register
pub mod hwrevision;
/**COLOR1 (w) register accessor: Base Color Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`color1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@color1`] module*/
pub type COLOR1 = crate::Reg<color1::COLOR1_SPEC>;
///Base Color Register
pub mod color1;
/**COLOR2 (w) register accessor: Secondary Color Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`color2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@color2`] module*/
pub type COLOR2 = crate::Reg<color2::COLOR2_SPEC>;
///Secondary Color Register
pub mod color2;
/**PATTERN (w) register accessor: Pattern Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pattern::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pattern`] module*/
pub type PATTERN = crate::Reg<pattern::PATTERN_SPEC>;
///Pattern Register
pub mod pattern;
/**LSTART (w) register accessor: Limiter %s Start Value Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lstart`] module*/
pub type LSTART = crate::Reg<lstart::LSTART_SPEC>;
///Limiter %s Start Value Register
pub mod lstart;
/**LXADD (w) register accessor: Limiter %s X-Axis Increment Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lxadd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lxadd`] module*/
pub type LXADD = crate::Reg<lxadd::LXADD_SPEC>;
///Limiter %s X-Axis Increment Register
pub mod lxadd;
/**LYADD (w) register accessor: Limiter %s Y-Axis Increment Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lyadd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lyadd`] module*/
pub type LYADD = crate::Reg<lyadd::LYADD_SPEC>;
///Limiter %s Y-Axis Increment Register
pub mod lyadd;
/**LBAND (w) register accessor: Limiter %s Band Width Parameter Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lband::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lband`] module*/
pub type LBAND = crate::Reg<lband::LBAND_SPEC>;
///Limiter %s Band Width Parameter Register
pub mod lband;
/**TEXORIGIN (w) register accessor: Texture Base Address Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`texorigin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@texorigin`] module*/
pub type TEXORIGIN = crate::Reg<texorigin::TEXORIGIN_SPEC>;
///Texture Base Address Register
pub mod texorigin;
/**TEXPITCH (w) register accessor: Texels Per Texture Line Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`texpitch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@texpitch`] module*/
pub type TEXPITCH = crate::Reg<texpitch::TEXPITCH_SPEC>;
///Texels Per Texture Line Register
pub mod texpitch;
/**TEXMASK (w) register accessor: Texture Size or Texture Address Mask Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`texmask::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@texmask`] module*/
pub type TEXMASK = crate::Reg<texmask::TEXMASK_SPEC>;
///Texture Size or Texture Address Mask Register
pub mod texmask;
/**LUSTART (w) register accessor: U Limiter Start Value Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lustart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lustart`] module*/
pub type LUSTART = crate::Reg<lustart::LUSTART_SPEC>;
///U Limiter Start Value Register
pub mod lustart;
/**LUXADD (w) register accessor: U Limiter X-Axis Increment Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`luxadd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@luxadd`] module*/
pub type LUXADD = crate::Reg<luxadd::LUXADD_SPEC>;
///U Limiter X-Axis Increment Register
pub mod luxadd;
/**LUYADD (w) register accessor: U Limiter Y-Axis Increment Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`luyadd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@luyadd`] module*/
pub type LUYADD = crate::Reg<luyadd::LUYADD_SPEC>;
///U Limiter Y-Axis Increment Register
pub mod luyadd;
/**LVSTARTI (w) register accessor: V Limiter Start Value Integer Part Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvstarti::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvstarti`] module*/
pub type LVSTARTI = crate::Reg<lvstarti::LVSTARTI_SPEC>;
///V Limiter Start Value Integer Part Register
pub mod lvstarti;
/**LVSTARTF (w) register accessor: V Limiter Start Value Fractional Part Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvstartf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvstartf`] module*/
pub type LVSTARTF = crate::Reg<lvstartf::LVSTARTF_SPEC>;
///V Limiter Start Value Fractional Part Register
pub mod lvstartf;
/**LVXADDI (w) register accessor: V Limiter X-Axis Increment Integer Part Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvxaddi::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvxaddi`] module*/
pub type LVXADDI = crate::Reg<lvxaddi::LVXADDI_SPEC>;
///V Limiter X-Axis Increment Integer Part Register
pub mod lvxaddi;
/**LVYADDI (w) register accessor: V Limiter Y-Axis Increment Integer Part Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvyaddi::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvyaddi`] module*/
pub type LVYADDI = crate::Reg<lvyaddi::LVYADDI_SPEC>;
///V Limiter Y-Axis Increment Integer Part Register
pub mod lvyaddi;
/**LVYXADDF (w) register accessor: V Limiter Increment Fractional Parts Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvyxaddf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvyxaddf`] module*/
pub type LVYXADDF = crate::Reg<lvyxaddf::LVYXADDF_SPEC>;
///V Limiter Increment Fractional Parts Register
pub mod lvyxaddf;
/**TEXCLADDR (w) register accessor: CLUT Start Address Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`texcladdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@texcladdr`] module*/
pub type TEXCLADDR = crate::Reg<texcladdr::TEXCLADDR_SPEC>;
///CLUT Start Address Register
pub mod texcladdr;
/**TEXCLDATA (w) register accessor: CLUT Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`texcldata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@texcldata`] module*/
pub type TEXCLDATA = crate::Reg<texcldata::TEXCLDATA_SPEC>;
///CLUT Data Register
pub mod texcldata;
/**TEXCLOFFSET (w) register accessor: CLUT Offset Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`texcloffset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@texcloffset`] module*/
pub type TEXCLOFFSET = crate::Reg<texcloffset::TEXCLOFFSET_SPEC>;
///CLUT Offset Register
pub mod texcloffset;
/**COLKEY (w) register accessor: Color Key Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`colkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@colkey`] module*/
pub type COLKEY = crate::Reg<colkey::COLKEY_SPEC>;
///Color Key Register
pub mod colkey;
/**SIZE (w) register accessor: Bounding Box Dimension Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`size::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@size`] module*/
pub type SIZE = crate::Reg<size::SIZE_SPEC>;
///Bounding Box Dimension Register
pub mod size;
/**PITCH (w) register accessor: Framebuffer Pitch And Spanstore Delay Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pitch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pitch`] module*/
pub type PITCH = crate::Reg<pitch::PITCH_SPEC>;
///Framebuffer Pitch And Spanstore Delay Register
pub mod pitch;
/**ORIGIN (w) register accessor: Framebuffer Base Address Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`origin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@origin`] module*/
pub type ORIGIN = crate::Reg<origin::ORIGIN_SPEC>;
///Framebuffer Base Address Register
pub mod origin;
/**DLISTSTART (w) register accessor: Display List Start Address Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dliststart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dliststart`] module*/
pub type DLISTSTART = crate::Reg<dliststart::DLISTSTART_SPEC>;
///Display List Start Address Register
pub mod dliststart;
/**PERFTRIGGER (w) register accessor: Performance Counters Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perftrigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@perftrigger`] module*/
pub type PERFTRIGGER = crate::Reg<perftrigger::PERFTRIGGER_SPEC>;
///Performance Counters Control Register
pub mod perftrigger;
/**PERFCOUNT (rw) register accessor: Performance Counter %s

You can [`read`](crate::Reg::read) this register and get [`perfcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perfcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@perfcount`] module*/
pub type PERFCOUNT = crate::Reg<perfcount::PERFCOUNT_SPEC>;
///Performance Counter %s
pub mod perfcount;
