///Register `PTRSTR` reader
pub type R = crate::R<PTRSTR_SPEC>;
///Register `PTRSTR` writer
pub type W = crate::W<PTRSTR_SPEC>;
/**EPTPC Software Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    ///0: Do not reset the EPTPC
    _0 = 0,
    ///1: Reset the EPTPC. Do not access the EPTPC-related registers other than this register while a software reset is being issued.
    _1 = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RESET` reader - EPTPC Software Reset
pub type RESET_R = crate::BitReader<RESET_A>;
impl RESET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::_0,
            true => RESET_A::_1,
        }
    }
    ///Do not reset the EPTPC
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESET_A::_0
    }
    ///Reset the EPTPC. Do not access the EPTPC-related registers other than this register while a software reset is being issued.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESET_A::_1
    }
}
///Field `RESET` writer - EPTPC Software Reset
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG, RESET_A>;
impl<'a, REG> RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not reset the EPTPC
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::_0)
    }
    ///Reset the EPTPC. Do not access the EPTPC-related registers other than this register while a software reset is being issued.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::_1)
    }
}
impl R {
    ///Bit 0 - EPTPC Software Reset
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - EPTPC Software Reset
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<PTRSTR_SPEC> {
        RESET_W::new(self, 0)
    }
}
/**EPTPC Reset Register

You can [`read`](crate::Reg::read) this register and get [`ptrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PTRSTR_SPEC;
impl crate::RegisterSpec for PTRSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ptrstr::R`](R) reader structure
impl crate::Readable for PTRSTR_SPEC {}
///`write(|w| ..)` method takes [`ptrstr::W`](W) writer structure
impl crate::Writable for PTRSTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTRSTR to value 0
impl crate::Resettable for PTRSTR_SPEC {}
