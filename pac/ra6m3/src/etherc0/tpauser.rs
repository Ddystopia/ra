///Register `TPAUSER` reader
pub type R = crate::R<TPAUSER_SPEC>;
///Register `TPAUSER` writer
pub type W = crate::W<TPAUSER_SPEC>;
/**Automatic PAUSE Frame Retransmit Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TPAUSE_A {
    ///0: Number of retransmissions is unlimited
    _0X0000 = 0,
    ///1: Maximum number of retransmissions is (TPAUSE)
    OTHERS = 1,
}
impl From<TPAUSE_A> for u16 {
    #[inline(always)]
    fn from(variant: TPAUSE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPAUSE_A {
    type Ux = u16;
}
impl crate::IsEnum for TPAUSE_A {}
///Field `TPAUSE` reader - Automatic PAUSE Frame Retransmit Setting
pub type TPAUSE_R = crate::FieldReader<TPAUSE_A>;
impl TPAUSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TPAUSE_A {
        match self.bits {
            0 => TPAUSE_A::_0X0000,
            _ => TPAUSE_A::OTHERS,
        }
    }
    ///Number of retransmissions is unlimited
    #[inline(always)]
    pub fn is_0x0000(&self) -> bool {
        *self == TPAUSE_A::_0X0000
    }
    ///Maximum number of retransmissions is (TPAUSE)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), TPAUSE_A::OTHERS)
    }
}
///Field `TPAUSE` writer - Automatic PAUSE Frame Retransmit Setting
pub type TPAUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, TPAUSE_A, crate::Safe>;
impl<'a, REG> TPAUSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Number of retransmissions is unlimited
    #[inline(always)]
    pub fn _0x0000(self) -> &'a mut crate::W<REG> {
        self.variant(TPAUSE_A::_0X0000)
    }
    ///Maximum number of retransmissions is (TPAUSE)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(TPAUSE_A::OTHERS)
    }
}
impl R {
    ///Bits 0:15 - Automatic PAUSE Frame Retransmit Setting
    #[inline(always)]
    pub fn tpause(&self) -> TPAUSE_R {
        TPAUSE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Automatic PAUSE Frame Retransmit Setting
    #[inline(always)]
    pub fn tpause(&mut self) -> TPAUSE_W<TPAUSER_SPEC> {
        TPAUSE_W::new(self, 0)
    }
}
/**PAUSE Frame Retransmit Count Setting Register

You can [`read`](crate::Reg::read) this register and get [`tpauser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpauser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TPAUSER_SPEC;
impl crate::RegisterSpec for TPAUSER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tpauser::R`](R) reader structure
impl crate::Readable for TPAUSER_SPEC {}
///`write(|w| ..)` method takes [`tpauser::W`](W) writer structure
impl crate::Writable for TPAUSER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TPAUSER to value 0
impl crate::Resettable for TPAUSER_SPEC {}
