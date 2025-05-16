#[repr(C)]
///Register block
pub struct RegisterBlock {
    gr1_clut0: [GR1_CLUT0; 256],
    gr1_clut1: [GR1_CLUT1; 256],
    gr2_clut0: [GR2_CLUT0; 256],
    gr2_clut1: [GR2_CLUT1; 256],
    bg_en: BG_EN,
    bg_peri: BG_PERI,
    bg_sync: BG_SYNC,
    bg_vsize: BG_VSIZE,
    bg_hsize: BG_HSIZE,
    bg_bgc: BG_BGC,
    bg_mon: BG_MON,
    _reserved11: [u8; 0xe4],
    gr_ven: (),
    _reserved12: [u8; 0x04],
    gr_flmrd: (),
    _reserved13: [u8; 0x04],
    gr_flm1: (),
    _reserved14: [u8; 0x04],
    gr_flm2: (),
    _reserved15: [u8; 0x04],
    gr_flm3: (),
    _reserved16: [u8; 0x08],
    gr_flm5: (),
    _reserved17: [u8; 0x04],
    gr_flm6: (),
    _reserved18: [u8; 0x04],
    gr_ab1: (),
    _reserved19: [u8; 0x04],
    gr_ab2: (),
    _reserved20: [u8; 0x04],
    gr_ab3: (),
    _reserved21: [u8; 0x04],
    gr_ab4: (),
    _reserved22: [u8; 0x04],
    gr_ab5: (),
    _reserved23: [u8; 0x04],
    gr_ab6: (),
    _reserved24: [u8; 0x04],
    gr_ab7: (),
    _reserved25: [u8; 0x04],
    gr_ab8: (),
    _reserved26: [u8; 0x04],
    gr_ab9: (),
    _reserved27: [u8; 0x0c],
    gr_base: (),
    _reserved28: [u8; 0x04],
    gr_clutint: (),
    _reserved29: [u8; 0x04],
    gr_mon: (),
    _reserved30: [u8; 0x01ac],
    gam_latch: (),
    _reserved31: [u8; 0x04],
    gam_sw: GAM_SW,
    gam_lut1: (),
    _reserved33: [u8; 0x04],
    gam_lut2: (),
    _reserved34: [u8; 0x04],
    gam_lut3: (),
    _reserved35: [u8; 0x04],
    gam_lut4: (),
    _reserved36: [u8; 0x04],
    gam_lut5: (),
    _reserved37: [u8; 0x04],
    gam_lut6: (),
    _reserved38: [u8; 0x04],
    gam_lut7: (),
    _reserved39: [u8; 0x04],
    gam_lut8: (),
    _reserved40: [u8; 0x04],
    gam_area1: (),
    _reserved41: [u8; 0x04],
    gam_area2: (),
    _reserved42: [u8; 0x04],
    gam_area3: (),
    _reserved43: [u8; 0x04],
    gam_area4: (),
    _reserved44: [u8; 0x04],
    gam_area5: (),
    _reserved45: [u8; 0x88],
    out_vlatch: OUT_VLATCH,
    out_set: OUT_SET,
    out_bright1: OUT_BRIGHT1,
    out_bright2: OUT_BRIGHT2,
    out_contrast: OUT_CONTRAST,
    out_pdtha: OUT_PDTHA,
    _reserved51: [u8; 0x0c],
    out_clkphase: OUT_CLKPHASE,
    _reserved52: [u8; 0x1c],
    tcon_tim: TCON_TIM,
    tcon_stv1: (),
    _reserved54: [u8; 0x04],
    tcon_stv2: (),
    _reserved55: [u8; 0x0c],
    tcon_sth1: (),
    _reserved56: [u8; 0x04],
    tcon_sth2: (),
    _reserved57: [u8; 0x0c],
    tcon_de: TCON_DE,
    _reserved58: [u8; 0x14],
    syscnt_dtcten: SYSCNT_DTCTEN,
    syscnt_inten: SYSCNT_INTEN,
    syscnt_stclr: SYSCNT_STCLR,
    syscnt_stmon: SYSCNT_STMON,
    syscnt_panel_clk: SYSCNT_PANEL_CLK,
}
impl RegisterBlock {
    ///0x00..0x400 - Color Palette 0 Plane for Graphics 1 Plane
    #[inline(always)]
    pub const fn gr1_clut0(&self, n: usize) -> &GR1_CLUT0 {
        &self.gr1_clut0[n]
    }
    ///Iterator for array of:
    ///0x00..0x400 - Color Palette 0 Plane for Graphics 1 Plane
    #[inline(always)]
    pub fn gr1_clut0_iter(&self) -> impl Iterator<Item = &GR1_CLUT0> {
        self.gr1_clut0.iter()
    }
    ///0x400..0x800 - Color Palette 1 Plane for Graphics 1 Plane
    #[inline(always)]
    pub const fn gr1_clut1(&self, n: usize) -> &GR1_CLUT1 {
        &self.gr1_clut1[n]
    }
    ///Iterator for array of:
    ///0x400..0x800 - Color Palette 1 Plane for Graphics 1 Plane
    #[inline(always)]
    pub fn gr1_clut1_iter(&self) -> impl Iterator<Item = &GR1_CLUT1> {
        self.gr1_clut1.iter()
    }
    ///0x800..0xc00 - Color Palette 0 Plane for Graphics 2 Plane
    #[inline(always)]
    pub const fn gr2_clut0(&self, n: usize) -> &GR2_CLUT0 {
        &self.gr2_clut0[n]
    }
    ///Iterator for array of:
    ///0x800..0xc00 - Color Palette 0 Plane for Graphics 2 Plane
    #[inline(always)]
    pub fn gr2_clut0_iter(&self) -> impl Iterator<Item = &GR2_CLUT0> {
        self.gr2_clut0.iter()
    }
    ///0xc00..0x1000 - Color Palette 1 Plane for Graphics 2 Plane
    #[inline(always)]
    pub const fn gr2_clut1(&self, n: usize) -> &GR2_CLUT1 {
        &self.gr2_clut1[n]
    }
    ///Iterator for array of:
    ///0xc00..0x1000 - Color Palette 1 Plane for Graphics 2 Plane
    #[inline(always)]
    pub fn gr2_clut1_iter(&self) -> impl Iterator<Item = &GR2_CLUT1> {
        self.gr2_clut1.iter()
    }
    ///0x1000 - Background Plane Setting Operation Control Register
    #[inline(always)]
    pub const fn bg_en(&self) -> &BG_EN {
        &self.bg_en
    }
    ///0x1004 - Background Plane Setting Free-Running Period Register
    #[inline(always)]
    pub const fn bg_peri(&self) -> &BG_PERI {
        &self.bg_peri
    }
    ///0x1008 - Background Plane Setting Synchronization Position Register
    #[inline(always)]
    pub const fn bg_sync(&self) -> &BG_SYNC {
        &self.bg_sync
    }
    ///0x100c - Background Plane Setting Full Image Vertical Size Register
    #[inline(always)]
    pub const fn bg_vsize(&self) -> &BG_VSIZE {
        &self.bg_vsize
    }
    ///0x1010 - Background Plane Setting Full Image Horizontal Size Register
    #[inline(always)]
    pub const fn bg_hsize(&self) -> &BG_HSIZE {
        &self.bg_hsize
    }
    ///0x1014 - Background Plane Setting Background Color Register
    #[inline(always)]
    pub const fn bg_bgc(&self) -> &BG_BGC {
        &self.bg_bgc
    }
    ///0x1018 - Background Plane Setting Status Monitor Register
    #[inline(always)]
    pub const fn bg_mon(&self) -> &BG_MON {
        &self.bg_mon
    }
    ///0x1100..0x1108 - Graphics %s Register Update Control Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_VEN` register.</div>
    #[inline(always)]
    pub const fn gr_ven(&self, n: usize) -> &GR_VEN {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4352)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1100..0x1108 - Graphics %s Register Update Control Register
    #[inline(always)]
    pub fn gr_ven_iter(&self) -> impl Iterator<Item = &GR_VEN> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4352)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1100 - Graphics 1 Register Update Control Register
    #[inline(always)]
    pub const fn gr1_ven(&self) -> &GR_VEN {
        self.gr_ven(0)
    }
    ///0x1200 - Graphics 2 Register Update Control Register
    #[inline(always)]
    pub const fn gr2_ven(&self) -> &GR_VEN {
        self.gr_ven(1)
    }
    ///0x1104..0x110c - Graphics %s Frame Buffer Read Control Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_FLMRD` register.</div>
    #[inline(always)]
    pub const fn gr_flmrd(&self, n: usize) -> &GR_FLMRD {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4356)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1104..0x110c - Graphics %s Frame Buffer Read Control Register
    #[inline(always)]
    pub fn gr_flmrd_iter(&self) -> impl Iterator<Item = &GR_FLMRD> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4356)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1104 - Graphics 1 Frame Buffer Read Control Register
    #[inline(always)]
    pub const fn gr1_flmrd(&self) -> &GR_FLMRD {
        self.gr_flmrd(0)
    }
    ///0x1204 - Graphics 2 Frame Buffer Read Control Register
    #[inline(always)]
    pub const fn gr2_flmrd(&self) -> &GR_FLMRD {
        self.gr_flmrd(1)
    }
    ///0x1108..0x1110 - Graphics %s Frame Buffer Control Register 1
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_FLM1` register.</div>
    #[inline(always)]
    pub const fn gr_flm1(&self, n: usize) -> &GR_FLM1 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4360)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1108..0x1110 - Graphics %s Frame Buffer Control Register 1
    #[inline(always)]
    pub fn gr_flm1_iter(&self) -> impl Iterator<Item = &GR_FLM1> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4360)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1108 - Graphics 1 Frame Buffer Control Register 1
    #[inline(always)]
    pub const fn gr1_flm1(&self) -> &GR_FLM1 {
        self.gr_flm1(0)
    }
    ///0x1208 - Graphics 2 Frame Buffer Control Register 1
    #[inline(always)]
    pub const fn gr2_flm1(&self) -> &GR_FLM1 {
        self.gr_flm1(1)
    }
    ///0x110c..0x1114 - Graphics %s Frame Buffer Control Register 2
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_FLM2` register.</div>
    #[inline(always)]
    pub const fn gr_flm2(&self, n: usize) -> &GR_FLM2 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4364)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x110c..0x1114 - Graphics %s Frame Buffer Control Register 2
    #[inline(always)]
    pub fn gr_flm2_iter(&self) -> impl Iterator<Item = &GR_FLM2> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4364)
                .add(256 * n)
                .cast()
        })
    }
    ///0x110c - Graphics 1 Frame Buffer Control Register 2
    #[inline(always)]
    pub const fn gr1_flm2(&self) -> &GR_FLM2 {
        self.gr_flm2(0)
    }
    ///0x120c - Graphics 2 Frame Buffer Control Register 2
    #[inline(always)]
    pub const fn gr2_flm2(&self) -> &GR_FLM2 {
        self.gr_flm2(1)
    }
    ///0x1110..0x1118 - Graphics %s Frame Buffer Control Register 3
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_FLM3` register.</div>
    #[inline(always)]
    pub const fn gr_flm3(&self, n: usize) -> &GR_FLM3 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4368)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1110..0x1118 - Graphics %s Frame Buffer Control Register 3
    #[inline(always)]
    pub fn gr_flm3_iter(&self) -> impl Iterator<Item = &GR_FLM3> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4368)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1110 - Graphics 1 Frame Buffer Control Register 3
    #[inline(always)]
    pub const fn gr1_flm3(&self) -> &GR_FLM3 {
        self.gr_flm3(0)
    }
    ///0x1210 - Graphics 2 Frame Buffer Control Register 3
    #[inline(always)]
    pub const fn gr2_flm3(&self) -> &GR_FLM3 {
        self.gr_flm3(1)
    }
    ///0x1118..0x1120 - Graphics %s Frame Buffer Control Register 5
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_FLM5` register.</div>
    #[inline(always)]
    pub const fn gr_flm5(&self, n: usize) -> &GR_FLM5 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4376)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1118..0x1120 - Graphics %s Frame Buffer Control Register 5
    #[inline(always)]
    pub fn gr_flm5_iter(&self) -> impl Iterator<Item = &GR_FLM5> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4376)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1118 - Graphics 1 Frame Buffer Control Register 5
    #[inline(always)]
    pub const fn gr1_flm5(&self) -> &GR_FLM5 {
        self.gr_flm5(0)
    }
    ///0x1218 - Graphics 2 Frame Buffer Control Register 5
    #[inline(always)]
    pub const fn gr2_flm5(&self) -> &GR_FLM5 {
        self.gr_flm5(1)
    }
    ///0x111c..0x1124 - Graphics %s Frame Buffer Control Register 6
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_FLM6` register.</div>
    #[inline(always)]
    pub const fn gr_flm6(&self, n: usize) -> &GR_FLM6 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4380)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x111c..0x1124 - Graphics %s Frame Buffer Control Register 6
    #[inline(always)]
    pub fn gr_flm6_iter(&self) -> impl Iterator<Item = &GR_FLM6> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4380)
                .add(256 * n)
                .cast()
        })
    }
    ///0x111c - Graphics 1 Frame Buffer Control Register 6
    #[inline(always)]
    pub const fn gr1_flm6(&self) -> &GR_FLM6 {
        self.gr_flm6(0)
    }
    ///0x121c - Graphics 2 Frame Buffer Control Register 6
    #[inline(always)]
    pub const fn gr2_flm6(&self) -> &GR_FLM6 {
        self.gr_flm6(1)
    }
    ///0x1120..0x1128 - Graphics %s Alpha Blending Control Register 1
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_AB1` register.</div>
    #[inline(always)]
    pub const fn gr_ab1(&self, n: usize) -> &GR_AB1 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4384)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1120..0x1128 - Graphics %s Alpha Blending Control Register 1
    #[inline(always)]
    pub fn gr_ab1_iter(&self) -> impl Iterator<Item = &GR_AB1> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4384)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1120 - Graphics 1 Alpha Blending Control Register 1
    #[inline(always)]
    pub const fn gr1_ab1(&self) -> &GR_AB1 {
        self.gr_ab1(0)
    }
    ///0x1220 - Graphics 2 Alpha Blending Control Register 1
    #[inline(always)]
    pub const fn gr2_ab1(&self) -> &GR_AB1 {
        self.gr_ab1(1)
    }
    ///0x1124..0x112c - Graphics %s Alpha Blending Control Register 2
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_AB2` register.</div>
    #[inline(always)]
    pub const fn gr_ab2(&self, n: usize) -> &GR_AB2 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4388)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1124..0x112c - Graphics %s Alpha Blending Control Register 2
    #[inline(always)]
    pub fn gr_ab2_iter(&self) -> impl Iterator<Item = &GR_AB2> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4388)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1124 - Graphics 1 Alpha Blending Control Register 2
    #[inline(always)]
    pub const fn gr1_ab2(&self) -> &GR_AB2 {
        self.gr_ab2(0)
    }
    ///0x1224 - Graphics 2 Alpha Blending Control Register 2
    #[inline(always)]
    pub const fn gr2_ab2(&self) -> &GR_AB2 {
        self.gr_ab2(1)
    }
    ///0x1128..0x1130 - Graphics %s Alpha Blending Control Register 3
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_AB3` register.</div>
    #[inline(always)]
    pub const fn gr_ab3(&self, n: usize) -> &GR_AB3 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4392)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1128..0x1130 - Graphics %s Alpha Blending Control Register 3
    #[inline(always)]
    pub fn gr_ab3_iter(&self) -> impl Iterator<Item = &GR_AB3> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4392)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1128 - Graphics 1 Alpha Blending Control Register 3
    #[inline(always)]
    pub const fn gr1_ab3(&self) -> &GR_AB3 {
        self.gr_ab3(0)
    }
    ///0x1228 - Graphics 2 Alpha Blending Control Register 3
    #[inline(always)]
    pub const fn gr2_ab3(&self) -> &GR_AB3 {
        self.gr_ab3(1)
    }
    ///0x112c..0x1134 - Graphics %s Alpha Blending Control Register 4
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_AB4` register.</div>
    #[inline(always)]
    pub const fn gr_ab4(&self, n: usize) -> &GR_AB4 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4396)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x112c..0x1134 - Graphics %s Alpha Blending Control Register 4
    #[inline(always)]
    pub fn gr_ab4_iter(&self) -> impl Iterator<Item = &GR_AB4> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4396)
                .add(256 * n)
                .cast()
        })
    }
    ///0x112c - Graphics 1 Alpha Blending Control Register 4
    #[inline(always)]
    pub const fn gr1_ab4(&self) -> &GR_AB4 {
        self.gr_ab4(0)
    }
    ///0x122c - Graphics 2 Alpha Blending Control Register 4
    #[inline(always)]
    pub const fn gr2_ab4(&self) -> &GR_AB4 {
        self.gr_ab4(1)
    }
    ///0x1130..0x1138 - Graphics %s Alpha Blending Control Register 5
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_AB5` register.</div>
    #[inline(always)]
    pub const fn gr_ab5(&self, n: usize) -> &GR_AB5 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4400)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1130..0x1138 - Graphics %s Alpha Blending Control Register 5
    #[inline(always)]
    pub fn gr_ab5_iter(&self) -> impl Iterator<Item = &GR_AB5> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4400)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1130 - Graphics 1 Alpha Blending Control Register 5
    #[inline(always)]
    pub const fn gr1_ab5(&self) -> &GR_AB5 {
        self.gr_ab5(0)
    }
    ///0x1230 - Graphics 2 Alpha Blending Control Register 5
    #[inline(always)]
    pub const fn gr2_ab5(&self) -> &GR_AB5 {
        self.gr_ab5(1)
    }
    ///0x1134..0x113c - Graphics %s Alpha Blending Control Register 6
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_AB6` register.</div>
    #[inline(always)]
    pub const fn gr_ab6(&self, n: usize) -> &GR_AB6 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4404)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1134..0x113c - Graphics %s Alpha Blending Control Register 6
    #[inline(always)]
    pub fn gr_ab6_iter(&self) -> impl Iterator<Item = &GR_AB6> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4404)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1134 - Graphics 1 Alpha Blending Control Register 6
    #[inline(always)]
    pub const fn gr1_ab6(&self) -> &GR_AB6 {
        self.gr_ab6(0)
    }
    ///0x1234 - Graphics 2 Alpha Blending Control Register 6
    #[inline(always)]
    pub const fn gr2_ab6(&self) -> &GR_AB6 {
        self.gr_ab6(1)
    }
    ///0x1138..0x1140 - Graphics %s Alpha Blending Control Register 7
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_AB7` register.</div>
    #[inline(always)]
    pub const fn gr_ab7(&self, n: usize) -> &GR_AB7 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4408)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1138..0x1140 - Graphics %s Alpha Blending Control Register 7
    #[inline(always)]
    pub fn gr_ab7_iter(&self) -> impl Iterator<Item = &GR_AB7> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4408)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1138 - Graphics 1 Alpha Blending Control Register 7
    #[inline(always)]
    pub const fn gr1_ab7(&self) -> &GR_AB7 {
        self.gr_ab7(0)
    }
    ///0x1238 - Graphics 2 Alpha Blending Control Register 7
    #[inline(always)]
    pub const fn gr2_ab7(&self) -> &GR_AB7 {
        self.gr_ab7(1)
    }
    ///0x113c..0x1144 - Graphics %s Alpha Blending Control Register 8
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_AB8` register.</div>
    #[inline(always)]
    pub const fn gr_ab8(&self, n: usize) -> &GR_AB8 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4412)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x113c..0x1144 - Graphics %s Alpha Blending Control Register 8
    #[inline(always)]
    pub fn gr_ab8_iter(&self) -> impl Iterator<Item = &GR_AB8> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4412)
                .add(256 * n)
                .cast()
        })
    }
    ///0x113c - Graphics 1 Alpha Blending Control Register 8
    #[inline(always)]
    pub const fn gr1_ab8(&self) -> &GR_AB8 {
        self.gr_ab8(0)
    }
    ///0x123c - Graphics 2 Alpha Blending Control Register 8
    #[inline(always)]
    pub const fn gr2_ab8(&self) -> &GR_AB8 {
        self.gr_ab8(1)
    }
    ///0x1140..0x1148 - Graphics %s Alpha Blending Control Register 9
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_AB9` register.</div>
    #[inline(always)]
    pub const fn gr_ab9(&self, n: usize) -> &GR_AB9 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4416)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1140..0x1148 - Graphics %s Alpha Blending Control Register 9
    #[inline(always)]
    pub fn gr_ab9_iter(&self) -> impl Iterator<Item = &GR_AB9> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4416)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1140 - Graphics 1 Alpha Blending Control Register 9
    #[inline(always)]
    pub const fn gr1_ab9(&self) -> &GR_AB9 {
        self.gr_ab9(0)
    }
    ///0x1240 - Graphics 2 Alpha Blending Control Register 9
    #[inline(always)]
    pub const fn gr2_ab9(&self) -> &GR_AB9 {
        self.gr_ab9(1)
    }
    ///0x114c..0x1154 - Graphics %s Background Color Control Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_BASE` register.</div>
    #[inline(always)]
    pub const fn gr_base(&self, n: usize) -> &GR_BASE {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4428)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x114c..0x1154 - Graphics %s Background Color Control Register
    #[inline(always)]
    pub fn gr_base_iter(&self) -> impl Iterator<Item = &GR_BASE> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4428)
                .add(256 * n)
                .cast()
        })
    }
    ///0x114c - Graphics 1 Background Color Control Register
    #[inline(always)]
    pub const fn gr1_base(&self) -> &GR_BASE {
        self.gr_base(0)
    }
    ///0x124c - Graphics 2 Background Color Control Register
    #[inline(always)]
    pub const fn gr2_base(&self) -> &GR_BASE {
        self.gr_base(1)
    }
    ///0x1150..0x1158 - Graphics %s CLUT Table Interrupt Control Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_CLUTINT` register.</div>
    #[inline(always)]
    pub const fn gr_clutint(&self, n: usize) -> &GR_CLUTINT {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4432)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1150..0x1158 - Graphics %s CLUT Table Interrupt Control Register
    #[inline(always)]
    pub fn gr_clutint_iter(&self) -> impl Iterator<Item = &GR_CLUTINT> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4432)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1150 - Graphics 1 CLUT Table Interrupt Control Register
    #[inline(always)]
    pub const fn gr1_clutint(&self) -> &GR_CLUTINT {
        self.gr_clutint(0)
    }
    ///0x1250 - Graphics 2 CLUT Table Interrupt Control Register
    #[inline(always)]
    pub const fn gr2_clutint(&self) -> &GR_CLUTINT {
        self.gr_clutint(1)
    }
    ///0x1154..0x115c - Graphics %s Status Monitor Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GR1_MON` register.</div>
    #[inline(always)]
    pub const fn gr_mon(&self, n: usize) -> &GR_MON {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4436)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1154..0x115c - Graphics %s Status Monitor Register
    #[inline(always)]
    pub fn gr_mon_iter(&self) -> impl Iterator<Item = &GR_MON> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4436)
                .add(256 * n)
                .cast()
        })
    }
    ///0x1154 - Graphics 1 Status Monitor Register
    #[inline(always)]
    pub const fn gr1_mon(&self) -> &GR_MON {
        self.gr_mon(0)
    }
    ///0x1254 - Graphics 2 Status Monitor Register
    #[inline(always)]
    pub const fn gr2_mon(&self) -> &GR_MON {
        self.gr_mon(1)
    }
    ///0x1300..0x130c - Gamma %s Register Update Control Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_LATCH` register.</div>
    #[inline(always)]
    pub const fn gam_latch(&self, n: usize) -> &GAM_LATCH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4864)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1300..0x130c - Gamma %s Register Update Control Register
    #[inline(always)]
    pub fn gam_latch_iter(&self) -> impl Iterator<Item = &GAM_LATCH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4864)
                .add(64 * n)
                .cast()
        })
    }
    ///0x1300 - Gamma G Register Update Control Register
    #[inline(always)]
    pub const fn gamg_latch(&self) -> &GAM_LATCH {
        self.gam_latch(0)
    }
    ///0x1340 - Gamma B Register Update Control Register
    #[inline(always)]
    pub const fn gamb_latch(&self) -> &GAM_LATCH {
        self.gam_latch(1)
    }
    ///0x1380 - Gamma R Register Update Control Register
    #[inline(always)]
    pub const fn gamr_latch(&self) -> &GAM_LATCH {
        self.gam_latch(2)
    }
    ///0x1304 - Gamma Correction Block Function Switch Register
    #[inline(always)]
    pub const fn gam_sw(&self) -> &GAM_SW {
        &self.gam_sw
    }
    ///0x1308..0x1314 - Gamma %s Correction Block Table Setting Register 1
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_LUT1` register.</div>
    #[inline(always)]
    pub const fn gam_lut1(&self, n: usize) -> &GAM_LUT1 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4872)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1308..0x1314 - Gamma %s Correction Block Table Setting Register 1
    #[inline(always)]
    pub fn gam_lut1_iter(&self) -> impl Iterator<Item = &GAM_LUT1> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4872)
                .add(64 * n)
                .cast()
        })
    }
    ///0x1308 - Gamma G Correction Block Table Setting Register 1
    #[inline(always)]
    pub const fn gamg_lut1(&self) -> &GAM_LUT1 {
        self.gam_lut1(0)
    }
    ///0x1348 - Gamma B Correction Block Table Setting Register 1
    #[inline(always)]
    pub const fn gamb_lut1(&self) -> &GAM_LUT1 {
        self.gam_lut1(1)
    }
    ///0x1388 - Gamma R Correction Block Table Setting Register 1
    #[inline(always)]
    pub const fn gamr_lut1(&self) -> &GAM_LUT1 {
        self.gam_lut1(2)
    }
    ///0x130c..0x1318 - Gamma %s Correction Block Table Setting Register 2
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_LUT2` register.</div>
    #[inline(always)]
    pub const fn gam_lut2(&self, n: usize) -> &GAM_LUT2 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4876)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x130c..0x1318 - Gamma %s Correction Block Table Setting Register 2
    #[inline(always)]
    pub fn gam_lut2_iter(&self) -> impl Iterator<Item = &GAM_LUT2> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4876)
                .add(64 * n)
                .cast()
        })
    }
    ///0x130c - Gamma G Correction Block Table Setting Register 2
    #[inline(always)]
    pub const fn gamg_lut2(&self) -> &GAM_LUT2 {
        self.gam_lut2(0)
    }
    ///0x134c - Gamma B Correction Block Table Setting Register 2
    #[inline(always)]
    pub const fn gamb_lut2(&self) -> &GAM_LUT2 {
        self.gam_lut2(1)
    }
    ///0x138c - Gamma R Correction Block Table Setting Register 2
    #[inline(always)]
    pub const fn gamr_lut2(&self) -> &GAM_LUT2 {
        self.gam_lut2(2)
    }
    ///0x1310..0x131c - Gamma %s Correction Block Table Setting Register 3
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_LUT3` register.</div>
    #[inline(always)]
    pub const fn gam_lut3(&self, n: usize) -> &GAM_LUT3 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4880)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1310..0x131c - Gamma %s Correction Block Table Setting Register 3
    #[inline(always)]
    pub fn gam_lut3_iter(&self) -> impl Iterator<Item = &GAM_LUT3> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4880)
                .add(64 * n)
                .cast()
        })
    }
    ///0x1310 - Gamma G Correction Block Table Setting Register 3
    #[inline(always)]
    pub const fn gamg_lut3(&self) -> &GAM_LUT3 {
        self.gam_lut3(0)
    }
    ///0x1350 - Gamma B Correction Block Table Setting Register 3
    #[inline(always)]
    pub const fn gamb_lut3(&self) -> &GAM_LUT3 {
        self.gam_lut3(1)
    }
    ///0x1390 - Gamma R Correction Block Table Setting Register 3
    #[inline(always)]
    pub const fn gamr_lut3(&self) -> &GAM_LUT3 {
        self.gam_lut3(2)
    }
    ///0x1314..0x1320 - Gamma %s Correction Block Table Setting Register 4
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_LUT4` register.</div>
    #[inline(always)]
    pub const fn gam_lut4(&self, n: usize) -> &GAM_LUT4 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4884)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1314..0x1320 - Gamma %s Correction Block Table Setting Register 4
    #[inline(always)]
    pub fn gam_lut4_iter(&self) -> impl Iterator<Item = &GAM_LUT4> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4884)
                .add(64 * n)
                .cast()
        })
    }
    ///0x1314 - Gamma G Correction Block Table Setting Register 4
    #[inline(always)]
    pub const fn gamg_lut4(&self) -> &GAM_LUT4 {
        self.gam_lut4(0)
    }
    ///0x1354 - Gamma B Correction Block Table Setting Register 4
    #[inline(always)]
    pub const fn gamb_lut4(&self) -> &GAM_LUT4 {
        self.gam_lut4(1)
    }
    ///0x1394 - Gamma R Correction Block Table Setting Register 4
    #[inline(always)]
    pub const fn gamr_lut4(&self) -> &GAM_LUT4 {
        self.gam_lut4(2)
    }
    ///0x1318..0x1324 - Gamma %s Correction Block Table Setting Register 5
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_LUT5` register.</div>
    #[inline(always)]
    pub const fn gam_lut5(&self, n: usize) -> &GAM_LUT5 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4888)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1318..0x1324 - Gamma %s Correction Block Table Setting Register 5
    #[inline(always)]
    pub fn gam_lut5_iter(&self) -> impl Iterator<Item = &GAM_LUT5> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4888)
                .add(64 * n)
                .cast()
        })
    }
    ///0x1318 - Gamma G Correction Block Table Setting Register 5
    #[inline(always)]
    pub const fn gamg_lut5(&self) -> &GAM_LUT5 {
        self.gam_lut5(0)
    }
    ///0x1358 - Gamma B Correction Block Table Setting Register 5
    #[inline(always)]
    pub const fn gamb_lut5(&self) -> &GAM_LUT5 {
        self.gam_lut5(1)
    }
    ///0x1398 - Gamma R Correction Block Table Setting Register 5
    #[inline(always)]
    pub const fn gamr_lut5(&self) -> &GAM_LUT5 {
        self.gam_lut5(2)
    }
    ///0x131c..0x1328 - Gamma %s Correction Block Table Setting Register 6
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_LUT6` register.</div>
    #[inline(always)]
    pub const fn gam_lut6(&self, n: usize) -> &GAM_LUT6 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4892)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x131c..0x1328 - Gamma %s Correction Block Table Setting Register 6
    #[inline(always)]
    pub fn gam_lut6_iter(&self) -> impl Iterator<Item = &GAM_LUT6> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4892)
                .add(64 * n)
                .cast()
        })
    }
    ///0x131c - Gamma G Correction Block Table Setting Register 6
    #[inline(always)]
    pub const fn gamg_lut6(&self) -> &GAM_LUT6 {
        self.gam_lut6(0)
    }
    ///0x135c - Gamma B Correction Block Table Setting Register 6
    #[inline(always)]
    pub const fn gamb_lut6(&self) -> &GAM_LUT6 {
        self.gam_lut6(1)
    }
    ///0x139c - Gamma R Correction Block Table Setting Register 6
    #[inline(always)]
    pub const fn gamr_lut6(&self) -> &GAM_LUT6 {
        self.gam_lut6(2)
    }
    ///0x1320..0x132c - Gamma %s Correction Block Table Setting Register 7
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_LUT7` register.</div>
    #[inline(always)]
    pub const fn gam_lut7(&self, n: usize) -> &GAM_LUT7 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4896)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1320..0x132c - Gamma %s Correction Block Table Setting Register 7
    #[inline(always)]
    pub fn gam_lut7_iter(&self) -> impl Iterator<Item = &GAM_LUT7> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4896)
                .add(64 * n)
                .cast()
        })
    }
    ///0x1320 - Gamma G Correction Block Table Setting Register 7
    #[inline(always)]
    pub const fn gamg_lut7(&self) -> &GAM_LUT7 {
        self.gam_lut7(0)
    }
    ///0x1360 - Gamma B Correction Block Table Setting Register 7
    #[inline(always)]
    pub const fn gamb_lut7(&self) -> &GAM_LUT7 {
        self.gam_lut7(1)
    }
    ///0x13a0 - Gamma R Correction Block Table Setting Register 7
    #[inline(always)]
    pub const fn gamr_lut7(&self) -> &GAM_LUT7 {
        self.gam_lut7(2)
    }
    ///0x1324..0x1330 - Gamma %s Correction Block Table Setting Register 8
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_LUT8` register.</div>
    #[inline(always)]
    pub const fn gam_lut8(&self, n: usize) -> &GAM_LUT8 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4900)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1324..0x1330 - Gamma %s Correction Block Table Setting Register 8
    #[inline(always)]
    pub fn gam_lut8_iter(&self) -> impl Iterator<Item = &GAM_LUT8> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4900)
                .add(64 * n)
                .cast()
        })
    }
    ///0x1324 - Gamma G Correction Block Table Setting Register 8
    #[inline(always)]
    pub const fn gamg_lut8(&self) -> &GAM_LUT8 {
        self.gam_lut8(0)
    }
    ///0x1364 - Gamma B Correction Block Table Setting Register 8
    #[inline(always)]
    pub const fn gamb_lut8(&self) -> &GAM_LUT8 {
        self.gam_lut8(1)
    }
    ///0x13a4 - Gamma R Correction Block Table Setting Register 8
    #[inline(always)]
    pub const fn gamr_lut8(&self) -> &GAM_LUT8 {
        self.gam_lut8(2)
    }
    ///0x1328..0x1334 - Gamma %s Correction Block Area Setting Register 1
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_AREA1` register.</div>
    #[inline(always)]
    pub const fn gam_area1(&self, n: usize) -> &GAM_AREA1 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4904)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1328..0x1334 - Gamma %s Correction Block Area Setting Register 1
    #[inline(always)]
    pub fn gam_area1_iter(&self) -> impl Iterator<Item = &GAM_AREA1> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4904)
                .add(64 * n)
                .cast()
        })
    }
    ///0x1328 - Gamma G Correction Block Area Setting Register 1
    #[inline(always)]
    pub const fn gamg_area1(&self) -> &GAM_AREA1 {
        self.gam_area1(0)
    }
    ///0x1368 - Gamma B Correction Block Area Setting Register 1
    #[inline(always)]
    pub const fn gamb_area1(&self) -> &GAM_AREA1 {
        self.gam_area1(1)
    }
    ///0x13a8 - Gamma R Correction Block Area Setting Register 1
    #[inline(always)]
    pub const fn gamr_area1(&self) -> &GAM_AREA1 {
        self.gam_area1(2)
    }
    ///0x132c..0x1338 - Gamma %s Correction Block Area Setting Register 2
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_AREA2` register.</div>
    #[inline(always)]
    pub const fn gam_area2(&self, n: usize) -> &GAM_AREA2 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4908)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x132c..0x1338 - Gamma %s Correction Block Area Setting Register 2
    #[inline(always)]
    pub fn gam_area2_iter(&self) -> impl Iterator<Item = &GAM_AREA2> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4908)
                .add(64 * n)
                .cast()
        })
    }
    ///0x132c - Gamma G Correction Block Area Setting Register 2
    #[inline(always)]
    pub const fn gamg_area2(&self) -> &GAM_AREA2 {
        self.gam_area2(0)
    }
    ///0x136c - Gamma B Correction Block Area Setting Register 2
    #[inline(always)]
    pub const fn gamb_area2(&self) -> &GAM_AREA2 {
        self.gam_area2(1)
    }
    ///0x13ac - Gamma R Correction Block Area Setting Register 2
    #[inline(always)]
    pub const fn gamr_area2(&self) -> &GAM_AREA2 {
        self.gam_area2(2)
    }
    ///0x1330..0x133c - Gamma %s Correction Block Area Setting Register 3
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_AREA3` register.</div>
    #[inline(always)]
    pub const fn gam_area3(&self, n: usize) -> &GAM_AREA3 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4912)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1330..0x133c - Gamma %s Correction Block Area Setting Register 3
    #[inline(always)]
    pub fn gam_area3_iter(&self) -> impl Iterator<Item = &GAM_AREA3> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4912)
                .add(64 * n)
                .cast()
        })
    }
    ///0x1330 - Gamma G Correction Block Area Setting Register 3
    #[inline(always)]
    pub const fn gamg_area3(&self) -> &GAM_AREA3 {
        self.gam_area3(0)
    }
    ///0x1370 - Gamma B Correction Block Area Setting Register 3
    #[inline(always)]
    pub const fn gamb_area3(&self) -> &GAM_AREA3 {
        self.gam_area3(1)
    }
    ///0x13b0 - Gamma R Correction Block Area Setting Register 3
    #[inline(always)]
    pub const fn gamr_area3(&self) -> &GAM_AREA3 {
        self.gam_area3(2)
    }
    ///0x1334..0x1340 - Gamma %s Correction Block Area Setting Register 4
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_AREA4` register.</div>
    #[inline(always)]
    pub const fn gam_area4(&self, n: usize) -> &GAM_AREA4 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4916)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1334..0x1340 - Gamma %s Correction Block Area Setting Register 4
    #[inline(always)]
    pub fn gam_area4_iter(&self) -> impl Iterator<Item = &GAM_AREA4> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4916)
                .add(64 * n)
                .cast()
        })
    }
    ///0x1334 - Gamma G Correction Block Area Setting Register 4
    #[inline(always)]
    pub const fn gamg_area4(&self) -> &GAM_AREA4 {
        self.gam_area4(0)
    }
    ///0x1374 - Gamma B Correction Block Area Setting Register 4
    #[inline(always)]
    pub const fn gamb_area4(&self) -> &GAM_AREA4 {
        self.gam_area4(1)
    }
    ///0x13b4 - Gamma R Correction Block Area Setting Register 4
    #[inline(always)]
    pub const fn gamr_area4(&self) -> &GAM_AREA4 {
        self.gam_area4(2)
    }
    ///0x1338..0x1344 - Gamma %s Correction Block Area Setting Register 5
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `GAMG_AREA5` register.</div>
    #[inline(always)]
    pub const fn gam_area5(&self, n: usize) -> &GAM_AREA5 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4920)
                .add(64 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1338..0x1344 - Gamma %s Correction Block Area Setting Register 5
    #[inline(always)]
    pub fn gam_area5_iter(&self) -> impl Iterator<Item = &GAM_AREA5> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4920)
                .add(64 * n)
                .cast()
        })
    }
    ///0x1338 - Gamma G Correction Block Area Setting Register 5
    #[inline(always)]
    pub const fn gamg_area5(&self) -> &GAM_AREA5 {
        self.gam_area5(0)
    }
    ///0x1378 - Gamma B Correction Block Area Setting Register 5
    #[inline(always)]
    pub const fn gamb_area5(&self) -> &GAM_AREA5 {
        self.gam_area5(1)
    }
    ///0x13b8 - Gamma R Correction Block Area Setting Register 5
    #[inline(always)]
    pub const fn gamr_area5(&self) -> &GAM_AREA5 {
        self.gam_area5(2)
    }
    ///0x13c0 - Output Control Block Register Update Control Register
    #[inline(always)]
    pub const fn out_vlatch(&self) -> &OUT_VLATCH {
        &self.out_vlatch
    }
    ///0x13c4 - Output Control Block Output Interface Register
    #[inline(always)]
    pub const fn out_set(&self) -> &OUT_SET {
        &self.out_set
    }
    ///0x13c8 - Output Control Block Brightness Correction Register 1
    #[inline(always)]
    pub const fn out_bright1(&self) -> &OUT_BRIGHT1 {
        &self.out_bright1
    }
    ///0x13cc - Output Control Block Brightness Correction Register 2
    #[inline(always)]
    pub const fn out_bright2(&self) -> &OUT_BRIGHT2 {
        &self.out_bright2
    }
    ///0x13d0 - Output Control Block Contrast Correction Register
    #[inline(always)]
    pub const fn out_contrast(&self) -> &OUT_CONTRAST {
        &self.out_contrast
    }
    ///0x13d4 - Output Control Block Panel Dither Correction Register
    #[inline(always)]
    pub const fn out_pdtha(&self) -> &OUT_PDTHA {
        &self.out_pdtha
    }
    ///0x13e4 - Output Control Block Output Phase Control Register
    #[inline(always)]
    pub const fn out_clkphase(&self) -> &OUT_CLKPHASE {
        &self.out_clkphase
    }
    ///0x1404 - TCON Reference Timing Setting Register
    #[inline(always)]
    pub const fn tcon_tim(&self) -> &TCON_TIM {
        &self.tcon_tim
    }
    ///0x1408..0x1410 - TCON Vertical Timing Setting Register %s1
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `TCON_STVA1` register.</div>
    #[inline(always)]
    pub const fn tcon_stv1(&self, n: usize) -> &TCON_STV1 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(5128)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1408..0x1410 - TCON Vertical Timing Setting Register %s1
    #[inline(always)]
    pub fn tcon_stv1_iter(&self) -> impl Iterator<Item = &TCON_STV1> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(5128)
                .add(8 * n)
                .cast()
        })
    }
    ///0x1408 - TCON Vertical Timing Setting Register A1
    #[inline(always)]
    pub const fn tcon_stva1(&self) -> &TCON_STV1 {
        self.tcon_stv1(0)
    }
    ///0x1410 - TCON Vertical Timing Setting Register B1
    #[inline(always)]
    pub const fn tcon_stvb1(&self) -> &TCON_STV1 {
        self.tcon_stv1(1)
    }
    ///0x140c..0x1414 - TCON Vertical Timing Setting Register %s2
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `TCON_STVA2` register.</div>
    #[inline(always)]
    pub const fn tcon_stv2(&self, n: usize) -> &TCON_STV2 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(5132)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x140c..0x1414 - TCON Vertical Timing Setting Register %s2
    #[inline(always)]
    pub fn tcon_stv2_iter(&self) -> impl Iterator<Item = &TCON_STV2> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(5132)
                .add(8 * n)
                .cast()
        })
    }
    ///0x140c - TCON Vertical Timing Setting Register A2
    #[inline(always)]
    pub const fn tcon_stva2(&self) -> &TCON_STV2 {
        self.tcon_stv2(0)
    }
    ///0x1414 - TCON Vertical Timing Setting Register B2
    #[inline(always)]
    pub const fn tcon_stvb2(&self) -> &TCON_STV2 {
        self.tcon_stv2(1)
    }
    ///0x1418..0x1420 - TCON Horizontal Timing Setting Register STH%s1
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `TCON_STHA1` register.</div>
    #[inline(always)]
    pub const fn tcon_sth1(&self, n: usize) -> &TCON_STH1 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(5144)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1418..0x1420 - TCON Horizontal Timing Setting Register STH%s1
    #[inline(always)]
    pub fn tcon_sth1_iter(&self) -> impl Iterator<Item = &TCON_STH1> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(5144)
                .add(8 * n)
                .cast()
        })
    }
    ///0x1418 - TCON Horizontal Timing Setting Register STHA1
    #[inline(always)]
    pub const fn tcon_stha1(&self) -> &TCON_STH1 {
        self.tcon_sth1(0)
    }
    ///0x1420 - TCON Horizontal Timing Setting Register STHB1
    #[inline(always)]
    pub const fn tcon_sthb1(&self) -> &TCON_STH1 {
        self.tcon_sth1(1)
    }
    ///0x141c..0x1424 - TCON Horizontal Timing Setting Register STH%s2
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `TCON_STHA2` register.</div>
    #[inline(always)]
    pub const fn tcon_sth2(&self, n: usize) -> &TCON_STH2 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(5148)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x141c..0x1424 - TCON Horizontal Timing Setting Register STH%s2
    #[inline(always)]
    pub fn tcon_sth2_iter(&self) -> impl Iterator<Item = &TCON_STH2> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(5148)
                .add(8 * n)
                .cast()
        })
    }
    ///0x141c - TCON Horizontal Timing Setting Register STHA2
    #[inline(always)]
    pub const fn tcon_stha2(&self) -> &TCON_STH2 {
        self.tcon_sth2(0)
    }
    ///0x1424 - TCON Horizontal Timing Setting Register STHB2
    #[inline(always)]
    pub const fn tcon_sthb2(&self) -> &TCON_STH2 {
        self.tcon_sth2(1)
    }
    ///0x1428 - TCON Data Enable Polarity Setting Register
    #[inline(always)]
    pub const fn tcon_de(&self) -> &TCON_DE {
        &self.tcon_de
    }
    ///0x1440 - System Control Block State Detection Control Register
    #[inline(always)]
    pub const fn syscnt_dtcten(&self) -> &SYSCNT_DTCTEN {
        &self.syscnt_dtcten
    }
    ///0x1444 - System Control Block Interrupt Request Enable Control Register
    #[inline(always)]
    pub const fn syscnt_inten(&self) -> &SYSCNT_INTEN {
        &self.syscnt_inten
    }
    ///0x1448 - System Control Block Status Clear Register
    #[inline(always)]
    pub const fn syscnt_stclr(&self) -> &SYSCNT_STCLR {
        &self.syscnt_stclr
    }
    ///0x144c - System Control Block Status Monitor Register
    #[inline(always)]
    pub const fn syscnt_stmon(&self) -> &SYSCNT_STMON {
        &self.syscnt_stmon
    }
    ///0x1450 - System Control Block Version and Panel Clock Control Register
    #[inline(always)]
    pub const fn syscnt_panel_clk(&self) -> &SYSCNT_PANEL_CLK {
        &self.syscnt_panel_clk
    }
}
/**GR1_CLUT0 (rw) register accessor: Color Palette 0 Plane for Graphics 1 Plane

You can [`read`](crate::Reg::read) this register and get [`gr1_clut0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr1_clut0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr1_clut0`] module*/
pub type GR1_CLUT0 = crate::Reg<gr1_clut0::GR1_CLUT0_SPEC>;
///Color Palette 0 Plane for Graphics 1 Plane
pub mod gr1_clut0;
/**GR1_CLUT1 (rw) register accessor: Color Palette 1 Plane for Graphics 1 Plane

You can [`read`](crate::Reg::read) this register and get [`gr1_clut1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr1_clut1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr1_clut1`] module*/
pub type GR1_CLUT1 = crate::Reg<gr1_clut1::GR1_CLUT1_SPEC>;
///Color Palette 1 Plane for Graphics 1 Plane
pub mod gr1_clut1;
/**GR2_CLUT0 (rw) register accessor: Color Palette 0 Plane for Graphics 2 Plane

You can [`read`](crate::Reg::read) this register and get [`gr2_clut0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr2_clut0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr2_clut0`] module*/
pub type GR2_CLUT0 = crate::Reg<gr2_clut0::GR2_CLUT0_SPEC>;
///Color Palette 0 Plane for Graphics 2 Plane
pub mod gr2_clut0;
/**GR2_CLUT1 (rw) register accessor: Color Palette 1 Plane for Graphics 2 Plane

You can [`read`](crate::Reg::read) this register and get [`gr2_clut1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr2_clut1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr2_clut1`] module*/
pub type GR2_CLUT1 = crate::Reg<gr2_clut1::GR2_CLUT1_SPEC>;
///Color Palette 1 Plane for Graphics 2 Plane
pub mod gr2_clut1;
/**BG_EN (rw) register accessor: Background Plane Setting Operation Control Register

You can [`read`](crate::Reg::read) this register and get [`bg_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bg_en`] module*/
pub type BG_EN = crate::Reg<bg_en::BG_EN_SPEC>;
///Background Plane Setting Operation Control Register
pub mod bg_en;
/**BG_PERI (rw) register accessor: Background Plane Setting Free-Running Period Register

You can [`read`](crate::Reg::read) this register and get [`bg_peri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_peri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bg_peri`] module*/
pub type BG_PERI = crate::Reg<bg_peri::BG_PERI_SPEC>;
///Background Plane Setting Free-Running Period Register
pub mod bg_peri;
/**BG_SYNC (rw) register accessor: Background Plane Setting Synchronization Position Register

You can [`read`](crate::Reg::read) this register and get [`bg_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bg_sync`] module*/
pub type BG_SYNC = crate::Reg<bg_sync::BG_SYNC_SPEC>;
///Background Plane Setting Synchronization Position Register
pub mod bg_sync;
/**BG_VSIZE (rw) register accessor: Background Plane Setting Full Image Vertical Size Register

You can [`read`](crate::Reg::read) this register and get [`bg_vsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_vsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bg_vsize`] module*/
pub type BG_VSIZE = crate::Reg<bg_vsize::BG_VSIZE_SPEC>;
///Background Plane Setting Full Image Vertical Size Register
pub mod bg_vsize;
/**BG_HSIZE (rw) register accessor: Background Plane Setting Full Image Horizontal Size Register

You can [`read`](crate::Reg::read) this register and get [`bg_hsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_hsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bg_hsize`] module*/
pub type BG_HSIZE = crate::Reg<bg_hsize::BG_HSIZE_SPEC>;
///Background Plane Setting Full Image Horizontal Size Register
pub mod bg_hsize;
/**BG_BGC (rw) register accessor: Background Plane Setting Background Color Register

You can [`read`](crate::Reg::read) this register and get [`bg_bgc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_bgc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bg_bgc`] module*/
pub type BG_BGC = crate::Reg<bg_bgc::BG_BGC_SPEC>;
///Background Plane Setting Background Color Register
pub mod bg_bgc;
/**BG_MON (r) register accessor: Background Plane Setting Status Monitor Register

You can [`read`](crate::Reg::read) this register and get [`bg_mon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bg_mon`] module*/
pub type BG_MON = crate::Reg<bg_mon::BG_MON_SPEC>;
///Background Plane Setting Status Monitor Register
pub mod bg_mon;
/**GR_VEN (rw) register accessor: Graphics %s Register Update Control Register

You can [`read`](crate::Reg::read) this register and get [`gr_ven::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ven::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_ven`] module*/
pub type GR_VEN = crate::Reg<gr_ven::GR_VEN_SPEC>;
///Graphics %s Register Update Control Register
pub mod gr_ven;
/**GR_FLMRD (rw) register accessor: Graphics %s Frame Buffer Read Control Register

You can [`read`](crate::Reg::read) this register and get [`gr_flmrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_flmrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_flmrd`] module*/
pub type GR_FLMRD = crate::Reg<gr_flmrd::GR_FLMRD_SPEC>;
///Graphics %s Frame Buffer Read Control Register
pub mod gr_flmrd;
/**GR_FLM1 (rw) register accessor: Graphics %s Frame Buffer Control Register 1

You can [`read`](crate::Reg::read) this register and get [`gr_flm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_flm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_flm1`] module*/
pub type GR_FLM1 = crate::Reg<gr_flm1::GR_FLM1_SPEC>;
///Graphics %s Frame Buffer Control Register 1
pub mod gr_flm1;
/**GR_FLM2 (rw) register accessor: Graphics %s Frame Buffer Control Register 2

You can [`read`](crate::Reg::read) this register and get [`gr_flm2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_flm2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_flm2`] module*/
pub type GR_FLM2 = crate::Reg<gr_flm2::GR_FLM2_SPEC>;
///Graphics %s Frame Buffer Control Register 2
pub mod gr_flm2;
/**GR_FLM3 (rw) register accessor: Graphics %s Frame Buffer Control Register 3

You can [`read`](crate::Reg::read) this register and get [`gr_flm3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_flm3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_flm3`] module*/
pub type GR_FLM3 = crate::Reg<gr_flm3::GR_FLM3_SPEC>;
///Graphics %s Frame Buffer Control Register 3
pub mod gr_flm3;
/**GR_FLM5 (rw) register accessor: Graphics %s Frame Buffer Control Register 5

You can [`read`](crate::Reg::read) this register and get [`gr_flm5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_flm5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_flm5`] module*/
pub type GR_FLM5 = crate::Reg<gr_flm5::GR_FLM5_SPEC>;
///Graphics %s Frame Buffer Control Register 5
pub mod gr_flm5;
/**GR_FLM6 (rw) register accessor: Graphics %s Frame Buffer Control Register 6

You can [`read`](crate::Reg::read) this register and get [`gr_flm6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_flm6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_flm6`] module*/
pub type GR_FLM6 = crate::Reg<gr_flm6::GR_FLM6_SPEC>;
///Graphics %s Frame Buffer Control Register 6
pub mod gr_flm6;
/**GR_AB1 (rw) register accessor: Graphics %s Alpha Blending Control Register 1

You can [`read`](crate::Reg::read) this register and get [`gr_ab1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_ab1`] module*/
pub type GR_AB1 = crate::Reg<gr_ab1::GR_AB1_SPEC>;
///Graphics %s Alpha Blending Control Register 1
pub mod gr_ab1;
/**GR_AB2 (rw) register accessor: Graphics %s Alpha Blending Control Register 2

You can [`read`](crate::Reg::read) this register and get [`gr_ab2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_ab2`] module*/
pub type GR_AB2 = crate::Reg<gr_ab2::GR_AB2_SPEC>;
///Graphics %s Alpha Blending Control Register 2
pub mod gr_ab2;
/**GR_AB3 (rw) register accessor: Graphics %s Alpha Blending Control Register 3

You can [`read`](crate::Reg::read) this register and get [`gr_ab3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_ab3`] module*/
pub type GR_AB3 = crate::Reg<gr_ab3::GR_AB3_SPEC>;
///Graphics %s Alpha Blending Control Register 3
pub mod gr_ab3;
/**GR_AB4 (rw) register accessor: Graphics %s Alpha Blending Control Register 4

You can [`read`](crate::Reg::read) this register and get [`gr_ab4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_ab4`] module*/
pub type GR_AB4 = crate::Reg<gr_ab4::GR_AB4_SPEC>;
///Graphics %s Alpha Blending Control Register 4
pub mod gr_ab4;
/**GR_AB5 (rw) register accessor: Graphics %s Alpha Blending Control Register 5

You can [`read`](crate::Reg::read) this register and get [`gr_ab5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_ab5`] module*/
pub type GR_AB5 = crate::Reg<gr_ab5::GR_AB5_SPEC>;
///Graphics %s Alpha Blending Control Register 5
pub mod gr_ab5;
/**GR_AB6 (rw) register accessor: Graphics %s Alpha Blending Control Register 6

You can [`read`](crate::Reg::read) this register and get [`gr_ab6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_ab6`] module*/
pub type GR_AB6 = crate::Reg<gr_ab6::GR_AB6_SPEC>;
///Graphics %s Alpha Blending Control Register 6
pub mod gr_ab6;
/**GR_AB7 (rw) register accessor: Graphics %s Alpha Blending Control Register 7

You can [`read`](crate::Reg::read) this register and get [`gr_ab7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_ab7`] module*/
pub type GR_AB7 = crate::Reg<gr_ab7::GR_AB7_SPEC>;
///Graphics %s Alpha Blending Control Register 7
pub mod gr_ab7;
/**GR_AB8 (rw) register accessor: Graphics %s Alpha Blending Control Register 8

You can [`read`](crate::Reg::read) this register and get [`gr_ab8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_ab8`] module*/
pub type GR_AB8 = crate::Reg<gr_ab8::GR_AB8_SPEC>;
///Graphics %s Alpha Blending Control Register 8
pub mod gr_ab8;
/**GR_AB9 (rw) register accessor: Graphics %s Alpha Blending Control Register 9

You can [`read`](crate::Reg::read) this register and get [`gr_ab9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_ab9`] module*/
pub type GR_AB9 = crate::Reg<gr_ab9::GR_AB9_SPEC>;
///Graphics %s Alpha Blending Control Register 9
pub mod gr_ab9;
/**GR_BASE (rw) register accessor: Graphics %s Background Color Control Register

You can [`read`](crate::Reg::read) this register and get [`gr_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_base`] module*/
pub type GR_BASE = crate::Reg<gr_base::GR_BASE_SPEC>;
///Graphics %s Background Color Control Register
pub mod gr_base;
/**GR_CLUTINT (rw) register accessor: Graphics %s CLUT Table Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`gr_clutint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_clutint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_clutint`] module*/
pub type GR_CLUTINT = crate::Reg<gr_clutint::GR_CLUTINT_SPEC>;
///Graphics %s CLUT Table Interrupt Control Register
pub mod gr_clutint;
/**GR_MON (r) register accessor: Graphics %s Status Monitor Register

You can [`read`](crate::Reg::read) this register and get [`gr_mon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gr_mon`] module*/
pub type GR_MON = crate::Reg<gr_mon::GR_MON_SPEC>;
///Graphics %s Status Monitor Register
pub mod gr_mon;
/**GAM_LATCH (rw) register accessor: Gamma %s Register Update Control Register

You can [`read`](crate::Reg::read) this register and get [`gam_latch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_latch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_latch`] module*/
pub type GAM_LATCH = crate::Reg<gam_latch::GAM_LATCH_SPEC>;
///Gamma %s Register Update Control Register
pub mod gam_latch;
/**GAM_SW (rw) register accessor: Gamma Correction Block Function Switch Register

You can [`read`](crate::Reg::read) this register and get [`gam_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_sw`] module*/
pub type GAM_SW = crate::Reg<gam_sw::GAM_SW_SPEC>;
///Gamma Correction Block Function Switch Register
pub mod gam_sw;
/**GAM_LUT1 (rw) register accessor: Gamma %s Correction Block Table Setting Register 1

You can [`read`](crate::Reg::read) this register and get [`gam_lut1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_lut1`] module*/
pub type GAM_LUT1 = crate::Reg<gam_lut1::GAM_LUT1_SPEC>;
///Gamma %s Correction Block Table Setting Register 1
pub mod gam_lut1;
/**GAM_LUT2 (rw) register accessor: Gamma %s Correction Block Table Setting Register 2

You can [`read`](crate::Reg::read) this register and get [`gam_lut2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_lut2`] module*/
pub type GAM_LUT2 = crate::Reg<gam_lut2::GAM_LUT2_SPEC>;
///Gamma %s Correction Block Table Setting Register 2
pub mod gam_lut2;
/**GAM_LUT3 (rw) register accessor: Gamma %s Correction Block Table Setting Register 3

You can [`read`](crate::Reg::read) this register and get [`gam_lut3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_lut3`] module*/
pub type GAM_LUT3 = crate::Reg<gam_lut3::GAM_LUT3_SPEC>;
///Gamma %s Correction Block Table Setting Register 3
pub mod gam_lut3;
/**GAM_LUT4 (rw) register accessor: Gamma %s Correction Block Table Setting Register 4

You can [`read`](crate::Reg::read) this register and get [`gam_lut4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_lut4`] module*/
pub type GAM_LUT4 = crate::Reg<gam_lut4::GAM_LUT4_SPEC>;
///Gamma %s Correction Block Table Setting Register 4
pub mod gam_lut4;
/**GAM_LUT5 (rw) register accessor: Gamma %s Correction Block Table Setting Register 5

You can [`read`](crate::Reg::read) this register and get [`gam_lut5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_lut5`] module*/
pub type GAM_LUT5 = crate::Reg<gam_lut5::GAM_LUT5_SPEC>;
///Gamma %s Correction Block Table Setting Register 5
pub mod gam_lut5;
/**GAM_LUT6 (rw) register accessor: Gamma %s Correction Block Table Setting Register 6

You can [`read`](crate::Reg::read) this register and get [`gam_lut6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_lut6`] module*/
pub type GAM_LUT6 = crate::Reg<gam_lut6::GAM_LUT6_SPEC>;
///Gamma %s Correction Block Table Setting Register 6
pub mod gam_lut6;
/**GAM_LUT7 (rw) register accessor: Gamma %s Correction Block Table Setting Register 7

You can [`read`](crate::Reg::read) this register and get [`gam_lut7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_lut7`] module*/
pub type GAM_LUT7 = crate::Reg<gam_lut7::GAM_LUT7_SPEC>;
///Gamma %s Correction Block Table Setting Register 7
pub mod gam_lut7;
/**GAM_LUT8 (rw) register accessor: Gamma %s Correction Block Table Setting Register 8

You can [`read`](crate::Reg::read) this register and get [`gam_lut8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_lut8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_lut8`] module*/
pub type GAM_LUT8 = crate::Reg<gam_lut8::GAM_LUT8_SPEC>;
///Gamma %s Correction Block Table Setting Register 8
pub mod gam_lut8;
/**GAM_AREA1 (rw) register accessor: Gamma %s Correction Block Area Setting Register 1

You can [`read`](crate::Reg::read) this register and get [`gam_area1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_area1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_area1`] module*/
pub type GAM_AREA1 = crate::Reg<gam_area1::GAM_AREA1_SPEC>;
///Gamma %s Correction Block Area Setting Register 1
pub mod gam_area1;
/**GAM_AREA2 (rw) register accessor: Gamma %s Correction Block Area Setting Register 2

You can [`read`](crate::Reg::read) this register and get [`gam_area2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_area2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_area2`] module*/
pub type GAM_AREA2 = crate::Reg<gam_area2::GAM_AREA2_SPEC>;
///Gamma %s Correction Block Area Setting Register 2
pub mod gam_area2;
/**GAM_AREA3 (rw) register accessor: Gamma %s Correction Block Area Setting Register 3

You can [`read`](crate::Reg::read) this register and get [`gam_area3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_area3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_area3`] module*/
pub type GAM_AREA3 = crate::Reg<gam_area3::GAM_AREA3_SPEC>;
///Gamma %s Correction Block Area Setting Register 3
pub mod gam_area3;
/**GAM_AREA4 (rw) register accessor: Gamma %s Correction Block Area Setting Register 4

You can [`read`](crate::Reg::read) this register and get [`gam_area4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_area4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_area4`] module*/
pub type GAM_AREA4 = crate::Reg<gam_area4::GAM_AREA4_SPEC>;
///Gamma %s Correction Block Area Setting Register 4
pub mod gam_area4;
/**GAM_AREA5 (rw) register accessor: Gamma %s Correction Block Area Setting Register 5

You can [`read`](crate::Reg::read) this register and get [`gam_area5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gam_area5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gam_area5`] module*/
pub type GAM_AREA5 = crate::Reg<gam_area5::GAM_AREA5_SPEC>;
///Gamma %s Correction Block Area Setting Register 5
pub mod gam_area5;
/**OUT_VLATCH (rw) register accessor: Output Control Block Register Update Control Register

You can [`read`](crate::Reg::read) this register and get [`out_vlatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_vlatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_vlatch`] module*/
pub type OUT_VLATCH = crate::Reg<out_vlatch::OUT_VLATCH_SPEC>;
///Output Control Block Register Update Control Register
pub mod out_vlatch;
/**OUT_SET (rw) register accessor: Output Control Block Output Interface Register

You can [`read`](crate::Reg::read) this register and get [`out_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_set`] module*/
pub type OUT_SET = crate::Reg<out_set::OUT_SET_SPEC>;
///Output Control Block Output Interface Register
pub mod out_set;
/**OUT_BRIGHT1 (rw) register accessor: Output Control Block Brightness Correction Register 1

You can [`read`](crate::Reg::read) this register and get [`out_bright1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_bright1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_bright1`] module*/
pub type OUT_BRIGHT1 = crate::Reg<out_bright1::OUT_BRIGHT1_SPEC>;
///Output Control Block Brightness Correction Register 1
pub mod out_bright1;
/**OUT_BRIGHT2 (rw) register accessor: Output Control Block Brightness Correction Register 2

You can [`read`](crate::Reg::read) this register and get [`out_bright2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_bright2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_bright2`] module*/
pub type OUT_BRIGHT2 = crate::Reg<out_bright2::OUT_BRIGHT2_SPEC>;
///Output Control Block Brightness Correction Register 2
pub mod out_bright2;
/**OUT_CONTRAST (rw) register accessor: Output Control Block Contrast Correction Register

You can [`read`](crate::Reg::read) this register and get [`out_contrast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_contrast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_contrast`] module*/
pub type OUT_CONTRAST = crate::Reg<out_contrast::OUT_CONTRAST_SPEC>;
///Output Control Block Contrast Correction Register
pub mod out_contrast;
/**OUT_PDTHA (rw) register accessor: Output Control Block Panel Dither Correction Register

You can [`read`](crate::Reg::read) this register and get [`out_pdtha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_pdtha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_pdtha`] module*/
pub type OUT_PDTHA = crate::Reg<out_pdtha::OUT_PDTHA_SPEC>;
///Output Control Block Panel Dither Correction Register
pub mod out_pdtha;
/**OUT_CLKPHASE (rw) register accessor: Output Control Block Output Phase Control Register

You can [`read`](crate::Reg::read) this register and get [`out_clkphase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_clkphase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_clkphase`] module*/
pub type OUT_CLKPHASE = crate::Reg<out_clkphase::OUT_CLKPHASE_SPEC>;
///Output Control Block Output Phase Control Register
pub mod out_clkphase;
/**TCON_TIM (rw) register accessor: TCON Reference Timing Setting Register

You can [`read`](crate::Reg::read) this register and get [`tcon_tim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_tim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcon_tim`] module*/
pub type TCON_TIM = crate::Reg<tcon_tim::TCON_TIM_SPEC>;
///TCON Reference Timing Setting Register
pub mod tcon_tim;
/**TCON_STV1 (rw) register accessor: TCON Vertical Timing Setting Register %s1

You can [`read`](crate::Reg::read) this register and get [`tcon_stv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_stv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcon_stv1`] module*/
pub type TCON_STV1 = crate::Reg<tcon_stv1::TCON_STV1_SPEC>;
///TCON Vertical Timing Setting Register %s1
pub mod tcon_stv1;
/**TCON_STV2 (rw) register accessor: TCON Vertical Timing Setting Register %s2

You can [`read`](crate::Reg::read) this register and get [`tcon_stv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_stv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcon_stv2`] module*/
pub type TCON_STV2 = crate::Reg<tcon_stv2::TCON_STV2_SPEC>;
///TCON Vertical Timing Setting Register %s2
pub mod tcon_stv2;
/**TCON_STH1 (rw) register accessor: TCON Horizontal Timing Setting Register STH%s1

You can [`read`](crate::Reg::read) this register and get [`tcon_sth1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_sth1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcon_sth1`] module*/
pub type TCON_STH1 = crate::Reg<tcon_sth1::TCON_STH1_SPEC>;
///TCON Horizontal Timing Setting Register STH%s1
pub mod tcon_sth1;
/**TCON_STH2 (rw) register accessor: TCON Horizontal Timing Setting Register STH%s2

You can [`read`](crate::Reg::read) this register and get [`tcon_sth2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_sth2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcon_sth2`] module*/
pub type TCON_STH2 = crate::Reg<tcon_sth2::TCON_STH2_SPEC>;
///TCON Horizontal Timing Setting Register STH%s2
pub mod tcon_sth2;
/**TCON_DE (rw) register accessor: TCON Data Enable Polarity Setting Register

You can [`read`](crate::Reg::read) this register and get [`tcon_de::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_de::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcon_de`] module*/
pub type TCON_DE = crate::Reg<tcon_de::TCON_DE_SPEC>;
///TCON Data Enable Polarity Setting Register
pub mod tcon_de;
/**SYSCNT_DTCTEN (rw) register accessor: System Control Block State Detection Control Register

You can [`read`](crate::Reg::read) this register and get [`syscnt_dtcten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscnt_dtcten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syscnt_dtcten`] module*/
pub type SYSCNT_DTCTEN = crate::Reg<syscnt_dtcten::SYSCNT_DTCTEN_SPEC>;
///System Control Block State Detection Control Register
pub mod syscnt_dtcten;
/**SYSCNT_INTEN (rw) register accessor: System Control Block Interrupt Request Enable Control Register

You can [`read`](crate::Reg::read) this register and get [`syscnt_inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscnt_inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syscnt_inten`] module*/
pub type SYSCNT_INTEN = crate::Reg<syscnt_inten::SYSCNT_INTEN_SPEC>;
///System Control Block Interrupt Request Enable Control Register
pub mod syscnt_inten;
/**SYSCNT_STCLR (rw) register accessor: System Control Block Status Clear Register

You can [`read`](crate::Reg::read) this register and get [`syscnt_stclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscnt_stclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syscnt_stclr`] module*/
pub type SYSCNT_STCLR = crate::Reg<syscnt_stclr::SYSCNT_STCLR_SPEC>;
///System Control Block Status Clear Register
pub mod syscnt_stclr;
/**SYSCNT_STMON (r) register accessor: System Control Block Status Monitor Register

You can [`read`](crate::Reg::read) this register and get [`syscnt_stmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syscnt_stmon`] module*/
pub type SYSCNT_STMON = crate::Reg<syscnt_stmon::SYSCNT_STMON_SPEC>;
///System Control Block Status Monitor Register
pub mod syscnt_stmon;
/**SYSCNT_PANEL_CLK (rw) register accessor: System Control Block Version and Panel Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`syscnt_panel_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscnt_panel_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syscnt_panel_clk`] module*/
pub type SYSCNT_PANEL_CLK = crate::Reg<syscnt_panel_clk::SYSCNT_PANEL_CLK_SPEC>;
///System Control Block Version and Panel Clock Control Register
pub mod syscnt_panel_clk;
