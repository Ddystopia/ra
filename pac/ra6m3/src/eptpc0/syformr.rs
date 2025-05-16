///Register `SYFORMR` reader
pub type R = crate::R<SYFORMR_SPEC>;
///Register `SYFORMR` writer
pub type W = crate::W<SYFORMR_SPEC>;
/**Ethernet/UDP Encapsulation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORM0_A {
    ///0: PTP directly over Ethernet
    _0 = 0,
    ///1: PTP over UDP/IPv4
    _1 = 1,
}
impl From<FORM0_A> for bool {
    #[inline(always)]
    fn from(variant: FORM0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FORM0` reader - Ethernet/UDP Encapsulation
pub type FORM0_R = crate::BitReader<FORM0_A>;
impl FORM0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FORM0_A {
        match self.bits {
            false => FORM0_A::_0,
            true => FORM0_A::_1,
        }
    }
    ///PTP directly over Ethernet
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FORM0_A::_0
    }
    ///PTP over UDP/IPv4
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FORM0_A::_1
    }
}
///Field `FORM0` writer - Ethernet/UDP Encapsulation
pub type FORM0_W<'a, REG> = crate::BitWriter<'a, REG, FORM0_A>;
impl<'a, REG> FORM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PTP directly over Ethernet
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FORM0_A::_0)
    }
    ///PTP over UDP/IPv4
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FORM0_A::_1)
    }
}
/**Ethernet Frame Format Setting

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORM1_A {
    ///0: Set this bit to 0 (Ethernet II frame format).
    _0 = 0,
    ///1: Setting prohibited
    _1 = 1,
}
impl From<FORM1_A> for bool {
    #[inline(always)]
    fn from(variant: FORM1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FORM1` reader - Ethernet Frame Format Setting
pub type FORM1_R = crate::BitReader<FORM1_A>;
impl FORM1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FORM1_A {
        match self.bits {
            false => FORM1_A::_0,
            true => FORM1_A::_1,
        }
    }
    ///Set this bit to 0 (Ethernet II frame format).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FORM1_A::_0
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FORM1_A::_1
    }
}
///Field `FORM1` writer - Ethernet Frame Format Setting
pub type FORM1_W<'a, REG> = crate::BitWriter<'a, REG, FORM1_A>;
impl<'a, REG> FORM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set this bit to 0 (Ethernet II frame format).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FORM1_A::_0)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FORM1_A::_1)
    }
}
impl R {
    ///Bit 0 - Ethernet/UDP Encapsulation
    #[inline(always)]
    pub fn form0(&self) -> FORM0_R {
        FORM0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Ethernet Frame Format Setting
    #[inline(always)]
    pub fn form1(&self) -> FORM1_R {
        FORM1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Ethernet/UDP Encapsulation
    #[inline(always)]
    pub fn form0(&mut self) -> FORM0_W<SYFORMR_SPEC> {
        FORM0_W::new(self, 0)
    }
    ///Bit 1 - Ethernet Frame Format Setting
    #[inline(always)]
    pub fn form1(&mut self) -> FORM1_W<SYFORMR_SPEC> {
        FORM1_W::new(self, 1)
    }
}
/**SYNFP Frame Format Setting Register

You can [`read`](crate::Reg::read) this register and get [`syformr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syformr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYFORMR_SPEC;
impl crate::RegisterSpec for SYFORMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syformr::R`](R) reader structure
impl crate::Readable for SYFORMR_SPEC {}
///`write(|w| ..)` method takes [`syformr::W`](W) writer structure
impl crate::Writable for SYFORMR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYFORMR to value 0
impl crate::Resettable for SYFORMR_SPEC {}
