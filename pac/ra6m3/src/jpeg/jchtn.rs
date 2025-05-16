///Register `JCHTN` reader
pub type R = crate::R<JCHTN_SPEC>;
///Register `JCHTN` writer
pub type W = crate::W<JCHTN_SPEC>;
/**Huffman table number (DC) for the first color component NOTE: Read-only in Decompression.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTD1_A {
    ///0: DC Huffman table 0(HTA1=0)/Setting prohibited(HTA1=1)
    _0 = 0,
    ///1: DC Huffman table 1(HTA1=1)/Setting prohibited(HTA1=0)
    _1 = 1,
}
impl From<HTD1_A> for bool {
    #[inline(always)]
    fn from(variant: HTD1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HTD1` reader - Huffman table number (DC) for the first color component NOTE: Read-only in Decompression.
pub type HTD1_R = crate::BitReader<HTD1_A>;
impl HTD1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTD1_A {
        match self.bits {
            false => HTD1_A::_0,
            true => HTD1_A::_1,
        }
    }
    ///DC Huffman table 0(HTA1=0)/Setting prohibited(HTA1=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTD1_A::_0
    }
    ///DC Huffman table 1(HTA1=1)/Setting prohibited(HTA1=0)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTD1_A::_1
    }
}
///Field `HTD1` writer - Huffman table number (DC) for the first color component NOTE: Read-only in Decompression.
pub type HTD1_W<'a, REG> = crate::BitWriter<'a, REG, HTD1_A>;
impl<'a, REG> HTD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DC Huffman table 0(HTA1=0)/Setting prohibited(HTA1=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HTD1_A::_0)
    }
    ///DC Huffman table 1(HTA1=1)/Setting prohibited(HTA1=0)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HTD1_A::_1)
    }
}
/**Huffman table number (AC) for the first color componentNOTE: Read-only in Decompression.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTA1_A {
    ///0: AC Huffman table 0(HTD1=0)/Setting prohibited(HTD1=1)
    _0 = 0,
    ///1: AC Huffman table 1(HTD1=1)/Setting prohibited(HTD1=0)
    _1 = 1,
}
impl From<HTA1_A> for bool {
    #[inline(always)]
    fn from(variant: HTA1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HTA1` reader - Huffman table number (AC) for the first color componentNOTE: Read-only in Decompression.
pub type HTA1_R = crate::BitReader<HTA1_A>;
impl HTA1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTA1_A {
        match self.bits {
            false => HTA1_A::_0,
            true => HTA1_A::_1,
        }
    }
    ///AC Huffman table 0(HTD1=0)/Setting prohibited(HTD1=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTA1_A::_0
    }
    ///AC Huffman table 1(HTD1=1)/Setting prohibited(HTD1=0)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTA1_A::_1
    }
}
///Field `HTA1` writer - Huffman table number (AC) for the first color componentNOTE: Read-only in Decompression.
pub type HTA1_W<'a, REG> = crate::BitWriter<'a, REG, HTA1_A>;
impl<'a, REG> HTA1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AC Huffman table 0(HTD1=0)/Setting prohibited(HTD1=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HTA1_A::_0)
    }
    ///AC Huffman table 1(HTD1=1)/Setting prohibited(HTD1=0)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HTA1_A::_1)
    }
}
/**Huffman table number (DC) for the second color component NOTE: Read-only in Decompression.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTD2_A {
    ///0: DC Huffman table 0(HTA2=0)/Setting prohibited(HTA2=1)
    _0 = 0,
    ///1: DC Huffman table 1(HTA2=1)/Setting prohibited(HTA2=0)
    _1 = 1,
}
impl From<HTD2_A> for bool {
    #[inline(always)]
    fn from(variant: HTD2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HTD2` reader - Huffman table number (DC) for the second color component NOTE: Read-only in Decompression.
pub type HTD2_R = crate::BitReader<HTD2_A>;
impl HTD2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTD2_A {
        match self.bits {
            false => HTD2_A::_0,
            true => HTD2_A::_1,
        }
    }
    ///DC Huffman table 0(HTA2=0)/Setting prohibited(HTA2=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTD2_A::_0
    }
    ///DC Huffman table 1(HTA2=1)/Setting prohibited(HTA2=0)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTD2_A::_1
    }
}
///Field `HTD2` writer - Huffman table number (DC) for the second color component NOTE: Read-only in Decompression.
pub type HTD2_W<'a, REG> = crate::BitWriter<'a, REG, HTD2_A>;
impl<'a, REG> HTD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DC Huffman table 0(HTA2=0)/Setting prohibited(HTA2=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HTD2_A::_0)
    }
    ///DC Huffman table 1(HTA2=1)/Setting prohibited(HTA2=0)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HTD2_A::_1)
    }
}
/**Huffman table number (AC) for the second color componentNOTE: Read-only in Decompression.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTA2_A {
    ///0: AC Huffman table 0(HTD2=0)/Setting prohibited(HTD2=1)
    _0 = 0,
    ///1: AC Huffman table 1(HTD2=1)/Setting prohibited(HTD2=0)
    _1 = 1,
}
impl From<HTA2_A> for bool {
    #[inline(always)]
    fn from(variant: HTA2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HTA2` reader - Huffman table number (AC) for the second color componentNOTE: Read-only in Decompression.
pub type HTA2_R = crate::BitReader<HTA2_A>;
impl HTA2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTA2_A {
        match self.bits {
            false => HTA2_A::_0,
            true => HTA2_A::_1,
        }
    }
    ///AC Huffman table 0(HTD2=0)/Setting prohibited(HTD2=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTA2_A::_0
    }
    ///AC Huffman table 1(HTD2=1)/Setting prohibited(HTD2=0)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTA2_A::_1
    }
}
///Field `HTA2` writer - Huffman table number (AC) for the second color componentNOTE: Read-only in Decompression.
pub type HTA2_W<'a, REG> = crate::BitWriter<'a, REG, HTA2_A>;
impl<'a, REG> HTA2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AC Huffman table 0(HTD2=0)/Setting prohibited(HTD2=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HTA2_A::_0)
    }
    ///AC Huffman table 1(HTD2=1)/Setting prohibited(HTD2=0)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HTA2_A::_1)
    }
}
/**Huffman table number (DC) for the third color component NOTE: Read-only in Decompression.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTD3_A {
    ///0: DC Huffman table 0(HTA3=0)/Setting prohibited(HTA3=1)
    _0 = 0,
    ///1: DC Huffman table 1(HTA3=1)/Setting prohibited(HTA3=0)
    _1 = 1,
}
impl From<HTD3_A> for bool {
    #[inline(always)]
    fn from(variant: HTD3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HTD3` reader - Huffman table number (DC) for the third color component NOTE: Read-only in Decompression.
pub type HTD3_R = crate::BitReader<HTD3_A>;
impl HTD3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTD3_A {
        match self.bits {
            false => HTD3_A::_0,
            true => HTD3_A::_1,
        }
    }
    ///DC Huffman table 0(HTA3=0)/Setting prohibited(HTA3=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTD3_A::_0
    }
    ///DC Huffman table 1(HTA3=1)/Setting prohibited(HTA3=0)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTD3_A::_1
    }
}
///Field `HTD3` writer - Huffman table number (DC) for the third color component NOTE: Read-only in Decompression.
pub type HTD3_W<'a, REG> = crate::BitWriter<'a, REG, HTD3_A>;
impl<'a, REG> HTD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DC Huffman table 0(HTA3=0)/Setting prohibited(HTA3=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HTD3_A::_0)
    }
    ///DC Huffman table 1(HTA3=1)/Setting prohibited(HTA3=0)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HTD3_A::_1)
    }
}
/**Huffman table number (AC) for the third color componentNOTE: Read-only in Decompression.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTA3_A {
    ///0: AC Huffman table 0(HTD3=0)/Setting prohibited(HTD3=1)
    _0 = 0,
    ///1: AC Huffman table 1(HTD3=1)/Setting prohibited(HTD3=0)
    _1 = 1,
}
impl From<HTA3_A> for bool {
    #[inline(always)]
    fn from(variant: HTA3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HTA3` reader - Huffman table number (AC) for the third color componentNOTE: Read-only in Decompression.
pub type HTA3_R = crate::BitReader<HTA3_A>;
impl HTA3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTA3_A {
        match self.bits {
            false => HTA3_A::_0,
            true => HTA3_A::_1,
        }
    }
    ///AC Huffman table 0(HTD3=0)/Setting prohibited(HTD3=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HTA3_A::_0
    }
    ///AC Huffman table 1(HTD3=1)/Setting prohibited(HTD3=0)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HTA3_A::_1
    }
}
///Field `HTA3` writer - Huffman table number (AC) for the third color componentNOTE: Read-only in Decompression.
pub type HTA3_W<'a, REG> = crate::BitWriter<'a, REG, HTA3_A>;
impl<'a, REG> HTA3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AC Huffman table 0(HTD3=0)/Setting prohibited(HTD3=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HTA3_A::_0)
    }
    ///AC Huffman table 1(HTD3=1)/Setting prohibited(HTD3=0)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HTA3_A::_1)
    }
}
impl R {
    ///Bit 0 - Huffman table number (DC) for the first color component NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn htd1(&self) -> HTD1_R {
        HTD1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Huffman table number (AC) for the first color componentNOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn hta1(&self) -> HTA1_R {
        HTA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Huffman table number (DC) for the second color component NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn htd2(&self) -> HTD2_R {
        HTD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Huffman table number (AC) for the second color componentNOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn hta2(&self) -> HTA2_R {
        HTA2_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Huffman table number (DC) for the third color component NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn htd3(&self) -> HTD3_R {
        HTD3_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Huffman table number (AC) for the third color componentNOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn hta3(&self) -> HTA3_R {
        HTA3_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Huffman table number (DC) for the first color component NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn htd1(&mut self) -> HTD1_W<JCHTN_SPEC> {
        HTD1_W::new(self, 0)
    }
    ///Bit 1 - Huffman table number (AC) for the first color componentNOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn hta1(&mut self) -> HTA1_W<JCHTN_SPEC> {
        HTA1_W::new(self, 1)
    }
    ///Bit 2 - Huffman table number (DC) for the second color component NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn htd2(&mut self) -> HTD2_W<JCHTN_SPEC> {
        HTD2_W::new(self, 2)
    }
    ///Bit 3 - Huffman table number (AC) for the second color componentNOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn hta2(&mut self) -> HTA2_W<JCHTN_SPEC> {
        HTA2_W::new(self, 3)
    }
    ///Bit 4 - Huffman table number (DC) for the third color component NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn htd3(&mut self) -> HTD3_W<JCHTN_SPEC> {
        HTD3_W::new(self, 4)
    }
    ///Bit 5 - Huffman table number (AC) for the third color componentNOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn hta3(&mut self) -> HTA3_W<JCHTN_SPEC> {
        HTA3_W::new(self, 5)
    }
}
/**JPEG Code Huffman Table Number Register

You can [`read`](crate::Reg::read) this register and get [`jchtn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jchtn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCHTN_SPEC;
impl crate::RegisterSpec for JCHTN_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jchtn::R`](R) reader structure
impl crate::Readable for JCHTN_SPEC {}
///`write(|w| ..)` method takes [`jchtn::W`](W) writer structure
impl crate::Writable for JCHTN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JCHTN to value 0
impl crate::Resettable for JCHTN_SPEC {}
