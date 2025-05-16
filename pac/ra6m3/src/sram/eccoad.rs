///Register `ECCOAD` reader
pub type R = crate::R<ECCOAD_SPEC>;
///Register `ECCOAD` writer
pub type W = crate::W<ECCOAD_SPEC>;
/**Operation after Detection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAD_A {
    ///0: Non maskable interrupt.
    _0 = 0,
    ///1: Internal reset.
    _1 = 1,
}
impl From<OAD_A> for bool {
    #[inline(always)]
    fn from(variant: OAD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OAD` reader - Operation after Detection
pub type OAD_R = crate::BitReader<OAD_A>;
impl OAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OAD_A {
        match self.bits {
            false => OAD_A::_0,
            true => OAD_A::_1,
        }
    }
    ///Non maskable interrupt.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAD_A::_0
    }
    ///Internal reset.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAD_A::_1
    }
}
///Field `OAD` writer - Operation after Detection
pub type OAD_W<'a, REG> = crate::BitWriter<'a, REG, OAD_A>;
impl<'a, REG> OAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non maskable interrupt.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OAD_A::_0)
    }
    ///Internal reset.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OAD_A::_1)
    }
}
impl R {
    ///Bit 0 - Operation after Detection
    #[inline(always)]
    pub fn oad(&self) -> OAD_R {
        OAD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Operation after Detection
    #[inline(always)]
    pub fn oad(&mut self) -> OAD_W<ECCOAD_SPEC> {
        OAD_W::new(self, 0)
    }
}
/**RAM ECC Error Operation After Detection Register

You can [`read`](crate::Reg::read) this register and get [`eccoad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccoad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECCOAD_SPEC;
impl crate::RegisterSpec for ECCOAD_SPEC {
    type Ux = u8;
}
///`read()` method returns [`eccoad::R`](R) reader structure
impl crate::Readable for ECCOAD_SPEC {}
///`write(|w| ..)` method takes [`eccoad::W`](W) writer structure
impl crate::Writable for ECCOAD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCOAD to value 0
impl crate::Resettable for ECCOAD_SPEC {}
