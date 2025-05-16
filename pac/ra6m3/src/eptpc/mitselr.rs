///Register `MITSELR` reader
pub type R = crate::R<MITSELR_SPEC>;
///Register `MITSELR` writer
pub type W = crate::W<MITSELR_SPEC>;
/**Pulse Output Timer 0 MINT Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINTEN0_A {
    ///0: Output of rising edges by pulse output timer 0 is not reflected by the MIESR.CYC0 flag as a MINT interrupt source.
    _0 = 0,
    ///1: Output of rising edges by pulse output timer 0 is reflected by the MIESR.CYC0 flag as a MINT interrupt source.
    _1 = 1,
}
impl From<MINTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: MINTEN0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MINTEN0` reader - Pulse Output Timer 0 MINT Interrupt Output Enable
pub type MINTEN0_R = crate::BitReader<MINTEN0_A>;
impl MINTEN0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MINTEN0_A {
        match self.bits {
            false => MINTEN0_A::_0,
            true => MINTEN0_A::_1,
        }
    }
    ///Output of rising edges by pulse output timer 0 is not reflected by the MIESR.CYC0 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MINTEN0_A::_0
    }
    ///Output of rising edges by pulse output timer 0 is reflected by the MIESR.CYC0 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MINTEN0_A::_1
    }
}
///Field `MINTEN0` writer - Pulse Output Timer 0 MINT Interrupt Output Enable
pub type MINTEN0_W<'a, REG> = crate::BitWriter<'a, REG, MINTEN0_A>;
impl<'a, REG> MINTEN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output of rising edges by pulse output timer 0 is not reflected by the MIESR.CYC0 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MINTEN0_A::_0)
    }
    ///Output of rising edges by pulse output timer 0 is reflected by the MIESR.CYC0 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MINTEN0_A::_1)
    }
}
/**Pulse Output Timer 1 MINT Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINTEN1_A {
    ///0: Output of rising edges by pulse output timer 1 is not reflected by the MIESR.CYC1 flag as a MINT interrupt source.
    _0 = 0,
    ///1: Output of rising edges by pulse output timer 1 is reflected by the MIESR.CYC1 flag as a MINT interrupt source.
    _1 = 1,
}
impl From<MINTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: MINTEN1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MINTEN1` reader - Pulse Output Timer 1 MINT Interrupt Output Enable
pub type MINTEN1_R = crate::BitReader<MINTEN1_A>;
impl MINTEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MINTEN1_A {
        match self.bits {
            false => MINTEN1_A::_0,
            true => MINTEN1_A::_1,
        }
    }
    ///Output of rising edges by pulse output timer 1 is not reflected by the MIESR.CYC1 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MINTEN1_A::_0
    }
    ///Output of rising edges by pulse output timer 1 is reflected by the MIESR.CYC1 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MINTEN1_A::_1
    }
}
///Field `MINTEN1` writer - Pulse Output Timer 1 MINT Interrupt Output Enable
pub type MINTEN1_W<'a, REG> = crate::BitWriter<'a, REG, MINTEN1_A>;
impl<'a, REG> MINTEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output of rising edges by pulse output timer 1 is not reflected by the MIESR.CYC1 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MINTEN1_A::_0)
    }
    ///Output of rising edges by pulse output timer 1 is reflected by the MIESR.CYC1 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MINTEN1_A::_1)
    }
}
/**Pulse Output Timer 2 MINT Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINTEN2_A {
    ///0: Output of rising edges by pulse output timer 2 is not reflected by the MIESR.CYC2 flag as a MINT interrupt source.
    _0 = 0,
    ///1: Output of rising edges by pulse output timer 2 is reflected by the MIESR.CYC2 flag as a MINT interrupt source.
    _1 = 1,
}
impl From<MINTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: MINTEN2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MINTEN2` reader - Pulse Output Timer 2 MINT Interrupt Output Enable
pub type MINTEN2_R = crate::BitReader<MINTEN2_A>;
impl MINTEN2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MINTEN2_A {
        match self.bits {
            false => MINTEN2_A::_0,
            true => MINTEN2_A::_1,
        }
    }
    ///Output of rising edges by pulse output timer 2 is not reflected by the MIESR.CYC2 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MINTEN2_A::_0
    }
    ///Output of rising edges by pulse output timer 2 is reflected by the MIESR.CYC2 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MINTEN2_A::_1
    }
}
///Field `MINTEN2` writer - Pulse Output Timer 2 MINT Interrupt Output Enable
pub type MINTEN2_W<'a, REG> = crate::BitWriter<'a, REG, MINTEN2_A>;
impl<'a, REG> MINTEN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output of rising edges by pulse output timer 2 is not reflected by the MIESR.CYC2 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MINTEN2_A::_0)
    }
    ///Output of rising edges by pulse output timer 2 is reflected by the MIESR.CYC2 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MINTEN2_A::_1)
    }
}
/**Pulse Output Timer 3 MINT Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINTEN3_A {
    ///0: Output of rising edges by pulse output timer 3 is not reflected by the MIESR.CYC3 flag as a MINT interrupt source.
    _0 = 0,
    ///1: Output of rising edges by pulse output timer 3 is reflected by the MIESR.CYC3 flag as a MINT interrupt source.
    _1 = 1,
}
impl From<MINTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: MINTEN3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MINTEN3` reader - Pulse Output Timer 3 MINT Interrupt Output Enable
pub type MINTEN3_R = crate::BitReader<MINTEN3_A>;
impl MINTEN3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MINTEN3_A {
        match self.bits {
            false => MINTEN3_A::_0,
            true => MINTEN3_A::_1,
        }
    }
    ///Output of rising edges by pulse output timer 3 is not reflected by the MIESR.CYC3 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MINTEN3_A::_0
    }
    ///Output of rising edges by pulse output timer 3 is reflected by the MIESR.CYC3 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MINTEN3_A::_1
    }
}
///Field `MINTEN3` writer - Pulse Output Timer 3 MINT Interrupt Output Enable
pub type MINTEN3_W<'a, REG> = crate::BitWriter<'a, REG, MINTEN3_A>;
impl<'a, REG> MINTEN3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output of rising edges by pulse output timer 3 is not reflected by the MIESR.CYC3 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MINTEN3_A::_0)
    }
    ///Output of rising edges by pulse output timer 3 is reflected by the MIESR.CYC3 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MINTEN3_A::_1)
    }
}
/**Pulse Output Timer 4 MINT Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINTEN4_A {
    ///0: Output of rising edges by pulse output timer 4 is not reflected by the MIESR.CYC4 flag as a MINT interrupt source.
    _0 = 0,
    ///1: Output of rising edges by pulse output timer 4 is reflected by the MIESR.CYC4 flag as a MINT interrupt source.
    _1 = 1,
}
impl From<MINTEN4_A> for bool {
    #[inline(always)]
    fn from(variant: MINTEN4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MINTEN4` reader - Pulse Output Timer 4 MINT Interrupt Output Enable
pub type MINTEN4_R = crate::BitReader<MINTEN4_A>;
impl MINTEN4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MINTEN4_A {
        match self.bits {
            false => MINTEN4_A::_0,
            true => MINTEN4_A::_1,
        }
    }
    ///Output of rising edges by pulse output timer 4 is not reflected by the MIESR.CYC4 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MINTEN4_A::_0
    }
    ///Output of rising edges by pulse output timer 4 is reflected by the MIESR.CYC4 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MINTEN4_A::_1
    }
}
///Field `MINTEN4` writer - Pulse Output Timer 4 MINT Interrupt Output Enable
pub type MINTEN4_W<'a, REG> = crate::BitWriter<'a, REG, MINTEN4_A>;
impl<'a, REG> MINTEN4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output of rising edges by pulse output timer 4 is not reflected by the MIESR.CYC4 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MINTEN4_A::_0)
    }
    ///Output of rising edges by pulse output timer 4 is reflected by the MIESR.CYC4 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MINTEN4_A::_1)
    }
}
/**Pulse Output Timer 5 MINT Interrupt Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINTEN5_A {
    ///0: Output of rising edges by pulse output timer 5 is not reflected by the MIESR.CYC5 flag as a MINT interrupt source.
    _0 = 0,
    ///1: Output of rising edges by pulse output timer 5 is reflected by the MIESR.CYC5 flag as a MINT interrupt source.
    _1 = 1,
}
impl From<MINTEN5_A> for bool {
    #[inline(always)]
    fn from(variant: MINTEN5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MINTEN5` reader - Pulse Output Timer 5 MINT Interrupt Output Enable
pub type MINTEN5_R = crate::BitReader<MINTEN5_A>;
impl MINTEN5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MINTEN5_A {
        match self.bits {
            false => MINTEN5_A::_0,
            true => MINTEN5_A::_1,
        }
    }
    ///Output of rising edges by pulse output timer 5 is not reflected by the MIESR.CYC5 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MINTEN5_A::_0
    }
    ///Output of rising edges by pulse output timer 5 is reflected by the MIESR.CYC5 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MINTEN5_A::_1
    }
}
///Field `MINTEN5` writer - Pulse Output Timer 5 MINT Interrupt Output Enable
pub type MINTEN5_W<'a, REG> = crate::BitWriter<'a, REG, MINTEN5_A>;
impl<'a, REG> MINTEN5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output of rising edges by pulse output timer 5 is not reflected by the MIESR.CYC5 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MINTEN5_A::_0)
    }
    ///Output of rising edges by pulse output timer 5 is reflected by the MIESR.CYC5 flag as a MINT interrupt source.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MINTEN5_A::_1)
    }
}
impl R {
    ///Bit 0 - Pulse Output Timer 0 MINT Interrupt Output Enable
    #[inline(always)]
    pub fn minten0(&self) -> MINTEN0_R {
        MINTEN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pulse Output Timer 1 MINT Interrupt Output Enable
    #[inline(always)]
    pub fn minten1(&self) -> MINTEN1_R {
        MINTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pulse Output Timer 2 MINT Interrupt Output Enable
    #[inline(always)]
    pub fn minten2(&self) -> MINTEN2_R {
        MINTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pulse Output Timer 3 MINT Interrupt Output Enable
    #[inline(always)]
    pub fn minten3(&self) -> MINTEN3_R {
        MINTEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pulse Output Timer 4 MINT Interrupt Output Enable
    #[inline(always)]
    pub fn minten4(&self) -> MINTEN4_R {
        MINTEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pulse Output Timer 5 MINT Interrupt Output Enable
    #[inline(always)]
    pub fn minten5(&self) -> MINTEN5_R {
        MINTEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Pulse Output Timer 0 MINT Interrupt Output Enable
    #[inline(always)]
    pub fn minten0(&mut self) -> MINTEN0_W<MITSELR_SPEC> {
        MINTEN0_W::new(self, 0)
    }
    ///Bit 1 - Pulse Output Timer 1 MINT Interrupt Output Enable
    #[inline(always)]
    pub fn minten1(&mut self) -> MINTEN1_W<MITSELR_SPEC> {
        MINTEN1_W::new(self, 1)
    }
    ///Bit 2 - Pulse Output Timer 2 MINT Interrupt Output Enable
    #[inline(always)]
    pub fn minten2(&mut self) -> MINTEN2_W<MITSELR_SPEC> {
        MINTEN2_W::new(self, 2)
    }
    ///Bit 3 - Pulse Output Timer 3 MINT Interrupt Output Enable
    #[inline(always)]
    pub fn minten3(&mut self) -> MINTEN3_W<MITSELR_SPEC> {
        MINTEN3_W::new(self, 3)
    }
    ///Bit 4 - Pulse Output Timer 4 MINT Interrupt Output Enable
    #[inline(always)]
    pub fn minten4(&mut self) -> MINTEN4_W<MITSELR_SPEC> {
        MINTEN4_W::new(self, 4)
    }
    ///Bit 5 - Pulse Output Timer 5 MINT Interrupt Output Enable
    #[inline(always)]
    pub fn minten5(&mut self) -> MINTEN5_W<MITSELR_SPEC> {
        MINTEN5_W::new(self, 5)
    }
}
/**MINT Interrupt Request Timer Select Register

You can [`read`](crate::Reg::read) this register and get [`mitselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mitselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MITSELR_SPEC;
impl crate::RegisterSpec for MITSELR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mitselr::R`](R) reader structure
impl crate::Readable for MITSELR_SPEC {}
///`write(|w| ..)` method takes [`mitselr::W`](W) writer structure
impl crate::Writable for MITSELR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MITSELR to value 0
impl crate::Resettable for MITSELR_SPEC {}
