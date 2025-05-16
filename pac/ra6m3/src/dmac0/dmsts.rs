///Register `DMSTS` reader
pub type R = crate::R<DMSTS_SPEC>;
///Register `DMSTS` writer
pub type W = crate::W<DMSTS_SPEC>;
/**Transfer Escape End Interrupt Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESIF_A {
    ///0: A transfer escape end interrupt has not been generated.
    _0 = 0,
    ///1: A transfer escape end interrupt has been generated.
    _1 = 1,
}
impl From<ESIF_A> for bool {
    #[inline(always)]
    fn from(variant: ESIF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ESIF` reader - Transfer Escape End Interrupt Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ESIF_R = crate::BitReader<ESIF_A>;
impl ESIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ESIF_A {
        match self.bits {
            false => ESIF_A::_0,
            true => ESIF_A::_1,
        }
    }
    ///A transfer escape end interrupt has not been generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESIF_A::_0
    }
    ///A transfer escape end interrupt has been generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESIF_A::_1
    }
}
///Field `ESIF` writer - Transfer Escape End Interrupt Flag
pub type ESIF_W<'a, REG> = crate::BitWriter0C<'a, REG, ESIF_A>;
impl<'a, REG> ESIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A transfer escape end interrupt has not been generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ESIF_A::_0)
    }
    ///A transfer escape end interrupt has been generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ESIF_A::_1)
    }
}
/**Transfer End Interrupt Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTIF_A {
    ///0: No interrupt occurred
    _0 = 0,
    ///1: Interrupt occurred.
    _1 = 1,
}
impl From<DTIF_A> for bool {
    #[inline(always)]
    fn from(variant: DTIF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DTIF` reader - Transfer End Interrupt Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DTIF_R = crate::BitReader<DTIF_A>;
impl DTIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTIF_A {
        match self.bits {
            false => DTIF_A::_0,
            true => DTIF_A::_1,
        }
    }
    ///No interrupt occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTIF_A::_0
    }
    ///Interrupt occurred.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTIF_A::_1
    }
}
///Field `DTIF` writer - Transfer End Interrupt Flag
pub type DTIF_W<'a, REG> = crate::BitWriter0C<'a, REG, DTIF_A>;
impl<'a, REG> DTIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTIF_A::_0)
    }
    ///Interrupt occurred.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTIF_A::_1)
    }
}
/**DMA Active Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACT_A {
    ///0: DMAC operation is suspended.
    _0 = 0,
    ///1: DMAC is operating.
    _1 = 1,
}
impl From<ACT_A> for bool {
    #[inline(always)]
    fn from(variant: ACT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ACT` reader - DMA Active Flag
pub type ACT_R = crate::BitReader<ACT_A>;
impl ACT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACT_A {
        match self.bits {
            false => ACT_A::_0,
            true => ACT_A::_1,
        }
    }
    ///DMAC operation is suspended.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACT_A::_0
    }
    ///DMAC is operating.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACT_A::_1
    }
}
impl R {
    ///Bit 0 - Transfer Escape End Interrupt Flag
    #[inline(always)]
    pub fn esif(&self) -> ESIF_R {
        ESIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Transfer End Interrupt Flag
    #[inline(always)]
    pub fn dtif(&self) -> DTIF_R {
        DTIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - DMA Active Flag
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Transfer Escape End Interrupt Flag
    #[inline(always)]
    pub fn esif(&mut self) -> ESIF_W<DMSTS_SPEC> {
        ESIF_W::new(self, 0)
    }
    ///Bit 4 - Transfer End Interrupt Flag
    #[inline(always)]
    pub fn dtif(&mut self) -> DTIF_W<DMSTS_SPEC> {
        DTIF_W::new(self, 4)
    }
}
/**DMAC Module Activation Register

You can [`read`](crate::Reg::read) this register and get [`dmsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMSTS_SPEC;
impl crate::RegisterSpec for DMSTS_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dmsts::R`](R) reader structure
impl crate::Readable for DMSTS_SPEC {}
///`write(|w| ..)` method takes [`dmsts::W`](W) writer structure
impl crate::Writable for DMSTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x11;
}
///`reset()` method sets DMSTS to value 0
impl crate::Resettable for DMSTS_SPEC {}
