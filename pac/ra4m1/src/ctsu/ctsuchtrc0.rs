///Register `CTSUCHTRC0` reader
pub type R = crate::R<CTSUCHTRC0_SPEC>;
///Register `CTSUCHTRC0` writer
pub type W = crate::W<CTSUCHTRC0_SPEC>;
/**CTSU Channel Transmit/Receive Control 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHTRC0_A {
    ///0: Reception
    _0 = 0,
    ///1: Transmission
    _1 = 1,
}
impl From<CTSUCHTRC0_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHTRC0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCHTRC0_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUCHTRC0_A {}
///Field `CTSUCHTRC0` reader - CTSU Channel Transmit/Receive Control 0
pub type CTSUCHTRC0_R = crate::FieldReader<CTSUCHTRC0_A>;
impl CTSUCHTRC0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTSUCHTRC0_A> {
        match self.bits {
            0 => Some(CTSUCHTRC0_A::_0),
            1 => Some(CTSUCHTRC0_A::_1),
            _ => None,
        }
    }
    ///Reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCHTRC0_A::_0
    }
    ///Transmission
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCHTRC0_A::_1
    }
}
///Field `CTSUCHTRC0` writer - CTSU Channel Transmit/Receive Control 0
pub type CTSUCHTRC0_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CTSUCHTRC0_A>;
impl<'a, REG> CTSUCHTRC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHTRC0_A::_0)
    }
    ///Transmission
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHTRC0_A::_1)
    }
}
impl R {
    ///Bits 0:7 - CTSU Channel Transmit/Receive Control 0
    #[inline(always)]
    pub fn ctsuchtrc0(&self) -> CTSUCHTRC0_R {
        CTSUCHTRC0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - CTSU Channel Transmit/Receive Control 0
    #[inline(always)]
    pub fn ctsuchtrc0(&mut self) -> CTSUCHTRC0_W<'_, CTSUCHTRC0_SPEC> {
        CTSUCHTRC0_W::new(self, 0)
    }
}
/**CTSU Channel Transmit/Receive Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHTRC0_SPEC;
impl crate::RegisterSpec for CTSUCHTRC0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchtrc0::R`](R) reader structure
impl crate::Readable for CTSUCHTRC0_SPEC {}
///`write(|w| ..)` method takes [`ctsuchtrc0::W`](W) writer structure
impl crate::Writable for CTSUCHTRC0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHTRC0 to value 0
impl crate::Resettable for CTSUCHTRC0_SPEC {}
