///Register `COMPMDR` reader
pub type R = crate::R<COMPMDR_SPEC>;
///Register `COMPMDR` writer
pub type W = crate::W<COMPMDR_SPEC>;
/**ACMPLP0 Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0ENB_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<C0ENB_A> for bool {
    #[inline(always)]
    fn from(variant: C0ENB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C0ENB` reader - ACMPLP0 Operation Enable
pub type C0ENB_R = crate::BitReader<C0ENB_A>;
impl C0ENB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C0ENB_A {
        match self.bits {
            false => C0ENB_A::_0,
            true => C0ENB_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0ENB_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0ENB_A::_1
    }
}
///Field `C0ENB` writer - ACMPLP0 Operation Enable
pub type C0ENB_W<'a, REG> = crate::BitWriter<'a, REG, C0ENB_A>;
impl<'a, REG> C0ENB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0ENB_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0ENB_A::_1)
    }
}
/**ACMPLP0 Window Function Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0WDE_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<C0WDE_A> for bool {
    #[inline(always)]
    fn from(variant: C0WDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C0WDE` reader - ACMPLP0 Window Function Mode Enable
pub type C0WDE_R = crate::BitReader<C0WDE_A>;
impl C0WDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C0WDE_A {
        match self.bits {
            false => C0WDE_A::_0,
            true => C0WDE_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0WDE_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0WDE_A::_1
    }
}
///Field `C0WDE` writer - ACMPLP0 Window Function Mode Enable
pub type C0WDE_W<'a, REG> = crate::BitWriter<'a, REG, C0WDE_A>;
impl<'a, REG> C0WDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0WDE_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0WDE_A::_1)
    }
}
/**ACMPLP0 Reference Voltage Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0VRF_A {
    ///0: IVREF0
    _0 = 0,
    ///1: internal reference voltage (Vref)
    _1 = 1,
}
impl From<C0VRF_A> for bool {
    #[inline(always)]
    fn from(variant: C0VRF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C0VRF` reader - ACMPLP0 Reference Voltage Selection
pub type C0VRF_R = crate::BitReader<C0VRF_A>;
impl C0VRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C0VRF_A {
        match self.bits {
            false => C0VRF_A::_0,
            true => C0VRF_A::_1,
        }
    }
    ///IVREF0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0VRF_A::_0
    }
    ///internal reference voltage (Vref)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0VRF_A::_1
    }
}
///Field `C0VRF` writer - ACMPLP0 Reference Voltage Selection
pub type C0VRF_W<'a, REG> = crate::BitWriter<'a, REG, C0VRF_A>;
impl<'a, REG> C0VRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IVREF0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0VRF_A::_0)
    }
    ///internal reference voltage (Vref)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0VRF_A::_1)
    }
}
/**ACMPLP0 Monitor Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0MON_A {
    ///0: CMPIN0 < CMPREF0, CMPIN0 < internal reference voltage, or ACMPLP0 operation disabled.(When the window function is disabled)/CMPIN0 < VRFL, CMPIN0 > VRFH, or ACMPLP0 operation disabled.(When the window function is enabled)
    _0 = 0,
    ///1: CMPIN0 > CMPREF0, or CMPIN0 > internal reference voltage.(When the window function is disabled)/VRFL < CMPIN0 < VRFH.(When the window function is enabled)
    _1 = 1,
}
impl From<C0MON_A> for bool {
    #[inline(always)]
    fn from(variant: C0MON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C0MON` reader - ACMPLP0 Monitor Flag
pub type C0MON_R = crate::BitReader<C0MON_A>;
impl C0MON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C0MON_A {
        match self.bits {
            false => C0MON_A::_0,
            true => C0MON_A::_1,
        }
    }
    ///CMPIN0 < CMPREF0, CMPIN0 < internal reference voltage, or ACMPLP0 operation disabled.(When the window function is disabled)/CMPIN0 < VRFL, CMPIN0 > VRFH, or ACMPLP0 operation disabled.(When the window function is enabled)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0MON_A::_0
    }
    ///CMPIN0 > CMPREF0, or CMPIN0 > internal reference voltage.(When the window function is disabled)/VRFL < CMPIN0 < VRFH.(When the window function is enabled)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0MON_A::_1
    }
}
/**ACMPLP1 Operation Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1ENB_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<C1ENB_A> for bool {
    #[inline(always)]
    fn from(variant: C1ENB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C1ENB` reader - ACMPLP1 Operation Enable
pub type C1ENB_R = crate::BitReader<C1ENB_A>;
impl C1ENB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C1ENB_A {
        match self.bits {
            false => C1ENB_A::_0,
            true => C1ENB_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1ENB_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1ENB_A::_1
    }
}
///Field `C1ENB` writer - ACMPLP1 Operation Enable
pub type C1ENB_W<'a, REG> = crate::BitWriter<'a, REG, C1ENB_A>;
impl<'a, REG> C1ENB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1ENB_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1ENB_A::_1)
    }
}
/**ACMPLP1 Window Function Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1WDE_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<C1WDE_A> for bool {
    #[inline(always)]
    fn from(variant: C1WDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C1WDE` reader - ACMPLP1 Window Function Mode Enable
pub type C1WDE_R = crate::BitReader<C1WDE_A>;
impl C1WDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C1WDE_A {
        match self.bits {
            false => C1WDE_A::_0,
            true => C1WDE_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1WDE_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1WDE_A::_1
    }
}
///Field `C1WDE` writer - ACMPLP1 Window Function Mode Enable
pub type C1WDE_W<'a, REG> = crate::BitWriter<'a, REG, C1WDE_A>;
impl<'a, REG> C1WDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1WDE_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1WDE_A::_1)
    }
}
/**ACMPLP1 Reference Voltage Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1VRF_A {
    ///0: IVREF0 or IVREF1
    _0 = 0,
    ///1: internal reference voltage (Vref)
    _1 = 1,
}
impl From<C1VRF_A> for bool {
    #[inline(always)]
    fn from(variant: C1VRF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C1VRF` reader - ACMPLP1 Reference Voltage Selection
pub type C1VRF_R = crate::BitReader<C1VRF_A>;
impl C1VRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C1VRF_A {
        match self.bits {
            false => C1VRF_A::_0,
            true => C1VRF_A::_1,
        }
    }
    ///IVREF0 or IVREF1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1VRF_A::_0
    }
    ///internal reference voltage (Vref)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1VRF_A::_1
    }
}
///Field `C1VRF` writer - ACMPLP1 Reference Voltage Selection
pub type C1VRF_W<'a, REG> = crate::BitWriter<'a, REG, C1VRF_A>;
impl<'a, REG> C1VRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IVREF0 or IVREF1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1VRF_A::_0)
    }
    ///internal reference voltage (Vref)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1VRF_A::_1)
    }
}
/**ACMPLP1 Monitor Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1MON_A {
    ///0: CMPIN1 < CMPREF1, CMPIN1 < internal reference voltage, or ACMPLP1 operation disabled.(When the window function is disabled)/CMPIN1 < VRFL, CMPIN1 > VRFH, or ACMPLP1 operation disabled.(When the window function is enabled)
    _0 = 0,
    ///1: CMPIN1 > CMPREF1, or CMPIN1 > internal reference voltage.(When the window function is disabled)/VRFL < CMPIN1 < VRFH.(When the window function is enabled)
    _1 = 1,
}
impl From<C1MON_A> for bool {
    #[inline(always)]
    fn from(variant: C1MON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C1MON` reader - ACMPLP1 Monitor Flag
pub type C1MON_R = crate::BitReader<C1MON_A>;
impl C1MON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C1MON_A {
        match self.bits {
            false => C1MON_A::_0,
            true => C1MON_A::_1,
        }
    }
    ///CMPIN1 < CMPREF1, CMPIN1 < internal reference voltage, or ACMPLP1 operation disabled.(When the window function is disabled)/CMPIN1 < VRFL, CMPIN1 > VRFH, or ACMPLP1 operation disabled.(When the window function is enabled)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1MON_A::_0
    }
    ///CMPIN1 > CMPREF1, or CMPIN1 > internal reference voltage.(When the window function is disabled)/VRFL < CMPIN1 < VRFH.(When the window function is enabled)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1MON_A::_1
    }
}
impl R {
    ///Bit 0 - ACMPLP0 Operation Enable
    #[inline(always)]
    pub fn c0enb(&self) -> C0ENB_R {
        C0ENB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ACMPLP0 Window Function Mode Enable
    #[inline(always)]
    pub fn c0wde(&self) -> C0WDE_R {
        C0WDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ACMPLP0 Reference Voltage Selection
    #[inline(always)]
    pub fn c0vrf(&self) -> C0VRF_R {
        C0VRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ACMPLP0 Monitor Flag
    #[inline(always)]
    pub fn c0mon(&self) -> C0MON_R {
        C0MON_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ACMPLP1 Operation Enable
    #[inline(always)]
    pub fn c1enb(&self) -> C1ENB_R {
        C1ENB_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ACMPLP1 Window Function Mode Enable
    #[inline(always)]
    pub fn c1wde(&self) -> C1WDE_R {
        C1WDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ACMPLP1 Reference Voltage Selection
    #[inline(always)]
    pub fn c1vrf(&self) -> C1VRF_R {
        C1VRF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ACMPLP1 Monitor Flag
    #[inline(always)]
    pub fn c1mon(&self) -> C1MON_R {
        C1MON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ACMPLP0 Operation Enable
    #[inline(always)]
    pub fn c0enb(&mut self) -> C0ENB_W<COMPMDR_SPEC> {
        C0ENB_W::new(self, 0)
    }
    ///Bit 1 - ACMPLP0 Window Function Mode Enable
    #[inline(always)]
    pub fn c0wde(&mut self) -> C0WDE_W<COMPMDR_SPEC> {
        C0WDE_W::new(self, 1)
    }
    ///Bit 2 - ACMPLP0 Reference Voltage Selection
    #[inline(always)]
    pub fn c0vrf(&mut self) -> C0VRF_W<COMPMDR_SPEC> {
        C0VRF_W::new(self, 2)
    }
    ///Bit 4 - ACMPLP1 Operation Enable
    #[inline(always)]
    pub fn c1enb(&mut self) -> C1ENB_W<COMPMDR_SPEC> {
        C1ENB_W::new(self, 4)
    }
    ///Bit 5 - ACMPLP1 Window Function Mode Enable
    #[inline(always)]
    pub fn c1wde(&mut self) -> C1WDE_W<COMPMDR_SPEC> {
        C1WDE_W::new(self, 5)
    }
    ///Bit 6 - ACMPLP1 Reference Voltage Selection
    #[inline(always)]
    pub fn c1vrf(&mut self) -> C1VRF_W<COMPMDR_SPEC> {
        C1VRF_W::new(self, 6)
    }
}
/**ACMPLP Mode Setting Register

You can [`read`](crate::Reg::read) this register and get [`compmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct COMPMDR_SPEC;
impl crate::RegisterSpec for COMPMDR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`compmdr::R`](R) reader structure
impl crate::Readable for COMPMDR_SPEC {}
///`write(|w| ..)` method takes [`compmdr::W`](W) writer structure
impl crate::Writable for COMPMDR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMPMDR to value 0
impl crate::Resettable for COMPMDR_SPEC {}
