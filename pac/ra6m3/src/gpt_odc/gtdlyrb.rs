///Register `GTDLYR%sB` reader
pub type R = crate::R<GTDLYRB_SPEC>;
///Register `GTDLYR%sB` writer
pub type W = crate::W<GTDLYRB_SPEC>;
/**GTIOCnB Output Rising Edge Delay Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLY_A {
    ///0: No delay on rising edges
    _00000 = 0,
    ///1: Delay of DLY/32 times the PCLKD period is applied.
    OTHERS = 1,
}
impl From<DLY_A> for u8 {
    #[inline(always)]
    fn from(variant: DLY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DLY_A {
    type Ux = u8;
}
impl crate::IsEnum for DLY_A {}
///Field `DLY` reader - GTIOCnB Output Rising Edge Delay Setting
pub type DLY_R = crate::FieldReader<DLY_A>;
impl DLY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLY_A {
        match self.bits {
            0 => DLY_A::_00000,
            _ => DLY_A::OTHERS,
        }
    }
    ///No delay on rising edges
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == DLY_A::_00000
    }
    ///Delay of DLY/32 times the PCLKD period is applied.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), DLY_A::OTHERS)
    }
}
///Field `DLY` writer - GTIOCnB Output Rising Edge Delay Setting
pub type DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DLY_A, crate::Safe>;
impl<'a, REG> DLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No delay on rising edges
    #[inline(always)]
    pub fn _00000(self) -> &'a mut crate::W<REG> {
        self.variant(DLY_A::_00000)
    }
    ///Delay of DLY/32 times the PCLKD period is applied.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(DLY_A::OTHERS)
    }
}
impl R {
    ///Bits 0:4 - GTIOCnB Output Rising Edge Delay Setting
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - GTIOCnB Output Rising Edge Delay Setting
    #[inline(always)]
    pub fn dly(&mut self) -> DLY_W<GTDLYRB_SPEC> {
        DLY_W::new(self, 0)
    }
}
/**GTIOC%sB Rising Output Delay Register

You can [`read`](crate::Reg::read) this register and get [`gtdlyrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdlyrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTDLYRB_SPEC;
impl crate::RegisterSpec for GTDLYRB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`gtdlyrb::R`](R) reader structure
impl crate::Readable for GTDLYRB_SPEC {}
///`write(|w| ..)` method takes [`gtdlyrb::W`](W) writer structure
impl crate::Writable for GTDLYRB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTDLYR%sB to value 0
impl crate::Resettable for GTDLYRB_SPEC {}
