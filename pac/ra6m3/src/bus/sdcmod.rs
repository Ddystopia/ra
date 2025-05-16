///Register `SDCMOD` reader
pub type R = crate::R<SDCMOD_SPEC>;
///Register `SDCMOD` writer
pub type W = crate::W<SDCMOD_SPEC>;
/**Endian Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMODE_A {
    ///0: Endian order of SDRAM address space is the same as the endian order of the operating mode
    _0 = 0,
    ///1: Endian order of SDRAM address space is not the endian order of the operating mode.
    _1 = 1,
}
impl From<EMODE_A> for bool {
    #[inline(always)]
    fn from(variant: EMODE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EMODE` reader - Endian Mode
pub type EMODE_R = crate::BitReader<EMODE_A>;
impl EMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EMODE_A {
        match self.bits {
            false => EMODE_A::_0,
            true => EMODE_A::_1,
        }
    }
    ///Endian order of SDRAM address space is the same as the endian order of the operating mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EMODE_A::_0
    }
    ///Endian order of SDRAM address space is not the endian order of the operating mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EMODE_A::_1
    }
}
///Field `EMODE` writer - Endian Mode
pub type EMODE_W<'a, REG> = crate::BitWriter<'a, REG, EMODE_A>;
impl<'a, REG> EMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Endian order of SDRAM address space is the same as the endian order of the operating mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EMODE_A::_0)
    }
    ///Endian order of SDRAM address space is not the endian order of the operating mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EMODE_A::_1)
    }
}
impl R {
    ///Bit 0 - Endian Mode
    #[inline(always)]
    pub fn emode(&self) -> EMODE_R {
        EMODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Endian Mode
    #[inline(always)]
    pub fn emode(&mut self) -> EMODE_W<SDCMOD_SPEC> {
        EMODE_W::new(self, 0)
    }
}
/**SDC Mode Register

You can [`read`](crate::Reg::read) this register and get [`sdcmod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcmod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDCMOD_SPEC;
impl crate::RegisterSpec for SDCMOD_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sdcmod::R`](R) reader structure
impl crate::Readable for SDCMOD_SPEC {}
///`write(|w| ..)` method takes [`sdcmod::W`](W) writer structure
impl crate::Writable for SDCMOD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDCMOD to value 0
impl crate::Resettable for SDCMOD_SPEC {}
