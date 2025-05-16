///Register `BUSSCNT%s` reader
pub type R = crate::R<BUSSCNT_SPEC>;
///Register `BUSSCNT%s` writer
pub type W = crate::W<BUSSCNT_SPEC>;
/**Arbitration MethodSpecify the priority between groups

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBMET_A {
    ///0: fixed priority
    _00 = 0,
    ///1: round-robin
    _01 = 1,
    ///2: Setting prohibited
    OTHERS = 2,
}
impl From<ARBMET_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBMET_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARBMET_A {
    type Ux = u8;
}
impl crate::IsEnum for ARBMET_A {}
///Field `ARBMET` reader - Arbitration MethodSpecify the priority between groups
pub type ARBMET_R = crate::FieldReader<ARBMET_A>;
impl ARBMET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARBMET_A {
        match self.bits {
            0 => ARBMET_A::_00,
            1 => ARBMET_A::_01,
            _ => ARBMET_A::OTHERS,
        }
    }
    ///fixed priority
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ARBMET_A::_00
    }
    ///round-robin
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ARBMET_A::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), ARBMET_A::OTHERS)
    }
}
///Field `ARBMET` writer - Arbitration MethodSpecify the priority between groups
pub type ARBMET_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ARBMET_A, crate::Safe>;
impl<'a, REG> ARBMET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///fixed priority
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(ARBMET_A::_00)
    }
    ///round-robin
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(ARBMET_A::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(ARBMET_A::OTHERS)
    }
}
/**Early Write ResponseWhether the next write request is accepted or not until a response for the write transaction comes back.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWRES_A {
    ///0: Not accepted.
    _0 = 0,
    ///1: Accepted but error response is ignored.
    _1 = 1,
}
impl From<EWRES_A> for bool {
    #[inline(always)]
    fn from(variant: EWRES_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWRES` reader - Early Write ResponseWhether the next write request is accepted or not until a response for the write transaction comes back.
pub type EWRES_R = crate::BitReader<EWRES_A>;
impl EWRES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWRES_A {
        match self.bits {
            false => EWRES_A::_0,
            true => EWRES_A::_1,
        }
    }
    ///Not accepted.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWRES_A::_0
    }
    ///Accepted but error response is ignored.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWRES_A::_1
    }
}
///Field `EWRES` writer - Early Write ResponseWhether the next write request is accepted or not until a response for the write transaction comes back.
pub type EWRES_W<'a, REG> = crate::BitWriter<'a, REG, EWRES_A>;
impl<'a, REG> EWRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not accepted.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EWRES_A::_0)
    }
    ///Accepted but error response is ignored.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EWRES_A::_1)
    }
}
impl R {
    ///Bits 4:5 - Arbitration MethodSpecify the priority between groups
    #[inline(always)]
    pub fn arbmet(&self) -> ARBMET_R {
        ARBMET_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - Early Write ResponseWhether the next write request is accepted or not until a response for the write transaction comes back.
    #[inline(always)]
    pub fn ewres(&self) -> EWRES_R {
        EWRES_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bits 4:5 - Arbitration MethodSpecify the priority between groups
    #[inline(always)]
    pub fn arbmet(&mut self) -> ARBMET_W<BUSSCNT_SPEC> {
        ARBMET_W::new(self, 4)
    }
    ///Bit 8 - Early Write ResponseWhether the next write request is accepted or not until a response for the write transaction comes back.
    #[inline(always)]
    pub fn ewres(&mut self) -> EWRES_W<BUSSCNT_SPEC> {
        EWRES_W::new(self, 8)
    }
}
/**Slave Bus Control Register %s

You can [`read`](crate::Reg::read) this register and get [`busscnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BUSSCNT_SPEC;
impl crate::RegisterSpec for BUSSCNT_SPEC {
    type Ux = u16;
}
///`read()` method returns [`busscnt::R`](R) reader structure
impl crate::Readable for BUSSCNT_SPEC {}
///`write(|w| ..)` method takes [`busscnt::W`](W) writer structure
impl crate::Writable for BUSSCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSSCNT%s to value 0
impl crate::Resettable for BUSSCNT_SPEC {}
