///Register `SD_CLK_CTRL` reader
pub type R = crate::R<SD_CLK_CTRL_SPEC>;
///Register `SD_CLK_CTRL` writer
pub type W = crate::W<SD_CLK_CTRL_SPEC>;
/**SDHI Clock Frequency Select

Value on reset: 32*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    ///0: PCLKA divided by 2
    _0X00 = 0,
    ///1: PCLKA divided by 4
    _0X01 = 1,
    ///2: PCLKA divided by 8
    _0X02 = 2,
    ///4: PCLKA divided by 16
    _0X04 = 4,
    ///8: PCLKA divided by 32
    _0X08 = 8,
    ///16: PCLKA divided by 64
    _0X10 = 16,
    ///32: PCLKA divided by 128
    _0X20 = 32,
    ///64: PCLKA divided by 256
    _0X40 = 64,
    ///128: PCLKA divided by 512
    _0X80 = 128,
    ///3: Settings prohibited.
    OTHERS = 3,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for CLKSEL_A {}
///Field `CLKSEL` reader - SDHI Clock Frequency Select
pub type CLKSEL_R = crate::FieldReader<CLKSEL_A>;
impl CLKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::_0X00,
            1 => CLKSEL_A::_0X01,
            2 => CLKSEL_A::_0X02,
            4 => CLKSEL_A::_0X04,
            8 => CLKSEL_A::_0X08,
            16 => CLKSEL_A::_0X10,
            32 => CLKSEL_A::_0X20,
            64 => CLKSEL_A::_0X40,
            128 => CLKSEL_A::_0X80,
            _ => CLKSEL_A::OTHERS,
        }
    }
    ///PCLKA divided by 2
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == CLKSEL_A::_0X00
    }
    ///PCLKA divided by 4
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == CLKSEL_A::_0X01
    }
    ///PCLKA divided by 8
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == CLKSEL_A::_0X02
    }
    ///PCLKA divided by 16
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == CLKSEL_A::_0X04
    }
    ///PCLKA divided by 32
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == CLKSEL_A::_0X08
    }
    ///PCLKA divided by 64
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == CLKSEL_A::_0X10
    }
    ///PCLKA divided by 128
    #[inline(always)]
    pub fn is_0x20(&self) -> bool {
        *self == CLKSEL_A::_0X20
    }
    ///PCLKA divided by 256
    #[inline(always)]
    pub fn is_0x40(&self) -> bool {
        *self == CLKSEL_A::_0X40
    }
    ///PCLKA divided by 512
    #[inline(always)]
    pub fn is_0x80(&self) -> bool {
        *self == CLKSEL_A::_0X80
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CLKSEL_A::OTHERS)
    }
}
///Field `CLKSEL` writer - SDHI Clock Frequency Select
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CLKSEL_A, crate::Safe>;
impl<'a, REG> CLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLKA divided by 2
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_0X00)
    }
    ///PCLKA divided by 4
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_0X01)
    }
    ///PCLKA divided by 8
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_0X02)
    }
    ///PCLKA divided by 16
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_0X04)
    }
    ///PCLKA divided by 32
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_0X08)
    }
    ///PCLKA divided by 64
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_0X10)
    }
    ///PCLKA divided by 128
    #[inline(always)]
    pub fn _0x20(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_0X20)
    }
    ///PCLKA divided by 256
    #[inline(always)]
    pub fn _0x40(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_0X40)
    }
    ///PCLKA divided by 512
    #[inline(always)]
    pub fn _0x80(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_0X80)
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::OTHERS)
    }
}
/**SD/MMC Clock Output Control Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN_A {
    ///0: SD/MMC Clock output is disabled. The SDCLK signal is fixed 0.
    _0 = 0,
    ///1: SD/MMC Clock output is enabled.
    _1 = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CLKEN` reader - SD/MMC Clock Output Control Enable
pub type CLKEN_R = crate::BitReader<CLKEN_A>;
impl CLKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::_0,
            true => CLKEN_A::_1,
        }
    }
    ///SD/MMC Clock output is disabled. The SDCLK signal is fixed 0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKEN_A::_0
    }
    ///SD/MMC Clock output is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKEN_A::_1
    }
}
///Field `CLKEN` writer - SD/MMC Clock Output Control Enable
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKEN_A>;
impl<'a, REG> CLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SD/MMC Clock output is disabled. The SDCLK signal is fixed 0.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN_A::_0)
    }
    ///SD/MMC Clock output is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN_A::_1)
    }
}
/**SD/MMC Clock Output Automatic Control Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKCTRLEN_A {
    ///0: Automatic control for SD/MMC Clock output is disabled.
    _0 = 0,
    ///1: Automatic control for SD/MMC Clock output is enabled.
    _1 = 1,
}
impl From<CLKCTRLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKCTRLEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CLKCTRLEN` reader - SD/MMC Clock Output Automatic Control Enable
pub type CLKCTRLEN_R = crate::BitReader<CLKCTRLEN_A>;
impl CLKCTRLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLKCTRLEN_A {
        match self.bits {
            false => CLKCTRLEN_A::_0,
            true => CLKCTRLEN_A::_1,
        }
    }
    ///Automatic control for SD/MMC Clock output is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKCTRLEN_A::_0
    }
    ///Automatic control for SD/MMC Clock output is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKCTRLEN_A::_1
    }
}
///Field `CLKCTRLEN` writer - SD/MMC Clock Output Automatic Control Enable
pub type CLKCTRLEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKCTRLEN_A>;
impl<'a, REG> CLKCTRLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic control for SD/MMC Clock output is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKCTRLEN_A::_0)
    }
    ///Automatic control for SD/MMC Clock output is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKCTRLEN_A::_1)
    }
}
impl R {
    ///Bits 0:7 - SDHI Clock Frequency Select
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - SD/MMC Clock Output Control Enable
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SD/MMC Clock Output Automatic Control Enable
    #[inline(always)]
    pub fn clkctrlen(&self) -> CLKCTRLEN_R {
        CLKCTRLEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - SDHI Clock Frequency Select
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W<SD_CLK_CTRL_SPEC> {
        CLKSEL_W::new(self, 0)
    }
    ///Bit 8 - SD/MMC Clock Output Control Enable
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<SD_CLK_CTRL_SPEC> {
        CLKEN_W::new(self, 8)
    }
    ///Bit 9 - SD/MMC Clock Output Automatic Control Enable
    #[inline(always)]
    pub fn clkctrlen(&mut self) -> CLKCTRLEN_W<SD_CLK_CTRL_SPEC> {
        CLKCTRLEN_W::new(self, 9)
    }
}
/**SD Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`sd_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_CLK_CTRL_SPEC;
impl crate::RegisterSpec for SD_CLK_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_clk_ctrl::R`](R) reader structure
impl crate::Readable for SD_CLK_CTRL_SPEC {}
///`write(|w| ..)` method takes [`sd_clk_ctrl::W`](W) writer structure
impl crate::Writable for SD_CLK_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_CLK_CTRL to value 0x20
impl crate::Resettable for SD_CLK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
