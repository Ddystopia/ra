///Register `GTWP` reader
pub type R = crate::R<GTWP_SPEC>;
///Register `GTWP` writer
pub type W = crate::W<GTWP_SPEC>;
/**Register Write Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_A {
    ///0: Enable writes to the register
    _0 = 0,
    ///1: Disable writes to the register
    _1 = 1,
}
impl From<WP_A> for bool {
    #[inline(always)]
    fn from(variant: WP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WP` reader - Register Write Disable
pub type WP_R = crate::BitReader<WP_A>;
impl WP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP_A {
        match self.bits {
            false => WP_A::_0,
            true => WP_A::_1,
        }
    }
    ///Enable writes to the register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_A::_0
    }
    ///Disable writes to the register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_A::_1
    }
}
///Field `WP` writer - Register Write Disable
pub type WP_W<'a, REG> = crate::BitWriter<'a, REG, WP_A>;
impl<'a, REG> WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable writes to the register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WP_A::_0)
    }
    ///Disable writes to the register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WP_A::_1)
    }
}
/**GTWP Key Code

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRKEY_A {
    ///165: Written to these bits, the WP bits write is permitted.
    _0X_A5 = 165,
    ///0: The WP bits write is not permitted.
    OTHERS = 0,
}
impl From<PRKEY_A> for u8 {
    #[inline(always)]
    fn from(variant: PRKEY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRKEY_A {
    type Ux = u8;
}
impl crate::IsEnum for PRKEY_A {}
///Field `PRKEY` writer - GTWP Key Code
pub type PRKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, PRKEY_A, crate::Safe>;
impl<'a, REG> PRKEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Written to these bits, the WP bits write is permitted.
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut crate::W<REG> {
        self.variant(PRKEY_A::_0X_A5)
    }
    ///The WP bits write is not permitted.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PRKEY_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - Register Write Disable
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Register Write Disable
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W<GTWP_SPEC> {
        WP_W::new(self, 0)
    }
    ///Bits 8:15 - GTWP Key Code
    #[inline(always)]
    pub fn prkey(&mut self) -> PRKEY_W<GTWP_SPEC> {
        PRKEY_W::new(self, 8)
    }
}
/**General PWM Timer Write-Protection Register

You can [`read`](crate::Reg::read) this register and get [`gtwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTWP_SPEC;
impl crate::RegisterSpec for GTWP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtwp::R`](R) reader structure
impl crate::Readable for GTWP_SPEC {}
///`write(|w| ..)` method takes [`gtwp::W`](W) writer structure
impl crate::Writable for GTWP_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTWP to value 0
impl crate::Resettable for GTWP_SPEC {}
