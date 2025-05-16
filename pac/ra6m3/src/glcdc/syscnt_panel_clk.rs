///Register `SYSCNT_PANEL_CLK` reader
pub type R = crate::R<SYSCNT_PANEL_CLK_SPEC>;
///Register `SYSCNT_PANEL_CLK` writer
pub type W = crate::W<SYSCNT_PANEL_CLK_SPEC>;
///Field `DCDR` reader - Clock division ratio setting controlRefer toTable 2.7.1 for details about setting value.Note: Settings that are not listed in table 2.7.1 are prohibited.
pub type DCDR_R = crate::FieldReader;
///Field `DCDR` writer - Clock division ratio setting controlRefer toTable 2.7.1 for details about setting value.Note: Settings that are not listed in table 2.7.1 are prohibited.
pub type DCDR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
/**Panel clock output enable controlNote: Before changing the PIXSEL,CLKSEL or DCDR bit, this bit must be set to 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN_A {
    ///0: Disable panel clock output
    _0 = 0,
    ///1: Enable panel clock output
    _1 = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CLKEN` reader - Panel clock output enable controlNote: Before changing the PIXSEL,CLKSEL or DCDR bit, this bit must be set to 0.
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
    ///Disable panel clock output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKEN_A::_0
    }
    ///Enable panel clock output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKEN_A::_1
    }
}
///Field `CLKEN` writer - Panel clock output enable controlNote: Before changing the PIXSEL,CLKSEL or DCDR bit, this bit must be set to 0.
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKEN_A>;
impl<'a, REG> CLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable panel clock output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN_A::_0)
    }
    ///Enable panel clock output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN_A::_1)
    }
}
/**Panel clock supply source select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKSEL_A {
    ///0: External clock select
    _0 = 0,
    ///1: PLL output select
    _1 = 1,
}
impl From<CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CLKSEL` reader - Panel clock supply source select
pub type CLKSEL_R = crate::BitReader<CLKSEL_A>;
impl CLKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLKSEL_A {
        match self.bits {
            false => CLKSEL_A::_0,
            true => CLKSEL_A::_1,
        }
    }
    ///External clock select
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKSEL_A::_0
    }
    ///PLL output select
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKSEL_A::_1
    }
}
///Field `CLKSEL` writer - Panel clock supply source select
pub type CLKSEL_W<'a, REG> = crate::BitWriter<'a, REG, CLKSEL_A>;
impl<'a, REG> CLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External clock select
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_0)
    }
    ///PLL output select
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::_1)
    }
}
/**Pixel clock select control.Must be set to the same value as OUT_SET.FRQSEL\[1\].

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIXSEL_A {
    ///0: No frequency division, parallel RGB
    _0 = 0,
    ///1: Quarter frequency,serial RGB
    _1 = 1,
}
impl From<PIXSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PIXSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIXSEL` reader - Pixel clock select control.Must be set to the same value as OUT_SET.FRQSEL\[1\].
pub type PIXSEL_R = crate::BitReader<PIXSEL_A>;
impl PIXSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIXSEL_A {
        match self.bits {
            false => PIXSEL_A::_0,
            true => PIXSEL_A::_1,
        }
    }
    ///No frequency division, parallel RGB
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIXSEL_A::_0
    }
    ///Quarter frequency,serial RGB
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIXSEL_A::_1
    }
}
///Field `PIXSEL` writer - Pixel clock select control.Must be set to the same value as OUT_SET.FRQSEL\[1\].
pub type PIXSEL_W<'a, REG> = crate::BitWriter<'a, REG, PIXSEL_A>;
impl<'a, REG> PIXSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No frequency division, parallel RGB
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIXSEL_A::_0)
    }
    ///Quarter frequency,serial RGB
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIXSEL_A::_1)
    }
}
///Field `VER` reader - Version informationVersion information of the GLCD
pub type VER_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:5 - Clock division ratio setting controlRefer toTable 2.7.1 for details about setting value.Note: Settings that are not listed in table 2.7.1 are prohibited.
    #[inline(always)]
    pub fn dcdr(&self) -> DCDR_R {
        DCDR_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6 - Panel clock output enable controlNote: Before changing the PIXSEL,CLKSEL or DCDR bit, this bit must be set to 0.
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Panel clock supply source select
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Pixel clock select control.Must be set to the same value as OUT_SET.FRQSEL\[1\].
    #[inline(always)]
    pub fn pixsel(&self) -> PIXSEL_R {
        PIXSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:31 - Version informationVersion information of the GLCD
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:5 - Clock division ratio setting controlRefer toTable 2.7.1 for details about setting value.Note: Settings that are not listed in table 2.7.1 are prohibited.
    #[inline(always)]
    pub fn dcdr(&mut self) -> DCDR_W<SYSCNT_PANEL_CLK_SPEC> {
        DCDR_W::new(self, 0)
    }
    ///Bit 6 - Panel clock output enable controlNote: Before changing the PIXSEL,CLKSEL or DCDR bit, this bit must be set to 0.
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<SYSCNT_PANEL_CLK_SPEC> {
        CLKEN_W::new(self, 6)
    }
    ///Bit 8 - Panel clock supply source select
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W<SYSCNT_PANEL_CLK_SPEC> {
        CLKSEL_W::new(self, 8)
    }
    ///Bit 12 - Pixel clock select control.Must be set to the same value as OUT_SET.FRQSEL\[1\].
    #[inline(always)]
    pub fn pixsel(&mut self) -> PIXSEL_W<SYSCNT_PANEL_CLK_SPEC> {
        PIXSEL_W::new(self, 12)
    }
}
/**System Control Block Version and Panel Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`syscnt_panel_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscnt_panel_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYSCNT_PANEL_CLK_SPEC;
impl crate::RegisterSpec for SYSCNT_PANEL_CLK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syscnt_panel_clk::R`](R) reader structure
impl crate::Readable for SYSCNT_PANEL_CLK_SPEC {}
///`write(|w| ..)` method takes [`syscnt_panel_clk::W`](W) writer structure
impl crate::Writable for SYSCNT_PANEL_CLK_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSCNT_PANEL_CLK to value 0x0110_0000
impl crate::Resettable for SYSCNT_PANEL_CLK_SPEC {
    const RESET_VALUE: u32 = 0x0110_0000;
}
