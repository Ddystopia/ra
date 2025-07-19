///Register `SYOCDCR` reader
pub type R = crate::R<SYOCDCR_SPEC>;
///Register `SYOCDCR` writer
pub type W = crate::W<SYOCDCR_SPEC>;
/**Debugger Enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGEN_A {
    ///0: On-chip debugger is disabled
    _0 = 0,
    ///1: On-chip debugger is enabled
    _1 = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGEN` reader - Debugger Enable bit
pub type DBGEN_R = crate::BitReader<DBGEN_A>;
impl DBGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::_0,
            true => DBGEN_A::_1,
        }
    }
    ///On-chip debugger is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGEN_A::_0
    }
    ///On-chip debugger is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGEN_A::_1
    }
}
///Field `DBGEN` writer - Debugger Enable bit
pub type DBGEN_W<'a, REG> = crate::BitWriter<'a, REG, DBGEN_A>;
impl<'a, REG> DBGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///On-chip debugger is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DBGEN_A::_0)
    }
    ///On-chip debugger is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DBGEN_A::_1)
    }
}
impl R {
    ///Bit 7 - Debugger Enable bit
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 7 - Debugger Enable bit
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W<'_, SYOCDCR_SPEC> {
        DBGEN_W::new(self, 7)
    }
}
/**System Control OCD Control Register

You can [`read`](crate::Reg::read) this register and get [`syocdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syocdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYOCDCR_SPEC;
impl crate::RegisterSpec for SYOCDCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`syocdcr::R`](R) reader structure
impl crate::Readable for SYOCDCR_SPEC {}
///`write(|w| ..)` method takes [`syocdcr::W`](W) writer structure
impl crate::Writable for SYOCDCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYOCDCR to value 0
impl crate::Resettable for SYOCDCR_SPEC {}
