///Register `MSMR` reader
pub type R = crate::R<MSMR_SPEC>;
///Register `MSMR` writer
pub type W = crate::W<MSMR_SPEC>;
/**Mailbox Search Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBSM_A {
    ///0: Receive mailbox search mode
    _00 = 0,
    ///1: Transmit mailbox search mode
    _01 = 1,
    ///2: Message lost search mode
    _10 = 2,
    ///3: Channel search mode
    _11 = 3,
}
impl From<MBSM_A> for u8 {
    #[inline(always)]
    fn from(variant: MBSM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MBSM_A {
    type Ux = u8;
}
impl crate::IsEnum for MBSM_A {}
///Field `MBSM` reader - Mailbox Search Mode Select
pub type MBSM_R = crate::FieldReader<MBSM_A>;
impl MBSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MBSM_A {
        match self.bits {
            0 => MBSM_A::_00,
            1 => MBSM_A::_01,
            2 => MBSM_A::_10,
            3 => MBSM_A::_11,
            _ => unreachable!(),
        }
    }
    ///Receive mailbox search mode
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MBSM_A::_00
    }
    ///Transmit mailbox search mode
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MBSM_A::_01
    }
    ///Message lost search mode
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MBSM_A::_10
    }
    ///Channel search mode
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MBSM_A::_11
    }
}
///Field `MBSM` writer - Mailbox Search Mode Select
pub type MBSM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MBSM_A, crate::Safe>;
impl<'a, REG> MBSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Receive mailbox search mode
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(MBSM_A::_00)
    }
    ///Transmit mailbox search mode
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(MBSM_A::_01)
    }
    ///Message lost search mode
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(MBSM_A::_10)
    }
    ///Channel search mode
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(MBSM_A::_11)
    }
}
impl R {
    ///Bits 0:1 - Mailbox Search Mode Select
    #[inline(always)]
    pub fn mbsm(&self) -> MBSM_R {
        MBSM_R::new(self.bits & 3)
    }
}
impl W {
    ///Bits 0:1 - Mailbox Search Mode Select
    #[inline(always)]
    pub fn mbsm(&mut self) -> MBSM_W<MSMR_SPEC> {
        MBSM_W::new(self, 0)
    }
}
/**Mailbox Search Mode Register

You can [`read`](crate::Reg::read) this register and get [`msmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MSMR_SPEC;
impl crate::RegisterSpec for MSMR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`msmr::R`](R) reader structure
impl crate::Readable for MSMR_SPEC {}
///`write(|w| ..)` method takes [`msmr::W`](W) writer structure
impl crate::Writable for MSMR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSMR to value 0
impl crate::Resettable for MSMR_SPEC {}
