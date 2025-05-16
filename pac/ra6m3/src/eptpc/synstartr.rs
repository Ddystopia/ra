///Register `SYNSTARTR` reader
pub type R = crate::R<SYNSTARTR_SPEC>;
///Register `SYNSTARTR` writer
pub type W = crate::W<SYNSTARTR_SPEC>;
/**Slave Time Synchronization Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STR_A {
    ///0: Slave time synchronization is stopped.
    _0 = 0,
    ///1: Slave time synchronization is started.
    _1 = 1,
}
impl From<STR_A> for bool {
    #[inline(always)]
    fn from(variant: STR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `STR` reader - Slave Time Synchronization Control
pub type STR_R = crate::BitReader<STR_A>;
impl STR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STR_A {
        match self.bits {
            false => STR_A::_0,
            true => STR_A::_1,
        }
    }
    ///Slave time synchronization is stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STR_A::_0
    }
    ///Slave time synchronization is started.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STR_A::_1
    }
}
///Field `STR` writer - Slave Time Synchronization Control
pub type STR_W<'a, REG> = crate::BitWriter<'a, REG, STR_A>;
impl<'a, REG> STR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave time synchronization is stopped.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(STR_A::_0)
    }
    ///Slave time synchronization is started.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(STR_A::_1)
    }
}
impl R {
    ///Bit 0 - Slave Time Synchronization Control
    #[inline(always)]
    pub fn str(&self) -> STR_R {
        STR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Slave Time Synchronization Control
    #[inline(always)]
    pub fn str(&mut self) -> STR_W<SYNSTARTR_SPEC> {
        STR_W::new(self, 0)
    }
}
/**Slave Time Synchronization Start Register

You can [`read`](crate::Reg::read) this register and get [`synstartr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synstartr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYNSTARTR_SPEC;
impl crate::RegisterSpec for SYNSTARTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`synstartr::R`](R) reader structure
impl crate::Readable for SYNSTARTR_SPEC {}
///`write(|w| ..)` method takes [`synstartr::W`](W) writer structure
impl crate::Writable for SYNSTARTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYNSTARTR to value 0
impl crate::Resettable for SYNSTARTR_SPEC {}
