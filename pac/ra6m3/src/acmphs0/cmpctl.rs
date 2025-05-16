///Register `CMPCTL` reader
pub type R = crate::R<CMPCTL_SPEC>;
///Register `CMPCTL` writer
pub type W = crate::W<CMPCTL_SPEC>;
/**Comparator output polarity selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINV_A {
    ///0: Comparator output not inverted
    _0 = 0,
    ///1: Comparator output inverted
    _1 = 1,
}
impl From<CINV_A> for bool {
    #[inline(always)]
    fn from(variant: CINV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CINV` reader - Comparator output polarity selection
pub type CINV_R = crate::BitReader<CINV_A>;
impl CINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CINV_A {
        match self.bits {
            false => CINV_A::_0,
            true => CINV_A::_1,
        }
    }
    ///Comparator output not inverted
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINV_A::_0
    }
    ///Comparator output inverted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINV_A::_1
    }
}
///Field `CINV` writer - Comparator output polarity selection
pub type CINV_W<'a, REG> = crate::BitWriter<'a, REG, CINV_A>;
impl<'a, REG> CINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator output not inverted
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CINV_A::_0)
    }
    ///Comparator output inverted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CINV_A::_1)
    }
}
/**Comparator output enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COE_A {
    ///0: Comparator output disabled (the output signal is low level).
    _0 = 0,
    ///1: Comparator output enabled
    _1 = 1,
}
impl From<COE_A> for bool {
    #[inline(always)]
    fn from(variant: COE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COE` reader - Comparator output enable
pub type COE_R = crate::BitReader<COE_A>;
impl COE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COE_A {
        match self.bits {
            false => COE_A::_0,
            true => COE_A::_1,
        }
    }
    ///Comparator output disabled (the output signal is low level).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COE_A::_0
    }
    ///Comparator output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COE_A::_1
    }
}
///Field `COE` writer - Comparator output enable
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG, COE_A>;
impl<'a, REG> COE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator output disabled (the output signal is low level).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(COE_A::_0)
    }
    ///Comparator output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(COE_A::_1)
    }
}
/**Interrupt Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTEN_A {
    ///0: Output via the Edge selector
    _0 = 0,
    ///1: Direct output
    _1 = 1,
}
impl From<CSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CSTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTEN` reader - Interrupt Select
pub type CSTEN_R = crate::BitReader<CSTEN_A>;
impl CSTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTEN_A {
        match self.bits {
            false => CSTEN_A::_0,
            true => CSTEN_A::_1,
        }
    }
    ///Output via the Edge selector
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTEN_A::_0
    }
    ///Direct output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTEN_A::_1
    }
}
///Field `CSTEN` writer - Interrupt Select
pub type CSTEN_W<'a, REG> = crate::BitWriter<'a, REG, CSTEN_A>;
impl<'a, REG> CSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output via the Edge selector
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTEN_A::_0)
    }
    ///Direct output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTEN_A::_1)
    }
}
/**Selection of valid edge (Edge selector)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CEG_A {
    ///0: No edge selection.
    _00 = 0,
    ///1: Rising edge selection.
    _01 = 1,
    ///2: Falling edge selection
    _10 = 2,
    ///3: Both-edge selection
    _11 = 3,
}
impl From<CEG_A> for u8 {
    #[inline(always)]
    fn from(variant: CEG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CEG_A {
    type Ux = u8;
}
impl crate::IsEnum for CEG_A {}
///Field `CEG` reader - Selection of valid edge (Edge selector)
pub type CEG_R = crate::FieldReader<CEG_A>;
impl CEG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CEG_A {
        match self.bits {
            0 => CEG_A::_00,
            1 => CEG_A::_01,
            2 => CEG_A::_10,
            3 => CEG_A::_11,
            _ => unreachable!(),
        }
    }
    ///No edge selection.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CEG_A::_00
    }
    ///Rising edge selection.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CEG_A::_01
    }
    ///Falling edge selection
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CEG_A::_10
    }
    ///Both-edge selection
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CEG_A::_11
    }
}
///Field `CEG` writer - Selection of valid edge (Edge selector)
pub type CEG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CEG_A, crate::Safe>;
impl<'a, REG> CEG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No edge selection.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CEG_A::_00)
    }
    ///Rising edge selection.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CEG_A::_01)
    }
    ///Falling edge selection
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CEG_A::_10)
    }
    ///Both-edge selection
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CEG_A::_11)
    }
}
/**Noise filter selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDFS_A {
    ///0: Noise filter not used.
    _00 = 0,
    ///1: Noise filter sampling frequency is 2^3/PCLKB.
    _01 = 1,
    ///2: Noise filter sampling frequency is 2^4/PCLKB.
    _10 = 2,
    ///3: Noise filter sampling frequency is 2^5/PCLKB.
    _11 = 3,
}
impl From<CDFS_A> for u8 {
    #[inline(always)]
    fn from(variant: CDFS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CDFS_A {
    type Ux = u8;
}
impl crate::IsEnum for CDFS_A {}
///Field `CDFS` reader - Noise filter selection
pub type CDFS_R = crate::FieldReader<CDFS_A>;
impl CDFS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CDFS_A {
        match self.bits {
            0 => CDFS_A::_00,
            1 => CDFS_A::_01,
            2 => CDFS_A::_10,
            3 => CDFS_A::_11,
            _ => unreachable!(),
        }
    }
    ///Noise filter not used.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CDFS_A::_00
    }
    ///Noise filter sampling frequency is 2^3/PCLKB.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CDFS_A::_01
    }
    ///Noise filter sampling frequency is 2^4/PCLKB.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CDFS_A::_10
    }
    ///Noise filter sampling frequency is 2^5/PCLKB.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CDFS_A::_11
    }
}
///Field `CDFS` writer - Noise filter selection
pub type CDFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CDFS_A, crate::Safe>;
impl<'a, REG> CDFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Noise filter not used.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CDFS_A::_00)
    }
    ///Noise filter sampling frequency is 2^3/PCLKB.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CDFS_A::_01)
    }
    ///Noise filter sampling frequency is 2^4/PCLKB.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CDFS_A::_10)
    }
    ///Noise filter sampling frequency is 2^5/PCLKB.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CDFS_A::_11)
    }
}
/**Comparator operation control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HCMPON_A {
    ///0: Operation stopped (the comparator outputs a low-level signal)
    _0 = 0,
    ///1: Operation enabled (input to the comparator pins is enabled
    _1 = 1,
}
impl From<HCMPON_A> for bool {
    #[inline(always)]
    fn from(variant: HCMPON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HCMPON` reader - Comparator operation control
pub type HCMPON_R = crate::BitReader<HCMPON_A>;
impl HCMPON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HCMPON_A {
        match self.bits {
            false => HCMPON_A::_0,
            true => HCMPON_A::_1,
        }
    }
    ///Operation stopped (the comparator outputs a low-level signal)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HCMPON_A::_0
    }
    ///Operation enabled (input to the comparator pins is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HCMPON_A::_1
    }
}
///Field `HCMPON` writer - Comparator operation control
pub type HCMPON_W<'a, REG> = crate::BitWriter<'a, REG, HCMPON_A>;
impl<'a, REG> HCMPON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operation stopped (the comparator outputs a low-level signal)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HCMPON_A::_0)
    }
    ///Operation enabled (input to the comparator pins is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HCMPON_A::_1)
    }
}
impl R {
    ///Bit 0 - Comparator output polarity selection
    #[inline(always)]
    pub fn cinv(&self) -> CINV_R {
        CINV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Comparator output enable
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt Select
    #[inline(always)]
    pub fn csten(&self) -> CSTEN_R {
        CSTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - Selection of valid edge (Edge selector)
    #[inline(always)]
    pub fn ceg(&self) -> CEG_R {
        CEG_R::new((self.bits >> 3) & 3)
    }
    ///Bits 5:6 - Noise filter selection
    #[inline(always)]
    pub fn cdfs(&self) -> CDFS_R {
        CDFS_R::new((self.bits >> 5) & 3)
    }
    ///Bit 7 - Comparator operation control
    #[inline(always)]
    pub fn hcmpon(&self) -> HCMPON_R {
        HCMPON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Comparator output polarity selection
    #[inline(always)]
    pub fn cinv(&mut self) -> CINV_W<CMPCTL_SPEC> {
        CINV_W::new(self, 0)
    }
    ///Bit 1 - Comparator output enable
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W<CMPCTL_SPEC> {
        COE_W::new(self, 1)
    }
    ///Bit 2 - Interrupt Select
    #[inline(always)]
    pub fn csten(&mut self) -> CSTEN_W<CMPCTL_SPEC> {
        CSTEN_W::new(self, 2)
    }
    ///Bits 3:4 - Selection of valid edge (Edge selector)
    #[inline(always)]
    pub fn ceg(&mut self) -> CEG_W<CMPCTL_SPEC> {
        CEG_W::new(self, 3)
    }
    ///Bits 5:6 - Noise filter selection
    #[inline(always)]
    pub fn cdfs(&mut self) -> CDFS_W<CMPCTL_SPEC> {
        CDFS_W::new(self, 5)
    }
    ///Bit 7 - Comparator operation control
    #[inline(always)]
    pub fn hcmpon(&mut self) -> HCMPON_W<CMPCTL_SPEC> {
        HCMPON_W::new(self, 7)
    }
}
/**Comparator Control Register

You can [`read`](crate::Reg::read) this register and get [`cmpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CMPCTL_SPEC;
impl crate::RegisterSpec for CMPCTL_SPEC {
    type Ux = u8;
}
///`read()` method returns [`cmpctl::R`](R) reader structure
impl crate::Readable for CMPCTL_SPEC {}
///`write(|w| ..)` method takes [`cmpctl::W`](W) writer structure
impl crate::Writable for CMPCTL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMPCTL to value 0
impl crate::Resettable for CMPCTL_SPEC {}
