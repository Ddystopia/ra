///Register `JINTE1` reader
pub type R = crate::R<JINTE1_SPEC>;
///Register `JINTE1` writer
pub type W = crate::W<JINTE1_SPEC>;
/**Enables or disables a data transfer processing interrupt request (JDTI) when the DOUTLF bit in JINTS1 is set to 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOUTLEN_A {
    ///0: Disables an interrupt request.
    _0 = 0,
    ///1: Enables an interrupt request.
    _1 = 1,
}
impl From<DOUTLEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOUTLEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOUTLEN` reader - Enables or disables a data transfer processing interrupt request (JDTI) when the DOUTLF bit in JINTS1 is set to 1
pub type DOUTLEN_R = crate::BitReader<DOUTLEN_A>;
impl DOUTLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOUTLEN_A {
        match self.bits {
            false => DOUTLEN_A::_0,
            true => DOUTLEN_A::_1,
        }
    }
    ///Disables an interrupt request.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOUTLEN_A::_0
    }
    ///Enables an interrupt request.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOUTLEN_A::_1
    }
}
///Field `DOUTLEN` writer - Enables or disables a data transfer processing interrupt request (JDTI) when the DOUTLF bit in JINTS1 is set to 1
pub type DOUTLEN_W<'a, REG> = crate::BitWriter<'a, REG, DOUTLEN_A>;
impl<'a, REG> DOUTLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an interrupt request.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTLEN_A::_0)
    }
    ///Enables an interrupt request.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DOUTLEN_A::_1)
    }
}
/**Enables or disables a data transfer processing interrupt request (JDTI) when the JINF bit in JINTS1 is set to 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JINEN_A {
    ///0: Disables an interrupt request.
    _0 = 0,
    ///1: Enables an interrupt request.
    _1 = 1,
}
impl From<JINEN_A> for bool {
    #[inline(always)]
    fn from(variant: JINEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `JINEN` reader - Enables or disables a data transfer processing interrupt request (JDTI) when the JINF bit in JINTS1 is set to 1.
pub type JINEN_R = crate::BitReader<JINEN_A>;
impl JINEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JINEN_A {
        match self.bits {
            false => JINEN_A::_0,
            true => JINEN_A::_1,
        }
    }
    ///Disables an interrupt request.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == JINEN_A::_0
    }
    ///Enables an interrupt request.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == JINEN_A::_1
    }
}
///Field `JINEN` writer - Enables or disables a data transfer processing interrupt request (JDTI) when the JINF bit in JINTS1 is set to 1.
pub type JINEN_W<'a, REG> = crate::BitWriter<'a, REG, JINEN_A>;
impl<'a, REG> JINEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an interrupt request.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(JINEN_A::_0)
    }
    ///Enables an interrupt request.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(JINEN_A::_1)
    }
}
/**Enables or disables a data transfer processing interrupt request (JDTI) when the DBTF bit in JINTS1 is set to 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBTEN_A {
    ///0: Disables an interrupt request.
    _0 = 0,
    ///1: Enables an interrupt request.
    _1 = 1,
}
impl From<DBTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBTEN` reader - Enables or disables a data transfer processing interrupt request (JDTI) when the DBTF bit in JINTS1 is set to 1.
pub type DBTEN_R = crate::BitReader<DBTEN_A>;
impl DBTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBTEN_A {
        match self.bits {
            false => DBTEN_A::_0,
            true => DBTEN_A::_1,
        }
    }
    ///Disables an interrupt request.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBTEN_A::_0
    }
    ///Enables an interrupt request.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBTEN_A::_1
    }
}
///Field `DBTEN` writer - Enables or disables a data transfer processing interrupt request (JDTI) when the DBTF bit in JINTS1 is set to 1.
pub type DBTEN_W<'a, REG> = crate::BitWriter<'a, REG, DBTEN_A>;
impl<'a, REG> DBTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an interrupt request.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DBTEN_A::_0)
    }
    ///Enables an interrupt request.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DBTEN_A::_1)
    }
}
/**Enables or disables a data transfer processing interrupt request (JDTI) when the DINLF bit in JINTS1 is set to 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINLEN_A {
    ///0: Disables an interrupt request.
    _0 = 0,
    ///1: Enables an interrupt request.
    _1 = 1,
}
impl From<DINLEN_A> for bool {
    #[inline(always)]
    fn from(variant: DINLEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DINLEN` reader - Enables or disables a data transfer processing interrupt request (JDTI) when the DINLF bit in JINTS1 is set to 1.
pub type DINLEN_R = crate::BitReader<DINLEN_A>;
impl DINLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DINLEN_A {
        match self.bits {
            false => DINLEN_A::_0,
            true => DINLEN_A::_1,
        }
    }
    ///Disables an interrupt request.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DINLEN_A::_0
    }
    ///Enables an interrupt request.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DINLEN_A::_1
    }
}
///Field `DINLEN` writer - Enables or disables a data transfer processing interrupt request (JDTI) when the DINLF bit in JINTS1 is set to 1.
pub type DINLEN_W<'a, REG> = crate::BitWriter<'a, REG, DINLEN_A>;
impl<'a, REG> DINLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an interrupt request.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DINLEN_A::_0)
    }
    ///Enables an interrupt request.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DINLEN_A::_1)
    }
}
/**Enables or disables a data transfer processing interrupt request (JDTI) when the CBTF bit in JINTS1 is set to 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBTEN_A {
    ///0: Disables an interrupt request.
    _0 = 0,
    ///1: Enables an interrupt request.
    _1 = 1,
}
impl From<CBTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CBTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CBTEN` reader - Enables or disables a data transfer processing interrupt request (JDTI) when the CBTF bit in JINTS1 is set to 1.
pub type CBTEN_R = crate::BitReader<CBTEN_A>;
impl CBTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CBTEN_A {
        match self.bits {
            false => CBTEN_A::_0,
            true => CBTEN_A::_1,
        }
    }
    ///Disables an interrupt request.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CBTEN_A::_0
    }
    ///Enables an interrupt request.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CBTEN_A::_1
    }
}
///Field `CBTEN` writer - Enables or disables a data transfer processing interrupt request (JDTI) when the CBTF bit in JINTS1 is set to 1.
pub type CBTEN_W<'a, REG> = crate::BitWriter<'a, REG, CBTEN_A>;
impl<'a, REG> CBTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disables an interrupt request.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CBTEN_A::_0)
    }
    ///Enables an interrupt request.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CBTEN_A::_1)
    }
}
impl R {
    ///Bit 0 - Enables or disables a data transfer processing interrupt request (JDTI) when the DOUTLF bit in JINTS1 is set to 1
    #[inline(always)]
    pub fn doutlen(&self) -> DOUTLEN_R {
        DOUTLEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enables or disables a data transfer processing interrupt request (JDTI) when the JINF bit in JINTS1 is set to 1.
    #[inline(always)]
    pub fn jinen(&self) -> JINEN_R {
        JINEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enables or disables a data transfer processing interrupt request (JDTI) when the DBTF bit in JINTS1 is set to 1.
    #[inline(always)]
    pub fn dbten(&self) -> DBTEN_R {
        DBTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Enables or disables a data transfer processing interrupt request (JDTI) when the DINLF bit in JINTS1 is set to 1.
    #[inline(always)]
    pub fn dinlen(&self) -> DINLEN_R {
        DINLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Enables or disables a data transfer processing interrupt request (JDTI) when the CBTF bit in JINTS1 is set to 1.
    #[inline(always)]
    pub fn cbten(&self) -> CBTEN_R {
        CBTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Enables or disables a data transfer processing interrupt request (JDTI) when the DOUTLF bit in JINTS1 is set to 1
    #[inline(always)]
    pub fn doutlen(&mut self) -> DOUTLEN_W<JINTE1_SPEC> {
        DOUTLEN_W::new(self, 0)
    }
    ///Bit 1 - Enables or disables a data transfer processing interrupt request (JDTI) when the JINF bit in JINTS1 is set to 1.
    #[inline(always)]
    pub fn jinen(&mut self) -> JINEN_W<JINTE1_SPEC> {
        JINEN_W::new(self, 1)
    }
    ///Bit 2 - Enables or disables a data transfer processing interrupt request (JDTI) when the DBTF bit in JINTS1 is set to 1.
    #[inline(always)]
    pub fn dbten(&mut self) -> DBTEN_W<JINTE1_SPEC> {
        DBTEN_W::new(self, 2)
    }
    ///Bit 5 - Enables or disables a data transfer processing interrupt request (JDTI) when the DINLF bit in JINTS1 is set to 1.
    #[inline(always)]
    pub fn dinlen(&mut self) -> DINLEN_W<JINTE1_SPEC> {
        DINLEN_W::new(self, 5)
    }
    ///Bit 6 - Enables or disables a data transfer processing interrupt request (JDTI) when the CBTF bit in JINTS1 is set to 1.
    #[inline(always)]
    pub fn cbten(&mut self) -> CBTEN_W<JINTE1_SPEC> {
        CBTEN_W::new(self, 6)
    }
}
/**JPEG Interrupt Enable Register 1

You can [`read`](crate::Reg::read) this register and get [`jinte1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jinte1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JINTE1_SPEC;
impl crate::RegisterSpec for JINTE1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jinte1::R`](R) reader structure
impl crate::Readable for JINTE1_SPEC {}
///`write(|w| ..)` method takes [`jinte1::W`](W) writer structure
impl crate::Writable for JINTE1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JINTE1 to value 0
impl crate::Resettable for JINTE1_SPEC {}
