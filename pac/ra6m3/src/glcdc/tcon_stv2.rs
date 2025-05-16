///Register `TCON_STV%s2` reader
pub type R = crate::R<TCON_STV2_SPEC>;
///Register `TCON_STV%s2` writer
pub type W = crate::W<TCON_STV2_SPEC>;
/**Output signal select control for VSOUT (controlled by TCON_STVA2 register)/VEOUT (controlled by the TCON_STVB2 register) pin

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    ///7: DE
    _111 = 7,
    ///6: Setting prohibited
    _110 = 6,
    ///5: Setting prohibited
    _101 = 5,
    ///4: Setting prohibited
    _100 = 4,
    ///3: STHB
    _011 = 3,
    ///2: STHA
    _010 = 2,
    ///1: STVB
    _001 = 1,
    ///0: STVA
    _000 = 0,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for SEL_A {}
///Field `SEL` reader - Output signal select control for VSOUT (controlled by TCON_STVA2 register)/VEOUT (controlled by the TCON_STVB2 register) pin
pub type SEL_R = crate::FieldReader<SEL_A>;
impl SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SEL_A {
        match self.bits {
            7 => SEL_A::_111,
            6 => SEL_A::_110,
            5 => SEL_A::_101,
            4 => SEL_A::_100,
            3 => SEL_A::_011,
            2 => SEL_A::_010,
            1 => SEL_A::_001,
            0 => SEL_A::_000,
            _ => unreachable!(),
        }
    }
    ///DE
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SEL_A::_111
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SEL_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SEL_A::_101
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SEL_A::_100
    }
    ///STHB
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SEL_A::_011
    }
    ///STHA
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SEL_A::_010
    }
    ///STVB
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SEL_A::_001
    }
    ///STVA
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SEL_A::_000
    }
}
///Field `SEL` writer - Output signal select control for VSOUT (controlled by TCON_STVA2 register)/VEOUT (controlled by the TCON_STVB2 register) pin
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SEL_A, crate::Safe>;
impl<'a, REG> SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///DE
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_111)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_101)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_100)
    }
    ///STHB
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_011)
    }
    ///STHA
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_010)
    }
    ///STVB
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_001)
    }
    ///STVA
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::_000)
    }
}
/**STVx signal polarity inversion control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    ///1: Inverted
    _1 = 1,
    ///0: Not inverted
    _0 = 0,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INV` reader - STVx signal polarity inversion control
pub type INV_R = crate::BitReader<INV_A>;
impl INV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INV_A {
        match self.bits {
            true => INV_A::_1,
            false => INV_A::_0,
        }
    }
    ///Inverted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV_A::_1
    }
    ///Not inverted
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV_A::_0
    }
}
///Field `INV` writer - STVx signal polarity inversion control
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG, INV_A>;
impl<'a, REG> INV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Inverted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INV_A::_1)
    }
    ///Not inverted
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INV_A::_0)
    }
}
impl R {
    ///Bits 0:2 - Output signal select control for VSOUT (controlled by TCON_STVA2 register)/VEOUT (controlled by the TCON_STVB2 register) pin
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - STVx signal polarity inversion control
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Output signal select control for VSOUT (controlled by TCON_STVA2 register)/VEOUT (controlled by the TCON_STVB2 register) pin
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<TCON_STV2_SPEC> {
        SEL_W::new(self, 0)
    }
    ///Bit 4 - STVx signal polarity inversion control
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W<TCON_STV2_SPEC> {
        INV_W::new(self, 4)
    }
}
/**TCON Vertical Timing Setting Register %s2

You can [`read`](crate::Reg::read) this register and get [`tcon_stv2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_stv2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TCON_STV2_SPEC;
impl crate::RegisterSpec for TCON_STV2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tcon_stv2::R`](R) reader structure
impl crate::Readable for TCON_STV2_SPEC {}
///`write(|w| ..)` method takes [`tcon_stv2::W`](W) writer structure
impl crate::Writable for TCON_STV2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCON_STV%s2 to value 0
impl crate::Resettable for TCON_STV2_SPEC {}
