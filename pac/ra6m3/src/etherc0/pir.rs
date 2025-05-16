///Register `PIR` reader
pub type R = crate::R<PIR_SPEC>;
///Register `PIR` writer
pub type W = crate::W<PIR_SPEC>;
///Field `MDC` reader - MII/RMII Management Data ClockThe MDC bit value is output from the ETn_MDC pin to supply the management data clock to the MII or RMII.
pub type MDC_R = crate::BitReader;
///Field `MDC` writer - MII/RMII Management Data ClockThe MDC bit value is output from the ETn_MDC pin to supply the management data clock to the MII or RMII.
pub type MDC_W<'a, REG> = crate::BitWriter<'a, REG>;
/**MII/RMII Management Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMD_A {
    ///0: Read
    _0 = 0,
    ///1: Write
    _1 = 1,
}
impl From<MMD_A> for bool {
    #[inline(always)]
    fn from(variant: MMD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MMD` reader - MII/RMII Management Mode
pub type MMD_R = crate::BitReader<MMD_A>;
impl MMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MMD_A {
        match self.bits {
            false => MMD_A::_0,
            true => MMD_A::_1,
        }
    }
    ///Read
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MMD_A::_0
    }
    ///Write
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MMD_A::_1
    }
}
///Field `MMD` writer - MII/RMII Management Mode
pub type MMD_W<'a, REG> = crate::BitWriter<'a, REG, MMD_A>;
impl<'a, REG> MMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MMD_A::_0)
    }
    ///Write
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MMD_A::_1)
    }
}
///Field `MDO` reader - MII/RMII Management Data-OutThe MDO bit value is output from the ETn_MDIO pin when the MMD bit is 1 (write). The value is not output when the MMD bit is 0 (read).
pub type MDO_R = crate::BitReader;
///Field `MDO` writer - MII/RMII Management Data-OutThe MDO bit value is output from the ETn_MDIO pin when the MMD bit is 1 (write). The value is not output when the MMD bit is 0 (read).
pub type MDO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDI` reader - MII/RMII Management Data-InThis bit indicates the level of the ETn_MDIO pin. The write value should be 0.
pub type MDI_R = crate::BitReader;
impl R {
    ///Bit 0 - MII/RMII Management Data ClockThe MDC bit value is output from the ETn_MDC pin to supply the management data clock to the MII or RMII.
    #[inline(always)]
    pub fn mdc(&self) -> MDC_R {
        MDC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MII/RMII Management Mode
    #[inline(always)]
    pub fn mmd(&self) -> MMD_R {
        MMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MII/RMII Management Data-OutThe MDO bit value is output from the ETn_MDIO pin when the MMD bit is 1 (write). The value is not output when the MMD bit is 0 (read).
    #[inline(always)]
    pub fn mdo(&self) -> MDO_R {
        MDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MII/RMII Management Data-InThis bit indicates the level of the ETn_MDIO pin. The write value should be 0.
    #[inline(always)]
    pub fn mdi(&self) -> MDI_R {
        MDI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MII/RMII Management Data ClockThe MDC bit value is output from the ETn_MDC pin to supply the management data clock to the MII or RMII.
    #[inline(always)]
    pub fn mdc(&mut self) -> MDC_W<PIR_SPEC> {
        MDC_W::new(self, 0)
    }
    ///Bit 1 - MII/RMII Management Mode
    #[inline(always)]
    pub fn mmd(&mut self) -> MMD_W<PIR_SPEC> {
        MMD_W::new(self, 1)
    }
    ///Bit 2 - MII/RMII Management Data-OutThe MDO bit value is output from the ETn_MDIO pin when the MMD bit is 1 (write). The value is not output when the MMD bit is 0 (read).
    #[inline(always)]
    pub fn mdo(&mut self) -> MDO_W<PIR_SPEC> {
        MDO_W::new(self, 2)
    }
}
/**PHY Interface Register

You can [`read`](crate::Reg::read) this register and get [`pir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIR_SPEC;
impl crate::RegisterSpec for PIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pir::R`](R) reader structure
impl crate::Readable for PIR_SPEC {}
///`write(|w| ..)` method takes [`pir::W`](W) writer structure
impl crate::Writable for PIR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIR to value 0
impl crate::Resettable for PIR_SPEC {}
