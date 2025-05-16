///Register `JCQTN` reader
pub type R = crate::R<JCQTN_SPEC>;
///Register `JCQTN` writer
pub type W = crate::W<JCQTN_SPEC>;
/**Quantization table number for the first color componentNOTE: Read-only in Decompression.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QT1_A {
    ///0: Use quantization table No.0 (JCQTBL0) as the first color component.
    _00 = 0,
    ///1: Use quantization table No.1 (JCQTBL1) as the first color component.
    _01 = 1,
    ///2: Use quantization table No.2 (JCQTBL2) as the first color component.
    _10 = 2,
    ///3: Use quantization table No.3 (JCQTBL3) as the first color component.
    _11 = 3,
}
impl From<QT1_A> for u8 {
    #[inline(always)]
    fn from(variant: QT1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QT1_A {
    type Ux = u8;
}
impl crate::IsEnum for QT1_A {}
///Field `QT1` reader - Quantization table number for the first color componentNOTE: Read-only in Decompression.
pub type QT1_R = crate::FieldReader<QT1_A>;
impl QT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> QT1_A {
        match self.bits {
            0 => QT1_A::_00,
            1 => QT1_A::_01,
            2 => QT1_A::_10,
            3 => QT1_A::_11,
            _ => unreachable!(),
        }
    }
    ///Use quantization table No.0 (JCQTBL0) as the first color component.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == QT1_A::_00
    }
    ///Use quantization table No.1 (JCQTBL1) as the first color component.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == QT1_A::_01
    }
    ///Use quantization table No.2 (JCQTBL2) as the first color component.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == QT1_A::_10
    }
    ///Use quantization table No.3 (JCQTBL3) as the first color component.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == QT1_A::_11
    }
}
///Field `QT1` writer - Quantization table number for the first color componentNOTE: Read-only in Decompression.
pub type QT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, QT1_A, crate::Safe>;
impl<'a, REG> QT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Use quantization table No.0 (JCQTBL0) as the first color component.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(QT1_A::_00)
    }
    ///Use quantization table No.1 (JCQTBL1) as the first color component.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(QT1_A::_01)
    }
    ///Use quantization table No.2 (JCQTBL2) as the first color component.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(QT1_A::_10)
    }
    ///Use quantization table No.3 (JCQTBL3) as the first color component.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(QT1_A::_11)
    }
}
/**Quantization table number for the second color component NOTE: Read-only in Decompression.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QT2_A {
    ///0: Use quantization table No.0 (JCQTBL0) as the second color component.
    _00 = 0,
    ///1: Use quantization table No.1 (JCQTBL1) as the second color component.
    _01 = 1,
    ///2: Use quantization table No.2 (JCQTBL2) as the second color component.
    _10 = 2,
    ///3: Use quantization table No.3 (JCQTBL3) as the second color component.
    _11 = 3,
}
impl From<QT2_A> for u8 {
    #[inline(always)]
    fn from(variant: QT2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QT2_A {
    type Ux = u8;
}
impl crate::IsEnum for QT2_A {}
///Field `QT2` reader - Quantization table number for the second color component NOTE: Read-only in Decompression.
pub type QT2_R = crate::FieldReader<QT2_A>;
impl QT2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> QT2_A {
        match self.bits {
            0 => QT2_A::_00,
            1 => QT2_A::_01,
            2 => QT2_A::_10,
            3 => QT2_A::_11,
            _ => unreachable!(),
        }
    }
    ///Use quantization table No.0 (JCQTBL0) as the second color component.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == QT2_A::_00
    }
    ///Use quantization table No.1 (JCQTBL1) as the second color component.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == QT2_A::_01
    }
    ///Use quantization table No.2 (JCQTBL2) as the second color component.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == QT2_A::_10
    }
    ///Use quantization table No.3 (JCQTBL3) as the second color component.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == QT2_A::_11
    }
}
///Field `QT2` writer - Quantization table number for the second color component NOTE: Read-only in Decompression.
pub type QT2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, QT2_A, crate::Safe>;
impl<'a, REG> QT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Use quantization table No.0 (JCQTBL0) as the second color component.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(QT2_A::_00)
    }
    ///Use quantization table No.1 (JCQTBL1) as the second color component.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(QT2_A::_01)
    }
    ///Use quantization table No.2 (JCQTBL2) as the second color component.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(QT2_A::_10)
    }
    ///Use quantization table No.3 (JCQTBL3) as the second color component.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(QT2_A::_11)
    }
}
/**Quantization table number for the third color component NOTE: Read-only in Decompression.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QT3_A {
    ///0: Use quantization table No.0 (JCQTBL0) as the third color component.
    _00 = 0,
    ///1: Use quantization table No.1 (JCQTBL1) as the third color component.
    _01 = 1,
    ///2: Use quantization table No.2 (JCQTBL2) as the third color component.
    _10 = 2,
    ///3: Use quantization table No.3 (JCQTBL3) as the third color component.
    _11 = 3,
}
impl From<QT3_A> for u8 {
    #[inline(always)]
    fn from(variant: QT3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QT3_A {
    type Ux = u8;
}
impl crate::IsEnum for QT3_A {}
///Field `QT3` reader - Quantization table number for the third color component NOTE: Read-only in Decompression.
pub type QT3_R = crate::FieldReader<QT3_A>;
impl QT3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> QT3_A {
        match self.bits {
            0 => QT3_A::_00,
            1 => QT3_A::_01,
            2 => QT3_A::_10,
            3 => QT3_A::_11,
            _ => unreachable!(),
        }
    }
    ///Use quantization table No.0 (JCQTBL0) as the third color component.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == QT3_A::_00
    }
    ///Use quantization table No.1 (JCQTBL1) as the third color component.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == QT3_A::_01
    }
    ///Use quantization table No.2 (JCQTBL2) as the third color component.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == QT3_A::_10
    }
    ///Use quantization table No.3 (JCQTBL3) as the third color component.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == QT3_A::_11
    }
}
///Field `QT3` writer - Quantization table number for the third color component NOTE: Read-only in Decompression.
pub type QT3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, QT3_A, crate::Safe>;
impl<'a, REG> QT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Use quantization table No.0 (JCQTBL0) as the third color component.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(QT3_A::_00)
    }
    ///Use quantization table No.1 (JCQTBL1) as the third color component.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(QT3_A::_01)
    }
    ///Use quantization table No.2 (JCQTBL2) as the third color component.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(QT3_A::_10)
    }
    ///Use quantization table No.3 (JCQTBL3) as the third color component.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(QT3_A::_11)
    }
}
impl R {
    ///Bits 0:1 - Quantization table number for the first color componentNOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn qt1(&self) -> QT1_R {
        QT1_R::new(self.bits & 3)
    }
    ///Bits 2:3 - Quantization table number for the second color component NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn qt2(&self) -> QT2_R {
        QT2_R::new((self.bits >> 2) & 3)
    }
    ///Bits 4:5 - Quantization table number for the third color component NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn qt3(&self) -> QT3_R {
        QT3_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    ///Bits 0:1 - Quantization table number for the first color componentNOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn qt1(&mut self) -> QT1_W<JCQTN_SPEC> {
        QT1_W::new(self, 0)
    }
    ///Bits 2:3 - Quantization table number for the second color component NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn qt2(&mut self) -> QT2_W<JCQTN_SPEC> {
        QT2_W::new(self, 2)
    }
    ///Bits 4:5 - Quantization table number for the third color component NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn qt3(&mut self) -> QT3_W<JCQTN_SPEC> {
        QT3_W::new(self, 4)
    }
}
/**JPEG Code Quantization Table Number Register

You can [`read`](crate::Reg::read) this register and get [`jcqtn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcqtn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCQTN_SPEC;
impl crate::RegisterSpec for JCQTN_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jcqtn::R`](R) reader structure
impl crate::Readable for JCQTN_SPEC {}
///`write(|w| ..)` method takes [`jcqtn::W`](W) writer structure
impl crate::Writable for JCQTN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JCQTN to value 0
impl crate::Resettable for JCQTN_SPEC {}
