///Register `CTSUCHTRC1` reader
pub type R = crate::R<CTSUCHTRC1_SPEC>;
///Register `CTSUCHTRC1` writer
pub type W = crate::W<CTSUCHTRC1_SPEC>;
/**CTSU Channel Transmit/Receive Control 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHTRC1_A {
    ///0: Reception
    _0 = 0,
    ///1: Transmission
    _1 = 1,
}
impl From<CTSUCHTRC1_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHTRC1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCHTRC1_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUCHTRC1_A {}
///Field `CTSUCHTRC1` reader - CTSU Channel Transmit/Receive Control 1
pub type CTSUCHTRC1_R = crate::FieldReader<CTSUCHTRC1_A>;
impl CTSUCHTRC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTSUCHTRC1_A> {
        match self.bits {
            0 => Some(CTSUCHTRC1_A::_0),
            1 => Some(CTSUCHTRC1_A::_1),
            _ => None,
        }
    }
    ///Reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCHTRC1_A::_0
    }
    ///Transmission
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCHTRC1_A::_1
    }
}
///Field `CTSUCHTRC1` writer - CTSU Channel Transmit/Receive Control 1
pub type CTSUCHTRC1_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CTSUCHTRC1_A>;
impl<'a, REG> CTSUCHTRC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHTRC1_A::_0)
    }
    ///Transmission
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHTRC1_A::_1)
    }
}
impl R {
    ///Bits 0:7 - CTSU Channel Transmit/Receive Control 1
    #[inline(always)]
    pub fn ctsuchtrc1(&self) -> CTSUCHTRC1_R {
        CTSUCHTRC1_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - CTSU Channel Transmit/Receive Control 1
    #[inline(always)]
    pub fn ctsuchtrc1(&mut self) -> CTSUCHTRC1_W<'_, CTSUCHTRC1_SPEC> {
        CTSUCHTRC1_W::new(self, 0)
    }
}
/**CTSU Channel Transmit/Receive Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHTRC1_SPEC;
impl crate::RegisterSpec for CTSUCHTRC1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchtrc1::R`](R) reader structure
impl crate::Readable for CTSUCHTRC1_SPEC {}
///`write(|w| ..)` method takes [`ctsuchtrc1::W`](W) writer structure
impl crate::Writable for CTSUCHTRC1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHTRC1 to value 0
impl crate::Resettable for CTSUCHTRC1_SPEC {}
