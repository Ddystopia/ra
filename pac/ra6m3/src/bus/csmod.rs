///Register `CS%sMOD` reader
pub type R = crate::R<CSMOD_SPEC>;
///Register `CS%sMOD` writer
pub type W = crate::W<CSMOD_SPEC>;
/**Write Access Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRMOD_A {
    ///0: Byte strobe mode
    _0 = 0,
    ///1: Single write strobe mode
    _1 = 1,
}
impl From<WRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: WRMOD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WRMOD` reader - Write Access Mode Select
pub type WRMOD_R = crate::BitReader<WRMOD_A>;
impl WRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WRMOD_A {
        match self.bits {
            false => WRMOD_A::_0,
            true => WRMOD_A::_1,
        }
    }
    ///Byte strobe mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WRMOD_A::_0
    }
    ///Single write strobe mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WRMOD_A::_1
    }
}
///Field `WRMOD` writer - Write Access Mode Select
pub type WRMOD_W<'a, REG> = crate::BitWriter<'a, REG, WRMOD_A>;
impl<'a, REG> WRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Byte strobe mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WRMOD_A::_0)
    }
    ///Single write strobe mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WRMOD_A::_1)
    }
}
/**External Wait Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWENB_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<EWENB_A> for bool {
    #[inline(always)]
    fn from(variant: EWENB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWENB` reader - External Wait Enable
pub type EWENB_R = crate::BitReader<EWENB_A>;
impl EWENB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWENB_A {
        match self.bits {
            false => EWENB_A::_0,
            true => EWENB_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWENB_A::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWENB_A::_1
    }
}
///Field `EWENB` writer - External Wait Enable
pub type EWENB_W<'a, REG> = crate::BitWriter<'a, REG, EWENB_A>;
impl<'a, REG> EWENB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EWENB_A::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EWENB_A::_1)
    }
}
/**Page Read Access Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRENB_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<PRENB_A> for bool {
    #[inline(always)]
    fn from(variant: PRENB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PRENB` reader - Page Read Access Enable
pub type PRENB_R = crate::BitReader<PRENB_A>;
impl PRENB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRENB_A {
        match self.bits {
            false => PRENB_A::_0,
            true => PRENB_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRENB_A::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRENB_A::_1
    }
}
///Field `PRENB` writer - Page Read Access Enable
pub type PRENB_W<'a, REG> = crate::BitWriter<'a, REG, PRENB_A>;
impl<'a, REG> PRENB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRENB_A::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRENB_A::_1)
    }
}
/**Page Write Access Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWENB_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable
    _1 = 1,
}
impl From<PWENB_A> for bool {
    #[inline(always)]
    fn from(variant: PWENB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PWENB` reader - Page Write Access Enable
pub type PWENB_R = crate::BitReader<PWENB_A>;
impl PWENB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PWENB_A {
        match self.bits {
            false => PWENB_A::_0,
            true => PWENB_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWENB_A::_0
    }
    ///Enable
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWENB_A::_1
    }
}
///Field `PWENB` writer - Page Write Access Enable
pub type PWENB_W<'a, REG> = crate::BitWriter<'a, REG, PWENB_A>;
impl<'a, REG> PWENB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PWENB_A::_0)
    }
    ///Enable
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PWENB_A::_1)
    }
}
/**Page Read Access Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRMOD_A {
    ///0: Normal access compatible mode
    _0 = 0,
    ///1: External data read continuous assertion mode
    _1 = 1,
}
impl From<PRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: PRMOD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PRMOD` reader - Page Read Access Mode Select
pub type PRMOD_R = crate::BitReader<PRMOD_A>;
impl PRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRMOD_A {
        match self.bits {
            false => PRMOD_A::_0,
            true => PRMOD_A::_1,
        }
    }
    ///Normal access compatible mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRMOD_A::_0
    }
    ///External data read continuous assertion mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRMOD_A::_1
    }
}
///Field `PRMOD` writer - Page Read Access Mode Select
pub type PRMOD_W<'a, REG> = crate::BitWriter<'a, REG, PRMOD_A>;
impl<'a, REG> PRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal access compatible mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PRMOD_A::_0)
    }
    ///External data read continuous assertion mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PRMOD_A::_1)
    }
}
impl R {
    ///Bit 0 - Write Access Mode Select
    #[inline(always)]
    pub fn wrmod(&self) -> WRMOD_R {
        WRMOD_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - External Wait Enable
    #[inline(always)]
    pub fn ewenb(&self) -> EWENB_R {
        EWENB_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Page Read Access Enable
    #[inline(always)]
    pub fn prenb(&self) -> PRENB_R {
        PRENB_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Page Write Access Enable
    #[inline(always)]
    pub fn pwenb(&self) -> PWENB_R {
        PWENB_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - Page Read Access Mode Select
    #[inline(always)]
    pub fn prmod(&self) -> PRMOD_R {
        PRMOD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Write Access Mode Select
    #[inline(always)]
    pub fn wrmod(&mut self) -> WRMOD_W<CSMOD_SPEC> {
        WRMOD_W::new(self, 0)
    }
    ///Bit 3 - External Wait Enable
    #[inline(always)]
    pub fn ewenb(&mut self) -> EWENB_W<CSMOD_SPEC> {
        EWENB_W::new(self, 3)
    }
    ///Bit 8 - Page Read Access Enable
    #[inline(always)]
    pub fn prenb(&mut self) -> PRENB_W<CSMOD_SPEC> {
        PRENB_W::new(self, 8)
    }
    ///Bit 9 - Page Write Access Enable
    #[inline(always)]
    pub fn pwenb(&mut self) -> PWENB_W<CSMOD_SPEC> {
        PWENB_W::new(self, 9)
    }
    ///Bit 15 - Page Read Access Mode Select
    #[inline(always)]
    pub fn prmod(&mut self) -> PRMOD_W<CSMOD_SPEC> {
        PRMOD_W::new(self, 15)
    }
}
/**CS%s Mode Register

You can [`read`](crate::Reg::read) this register and get [`csmod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csmod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CSMOD_SPEC;
impl crate::RegisterSpec for CSMOD_SPEC {
    type Ux = u16;
}
///`read()` method returns [`csmod::R`](R) reader structure
impl crate::Readable for CSMOD_SPEC {}
///`write(|w| ..)` method takes [`csmod::W`](W) writer structure
impl crate::Writable for CSMOD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CS%sMOD to value 0
impl crate::Resettable for CSMOD_SPEC {}
