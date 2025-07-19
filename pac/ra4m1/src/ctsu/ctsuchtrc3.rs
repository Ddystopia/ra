///Register `CTSUCHTRC3` reader
pub type R = crate::R<CTSUCHTRC3_SPEC>;
///Register `CTSUCHTRC3` writer
pub type W = crate::W<CTSUCHTRC3_SPEC>;
/**CTSU Channel Transmit/Receive Control 3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHTRC3_A {
    ///0: Reception
    _0 = 0,
    ///1: Transmission
    _1 = 1,
}
impl From<CTSUCHTRC3_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHTRC3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCHTRC3_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUCHTRC3_A {}
///Field `CTSUCHTRC3` reader - CTSU Channel Transmit/Receive Control 3
pub type CTSUCHTRC3_R = crate::FieldReader<CTSUCHTRC3_A>;
impl CTSUCHTRC3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTSUCHTRC3_A> {
        match self.bits {
            0 => Some(CTSUCHTRC3_A::_0),
            1 => Some(CTSUCHTRC3_A::_1),
            _ => None,
        }
    }
    ///Reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCHTRC3_A::_0
    }
    ///Transmission
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCHTRC3_A::_1
    }
}
///Field `CTSUCHTRC3` writer - CTSU Channel Transmit/Receive Control 3
pub type CTSUCHTRC3_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CTSUCHTRC3_A>;
impl<'a, REG> CTSUCHTRC3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHTRC3_A::_0)
    }
    ///Transmission
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHTRC3_A::_1)
    }
}
impl R {
    ///Bits 0:7 - CTSU Channel Transmit/Receive Control 3
    #[inline(always)]
    pub fn ctsuchtrc3(&self) -> CTSUCHTRC3_R {
        CTSUCHTRC3_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - CTSU Channel Transmit/Receive Control 3
    #[inline(always)]
    pub fn ctsuchtrc3(&mut self) -> CTSUCHTRC3_W<'_, CTSUCHTRC3_SPEC> {
        CTSUCHTRC3_W::new(self, 0)
    }
}
/**CTSU Channel Transmit/Receive Control Register 3

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHTRC3_SPEC;
impl crate::RegisterSpec for CTSUCHTRC3_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchtrc3::R`](R) reader structure
impl crate::Readable for CTSUCHTRC3_SPEC {}
///`write(|w| ..)` method takes [`ctsuchtrc3::W`](W) writer structure
impl crate::Writable for CTSUCHTRC3_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHTRC3 to value 0
impl crate::Resettable for CTSUCHTRC3_SPEC {}
