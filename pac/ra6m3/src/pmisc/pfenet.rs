///Register `PFENET` reader
pub type R = crate::R<PFENET_SPEC>;
///Register `PFENET` writer
pub type W = crate::W<PFENET_SPEC>;
/**Ethernet Mode Setting ch0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHYMODE0_A {
    ///0: RMII mode (ETHERC channel 0)
    _0 = 0,
    ///1: MII mode (ETHERC channel 0)
    _1 = 1,
}
impl From<PHYMODE0_A> for bool {
    #[inline(always)]
    fn from(variant: PHYMODE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PHYMODE0` reader - Ethernet Mode Setting ch0
pub type PHYMODE0_R = crate::BitReader<PHYMODE0_A>;
impl PHYMODE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PHYMODE0_A {
        match self.bits {
            false => PHYMODE0_A::_0,
            true => PHYMODE0_A::_1,
        }
    }
    ///RMII mode (ETHERC channel 0)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PHYMODE0_A::_0
    }
    ///MII mode (ETHERC channel 0)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PHYMODE0_A::_1
    }
}
///Field `PHYMODE0` writer - Ethernet Mode Setting ch0
pub type PHYMODE0_W<'a, REG> = crate::BitWriter<'a, REG, PHYMODE0_A>;
impl<'a, REG> PHYMODE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RMII mode (ETHERC channel 0)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PHYMODE0_A::_0)
    }
    ///MII mode (ETHERC channel 0)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PHYMODE0_A::_1)
    }
}
impl R {
    ///Bit 4 - Ethernet Mode Setting ch0
    #[inline(always)]
    pub fn phymode0(&self) -> PHYMODE0_R {
        PHYMODE0_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - Ethernet Mode Setting ch0
    #[inline(always)]
    pub fn phymode0(&mut self) -> PHYMODE0_W<PFENET_SPEC> {
        PHYMODE0_W::new(self, 4)
    }
}
/**Ethernet Control Register

You can [`read`](crate::Reg::read) this register and get [`pfenet::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfenet::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PFENET_SPEC;
impl crate::RegisterSpec for PFENET_SPEC {
    type Ux = u8;
}
///`read()` method returns [`pfenet::R`](R) reader structure
impl crate::Readable for PFENET_SPEC {}
///`write(|w| ..)` method takes [`pfenet::W`](W) writer structure
impl crate::Writable for PFENET_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PFENET to value 0
impl crate::Resettable for PFENET_SPEC {}
