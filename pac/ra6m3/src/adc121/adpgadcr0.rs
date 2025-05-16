///Register `ADPGADCR0` reader
pub type R = crate::R<ADPGADCR0_SPEC>;
///Register `ADPGADCR0` writer
pub type W = crate::W<ADPGADCR0_SPEC>;
/**P000 Differential Input Gain SettingNOTE: When these bits are used, set {P000DEN, P000GEN} to 11b.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P000DG_A {
    ///0: x 1.5
    _00 = 0,
    ///1: x 2.333
    _01 = 1,
    ///2: x 4.0
    _10 = 2,
    ///3: x 5.667
    _11 = 3,
}
impl From<P000DG_A> for u8 {
    #[inline(always)]
    fn from(variant: P000DG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P000DG_A {
    type Ux = u8;
}
impl crate::IsEnum for P000DG_A {}
///Field `P000DG` reader - P000 Differential Input Gain SettingNOTE: When these bits are used, set {P000DEN, P000GEN} to 11b.
pub type P000DG_R = crate::FieldReader<P000DG_A>;
impl P000DG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P000DG_A {
        match self.bits {
            0 => P000DG_A::_00,
            1 => P000DG_A::_01,
            2 => P000DG_A::_10,
            3 => P000DG_A::_11,
            _ => unreachable!(),
        }
    }
    ///x 1.5
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == P000DG_A::_00
    }
    ///x 2.333
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == P000DG_A::_01
    }
    ///x 4.0
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == P000DG_A::_10
    }
    ///x 5.667
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == P000DG_A::_11
    }
}
///Field `P000DG` writer - P000 Differential Input Gain SettingNOTE: When these bits are used, set {P000DEN, P000GEN} to 11b.
pub type P000DG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, P000DG_A, crate::Safe>;
impl<'a, REG> P000DG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///x 1.5
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(P000DG_A::_00)
    }
    ///x 2.333
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(P000DG_A::_01)
    }
    ///x 4.0
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(P000DG_A::_10)
    }
    ///x 5.667
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(P000DG_A::_11)
    }
}
/**P000 Differential Input Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P000DEN_A {
    ///0: Differential input is disabled.
    _0 = 0,
    ///1: Differential input is enabled.
    _1 = 1,
}
impl From<P000DEN_A> for bool {
    #[inline(always)]
    fn from(variant: P000DEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P000DEN` reader - P000 Differential Input Enable
pub type P000DEN_R = crate::BitReader<P000DEN_A>;
impl P000DEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P000DEN_A {
        match self.bits {
            false => P000DEN_A::_0,
            true => P000DEN_A::_1,
        }
    }
    ///Differential input is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P000DEN_A::_0
    }
    ///Differential input is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P000DEN_A::_1
    }
}
///Field `P000DEN` writer - P000 Differential Input Enable
pub type P000DEN_W<'a, REG> = crate::BitWriter<'a, REG, P000DEN_A>;
impl<'a, REG> P000DEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Differential input is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P000DEN_A::_0)
    }
    ///Differential input is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P000DEN_A::_1)
    }
}
/**P001 Differential Input Gain SettingNOTE: When these bits are used, set {P001DEN, P001GEN} to 11b.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P001DG_A {
    ///0: x 1.5
    _00 = 0,
    ///1: x 2.333
    _01 = 1,
    ///2: x 4.0
    _10 = 2,
    ///3: x 5.667
    _11 = 3,
}
impl From<P001DG_A> for u8 {
    #[inline(always)]
    fn from(variant: P001DG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P001DG_A {
    type Ux = u8;
}
impl crate::IsEnum for P001DG_A {}
///Field `P001DG` reader - P001 Differential Input Gain SettingNOTE: When these bits are used, set {P001DEN, P001GEN} to 11b.
pub type P001DG_R = crate::FieldReader<P001DG_A>;
impl P001DG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P001DG_A {
        match self.bits {
            0 => P001DG_A::_00,
            1 => P001DG_A::_01,
            2 => P001DG_A::_10,
            3 => P001DG_A::_11,
            _ => unreachable!(),
        }
    }
    ///x 1.5
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == P001DG_A::_00
    }
    ///x 2.333
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == P001DG_A::_01
    }
    ///x 4.0
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == P001DG_A::_10
    }
    ///x 5.667
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == P001DG_A::_11
    }
}
///Field `P001DG` writer - P001 Differential Input Gain SettingNOTE: When these bits are used, set {P001DEN, P001GEN} to 11b.
pub type P001DG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, P001DG_A, crate::Safe>;
impl<'a, REG> P001DG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///x 1.5
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(P001DG_A::_00)
    }
    ///x 2.333
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(P001DG_A::_01)
    }
    ///x 4.0
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(P001DG_A::_10)
    }
    ///x 5.667
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(P001DG_A::_11)
    }
}
/**P001 Differential Input Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P001DEN_A {
    ///0: Differential input is disabled.
    _0 = 0,
    ///1: Differential input is enabled.
    _1 = 1,
}
impl From<P001DEN_A> for bool {
    #[inline(always)]
    fn from(variant: P001DEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P001DEN` reader - P001 Differential Input Enable
pub type P001DEN_R = crate::BitReader<P001DEN_A>;
impl P001DEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P001DEN_A {
        match self.bits {
            false => P001DEN_A::_0,
            true => P001DEN_A::_1,
        }
    }
    ///Differential input is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P001DEN_A::_0
    }
    ///Differential input is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P001DEN_A::_1
    }
}
///Field `P001DEN` writer - P001 Differential Input Enable
pub type P001DEN_W<'a, REG> = crate::BitWriter<'a, REG, P001DEN_A>;
impl<'a, REG> P001DEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Differential input is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P001DEN_A::_0)
    }
    ///Differential input is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P001DEN_A::_1)
    }
}
/**P002 Differential Input Gain SettingNOTE: When these bits are used, set {P002DEN, P002GEN} to 11b.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P002DG_A {
    ///0: x 1.5
    _00 = 0,
    ///1: x 2.333
    _01 = 1,
    ///2: x 4.0
    _10 = 2,
    ///3: x 5.667
    _11 = 3,
}
impl From<P002DG_A> for u8 {
    #[inline(always)]
    fn from(variant: P002DG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P002DG_A {
    type Ux = u8;
}
impl crate::IsEnum for P002DG_A {}
///Field `P002DG` reader - P002 Differential Input Gain SettingNOTE: When these bits are used, set {P002DEN, P002GEN} to 11b.
pub type P002DG_R = crate::FieldReader<P002DG_A>;
impl P002DG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P002DG_A {
        match self.bits {
            0 => P002DG_A::_00,
            1 => P002DG_A::_01,
            2 => P002DG_A::_10,
            3 => P002DG_A::_11,
            _ => unreachable!(),
        }
    }
    ///x 1.5
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == P002DG_A::_00
    }
    ///x 2.333
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == P002DG_A::_01
    }
    ///x 4.0
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == P002DG_A::_10
    }
    ///x 5.667
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == P002DG_A::_11
    }
}
///Field `P002DG` writer - P002 Differential Input Gain SettingNOTE: When these bits are used, set {P002DEN, P002GEN} to 11b.
pub type P002DG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, P002DG_A, crate::Safe>;
impl<'a, REG> P002DG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///x 1.5
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(P002DG_A::_00)
    }
    ///x 2.333
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(P002DG_A::_01)
    }
    ///x 4.0
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(P002DG_A::_10)
    }
    ///x 5.667
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(P002DG_A::_11)
    }
}
/**P002 Differential Input Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P002DEN_A {
    ///0: Differential input is disabled.
    _0 = 0,
    ///1: Differential input is enabled.
    _1 = 1,
}
impl From<P002DEN_A> for bool {
    #[inline(always)]
    fn from(variant: P002DEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P002DEN` reader - P002 Differential Input Enable
pub type P002DEN_R = crate::BitReader<P002DEN_A>;
impl P002DEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P002DEN_A {
        match self.bits {
            false => P002DEN_A::_0,
            true => P002DEN_A::_1,
        }
    }
    ///Differential input is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P002DEN_A::_0
    }
    ///Differential input is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P002DEN_A::_1
    }
}
///Field `P002DEN` writer - P002 Differential Input Enable
pub type P002DEN_W<'a, REG> = crate::BitWriter<'a, REG, P002DEN_A>;
impl<'a, REG> P002DEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Differential input is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P002DEN_A::_0)
    }
    ///Differential input is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P002DEN_A::_1)
    }
}
/**P003 Differential Input Gain SettingNOTE: When these bits are used, set {P003DEN, P003GEN} to 11b.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P003DG_A {
    ///0: x 1.5
    _00 = 0,
    ///1: x 2.333
    _01 = 1,
    ///2: x 4.0
    _10 = 2,
    ///3: x 5.667
    _11 = 3,
}
impl From<P003DG_A> for u8 {
    #[inline(always)]
    fn from(variant: P003DG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P003DG_A {
    type Ux = u8;
}
impl crate::IsEnum for P003DG_A {}
///Field `P003DG` reader - P003 Differential Input Gain SettingNOTE: When these bits are used, set {P003DEN, P003GEN} to 11b.
pub type P003DG_R = crate::FieldReader<P003DG_A>;
impl P003DG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P003DG_A {
        match self.bits {
            0 => P003DG_A::_00,
            1 => P003DG_A::_01,
            2 => P003DG_A::_10,
            3 => P003DG_A::_11,
            _ => unreachable!(),
        }
    }
    ///x 1.5
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == P003DG_A::_00
    }
    ///x 2.333
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == P003DG_A::_01
    }
    ///x 4.0
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == P003DG_A::_10
    }
    ///x 5.667
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == P003DG_A::_11
    }
}
///Field `P003DG` writer - P003 Differential Input Gain SettingNOTE: When these bits are used, set {P003DEN, P003GEN} to 11b.
pub type P003DG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, P003DG_A, crate::Safe>;
impl<'a, REG> P003DG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///x 1.5
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(P003DG_A::_00)
    }
    ///x 2.333
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(P003DG_A::_01)
    }
    ///x 4.0
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(P003DG_A::_10)
    }
    ///x 5.667
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(P003DG_A::_11)
    }
}
impl R {
    ///Bits 0:1 - P000 Differential Input Gain SettingNOTE: When these bits are used, set {P000DEN, P000GEN} to 11b.
    #[inline(always)]
    pub fn p000dg(&self) -> P000DG_R {
        P000DG_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - P000 Differential Input Enable
    #[inline(always)]
    pub fn p000den(&self) -> P000DEN_R {
        P000DEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - P001 Differential Input Gain SettingNOTE: When these bits are used, set {P001DEN, P001GEN} to 11b.
    #[inline(always)]
    pub fn p001dg(&self) -> P001DG_R {
        P001DG_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - P001 Differential Input Enable
    #[inline(always)]
    pub fn p001den(&self) -> P001DEN_R {
        P001DEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - P002 Differential Input Gain SettingNOTE: When these bits are used, set {P002DEN, P002GEN} to 11b.
    #[inline(always)]
    pub fn p002dg(&self) -> P002DG_R {
        P002DG_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - P002 Differential Input Enable
    #[inline(always)]
    pub fn p002den(&self) -> P002DEN_R {
        P002DEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - P003 Differential Input Gain SettingNOTE: When these bits are used, set {P003DEN, P003GEN} to 11b.
    #[inline(always)]
    pub fn p003dg(&self) -> P003DG_R {
        P003DG_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - P000 Differential Input Gain SettingNOTE: When these bits are used, set {P000DEN, P000GEN} to 11b.
    #[inline(always)]
    pub fn p000dg(&mut self) -> P000DG_W<ADPGADCR0_SPEC> {
        P000DG_W::new(self, 0)
    }
    ///Bit 3 - P000 Differential Input Enable
    #[inline(always)]
    pub fn p000den(&mut self) -> P000DEN_W<ADPGADCR0_SPEC> {
        P000DEN_W::new(self, 3)
    }
    ///Bits 4:5 - P001 Differential Input Gain SettingNOTE: When these bits are used, set {P001DEN, P001GEN} to 11b.
    #[inline(always)]
    pub fn p001dg(&mut self) -> P001DG_W<ADPGADCR0_SPEC> {
        P001DG_W::new(self, 4)
    }
    ///Bit 7 - P001 Differential Input Enable
    #[inline(always)]
    pub fn p001den(&mut self) -> P001DEN_W<ADPGADCR0_SPEC> {
        P001DEN_W::new(self, 7)
    }
    ///Bits 8:9 - P002 Differential Input Gain SettingNOTE: When these bits are used, set {P002DEN, P002GEN} to 11b.
    #[inline(always)]
    pub fn p002dg(&mut self) -> P002DG_W<ADPGADCR0_SPEC> {
        P002DG_W::new(self, 8)
    }
    ///Bit 11 - P002 Differential Input Enable
    #[inline(always)]
    pub fn p002den(&mut self) -> P002DEN_W<ADPGADCR0_SPEC> {
        P002DEN_W::new(self, 11)
    }
    ///Bits 12:13 - P003 Differential Input Gain SettingNOTE: When these bits are used, set {P003DEN, P003GEN} to 11b.
    #[inline(always)]
    pub fn p003dg(&mut self) -> P003DG_W<ADPGADCR0_SPEC> {
        P003DG_W::new(self, 12)
    }
}
/**A/D Programmable Gain Amplifier Differential Input Control Register

You can [`read`](crate::Reg::read) this register and get [`adpgadcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adpgadcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADPGADCR0_SPEC;
impl crate::RegisterSpec for ADPGADCR0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adpgadcr0::R`](R) reader structure
impl crate::Readable for ADPGADCR0_SPEC {}
///`write(|w| ..)` method takes [`adpgadcr0::W`](W) writer structure
impl crate::Writable for ADPGADCR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADPGADCR0 to value 0
impl crate::Resettable for ADPGADCR0_SPEC {}
