///Register `FCACHEIV` reader
pub type R = crate::R<FCACHEIV_SPEC>;
///Register `FCACHEIV` writer
pub type W = crate::W<FCACHEIV_SPEC>;
/**FCACHE Invalidation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCACHEIV_A {
    ///0: (Read)not in progress / (Write) no effect.
    _0 = 0,
    ///1: (Read)in progress /(Write) Starting Cache Invalidation
    _1 = 1,
}
impl From<FCACHEIV_A> for bool {
    #[inline(always)]
    fn from(variant: FCACHEIV_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `FCACHEIV` reader - FCACHE Invalidation

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type FCACHEIV_R = crate::BitReader<FCACHEIV_A>;
impl FCACHEIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FCACHEIV_A {
        match self.bits {
            false => FCACHEIV_A::_0,
            true => FCACHEIV_A::_1,
        }
    }
    ///(Read)not in progress / (Write) no effect.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FCACHEIV_A::_0
    }
    ///(Read)in progress /(Write) Starting Cache Invalidation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FCACHEIV_A::_1
    }
}
///Field `FCACHEIV` writer - FCACHE Invalidation
pub type FCACHEIV_W<'a, REG> = crate::BitWriter1S<'a, REG, FCACHEIV_A>;
impl<'a, REG> FCACHEIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///(Read)not in progress / (Write) no effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FCACHEIV_A::_0)
    }
    ///(Read)in progress /(Write) Starting Cache Invalidation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FCACHEIV_A::_1)
    }
}
impl R {
    ///Bit 0 - FCACHE Invalidation
    #[inline(always)]
    pub fn fcacheiv(&self) -> FCACHEIV_R {
        FCACHEIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - FCACHE Invalidation
    #[inline(always)]
    pub fn fcacheiv(&mut self) -> FCACHEIV_W<FCACHEIV_SPEC> {
        FCACHEIV_W::new(self, 0)
    }
}
/**Flash Cache Invalidate Register

You can [`read`](crate::Reg::read) this register and get [`fcacheiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcacheiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FCACHEIV_SPEC;
impl crate::RegisterSpec for FCACHEIV_SPEC {
    type Ux = u16;
}
///`read()` method returns [`fcacheiv::R`](R) reader structure
impl crate::Readable for FCACHEIV_SPEC {}
///`write(|w| ..)` method takes [`fcacheiv::W`](W) writer structure
impl crate::Writable for FCACHEIV_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0x01;
}
///`reset()` method sets FCACHEIV to value 0
impl crate::Resettable for FCACHEIV_SPEC {}
