///Register `FRMNUM` reader
pub type R = crate::R<FRMNUM_SPEC>;
///Register `FRMNUM` writer
pub type W = crate::W<FRMNUM_SPEC>;
///Field `FRNM` reader - Frame Number.Indicate the latest frame number.
pub type FRNM_R = crate::FieldReader<u16>;
/**CRC Error Detection Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCE_A {
    ///0: No error
    _0 = 0,
    ///1: An error occurred
    _1 = 1,
}
impl From<CRCE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCE` reader - CRC Error Detection Status
pub type CRCE_R = crate::BitReader<CRCE_A>;
impl CRCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCE_A {
        match self.bits {
            false => CRCE_A::_0,
            true => CRCE_A::_1,
        }
    }
    ///No error
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCE_A::_0
    }
    ///An error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCE_A::_1
    }
}
///Field `CRCE` writer - CRC Error Detection Status
pub type CRCE_W<'a, REG> = crate::BitWriter<'a, REG, CRCE_A>;
impl<'a, REG> CRCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No error
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCE_A::_0)
    }
    ///An error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCE_A::_1)
    }
}
/**Overrun/Underrun Detection Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRN_A {
    ///0: No error
    _0 = 0,
    ///1: An error occurred
    _1 = 1,
}
impl From<OVRN_A> for bool {
    #[inline(always)]
    fn from(variant: OVRN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRN` reader - Overrun/Underrun Detection Status
pub type OVRN_R = crate::BitReader<OVRN_A>;
impl OVRN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRN_A {
        match self.bits {
            false => OVRN_A::_0,
            true => OVRN_A::_1,
        }
    }
    ///No error
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRN_A::_0
    }
    ///An error occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRN_A::_1
    }
}
///Field `OVRN` writer - Overrun/Underrun Detection Status
pub type OVRN_W<'a, REG> = crate::BitWriter<'a, REG, OVRN_A>;
impl<'a, REG> OVRN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No error
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRN_A::_0)
    }
    ///An error occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRN_A::_1)
    }
}
impl R {
    ///Bits 0:10 - Frame Number.Indicate the latest frame number.
    #[inline(always)]
    pub fn frnm(&self) -> FRNM_R {
        FRNM_R::new(self.bits & 0x07ff)
    }
    ///Bit 14 - CRC Error Detection Status
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Overrun/Underrun Detection Status
    #[inline(always)]
    pub fn ovrn(&self) -> OVRN_R {
        OVRN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 14 - CRC Error Detection Status
    #[inline(always)]
    pub fn crce(&mut self) -> CRCE_W<FRMNUM_SPEC> {
        CRCE_W::new(self, 14)
    }
    ///Bit 15 - Overrun/Underrun Detection Status
    #[inline(always)]
    pub fn ovrn(&mut self) -> OVRN_W<FRMNUM_SPEC> {
        OVRN_W::new(self, 15)
    }
}
/**Frame Number Register

You can [`read`](crate::Reg::read) this register and get [`frmnum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmnum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FRMNUM_SPEC;
impl crate::RegisterSpec for FRMNUM_SPEC {
    type Ux = u16;
}
///`read()` method returns [`frmnum::R`](R) reader structure
impl crate::Readable for FRMNUM_SPEC {}
///`write(|w| ..)` method takes [`frmnum::W`](W) writer structure
impl crate::Writable for FRMNUM_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FRMNUM to value 0
impl crate::Resettable for FRMNUM_SPEC {}
