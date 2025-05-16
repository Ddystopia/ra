///Register `JINTE0` reader
pub type R = crate::R<JINTE0_SPEC>;
///Register `JINTE0` writer
pub type W = crate::W<JINTE0_SPEC>;
/**This bit enables an interrupt to be generated when it has been determined that the image size and the subsampling setting of the compressed data can be read through analyzing the data.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT3_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<INT3_A> for bool {
    #[inline(always)]
    fn from(variant: INT3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INT3` reader - This bit enables an interrupt to be generated when it has been determined that the image size and the subsampling setting of the compressed data can be read through analyzing the data.
pub type INT3_R = crate::BitReader<INT3_A>;
impl INT3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INT3_A {
        match self.bits {
            false => INT3_A::_0,
            true => INT3_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT3_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT3_A::_1
    }
}
///Field `INT3` writer - This bit enables an interrupt to be generated when it has been determined that the image size and the subsampling setting of the compressed data can be read through analyzing the data.
pub type INT3_W<'a, REG> = crate::BitWriter<'a, REG, INT3_A>;
impl<'a, REG> INT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INT3_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INT3_A::_1)
    }
}
/**This bit enables an interrupt to be generated when the final number of MCU data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT5_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<INT5_A> for bool {
    #[inline(always)]
    fn from(variant: INT5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INT5` reader - This bit enables an interrupt to be generated when the final number of MCU data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned.
pub type INT5_R = crate::BitReader<INT5_A>;
impl INT5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INT5_A {
        match self.bits {
            false => INT5_A::_0,
            true => INT5_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT5_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT5_A::_1
    }
}
///Field `INT5` writer - This bit enables an interrupt to be generated when the final number of MCU data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned.
pub type INT5_W<'a, REG> = crate::BitWriter<'a, REG, INT5_A>;
impl<'a, REG> INT5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INT5_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INT5_A::_1)
    }
}
/**This bit enables an interrupt to be generated when the total number of data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT6_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<INT6_A> for bool {
    #[inline(always)]
    fn from(variant: INT6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INT6` reader - This bit enables an interrupt to be generated when the total number of data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned.
pub type INT6_R = crate::BitReader<INT6_A>;
impl INT6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INT6_A {
        match self.bits {
            false => INT6_A::_0,
            true => INT6_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT6_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT6_A::_1
    }
}
///Field `INT6` writer - This bit enables an interrupt to be generated when the total number of data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned.
pub type INT6_W<'a, REG> = crate::BitWriter<'a, REG, INT6_A>;
impl<'a, REG> INT6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INT6_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INT6_A::_1)
    }
}
/**This bit enables an interrupt to be generated when the number of data in the restart interval of the Huffman-coding segment is not correct in decompression.When this bit is not set to enable interrupt generation, an error code is not returned.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT7_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<INT7_A> for bool {
    #[inline(always)]
    fn from(variant: INT7_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INT7` reader - This bit enables an interrupt to be generated when the number of data in the restart interval of the Huffman-coding segment is not correct in decompression.When this bit is not set to enable interrupt generation, an error code is not returned.
pub type INT7_R = crate::BitReader<INT7_A>;
impl INT7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INT7_A {
        match self.bits {
            false => INT7_A::_0,
            true => INT7_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT7_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT7_A::_1
    }
}
///Field `INT7` writer - This bit enables an interrupt to be generated when the number of data in the restart interval of the Huffman-coding segment is not correct in decompression.When this bit is not set to enable interrupt generation, an error code is not returned.
pub type INT7_W<'a, REG> = crate::BitWriter<'a, REG, INT7_A>;
impl<'a, REG> INT7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INT7_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INT7_A::_1)
    }
}
impl R {
    ///Bit 3 - This bit enables an interrupt to be generated when it has been determined that the image size and the subsampling setting of the compressed data can be read through analyzing the data.
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - This bit enables an interrupt to be generated when the final number of MCU data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned.
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - This bit enables an interrupt to be generated when the total number of data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned.
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - This bit enables an interrupt to be generated when the number of data in the restart interval of the Huffman-coding segment is not correct in decompression.When this bit is not set to enable interrupt generation, an error code is not returned.
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - This bit enables an interrupt to be generated when it has been determined that the image size and the subsampling setting of the compressed data can be read through analyzing the data.
    #[inline(always)]
    pub fn int3(&mut self) -> INT3_W<JINTE0_SPEC> {
        INT3_W::new(self, 3)
    }
    ///Bit 5 - This bit enables an interrupt to be generated when the final number of MCU data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned.
    #[inline(always)]
    pub fn int5(&mut self) -> INT5_W<JINTE0_SPEC> {
        INT5_W::new(self, 5)
    }
    ///Bit 6 - This bit enables an interrupt to be generated when the total number of data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned.
    #[inline(always)]
    pub fn int6(&mut self) -> INT6_W<JINTE0_SPEC> {
        INT6_W::new(self, 6)
    }
    ///Bit 7 - This bit enables an interrupt to be generated when the number of data in the restart interval of the Huffman-coding segment is not correct in decompression.When this bit is not set to enable interrupt generation, an error code is not returned.
    #[inline(always)]
    pub fn int7(&mut self) -> INT7_W<JINTE0_SPEC> {
        INT7_W::new(self, 7)
    }
}
/**JPEG Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`jinte0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jinte0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JINTE0_SPEC;
impl crate::RegisterSpec for JINTE0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jinte0::R`](R) reader structure
impl crate::Readable for JINTE0_SPEC {}
///`write(|w| ..)` method takes [`jinte0::W`](W) writer structure
impl crate::Writable for JINTE0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JINTE0 to value 0
impl crate::Resettable for JINTE0_SPEC {}
