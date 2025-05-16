///Register `SFMSAC` reader
pub type R = crate::R<SFMSAC_SPEC>;
///Register `SFMSAC` writer
pub type W = crate::W<SFMSAC_SPEC>;
/**Selection the number of address bits of the serial interface

Value on reset: 2*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMAS_A {
    ///0: 1byte
    _00 = 0,
    ///1: 2bytes
    _01 = 1,
    ///2: 3bytes
    _10 = 2,
    ///3: 4 bytes
    _11 = 3,
}
impl From<SFMAS_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMAS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SFMAS_A {
    type Ux = u8;
}
impl crate::IsEnum for SFMAS_A {}
///Field `SFMAS` reader - Selection the number of address bits of the serial interface
pub type SFMAS_R = crate::FieldReader<SFMAS_A>;
impl SFMAS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMAS_A {
        match self.bits {
            0 => SFMAS_A::_00,
            1 => SFMAS_A::_01,
            2 => SFMAS_A::_10,
            3 => SFMAS_A::_11,
            _ => unreachable!(),
        }
    }
    ///1byte
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SFMAS_A::_00
    }
    ///2bytes
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SFMAS_A::_01
    }
    ///3bytes
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SFMAS_A::_10
    }
    ///4 bytes
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SFMAS_A::_11
    }
}
///Field `SFMAS` writer - Selection the number of address bits of the serial interface
pub type SFMAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SFMAS_A, crate::Safe>;
impl<'a, REG> SFMAS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1byte
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SFMAS_A::_00)
    }
    ///2bytes
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SFMAS_A::_01)
    }
    ///3bytes
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SFMAS_A::_10)
    }
    ///4 bytes
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SFMAS_A::_11)
    }
}
/**Selection of a default instruction code, when Serial Interface address width is selected 4 bytes.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFM4BC_A {
    ///0: Does not use 4 Byte address read Instruction code
    _0 = 0,
    ///1: Use 4 Byte address read Instruction code
    _1 = 1,
}
impl From<SFM4BC_A> for bool {
    #[inline(always)]
    fn from(variant: SFM4BC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFM4BC` reader - Selection of a default instruction code, when Serial Interface address width is selected 4 bytes.
pub type SFM4BC_R = crate::BitReader<SFM4BC_A>;
impl SFM4BC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFM4BC_A {
        match self.bits {
            false => SFM4BC_A::_0,
            true => SFM4BC_A::_1,
        }
    }
    ///Does not use 4 Byte address read Instruction code
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFM4BC_A::_0
    }
    ///Use 4 Byte address read Instruction code
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFM4BC_A::_1
    }
}
///Field `SFM4BC` writer - Selection of a default instruction code, when Serial Interface address width is selected 4 bytes.
pub type SFM4BC_W<'a, REG> = crate::BitWriter<'a, REG, SFM4BC_A>;
impl<'a, REG> SFM4BC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not use 4 Byte address read Instruction code
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFM4BC_A::_0)
    }
    ///Use 4 Byte address read Instruction code
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFM4BC_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Selection the number of address bits of the serial interface
    #[inline(always)]
    pub fn sfmas(&self) -> SFMAS_R {
        SFMAS_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - Selection of a default instruction code, when Serial Interface address width is selected 4 bytes.
    #[inline(always)]
    pub fn sfm4bc(&self) -> SFM4BC_R {
        SFM4BC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Selection the number of address bits of the serial interface
    #[inline(always)]
    pub fn sfmas(&mut self) -> SFMAS_W<SFMSAC_SPEC> {
        SFMAS_W::new(self, 0)
    }
    ///Bit 4 - Selection of a default instruction code, when Serial Interface address width is selected 4 bytes.
    #[inline(always)]
    pub fn sfm4bc(&mut self) -> SFM4BC_W<SFMSAC_SPEC> {
        SFM4BC_W::new(self, 4)
    }
}
/**Address Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmsac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMSAC_SPEC;
impl crate::RegisterSpec for SFMSAC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmsac::R`](R) reader structure
impl crate::Readable for SFMSAC_SPEC {}
///`write(|w| ..)` method takes [`sfmsac::W`](W) writer structure
impl crate::Writable for SFMSAC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSAC to value 0x02
impl crate::Resettable for SFMSAC_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
