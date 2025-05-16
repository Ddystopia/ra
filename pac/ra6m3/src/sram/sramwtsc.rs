///Register `SRAMWTSC` reader
pub type R = crate::R<SRAMWTSC_SPEC>;
///Register `SRAMWTSC` writer
pub type W = crate::W<SRAMWTSC_SPEC>;
/**ECCRAM Read wait enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCRAMRDWTEN_A {
    ///0: Not add wait state in read access cycle to SRAM0 (ECC area)
    _0 = 0,
    ///1: Add wait state in read access cycle to SRAM0 (ECC area)
    _1 = 1,
}
impl From<ECCRAMRDWTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ECCRAMRDWTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCRAMRDWTEN` reader - ECCRAM Read wait enable
pub type ECCRAMRDWTEN_R = crate::BitReader<ECCRAMRDWTEN_A>;
impl ECCRAMRDWTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCRAMRDWTEN_A {
        match self.bits {
            false => ECCRAMRDWTEN_A::_0,
            true => ECCRAMRDWTEN_A::_1,
        }
    }
    ///Not add wait state in read access cycle to SRAM0 (ECC area)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECCRAMRDWTEN_A::_0
    }
    ///Add wait state in read access cycle to SRAM0 (ECC area)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECCRAMRDWTEN_A::_1
    }
}
///Field `ECCRAMRDWTEN` writer - ECCRAM Read wait enable
pub type ECCRAMRDWTEN_W<'a, REG> = crate::BitWriter<'a, REG, ECCRAMRDWTEN_A>;
impl<'a, REG> ECCRAMRDWTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not add wait state in read access cycle to SRAM0 (ECC area)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ECCRAMRDWTEN_A::_0)
    }
    ///Add wait state in read access cycle to SRAM0 (ECC area)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ECCRAMRDWTEN_A::_1)
    }
}
/**SRAM0 Wait Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM0WTEN_A {
    ///0: Not add wait state in read access cycle to SRAM0
    _0 = 0,
    ///1: Add wait state in read access cycle to SRAM0
    _1 = 1,
}
impl From<SRAM0WTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM0WTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM0WTEN` reader - SRAM0 Wait Enable
pub type SRAM0WTEN_R = crate::BitReader<SRAM0WTEN_A>;
impl SRAM0WTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM0WTEN_A {
        match self.bits {
            false => SRAM0WTEN_A::_0,
            true => SRAM0WTEN_A::_1,
        }
    }
    ///Not add wait state in read access cycle to SRAM0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRAM0WTEN_A::_0
    }
    ///Add wait state in read access cycle to SRAM0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRAM0WTEN_A::_1
    }
}
///Field `SRAM0WTEN` writer - SRAM0 Wait Enable
pub type SRAM0WTEN_W<'a, REG> = crate::BitWriter<'a, REG, SRAM0WTEN_A>;
impl<'a, REG> SRAM0WTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not add wait state in read access cycle to SRAM0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM0WTEN_A::_0)
    }
    ///Add wait state in read access cycle to SRAM0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM0WTEN_A::_1)
    }
}
/**SRAM1 Wait Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1WTEN_A {
    ///0: Not add wait state in read access cycle to SRAM1
    _0 = 0,
    ///1: Add wait state in read access cycle to SRAM1
    _1 = 1,
}
impl From<SRAM1WTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1WTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM1WTEN` reader - SRAM1 Wait Enable
pub type SRAM1WTEN_R = crate::BitReader<SRAM1WTEN_A>;
impl SRAM1WTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1WTEN_A {
        match self.bits {
            false => SRAM1WTEN_A::_0,
            true => SRAM1WTEN_A::_1,
        }
    }
    ///Not add wait state in read access cycle to SRAM1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRAM1WTEN_A::_0
    }
    ///Add wait state in read access cycle to SRAM1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRAM1WTEN_A::_1
    }
}
///Field `SRAM1WTEN` writer - SRAM1 Wait Enable
pub type SRAM1WTEN_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1WTEN_A>;
impl<'a, REG> SRAM1WTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not add wait state in read access cycle to SRAM1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1WTEN_A::_0)
    }
    ///Add wait state in read access cycle to SRAM1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1WTEN_A::_1)
    }
}
impl R {
    ///Bit 1 - ECCRAM Read wait enable
    #[inline(always)]
    pub fn eccramrdwten(&self) -> ECCRAMRDWTEN_R {
        ECCRAMRDWTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SRAM0 Wait Enable
    #[inline(always)]
    pub fn sram0wten(&self) -> SRAM0WTEN_R {
        SRAM0WTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SRAM1 Wait Enable
    #[inline(always)]
    pub fn sram1wten(&self) -> SRAM1WTEN_R {
        SRAM1WTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - ECCRAM Read wait enable
    #[inline(always)]
    pub fn eccramrdwten(&mut self) -> ECCRAMRDWTEN_W<SRAMWTSC_SPEC> {
        ECCRAMRDWTEN_W::new(self, 1)
    }
    ///Bit 2 - SRAM0 Wait Enable
    #[inline(always)]
    pub fn sram0wten(&mut self) -> SRAM0WTEN_W<SRAMWTSC_SPEC> {
        SRAM0WTEN_W::new(self, 2)
    }
    ///Bit 3 - SRAM1 Wait Enable
    #[inline(always)]
    pub fn sram1wten(&mut self) -> SRAM1WTEN_W<SRAMWTSC_SPEC> {
        SRAM1WTEN_W::new(self, 3)
    }
}
/**RAM Wait State Control Register

You can [`read`](crate::Reg::read) this register and get [`sramwtsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramwtsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRAMWTSC_SPEC;
impl crate::RegisterSpec for SRAMWTSC_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sramwtsc::R`](R) reader structure
impl crate::Readable for SRAMWTSC_SPEC {}
///`write(|w| ..)` method takes [`sramwtsc::W`](W) writer structure
impl crate::Writable for SRAMWTSC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRAMWTSC to value 0x0e
impl crate::Resettable for SRAMWTSC_SPEC {
    const RESET_VALUE: u8 = 0x0e;
}
