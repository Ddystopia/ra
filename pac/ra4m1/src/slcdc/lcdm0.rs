///Register `LCDM0` reader
pub type R = crate::R<LCDM0_SPEC>;
///Register `LCDM0` writer
pub type W = crate::W<LCDM0_SPEC>;
/**LCD Display Bias Method Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LBAS_A {
    ///0: 1/2 bias method
    _00 = 0,
    ///1: 1/3 bias method
    _01 = 1,
    ///2: 1/4 bias method
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<LBAS_A> for u8 {
    #[inline(always)]
    fn from(variant: LBAS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LBAS_A {
    type Ux = u8;
}
impl crate::IsEnum for LBAS_A {}
///Field `LBAS` reader - LCD Display Bias Method Select
pub type LBAS_R = crate::FieldReader<LBAS_A>;
impl LBAS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBAS_A {
        match self.bits {
            0 => LBAS_A::_00,
            1 => LBAS_A::_01,
            2 => LBAS_A::_10,
            3 => LBAS_A::_11,
            _ => unreachable!(),
        }
    }
    ///1/2 bias method
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LBAS_A::_00
    }
    ///1/3 bias method
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LBAS_A::_01
    }
    ///1/4 bias method
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LBAS_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LBAS_A::_11
    }
}
///Field `LBAS` writer - LCD Display Bias Method Select
pub type LBAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LBAS_A, crate::Safe>;
impl<'a, REG> LBAS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1/2 bias method
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(LBAS_A::_00)
    }
    ///1/3 bias method
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(LBAS_A::_01)
    }
    ///1/4 bias method
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(LBAS_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(LBAS_A::_11)
    }
}
/**Time Slice of LCD Display Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDTY_A {
    ///0: Static
    _000 = 0,
    ///1: 2-time slice
    _001 = 1,
    ///2: 3-time slice
    _010 = 2,
    ///3: 4-time slice
    _011 = 3,
    ///5: 8-time slice
    _101 = 5,
    ///4: Setting prohibited
    OTHERS = 4,
}
impl From<LDTY_A> for u8 {
    #[inline(always)]
    fn from(variant: LDTY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LDTY_A {
    type Ux = u8;
}
impl crate::IsEnum for LDTY_A {}
///Field `LDTY` reader - Time Slice of LCD Display Select
pub type LDTY_R = crate::FieldReader<LDTY_A>;
impl LDTY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LDTY_A {
        match self.bits {
            0 => LDTY_A::_000,
            1 => LDTY_A::_001,
            2 => LDTY_A::_010,
            3 => LDTY_A::_011,
            5 => LDTY_A::_101,
            _ => LDTY_A::OTHERS,
        }
    }
    ///Static
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == LDTY_A::_000
    }
    ///2-time slice
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == LDTY_A::_001
    }
    ///3-time slice
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == LDTY_A::_010
    }
    ///4-time slice
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == LDTY_A::_011
    }
    ///8-time slice
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == LDTY_A::_101
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), LDTY_A::OTHERS)
    }
}
///Field `LDTY` writer - Time Slice of LCD Display Select
pub type LDTY_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LDTY_A, crate::Safe>;
impl<'a, REG> LDTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Static
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(LDTY_A::_000)
    }
    ///2-time slice
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(LDTY_A::_001)
    }
    ///3-time slice
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(LDTY_A::_010)
    }
    ///4-time slice
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(LDTY_A::_011)
    }
    ///8-time slice
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(LDTY_A::_101)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(LDTY_A::OTHERS)
    }
}
/**LCD display waveform selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LWAVE_A {
    ///0: Waveform A
    _0 = 0,
    ///1: Waveform B
    _1 = 1,
}
impl From<LWAVE_A> for bool {
    #[inline(always)]
    fn from(variant: LWAVE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LWAVE` reader - LCD display waveform selection
pub type LWAVE_R = crate::BitReader<LWAVE_A>;
impl LWAVE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LWAVE_A {
        match self.bits {
            false => LWAVE_A::_0,
            true => LWAVE_A::_1,
        }
    }
    ///Waveform A
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LWAVE_A::_0
    }
    ///Waveform B
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LWAVE_A::_1
    }
}
///Field `LWAVE` writer - LCD display waveform selection
pub type LWAVE_W<'a, REG> = crate::BitWriter<'a, REG, LWAVE_A>;
impl<'a, REG> LWAVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Waveform A
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LWAVE_A::_0)
    }
    ///Waveform B
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LWAVE_A::_1)
    }
}
/**LCD drive voltage generator selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDSET_A {
    ///0: External resistance division method
    _00 = 0,
    ///1: Internal voltage boosting method
    _01 = 1,
    ///2: Capacitor split method
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<MDSET_A> for u8 {
    #[inline(always)]
    fn from(variant: MDSET_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MDSET_A {
    type Ux = u8;
}
impl crate::IsEnum for MDSET_A {}
///Field `MDSET` reader - LCD drive voltage generator selection
pub type MDSET_R = crate::FieldReader<MDSET_A>;
impl MDSET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MDSET_A {
        match self.bits {
            0 => MDSET_A::_00,
            1 => MDSET_A::_01,
            2 => MDSET_A::_10,
            3 => MDSET_A::_11,
            _ => unreachable!(),
        }
    }
    ///External resistance division method
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MDSET_A::_00
    }
    ///Internal voltage boosting method
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MDSET_A::_01
    }
    ///Capacitor split method
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MDSET_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MDSET_A::_11
    }
}
///Field `MDSET` writer - LCD drive voltage generator selection
pub type MDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MDSET_A, crate::Safe>;
impl<'a, REG> MDSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///External resistance division method
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(MDSET_A::_00)
    }
    ///Internal voltage boosting method
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(MDSET_A::_01)
    }
    ///Capacitor split method
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(MDSET_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(MDSET_A::_11)
    }
}
impl R {
    ///Bits 0:1 - LCD Display Bias Method Select
    #[inline(always)]
    pub fn lbas(&self) -> LBAS_R {
        LBAS_R::new(self.bits & 3)
    }
    ///Bits 2:4 - Time Slice of LCD Display Select
    #[inline(always)]
    pub fn ldty(&self) -> LDTY_R {
        LDTY_R::new((self.bits >> 2) & 7)
    }
    ///Bit 5 - LCD display waveform selection
    #[inline(always)]
    pub fn lwave(&self) -> LWAVE_R {
        LWAVE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - LCD drive voltage generator selection
    #[inline(always)]
    pub fn mdset(&self) -> MDSET_R {
        MDSET_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    ///Bits 0:1 - LCD Display Bias Method Select
    #[inline(always)]
    pub fn lbas(&mut self) -> LBAS_W<LCDM0_SPEC> {
        LBAS_W::new(self, 0)
    }
    ///Bits 2:4 - Time Slice of LCD Display Select
    #[inline(always)]
    pub fn ldty(&mut self) -> LDTY_W<LCDM0_SPEC> {
        LDTY_W::new(self, 2)
    }
    ///Bit 5 - LCD display waveform selection
    #[inline(always)]
    pub fn lwave(&mut self) -> LWAVE_W<LCDM0_SPEC> {
        LWAVE_W::new(self, 5)
    }
    ///Bits 6:7 - LCD drive voltage generator selection
    #[inline(always)]
    pub fn mdset(&mut self) -> MDSET_W<LCDM0_SPEC> {
        MDSET_W::new(self, 6)
    }
}
/**LCD Mode Register 0

You can [`read`](crate::Reg::read) this register and get [`lcdm0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCDM0_SPEC;
impl crate::RegisterSpec for LCDM0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`lcdm0::R`](R) reader structure
impl crate::Readable for LCDM0_SPEC {}
///`write(|w| ..)` method takes [`lcdm0::W`](W) writer structure
impl crate::Writable for LCDM0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCDM0 to value 0
impl crate::Resettable for LCDM0_SPEC {}
