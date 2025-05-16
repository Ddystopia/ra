///Register `DMREQ` reader
pub type R = crate::R<DMREQ_SPEC>;
///Register `DMREQ` writer
pub type W = crate::W<DMREQ_SPEC>;
/**DMA Software Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWREQ_A {
    ///0: Do not request DMA transfer
    _0 = 0,
    ///1: Request DMA transfer.
    _1 = 1,
}
impl From<SWREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SWREQ_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `SWREQ` reader - DMA Software Start

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type SWREQ_R = crate::BitReader<SWREQ_A>;
impl SWREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWREQ_A {
        match self.bits {
            false => SWREQ_A::_0,
            true => SWREQ_A::_1,
        }
    }
    ///Do not request DMA transfer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWREQ_A::_0
    }
    ///Request DMA transfer.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWREQ_A::_1
    }
}
///Field `SWREQ` writer - DMA Software Start
pub type SWREQ_W<'a, REG> = crate::BitWriter<'a, REG, SWREQ_A>;
impl<'a, REG> SWREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not request DMA transfer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SWREQ_A::_0)
    }
    ///Request DMA transfer.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SWREQ_A::_1)
    }
}
/**DMA Software Start Bit Auto Clear Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRS_A {
    ///0: Clear SWREQ bit after DMA transfer is started by software
    _0 = 0,
    ///1: Do not clear SWREQ bit after DMA transfer is started by software
    _1 = 1,
}
impl From<CLRS_A> for bool {
    #[inline(always)]
    fn from(variant: CLRS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CLRS` reader - DMA Software Start Bit Auto Clear Select
pub type CLRS_R = crate::BitReader<CLRS_A>;
impl CLRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLRS_A {
        match self.bits {
            false => CLRS_A::_0,
            true => CLRS_A::_1,
        }
    }
    ///Clear SWREQ bit after DMA transfer is started by software
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLRS_A::_0
    }
    ///Do not clear SWREQ bit after DMA transfer is started by software
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLRS_A::_1
    }
}
///Field `CLRS` writer - DMA Software Start Bit Auto Clear Select
pub type CLRS_W<'a, REG> = crate::BitWriter<'a, REG, CLRS_A>;
impl<'a, REG> CLRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear SWREQ bit after DMA transfer is started by software
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CLRS_A::_0)
    }
    ///Do not clear SWREQ bit after DMA transfer is started by software
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CLRS_A::_1)
    }
}
impl R {
    ///Bit 0 - DMA Software Start
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DMA Software Start Bit Auto Clear Select
    #[inline(always)]
    pub fn clrs(&self) -> CLRS_R {
        CLRS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA Software Start
    #[inline(always)]
    pub fn swreq(&mut self) -> SWREQ_W<DMREQ_SPEC> {
        SWREQ_W::new(self, 0)
    }
    ///Bit 4 - DMA Software Start Bit Auto Clear Select
    #[inline(always)]
    pub fn clrs(&mut self) -> CLRS_W<DMREQ_SPEC> {
        CLRS_W::new(self, 4)
    }
}
/**DMA Software Start Register

You can [`read`](crate::Reg::read) this register and get [`dmreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMREQ_SPEC;
impl crate::RegisterSpec for DMREQ_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dmreq::R`](R) reader structure
impl crate::Readable for DMREQ_SPEC {}
///`write(|w| ..)` method takes [`dmreq::W`](W) writer structure
impl crate::Writable for DMREQ_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMREQ to value 0
impl crate::Resettable for DMREQ_SPEC {}
