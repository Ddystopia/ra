///Register `CTSUCHTRC4` reader
pub type R = crate::R<CTSUCHTRC4_SPEC>;
///Register `CTSUCHTRC4` writer
pub type W = crate::W<CTSUCHTRC4_SPEC>;
/**CTSU Channel Transmit/Receive Control 4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHAC4_A {
    ///0: Reception
    _0 = 0,
    ///1: Transmission
    _1 = 1,
}
impl From<CTSUCHAC4_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHAC4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCHAC4_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUCHAC4_A {}
///Field `CTSUCHAC4` reader - CTSU Channel Transmit/Receive Control 4
pub type CTSUCHAC4_R = crate::FieldReader<CTSUCHAC4_A>;
impl CTSUCHAC4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTSUCHAC4_A> {
        match self.bits {
            0 => Some(CTSUCHAC4_A::_0),
            1 => Some(CTSUCHAC4_A::_1),
            _ => None,
        }
    }
    ///Reception
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCHAC4_A::_0
    }
    ///Transmission
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCHAC4_A::_1
    }
}
///Field `CTSUCHAC4` writer - CTSU Channel Transmit/Receive Control 4
pub type CTSUCHAC4_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CTSUCHAC4_A>;
impl<'a, REG> CTSUCHAC4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reception
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHAC4_A::_0)
    }
    ///Transmission
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHAC4_A::_1)
    }
}
impl R {
    ///Bits 0:3 - CTSU Channel Transmit/Receive Control 4
    #[inline(always)]
    pub fn ctsuchac4(&self) -> CTSUCHAC4_R {
        CTSUCHAC4_R::new(self.bits & 0x0f)
    }
}
impl W {
    ///Bits 0:3 - CTSU Channel Transmit/Receive Control 4
    #[inline(always)]
    pub fn ctsuchac4(&mut self) -> CTSUCHAC4_W<CTSUCHTRC4_SPEC> {
        CTSUCHAC4_W::new(self, 0)
    }
}
/**CTSU Channel Transmit/Receive Control Register 4

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHTRC4_SPEC;
impl crate::RegisterSpec for CTSUCHTRC4_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchtrc4::R`](R) reader structure
impl crate::Readable for CTSUCHTRC4_SPEC {}
///`write(|w| ..)` method takes [`ctsuchtrc4::W`](W) writer structure
impl crate::Writable for CTSUCHTRC4_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHTRC4 to value 0
impl crate::Resettable for CTSUCHTRC4_SPEC {}
