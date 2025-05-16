///Register `SFMPMD` reader
pub type R = crate::R<SFMPMD_SPEC>;
///Register `SFMPMD` writer
pub type W = crate::W<SFMPMD_SPEC>;
/**Specify level of WP pin

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMWPL_A {
    ///0: Low level
    _0 = 0,
    ///1: High level
    _1 = 1,
}
impl From<SFMWPL_A> for bool {
    #[inline(always)]
    fn from(variant: SFMWPL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMWPL` reader - Specify level of WP pin
pub type SFMWPL_R = crate::BitReader<SFMWPL_A>;
impl SFMWPL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMWPL_A {
        match self.bits {
            false => SFMWPL_A::_0,
            true => SFMWPL_A::_1,
        }
    }
    ///Low level
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMWPL_A::_0
    }
    ///High level
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMWPL_A::_1
    }
}
///Field `SFMWPL` writer - Specify level of WP pin
pub type SFMWPL_W<'a, REG> = crate::BitWriter<'a, REG, SFMWPL_A>;
impl<'a, REG> SFMWPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low level
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMWPL_A::_0)
    }
    ///High level
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMWPL_A::_1)
    }
}
impl R {
    ///Bit 2 - Specify level of WP pin
    #[inline(always)]
    pub fn sfmwpl(&self) -> SFMWPL_R {
        SFMWPL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Specify level of WP pin
    #[inline(always)]
    pub fn sfmwpl(&mut self) -> SFMWPL_W<SFMPMD_SPEC> {
        SFMWPL_W::new(self, 2)
    }
}
/**Port Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmpmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmpmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMPMD_SPEC;
impl crate::RegisterSpec for SFMPMD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmpmd::R`](R) reader structure
impl crate::Readable for SFMPMD_SPEC {}
///`write(|w| ..)` method takes [`sfmpmd::W`](W) writer structure
impl crate::Writable for SFMPMD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMPMD to value 0
impl crate::Resettable for SFMPMD_SPEC {}
