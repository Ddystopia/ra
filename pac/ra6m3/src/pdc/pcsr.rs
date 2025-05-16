///Register `PCSR` reader
pub type R = crate::R<PCSR_SPEC>;
///Register `PCSR` writer
pub type W = crate::W<PCSR_SPEC>;
/**Frame Busy Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FBSY_A {
    ///0: Operations for reception are stopped.
    _0 = 0,
    ///1: Operations for reception are ongoing.
    _1 = 1,
}
impl From<FBSY_A> for bool {
    #[inline(always)]
    fn from(variant: FBSY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FBSY` reader - Frame Busy Flag
pub type FBSY_R = crate::BitReader<FBSY_A>;
impl FBSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FBSY_A {
        match self.bits {
            false => FBSY_A::_0,
            true => FBSY_A::_1,
        }
    }
    ///Operations for reception are stopped.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FBSY_A::_0
    }
    ///Operations for reception are ongoing.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FBSY_A::_1
    }
}
/**FIFO Empty Flag

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEMPF_A {
    ///0: FIFO is not empty.
    _0 = 0,
    ///1: FIFO is empty.
    _1 = 1,
}
impl From<FEMPF_A> for bool {
    #[inline(always)]
    fn from(variant: FEMPF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FEMPF` reader - FIFO Empty Flag
pub type FEMPF_R = crate::BitReader<FEMPF_A>;
impl FEMPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FEMPF_A {
        match self.bits {
            false => FEMPF_A::_0,
            true => FEMPF_A::_1,
        }
    }
    ///FIFO is not empty.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEMPF_A::_0
    }
    ///FIFO is empty.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEMPF_A::_1
    }
}
/**Frame End Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEF_A {
    ///0: Frame end has not been generated.
    _0 = 0,
    ///1: Frame end has been generated.
    _1 = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `FEF` reader - Frame End Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type FEF_R = crate::BitReader<FEF_A>;
impl FEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::_0,
            true => FEF_A::_1,
        }
    }
    ///Frame end has not been generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEF_A::_0
    }
    ///Frame end has been generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEF_A::_1
    }
}
///Field `FEF` writer - Frame End Flag
pub type FEF_W<'a, REG> = crate::BitWriter0C<'a, REG, FEF_A>;
impl<'a, REG> FEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frame end has not been generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FEF_A::_0)
    }
    ///Frame end has been generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FEF_A::_1)
    }
}
/**Overrun Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRF_A {
    ///0: FIFO overrun has not been generated.
    _0 = 0,
    ///1: FIFO overrun has been generated.
    _1 = 1,
}
impl From<OVRF_A> for bool {
    #[inline(always)]
    fn from(variant: OVRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `OVRF` reader - Overrun Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type OVRF_R = crate::BitReader<OVRF_A>;
impl OVRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRF_A {
        match self.bits {
            false => OVRF_A::_0,
            true => OVRF_A::_1,
        }
    }
    ///FIFO overrun has not been generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRF_A::_0
    }
    ///FIFO overrun has been generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRF_A::_1
    }
}
///Field `OVRF` writer - Overrun Flag
pub type OVRF_W<'a, REG> = crate::BitWriter0C<'a, REG, OVRF_A>;
impl<'a, REG> OVRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO overrun has not been generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRF_A::_0)
    }
    ///FIFO overrun has been generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRF_A::_1)
    }
}
/**Underrun Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRF_A {
    ///0: Underrun has not been generated.
    _0 = 0,
    ///1: Underrun has been generated.
    _1 = 1,
}
impl From<UDRF_A> for bool {
    #[inline(always)]
    fn from(variant: UDRF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `UDRF` reader - Underrun Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type UDRF_R = crate::BitReader<UDRF_A>;
impl UDRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDRF_A {
        match self.bits {
            false => UDRF_A::_0,
            true => UDRF_A::_1,
        }
    }
    ///Underrun has not been generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDRF_A::_0
    }
    ///Underrun has been generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDRF_A::_1
    }
}
///Field `UDRF` writer - Underrun Flag
pub type UDRF_W<'a, REG> = crate::BitWriter0C<'a, REG, UDRF_A>;
impl<'a, REG> UDRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Underrun has not been generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(UDRF_A::_0)
    }
    ///Underrun has been generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(UDRF_A::_1)
    }
}
/**Vertical Line Number Setting Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VERF_A {
    ///0: Vertical line number setting error has not been generated.
    _0 = 0,
    ///1: Vertical line number setting error has been generated.
    _1 = 1,
}
impl From<VERF_A> for bool {
    #[inline(always)]
    fn from(variant: VERF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `VERF` reader - Vertical Line Number Setting Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type VERF_R = crate::BitReader<VERF_A>;
impl VERF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VERF_A {
        match self.bits {
            false => VERF_A::_0,
            true => VERF_A::_1,
        }
    }
    ///Vertical line number setting error has not been generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VERF_A::_0
    }
    ///Vertical line number setting error has been generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VERF_A::_1
    }
}
///Field `VERF` writer - Vertical Line Number Setting Error Flag
pub type VERF_W<'a, REG> = crate::BitWriter0C<'a, REG, VERF_A>;
impl<'a, REG> VERF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Vertical line number setting error has not been generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VERF_A::_0)
    }
    ///Vertical line number setting error has been generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VERF_A::_1)
    }
}
/**Horizontal Byte Number Setting Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HERF_A {
    ///0: Horizontal byte number setting error has not been generated.
    _0 = 0,
    ///1: Horizontal byte number setting error has been generated.
    _1 = 1,
}
impl From<HERF_A> for bool {
    #[inline(always)]
    fn from(variant: HERF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `HERF` reader - Horizontal Byte Number Setting Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type HERF_R = crate::BitReader<HERF_A>;
impl HERF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HERF_A {
        match self.bits {
            false => HERF_A::_0,
            true => HERF_A::_1,
        }
    }
    ///Horizontal byte number setting error has not been generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HERF_A::_0
    }
    ///Horizontal byte number setting error has been generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HERF_A::_1
    }
}
///Field `HERF` writer - Horizontal Byte Number Setting Error Flag
pub type HERF_W<'a, REG> = crate::BitWriter0C<'a, REG, HERF_A>;
impl<'a, REG> HERF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Horizontal byte number setting error has not been generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HERF_A::_0)
    }
    ///Horizontal byte number setting error has been generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HERF_A::_1)
    }
}
impl R {
    ///Bit 0 - Frame Busy Flag
    #[inline(always)]
    pub fn fbsy(&self) -> FBSY_R {
        FBSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FIFO Empty Flag
    #[inline(always)]
    pub fn fempf(&self) -> FEMPF_R {
        FEMPF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Frame End Flag
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun Flag
    #[inline(always)]
    pub fn ovrf(&self) -> OVRF_R {
        OVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Underrun Flag
    #[inline(always)]
    pub fn udrf(&self) -> UDRF_R {
        UDRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Vertical Line Number Setting Error Flag
    #[inline(always)]
    pub fn verf(&self) -> VERF_R {
        VERF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Horizontal Byte Number Setting Error Flag
    #[inline(always)]
    pub fn herf(&self) -> HERF_R {
        HERF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Frame End Flag
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W<PCSR_SPEC> {
        FEF_W::new(self, 2)
    }
    ///Bit 3 - Overrun Flag
    #[inline(always)]
    pub fn ovrf(&mut self) -> OVRF_W<PCSR_SPEC> {
        OVRF_W::new(self, 3)
    }
    ///Bit 4 - Underrun Flag
    #[inline(always)]
    pub fn udrf(&mut self) -> UDRF_W<PCSR_SPEC> {
        UDRF_W::new(self, 4)
    }
    ///Bit 5 - Vertical Line Number Setting Error Flag
    #[inline(always)]
    pub fn verf(&mut self) -> VERF_W<PCSR_SPEC> {
        VERF_W::new(self, 5)
    }
    ///Bit 6 - Horizontal Byte Number Setting Error Flag
    #[inline(always)]
    pub fn herf(&mut self) -> HERF_W<PCSR_SPEC> {
        HERF_W::new(self, 6)
    }
}
/**PDC Status Register

You can [`read`](crate::Reg::read) this register and get [`pcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PCSR_SPEC;
impl crate::RegisterSpec for PCSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pcsr::R`](R) reader structure
impl crate::Readable for PCSR_SPEC {}
///`write(|w| ..)` method takes [`pcsr::W`](W) writer structure
impl crate::Writable for PCSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7c;
}
///`reset()` method sets PCSR to value 0x02
impl crate::Resettable for PCSR_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
