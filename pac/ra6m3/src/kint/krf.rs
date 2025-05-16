///Register `KRF` reader
pub type R = crate::R<KRF_SPEC>;
///Register `KRF` writer
pub type W = crate::W<KRF_SPEC>;
/**Key interrupt flag 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF0_A {
    ///0: No interrupt detected
    _0 = 0,
    ///1: Interrupt detected.
    _1 = 1,
}
impl From<KRF0_A> for bool {
    #[inline(always)]
    fn from(variant: KRF0_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `KRF0` reader - Key interrupt flag 0

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type KRF0_R = crate::BitReader<KRF0_A>;
impl KRF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KRF0_A {
        match self.bits {
            false => KRF0_A::_0,
            true => KRF0_A::_1,
        }
    }
    ///No interrupt detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF0_A::_0
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF0_A::_1
    }
}
///Field `KRF0` writer - Key interrupt flag 0
pub type KRF0_W<'a, REG> = crate::BitWriter0C<'a, REG, KRF0_A>;
impl<'a, REG> KRF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KRF0_A::_0)
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KRF0_A::_1)
    }
}
/**Key interrupt flag 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF1_A {
    ///0: No interrupt detected
    _0 = 0,
    ///1: Interrupt detected.
    _1 = 1,
}
impl From<KRF1_A> for bool {
    #[inline(always)]
    fn from(variant: KRF1_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `KRF1` reader - Key interrupt flag 1

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type KRF1_R = crate::BitReader<KRF1_A>;
impl KRF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KRF1_A {
        match self.bits {
            false => KRF1_A::_0,
            true => KRF1_A::_1,
        }
    }
    ///No interrupt detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF1_A::_0
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF1_A::_1
    }
}
///Field `KRF1` writer - Key interrupt flag 1
pub type KRF1_W<'a, REG> = crate::BitWriter0C<'a, REG, KRF1_A>;
impl<'a, REG> KRF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KRF1_A::_0)
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KRF1_A::_1)
    }
}
/**Key interrupt flag 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF2_A {
    ///0: No interrupt detected
    _0 = 0,
    ///1: Interrupt detected.
    _1 = 1,
}
impl From<KRF2_A> for bool {
    #[inline(always)]
    fn from(variant: KRF2_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `KRF2` reader - Key interrupt flag 2

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type KRF2_R = crate::BitReader<KRF2_A>;
impl KRF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KRF2_A {
        match self.bits {
            false => KRF2_A::_0,
            true => KRF2_A::_1,
        }
    }
    ///No interrupt detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF2_A::_0
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF2_A::_1
    }
}
///Field `KRF2` writer - Key interrupt flag 2
pub type KRF2_W<'a, REG> = crate::BitWriter0C<'a, REG, KRF2_A>;
impl<'a, REG> KRF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KRF2_A::_0)
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KRF2_A::_1)
    }
}
/**Key interrupt flag 3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF3_A {
    ///0: No interrupt detected
    _0 = 0,
    ///1: Interrupt detected.
    _1 = 1,
}
impl From<KRF3_A> for bool {
    #[inline(always)]
    fn from(variant: KRF3_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `KRF3` reader - Key interrupt flag 3

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type KRF3_R = crate::BitReader<KRF3_A>;
impl KRF3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KRF3_A {
        match self.bits {
            false => KRF3_A::_0,
            true => KRF3_A::_1,
        }
    }
    ///No interrupt detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF3_A::_0
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF3_A::_1
    }
}
///Field `KRF3` writer - Key interrupt flag 3
pub type KRF3_W<'a, REG> = crate::BitWriter0C<'a, REG, KRF3_A>;
impl<'a, REG> KRF3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KRF3_A::_0)
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KRF3_A::_1)
    }
}
/**Key interrupt flag 4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF4_A {
    ///0: No interrupt detected
    _0 = 0,
    ///1: Interrupt detected.
    _1 = 1,
}
impl From<KRF4_A> for bool {
    #[inline(always)]
    fn from(variant: KRF4_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `KRF4` reader - Key interrupt flag 4

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type KRF4_R = crate::BitReader<KRF4_A>;
impl KRF4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KRF4_A {
        match self.bits {
            false => KRF4_A::_0,
            true => KRF4_A::_1,
        }
    }
    ///No interrupt detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF4_A::_0
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF4_A::_1
    }
}
///Field `KRF4` writer - Key interrupt flag 4
pub type KRF4_W<'a, REG> = crate::BitWriter0C<'a, REG, KRF4_A>;
impl<'a, REG> KRF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KRF4_A::_0)
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KRF4_A::_1)
    }
}
/**Key interrupt flag 5

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF5_A {
    ///0: No interrupt detected
    _0 = 0,
    ///1: Interrupt detected.
    _1 = 1,
}
impl From<KRF5_A> for bool {
    #[inline(always)]
    fn from(variant: KRF5_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `KRF5` reader - Key interrupt flag 5

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type KRF5_R = crate::BitReader<KRF5_A>;
impl KRF5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KRF5_A {
        match self.bits {
            false => KRF5_A::_0,
            true => KRF5_A::_1,
        }
    }
    ///No interrupt detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF5_A::_0
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF5_A::_1
    }
}
///Field `KRF5` writer - Key interrupt flag 5
pub type KRF5_W<'a, REG> = crate::BitWriter0C<'a, REG, KRF5_A>;
impl<'a, REG> KRF5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KRF5_A::_0)
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KRF5_A::_1)
    }
}
/**Key interrupt flag 6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF6_A {
    ///0: No interrupt detected
    _0 = 0,
    ///1: Interrupt detected.
    _1 = 1,
}
impl From<KRF6_A> for bool {
    #[inline(always)]
    fn from(variant: KRF6_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `KRF6` reader - Key interrupt flag 6

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type KRF6_R = crate::BitReader<KRF6_A>;
impl KRF6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KRF6_A {
        match self.bits {
            false => KRF6_A::_0,
            true => KRF6_A::_1,
        }
    }
    ///No interrupt detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF6_A::_0
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF6_A::_1
    }
}
///Field `KRF6` writer - Key interrupt flag 6
pub type KRF6_W<'a, REG> = crate::BitWriter0C<'a, REG, KRF6_A>;
impl<'a, REG> KRF6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KRF6_A::_0)
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KRF6_A::_1)
    }
}
/**Key interrupt flag 7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRF7_A {
    ///0: No interrupt detected
    _0 = 0,
    ///1: Interrupt detected.
    _1 = 1,
}
impl From<KRF7_A> for bool {
    #[inline(always)]
    fn from(variant: KRF7_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `KRF7` reader - Key interrupt flag 7

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type KRF7_R = crate::BitReader<KRF7_A>;
impl KRF7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KRF7_A {
        match self.bits {
            false => KRF7_A::_0,
            true => KRF7_A::_1,
        }
    }
    ///No interrupt detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRF7_A::_0
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRF7_A::_1
    }
}
///Field `KRF7` writer - Key interrupt flag 7
pub type KRF7_W<'a, REG> = crate::BitWriter0C<'a, REG, KRF7_A>;
impl<'a, REG> KRF7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KRF7_A::_0)
    }
    ///Interrupt detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KRF7_A::_1)
    }
}
impl R {
    ///Bit 0 - Key interrupt flag 0
    #[inline(always)]
    pub fn krf0(&self) -> KRF0_R {
        KRF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Key interrupt flag 1
    #[inline(always)]
    pub fn krf1(&self) -> KRF1_R {
        KRF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key interrupt flag 2
    #[inline(always)]
    pub fn krf2(&self) -> KRF2_R {
        KRF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Key interrupt flag 3
    #[inline(always)]
    pub fn krf3(&self) -> KRF3_R {
        KRF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Key interrupt flag 4
    #[inline(always)]
    pub fn krf4(&self) -> KRF4_R {
        KRF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Key interrupt flag 5
    #[inline(always)]
    pub fn krf5(&self) -> KRF5_R {
        KRF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Key interrupt flag 6
    #[inline(always)]
    pub fn krf6(&self) -> KRF6_R {
        KRF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Key interrupt flag 7
    #[inline(always)]
    pub fn krf7(&self) -> KRF7_R {
        KRF7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Key interrupt flag 0
    #[inline(always)]
    pub fn krf0(&mut self) -> KRF0_W<KRF_SPEC> {
        KRF0_W::new(self, 0)
    }
    ///Bit 1 - Key interrupt flag 1
    #[inline(always)]
    pub fn krf1(&mut self) -> KRF1_W<KRF_SPEC> {
        KRF1_W::new(self, 1)
    }
    ///Bit 2 - Key interrupt flag 2
    #[inline(always)]
    pub fn krf2(&mut self) -> KRF2_W<KRF_SPEC> {
        KRF2_W::new(self, 2)
    }
    ///Bit 3 - Key interrupt flag 3
    #[inline(always)]
    pub fn krf3(&mut self) -> KRF3_W<KRF_SPEC> {
        KRF3_W::new(self, 3)
    }
    ///Bit 4 - Key interrupt flag 4
    #[inline(always)]
    pub fn krf4(&mut self) -> KRF4_W<KRF_SPEC> {
        KRF4_W::new(self, 4)
    }
    ///Bit 5 - Key interrupt flag 5
    #[inline(always)]
    pub fn krf5(&mut self) -> KRF5_W<KRF_SPEC> {
        KRF5_W::new(self, 5)
    }
    ///Bit 6 - Key interrupt flag 6
    #[inline(always)]
    pub fn krf6(&mut self) -> KRF6_W<KRF_SPEC> {
        KRF6_W::new(self, 6)
    }
    ///Bit 7 - Key interrupt flag 7
    #[inline(always)]
    pub fn krf7(&mut self) -> KRF7_W<KRF_SPEC> {
        KRF7_W::new(self, 7)
    }
}
/**KEY Return Flag Register

You can [`read`](crate::Reg::read) this register and get [`krf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

<div class="warning">The register is <b>modified</b> in some way after a read operation.</div>*/
pub struct KRF_SPEC;
impl crate::RegisterSpec for KRF_SPEC {
    type Ux = u8;
}
///`read()` method returns [`krf::R`](R) reader structure
impl crate::Readable for KRF_SPEC {}
///`write(|w| ..)` method takes [`krf::W`](W) writer structure
impl crate::Writable for KRF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xff;
}
///`reset()` method sets KRF to value 0
impl crate::Resettable for KRF_SPEC {}
