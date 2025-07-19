///Register `BUSSCNTFLI` reader
pub type R = crate::R<BUSSCNTFLI_SPEC>;
///Register `BUSSCNTFLI` writer
pub type W = crate::W<BUSSCNTFLI_SPEC>;
/**Arbitration Method Specify the priority between groups

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
///Field `ARBMET` reader - Arbitration Method Specify the priority between groups
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
///Field `ARBMET` writer - Arbitration Method Specify the priority between groups
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
impl R {
    ///Bits 4:5 - Arbitration Method Specify the priority between groups
    #[inline(always)]
    pub fn arbmet(&self) -> ARBMET_R {
        ARBMET_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    ///Bits 4:5 - Arbitration Method Specify the priority between groups
    #[inline(always)]
    pub fn arbmet(&mut self) -> ARBMET_W<'_, BUSSCNTFLI_SPEC> {
        ARBMET_W::new(self, 4)
    }
}
/**Slave Bus Control Register FLI

You can [`read`](crate::Reg::read) this register and get [`busscntfli::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntfli::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BUSSCNTFLI_SPEC;
impl crate::RegisterSpec for BUSSCNTFLI_SPEC {
    type Ux = u16;
}
///`read()` method returns [`busscntfli::R`](R) reader structure
impl crate::Readable for BUSSCNTFLI_SPEC {}
///`write(|w| ..)` method takes [`busscntfli::W`](W) writer structure
impl crate::Writable for BUSSCNTFLI_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSSCNTFLI to value 0
impl crate::Resettable for BUSSCNTFLI_SPEC {}
