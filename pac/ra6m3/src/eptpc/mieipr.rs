///Register `MIEIPR` reader
pub type R = crate::R<MIEIPR_SPEC>;
///Register `MIEIPR` writer
pub type W = crate::W<MIEIPR_SPEC>;
/**STCA Status Interrupt Request Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST_A {
    ///0: Prohibits the generation of MINT interrupt requests by the STCA status flag.
    _0 = 0,
    ///1: Permits the generation of MINT interrupt requests by the STCA status flag.
    _1 = 1,
}
impl From<ST_A> for bool {
    #[inline(always)]
    fn from(variant: ST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ST` reader - STCA Status Interrupt Request Permission
pub type ST_R = crate::BitReader<ST_A>;
impl ST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ST_A {
        match self.bits {
            false => ST_A::_0,
            true => ST_A::_1,
        }
    }
    ///Prohibits the generation of MINT interrupt requests by the STCA status flag.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ST_A::_0
    }
    ///Permits the generation of MINT interrupt requests by the STCA status flag.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ST_A::_1
    }
}
///Field `ST` writer - STCA Status Interrupt Request Permission
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG, ST_A>;
impl<'a, REG> ST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits the generation of MINT interrupt requests by the STCA status flag.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ST_A::_0)
    }
    ///Permits the generation of MINT interrupt requests by the STCA status flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ST_A::_1)
    }
}
/**SYNFP0 Status Interrupt Request Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SY0_A {
    ///0: Prohibits the generation of MINT interrupt requests by the SYNFP0 status flag.
    _0 = 0,
    ///1: Permits the generation of MINT interrupt requests by the SYNFP0 status flag.
    _1 = 1,
}
impl From<SY0_A> for bool {
    #[inline(always)]
    fn from(variant: SY0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SY0` reader - SYNFP0 Status Interrupt Request Permission
pub type SY0_R = crate::BitReader<SY0_A>;
impl SY0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SY0_A {
        match self.bits {
            false => SY0_A::_0,
            true => SY0_A::_1,
        }
    }
    ///Prohibits the generation of MINT interrupt requests by the SYNFP0 status flag.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SY0_A::_0
    }
    ///Permits the generation of MINT interrupt requests by the SYNFP0 status flag.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SY0_A::_1
    }
}
///Field `SY0` writer - SYNFP0 Status Interrupt Request Permission
pub type SY0_W<'a, REG> = crate::BitWriter<'a, REG, SY0_A>;
impl<'a, REG> SY0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits the generation of MINT interrupt requests by the SYNFP0 status flag.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SY0_A::_0)
    }
    ///Permits the generation of MINT interrupt requests by the SYNFP0 status flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SY0_A::_1)
    }
}
/**Pulse Output Timer 0 Rising Edge Detection Interrupt Request Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYC0_A {
    ///0: Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 0.
    _0 = 0,
    ///1: Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 0.
    _1 = 1,
}
impl From<CYC0_A> for bool {
    #[inline(always)]
    fn from(variant: CYC0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYC0` reader - Pulse Output Timer 0 Rising Edge Detection Interrupt Request Permission
pub type CYC0_R = crate::BitReader<CYC0_A>;
impl CYC0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYC0_A {
        match self.bits {
            false => CYC0_A::_0,
            true => CYC0_A::_1,
        }
    }
    ///Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYC0_A::_0
    }
    ///Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYC0_A::_1
    }
}
///Field `CYC0` writer - Pulse Output Timer 0 Rising Edge Detection Interrupt Request Permission
pub type CYC0_W<'a, REG> = crate::BitWriter<'a, REG, CYC0_A>;
impl<'a, REG> CYC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 0.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYC0_A::_0)
    }
    ///Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYC0_A::_1)
    }
}
/**Pulse Output Timer 1 Rising Edge Detection Interrupt Request Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYC1_A {
    ///0: Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 1.
    _0 = 0,
    ///1: Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 1.
    _1 = 1,
}
impl From<CYC1_A> for bool {
    #[inline(always)]
    fn from(variant: CYC1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYC1` reader - Pulse Output Timer 1 Rising Edge Detection Interrupt Request Permission
pub type CYC1_R = crate::BitReader<CYC1_A>;
impl CYC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYC1_A {
        match self.bits {
            false => CYC1_A::_0,
            true => CYC1_A::_1,
        }
    }
    ///Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 1.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYC1_A::_0
    }
    ///Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYC1_A::_1
    }
}
///Field `CYC1` writer - Pulse Output Timer 1 Rising Edge Detection Interrupt Request Permission
pub type CYC1_W<'a, REG> = crate::BitWriter<'a, REG, CYC1_A>;
impl<'a, REG> CYC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 1.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYC1_A::_0)
    }
    ///Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYC1_A::_1)
    }
}
/**Pulse Output Timer 2 Rising Edge Detection Interrupt Request Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYC2_A {
    ///0: Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 2.
    _0 = 0,
    ///1: Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 2.
    _1 = 1,
}
impl From<CYC2_A> for bool {
    #[inline(always)]
    fn from(variant: CYC2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYC2` reader - Pulse Output Timer 2 Rising Edge Detection Interrupt Request Permission
pub type CYC2_R = crate::BitReader<CYC2_A>;
impl CYC2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYC2_A {
        match self.bits {
            false => CYC2_A::_0,
            true => CYC2_A::_1,
        }
    }
    ///Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 2.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYC2_A::_0
    }
    ///Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 2.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYC2_A::_1
    }
}
///Field `CYC2` writer - Pulse Output Timer 2 Rising Edge Detection Interrupt Request Permission
pub type CYC2_W<'a, REG> = crate::BitWriter<'a, REG, CYC2_A>;
impl<'a, REG> CYC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 2.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYC2_A::_0)
    }
    ///Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 2.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYC2_A::_1)
    }
}
/**Pulse Output Timer 3 Rising Edge Detection Interrupt Request Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYC3_A {
    ///0: Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 3.
    _0 = 0,
    ///1: Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 3.
    _1 = 1,
}
impl From<CYC3_A> for bool {
    #[inline(always)]
    fn from(variant: CYC3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYC3` reader - Pulse Output Timer 3 Rising Edge Detection Interrupt Request Permission
pub type CYC3_R = crate::BitReader<CYC3_A>;
impl CYC3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYC3_A {
        match self.bits {
            false => CYC3_A::_0,
            true => CYC3_A::_1,
        }
    }
    ///Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 3.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYC3_A::_0
    }
    ///Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 3.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYC3_A::_1
    }
}
///Field `CYC3` writer - Pulse Output Timer 3 Rising Edge Detection Interrupt Request Permission
pub type CYC3_W<'a, REG> = crate::BitWriter<'a, REG, CYC3_A>;
impl<'a, REG> CYC3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 3.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYC3_A::_0)
    }
    ///Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 3.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYC3_A::_1)
    }
}
/**Pulse Output Timer 4 Rising Edge Detection Interrupt Request Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYC4_A {
    ///0: Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 4.
    _0 = 0,
    ///1: Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 4.
    _1 = 1,
}
impl From<CYC4_A> for bool {
    #[inline(always)]
    fn from(variant: CYC4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYC4` reader - Pulse Output Timer 4 Rising Edge Detection Interrupt Request Permission
pub type CYC4_R = crate::BitReader<CYC4_A>;
impl CYC4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYC4_A {
        match self.bits {
            false => CYC4_A::_0,
            true => CYC4_A::_1,
        }
    }
    ///Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 4.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYC4_A::_0
    }
    ///Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 4.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYC4_A::_1
    }
}
///Field `CYC4` writer - Pulse Output Timer 4 Rising Edge Detection Interrupt Request Permission
pub type CYC4_W<'a, REG> = crate::BitWriter<'a, REG, CYC4_A>;
impl<'a, REG> CYC4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 4.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYC4_A::_0)
    }
    ///Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 4.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYC4_A::_1)
    }
}
/**Pulse Output Timer 5 Rising Edge Detection Interrupt Request Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYC5_A {
    ///0: Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 5.
    _0 = 0,
    ///1: Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 5.
    _1 = 1,
}
impl From<CYC5_A> for bool {
    #[inline(always)]
    fn from(variant: CYC5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYC5` reader - Pulse Output Timer 5 Rising Edge Detection Interrupt Request Permission
pub type CYC5_R = crate::BitReader<CYC5_A>;
impl CYC5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYC5_A {
        match self.bits {
            false => CYC5_A::_0,
            true => CYC5_A::_1,
        }
    }
    ///Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 5.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYC5_A::_0
    }
    ///Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 5.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYC5_A::_1
    }
}
///Field `CYC5` writer - Pulse Output Timer 5 Rising Edge Detection Interrupt Request Permission
pub type CYC5_W<'a, REG> = crate::BitWriter<'a, REG, CYC5_A>;
impl<'a, REG> CYC5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 5.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYC5_A::_0)
    }
    ///Permits the generation of MINT interrupt requests in response to detection of a rising edge of pulse output timer 5.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYC5_A::_1)
    }
}
impl R {
    ///Bit 0 - STCA Status Interrupt Request Permission
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SYNFP0 Status Interrupt Request Permission
    #[inline(always)]
    pub fn sy0(&self) -> SY0_R {
        SY0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - Pulse Output Timer 0 Rising Edge Detection Interrupt Request Permission
    #[inline(always)]
    pub fn cyc0(&self) -> CYC0_R {
        CYC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Pulse Output Timer 1 Rising Edge Detection Interrupt Request Permission
    #[inline(always)]
    pub fn cyc1(&self) -> CYC1_R {
        CYC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Pulse Output Timer 2 Rising Edge Detection Interrupt Request Permission
    #[inline(always)]
    pub fn cyc2(&self) -> CYC2_R {
        CYC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Pulse Output Timer 3 Rising Edge Detection Interrupt Request Permission
    #[inline(always)]
    pub fn cyc3(&self) -> CYC3_R {
        CYC3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Pulse Output Timer 4 Rising Edge Detection Interrupt Request Permission
    #[inline(always)]
    pub fn cyc4(&self) -> CYC4_R {
        CYC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Pulse Output Timer 5 Rising Edge Detection Interrupt Request Permission
    #[inline(always)]
    pub fn cyc5(&self) -> CYC5_R {
        CYC5_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - STCA Status Interrupt Request Permission
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<MIEIPR_SPEC> {
        ST_W::new(self, 0)
    }
    ///Bit 1 - SYNFP0 Status Interrupt Request Permission
    #[inline(always)]
    pub fn sy0(&mut self) -> SY0_W<MIEIPR_SPEC> {
        SY0_W::new(self, 1)
    }
    ///Bit 16 - Pulse Output Timer 0 Rising Edge Detection Interrupt Request Permission
    #[inline(always)]
    pub fn cyc0(&mut self) -> CYC0_W<MIEIPR_SPEC> {
        CYC0_W::new(self, 16)
    }
    ///Bit 17 - Pulse Output Timer 1 Rising Edge Detection Interrupt Request Permission
    #[inline(always)]
    pub fn cyc1(&mut self) -> CYC1_W<MIEIPR_SPEC> {
        CYC1_W::new(self, 17)
    }
    ///Bit 18 - Pulse Output Timer 2 Rising Edge Detection Interrupt Request Permission
    #[inline(always)]
    pub fn cyc2(&mut self) -> CYC2_W<MIEIPR_SPEC> {
        CYC2_W::new(self, 18)
    }
    ///Bit 19 - Pulse Output Timer 3 Rising Edge Detection Interrupt Request Permission
    #[inline(always)]
    pub fn cyc3(&mut self) -> CYC3_W<MIEIPR_SPEC> {
        CYC3_W::new(self, 19)
    }
    ///Bit 20 - Pulse Output Timer 4 Rising Edge Detection Interrupt Request Permission
    #[inline(always)]
    pub fn cyc4(&mut self) -> CYC4_W<MIEIPR_SPEC> {
        CYC4_W::new(self, 20)
    }
    ///Bit 21 - Pulse Output Timer 5 Rising Edge Detection Interrupt Request Permission
    #[inline(always)]
    pub fn cyc5(&mut self) -> CYC5_W<MIEIPR_SPEC> {
        CYC5_W::new(self, 21)
    }
}
/**MINT Interrupt Request Permission Register

You can [`read`](crate::Reg::read) this register and get [`mieipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mieipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MIEIPR_SPEC;
impl crate::RegisterSpec for MIEIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mieipr::R`](R) reader structure
impl crate::Readable for MIEIPR_SPEC {}
///`write(|w| ..)` method takes [`mieipr::W`](W) writer structure
impl crate::Writable for MIEIPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MIEIPR to value 0
impl crate::Resettable for MIEIPR_SPEC {}
