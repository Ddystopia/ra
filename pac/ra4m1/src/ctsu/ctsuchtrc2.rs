///Register `CTSUCHTRC2` reader
pub type R = crate::R<CTSUCHTRC2_SPEC>;
///Register `CTSUCHTRC2` writer
pub type W = crate::W<CTSUCHTRC2_SPEC>;
/**CTSU Channel Transmit/Receive Control 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHTRC2_A {
    ///0: Reception
    _0 = 0,
    ///1: Transmission
    _1 = 1,
}
impl From<CTSUCHTRC2_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHTRC2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCHTRC2_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUCHTRC2_A {}
///Field `CTSUCHTRC2` reader - CTSU Channel Transmit/Receive Control 2
pub type CTSUCHTRC2_R = crate::FieldReader<CTSUCHTRC2_A>;
impl CTSUCHTRC2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTSUCHTRC2_A> {
        match self.bits {
            0 => Some(CTSUCHTRC2_A::_0),
            1 => Some(CTSUCHTRC2_A::_1),
            _ => None,
        }
    }
    ///Reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCHTRC2_A::_0
    }
    ///Transmission
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCHTRC2_A::_1
    }
}
///Field `CTSUCHTRC2` writer - CTSU Channel Transmit/Receive Control 2
pub type CTSUCHTRC2_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CTSUCHTRC2_A>;
impl<'a, REG> CTSUCHTRC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHTRC2_A::_0)
    }
    ///Transmission
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHTRC2_A::_1)
    }
}
impl R {
    ///Bits 0:7 - CTSU Channel Transmit/Receive Control 2
    #[inline(always)]
    pub fn ctsuchtrc2(&self) -> CTSUCHTRC2_R {
        CTSUCHTRC2_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - CTSU Channel Transmit/Receive Control 2
    #[inline(always)]
    pub fn ctsuchtrc2(&mut self) -> CTSUCHTRC2_W<'_, CTSUCHTRC2_SPEC> {
        CTSUCHTRC2_W::new(self, 0)
    }
}
/**CTSU Channel Transmit/Receive Control Register 3

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHTRC2_SPEC;
impl crate::RegisterSpec for CTSUCHTRC2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchtrc2::R`](R) reader structure
impl crate::Readable for CTSUCHTRC2_SPEC {}
///`write(|w| ..)` method takes [`ctsuchtrc2::W`](W) writer structure
impl crate::Writable for CTSUCHTRC2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHTRC2 to value 0
impl crate::Resettable for CTSUCHTRC2_SPEC {}
