///Register `ADDISCR` reader
pub type R = crate::R<ADDISCR_SPEC>;
///Register `ADDISCR` writer
pub type W = crate::W<ADDISCR_SPEC>;
/**The charging time

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADNDIS_A {
    ///0: Disconnection detection is disabled
    _0000 = 0,
    ///1: Setting prohibited
    _0001 = 1,
    ///2: ( 1 / ADCLK ) x ADNDIS
    OTHERS = 2,
}
impl From<ADNDIS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADNDIS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADNDIS_A {
    type Ux = u8;
}
impl crate::IsEnum for ADNDIS_A {}
///Field `ADNDIS` reader - The charging time
pub type ADNDIS_R = crate::FieldReader<ADNDIS_A>;
impl ADNDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADNDIS_A {
        match self.bits {
            0 => ADNDIS_A::_0000,
            1 => ADNDIS_A::_0001,
            _ => ADNDIS_A::OTHERS,
        }
    }
    ///Disconnection detection is disabled
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == ADNDIS_A::_0000
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == ADNDIS_A::_0001
    }
    ///( 1 / ADCLK ) x ADNDIS
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), ADNDIS_A::OTHERS)
    }
}
///Field `ADNDIS` writer - The charging time
pub type ADNDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ADNDIS_A, crate::Safe>;
impl<'a, REG> ADNDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Disconnection detection is disabled
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(ADNDIS_A::_0000)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(ADNDIS_A::_0001)
    }
    ///( 1 / ADCLK ) x ADNDIS
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(ADNDIS_A::OTHERS)
    }
}
/**Selection of Precharge or Discharge

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHARGE_A {
    ///0: Discharge
    _0 = 0,
    ///1: Precharge
    _1 = 1,
}
impl From<CHARGE_A> for bool {
    #[inline(always)]
    fn from(variant: CHARGE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CHARGE` reader - Selection of Precharge or Discharge
pub type CHARGE_R = crate::BitReader<CHARGE_A>;
impl CHARGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHARGE_A {
        match self.bits {
            false => CHARGE_A::_0,
            true => CHARGE_A::_1,
        }
    }
    ///Discharge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHARGE_A::_0
    }
    ///Precharge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHARGE_A::_1
    }
}
///Field `CHARGE` writer - Selection of Precharge or Discharge
pub type CHARGE_W<'a, REG> = crate::BitWriter<'a, REG, CHARGE_A>;
impl<'a, REG> CHARGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discharge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CHARGE_A::_0)
    }
    ///Precharge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CHARGE_A::_1)
    }
}
impl R {
    ///Bits 0:3 - The charging time
    #[inline(always)]
    pub fn adndis(&self) -> ADNDIS_R {
        ADNDIS_R::new(self.bits & 0x0f)
    }
    ///Bit 4 - Selection of Precharge or Discharge
    #[inline(always)]
    pub fn charge(&self) -> CHARGE_R {
        CHARGE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - The charging time
    #[inline(always)]
    pub fn adndis(&mut self) -> ADNDIS_W<ADDISCR_SPEC> {
        ADNDIS_W::new(self, 0)
    }
    ///Bit 4 - Selection of Precharge or Discharge
    #[inline(always)]
    pub fn charge(&mut self) -> CHARGE_W<ADDISCR_SPEC> {
        CHARGE_W::new(self, 4)
    }
}
/**A/D Disconnection Detection Control Register

You can [`read`](crate::Reg::read) this register and get [`addiscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addiscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADDISCR_SPEC;
impl crate::RegisterSpec for ADDISCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`addiscr::R`](R) reader structure
impl crate::Readable for ADDISCR_SPEC {}
///`write(|w| ..)` method takes [`addiscr::W`](W) writer structure
impl crate::Writable for ADDISCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDISCR to value 0
impl crate::Resettable for ADDISCR_SPEC {}
