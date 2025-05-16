///Register `GTCR` reader
pub type R = crate::R<GTCR_SPEC>;
///Register `GTCR` writer
pub type W = crate::W<GTCR_SPEC>;
/**Count Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST_A {
    ///0: Count operation is stopped
    _0 = 0,
    ///1: Count operation is performed
    _1 = 1,
}
impl From<CST_A> for bool {
    #[inline(always)]
    fn from(variant: CST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CST` reader - Count Start
pub type CST_R = crate::BitReader<CST_A>;
impl CST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CST_A {
        match self.bits {
            false => CST_A::_0,
            true => CST_A::_1,
        }
    }
    ///Count operation is stopped
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CST_A::_0
    }
    ///Count operation is performed
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CST_A::_1
    }
}
///Field `CST` writer - Count Start
pub type CST_W<'a, REG> = crate::BitWriter<'a, REG, CST_A>;
impl<'a, REG> CST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Count operation is stopped
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CST_A::_0)
    }
    ///Count operation is performed
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CST_A::_1)
    }
}
/**Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MD_A {
    ///0: Saw-wave PWM mode (single buffer or double buffer possible)
    _000 = 0,
    ///1: Saw-wave one-shot pulse mode (fixed buffer operation)
    _001 = 1,
    ///2: Setting prohibited
    _010 = 2,
    ///3: Setting prohibited
    _011 = 3,
    ///4: Triangle-wave PWM mode 1 (32-bit transfer at crest) (single buffer or double buffer possible)
    _100 = 4,
    ///5: Triangle-wave PWM mode 2 (32-bit transfer at crest and trough) (single buffer or double buffer possible)
    _101 = 5,
    ///6: Triangle-wave PWM mode 3 (64-bit transfer at trough) fixed buffer operation)
    _110 = 6,
    ///7: Setting prohibited
    _111 = 7,
}
impl From<MD_A> for u8 {
    #[inline(always)]
    fn from(variant: MD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MD_A {
    type Ux = u8;
}
impl crate::IsEnum for MD_A {}
///Field `MD` reader - Mode Select
pub type MD_R = crate::FieldReader<MD_A>;
impl MD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MD_A {
        match self.bits {
            0 => MD_A::_000,
            1 => MD_A::_001,
            2 => MD_A::_010,
            3 => MD_A::_011,
            4 => MD_A::_100,
            5 => MD_A::_101,
            6 => MD_A::_110,
            7 => MD_A::_111,
            _ => unreachable!(),
        }
    }
    ///Saw-wave PWM mode (single buffer or double buffer possible)
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MD_A::_000
    }
    ///Saw-wave one-shot pulse mode (fixed buffer operation)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MD_A::_001
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MD_A::_010
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MD_A::_011
    }
    ///Triangle-wave PWM mode 1 (32-bit transfer at crest) (single buffer or double buffer possible)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MD_A::_100
    }
    ///Triangle-wave PWM mode 2 (32-bit transfer at crest and trough) (single buffer or double buffer possible)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == MD_A::_101
    }
    ///Triangle-wave PWM mode 3 (64-bit transfer at trough) fixed buffer operation)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == MD_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == MD_A::_111
    }
}
///Field `MD` writer - Mode Select
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MD_A, crate::Safe>;
impl<'a, REG> MD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Saw-wave PWM mode (single buffer or double buffer possible)
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_000)
    }
    ///Saw-wave one-shot pulse mode (fixed buffer operation)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_001)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_010)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_011)
    }
    ///Triangle-wave PWM mode 1 (32-bit transfer at crest) (single buffer or double buffer possible)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_100)
    }
    ///Triangle-wave PWM mode 2 (32-bit transfer at crest and trough) (single buffer or double buffer possible)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_101)
    }
    ///Triangle-wave PWM mode 3 (64-bit transfer at trough) fixed buffer operation)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::_111)
    }
}
/**Timer Prescaler Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPCS_A {
    ///0: PCLK/1
    _000 = 0,
    ///1: PCLK/4
    _001 = 1,
    ///2: PCLK/16
    _010 = 2,
    ///3: PCLK/64
    _011 = 3,
    ///4: PCLK/256
    _100 = 4,
    ///5: PCLK/1024
    _101 = 5,
    ///6: Setting prohibited
    OTHERS = 6,
}
impl From<TPCS_A> for u8 {
    #[inline(always)]
    fn from(variant: TPCS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPCS_A {
    type Ux = u8;
}
impl crate::IsEnum for TPCS_A {}
///Field `TPCS` reader - Timer Prescaler Select
pub type TPCS_R = crate::FieldReader<TPCS_A>;
impl TPCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TPCS_A {
        match self.bits {
            0 => TPCS_A::_000,
            1 => TPCS_A::_001,
            2 => TPCS_A::_010,
            3 => TPCS_A::_011,
            4 => TPCS_A::_100,
            5 => TPCS_A::_101,
            _ => TPCS_A::OTHERS,
        }
    }
    ///PCLK/1
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TPCS_A::_000
    }
    ///PCLK/4
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TPCS_A::_001
    }
    ///PCLK/16
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TPCS_A::_010
    }
    ///PCLK/64
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TPCS_A::_011
    }
    ///PCLK/256
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TPCS_A::_100
    }
    ///PCLK/1024
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == TPCS_A::_101
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), TPCS_A::OTHERS)
    }
}
///Field `TPCS` writer - Timer Prescaler Select
pub type TPCS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TPCS_A, crate::Safe>;
impl<'a, REG> TPCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK/1
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::_000)
    }
    ///PCLK/4
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::_001)
    }
    ///PCLK/16
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::_010)
    }
    ///PCLK/64
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::_011)
    }
    ///PCLK/256
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::_100)
    }
    ///PCLK/1024
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::_101)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(TPCS_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - Count Start
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new((self.bits & 1) != 0)
    }
    ///Bits 16:18 - Mode Select
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 24:26 - Timer Prescaler Select
    #[inline(always)]
    pub fn tpcs(&self) -> TPCS_R {
        TPCS_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    ///Bit 0 - Count Start
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W<GTCR_SPEC> {
        CST_W::new(self, 0)
    }
    ///Bits 16:18 - Mode Select
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<GTCR_SPEC> {
        MD_W::new(self, 16)
    }
    ///Bits 24:26 - Timer Prescaler Select
    #[inline(always)]
    pub fn tpcs(&mut self) -> TPCS_W<GTCR_SPEC> {
        TPCS_W::new(self, 24)
    }
}
/**General PWM Timer Control Register

You can [`read`](crate::Reg::read) this register and get [`gtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTCR_SPEC;
impl crate::RegisterSpec for GTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtcr::R`](R) reader structure
impl crate::Readable for GTCR_SPEC {}
///`write(|w| ..)` method takes [`gtcr::W`](W) writer structure
impl crate::Writable for GTCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCR to value 0
impl crate::Resettable for GTCR_SPEC {}
