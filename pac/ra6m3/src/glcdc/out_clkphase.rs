///Register `OUT_CLKPHASE` reader
pub type R = crate::R<OUT_CLKPHASE_SPEC>;
///Register `OUT_CLKPHASE` writer
pub type W = crate::W<OUT_CLKPHASE_SPEC>;
/**LCD_TCON3 Output Phase Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCON3EDGE_A {
    ///1: In synchronization with the falling edge of LCD_CLK.
    _1 = 1,
    ///0: In synchronization with the rising edge of LCD_CLK.
    _0 = 0,
}
impl From<TCON3EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TCON3EDGE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCON3EDGE` reader - LCD_TCON3 Output Phase Control
pub type TCON3EDGE_R = crate::BitReader<TCON3EDGE_A>;
impl TCON3EDGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCON3EDGE_A {
        match self.bits {
            true => TCON3EDGE_A::_1,
            false => TCON3EDGE_A::_0,
        }
    }
    ///In synchronization with the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCON3EDGE_A::_1
    }
    ///In synchronization with the rising edge of LCD_CLK.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCON3EDGE_A::_0
    }
}
///Field `TCON3EDGE` writer - LCD_TCON3 Output Phase Control
pub type TCON3EDGE_W<'a, REG> = crate::BitWriter<'a, REG, TCON3EDGE_A>;
impl<'a, REG> TCON3EDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In synchronization with the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCON3EDGE_A::_1)
    }
    ///In synchronization with the rising edge of LCD_CLK.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCON3EDGE_A::_0)
    }
}
/**LCD_TCON2 Output Phase Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCON2EDGE_A {
    ///1: In synchronization with the falling edge of LCD_CLK.
    _1 = 1,
    ///0: In synchronization with the rising edge of LCD_CLK.
    _0 = 0,
}
impl From<TCON2EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TCON2EDGE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCON2EDGE` reader - LCD_TCON2 Output Phase Control
pub type TCON2EDGE_R = crate::BitReader<TCON2EDGE_A>;
impl TCON2EDGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCON2EDGE_A {
        match self.bits {
            true => TCON2EDGE_A::_1,
            false => TCON2EDGE_A::_0,
        }
    }
    ///In synchronization with the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCON2EDGE_A::_1
    }
    ///In synchronization with the rising edge of LCD_CLK.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCON2EDGE_A::_0
    }
}
///Field `TCON2EDGE` writer - LCD_TCON2 Output Phase Control
pub type TCON2EDGE_W<'a, REG> = crate::BitWriter<'a, REG, TCON2EDGE_A>;
impl<'a, REG> TCON2EDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In synchronization with the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCON2EDGE_A::_1)
    }
    ///In synchronization with the rising edge of LCD_CLK.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCON2EDGE_A::_0)
    }
}
/**LCD_TCON1 Output Phase Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCON1EDGE_A {
    ///1: In synchronization with the falling edge of LCD_CLK.
    _1 = 1,
    ///0: In synchronization with the rising edge of LCD_CLK.
    _0 = 0,
}
impl From<TCON1EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TCON1EDGE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCON1EDGE` reader - LCD_TCON1 Output Phase Control
pub type TCON1EDGE_R = crate::BitReader<TCON1EDGE_A>;
impl TCON1EDGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCON1EDGE_A {
        match self.bits {
            true => TCON1EDGE_A::_1,
            false => TCON1EDGE_A::_0,
        }
    }
    ///In synchronization with the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCON1EDGE_A::_1
    }
    ///In synchronization with the rising edge of LCD_CLK.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCON1EDGE_A::_0
    }
}
///Field `TCON1EDGE` writer - LCD_TCON1 Output Phase Control
pub type TCON1EDGE_W<'a, REG> = crate::BitWriter<'a, REG, TCON1EDGE_A>;
impl<'a, REG> TCON1EDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In synchronization with the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCON1EDGE_A::_1)
    }
    ///In synchronization with the rising edge of LCD_CLK.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCON1EDGE_A::_0)
    }
}
/**LCD_TCON0 Output Phase Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCON0EDGE_A {
    ///1: In synchronization with the falling edge of LCD_CLK.
    _1 = 1,
    ///0: In synchronization with the rising edge of LCD_CLK.
    _0 = 0,
}
impl From<TCON0EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TCON0EDGE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCON0EDGE` reader - LCD_TCON0 Output Phase Control
pub type TCON0EDGE_R = crate::BitReader<TCON0EDGE_A>;
impl TCON0EDGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCON0EDGE_A {
        match self.bits {
            true => TCON0EDGE_A::_1,
            false => TCON0EDGE_A::_0,
        }
    }
    ///In synchronization with the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCON0EDGE_A::_1
    }
    ///In synchronization with the rising edge of LCD_CLK.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCON0EDGE_A::_0
    }
}
///Field `TCON0EDGE` writer - LCD_TCON0 Output Phase Control
pub type TCON0EDGE_W<'a, REG> = crate::BitWriter<'a, REG, TCON0EDGE_A>;
impl<'a, REG> TCON0EDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In synchronization with the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCON0EDGE_A::_1)
    }
    ///In synchronization with the rising edge of LCD_CLK.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCON0EDGE_A::_0)
    }
}
/**LCD_DATA Output Phase Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDEDGE_A {
    ///0: In synchronization with the rising edge of LCD_CLK.
    _0 = 0,
    ///1: In synchronization with the falling edge of LCD_CLK
    _1 = 1,
}
impl From<LCDEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: LCDEDGE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LCDEDGE` reader - LCD_DATA Output Phase Control
pub type LCDEDGE_R = crate::BitReader<LCDEDGE_A>;
impl LCDEDGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCDEDGE_A {
        match self.bits {
            false => LCDEDGE_A::_0,
            true => LCDEDGE_A::_1,
        }
    }
    ///In synchronization with the rising edge of LCD_CLK.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCDEDGE_A::_0
    }
    ///In synchronization with the falling edge of LCD_CLK
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCDEDGE_A::_1
    }
}
///Field `LCDEDGE` writer - LCD_DATA Output Phase Control
pub type LCDEDGE_W<'a, REG> = crate::BitWriter<'a, REG, LCDEDGE_A>;
impl<'a, REG> LCDEDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///In synchronization with the rising edge of LCD_CLK.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LCDEDGE_A::_0)
    }
    ///In synchronization with the falling edge of LCD_CLK
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LCDEDGE_A::_1)
    }
}
/**Correction control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRONTGAM_A {
    ///1: Gamma correction is followed by brightness/contrast correction.
    _1 = 1,
    ///0: Brightness/contrast correction is followed by gamma correction.
    _0 = 0,
}
impl From<FRONTGAM_A> for bool {
    #[inline(always)]
    fn from(variant: FRONTGAM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FRONTGAM` reader - Correction control
pub type FRONTGAM_R = crate::BitReader<FRONTGAM_A>;
impl FRONTGAM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRONTGAM_A {
        match self.bits {
            true => FRONTGAM_A::_1,
            false => FRONTGAM_A::_0,
        }
    }
    ///Gamma correction is followed by brightness/contrast correction.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRONTGAM_A::_1
    }
    ///Brightness/contrast correction is followed by gamma correction.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRONTGAM_A::_0
    }
}
///Field `FRONTGAM` writer - Correction control
pub type FRONTGAM_W<'a, REG> = crate::BitWriter<'a, REG, FRONTGAM_A>;
impl<'a, REG> FRONTGAM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Gamma correction is followed by brightness/contrast correction.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FRONTGAM_A::_1)
    }
    ///Brightness/contrast correction is followed by gamma correction.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FRONTGAM_A::_0)
    }
}
impl R {
    ///Bit 3 - LCD_TCON3 Output Phase Control
    #[inline(always)]
    pub fn tcon3edge(&self) -> TCON3EDGE_R {
        TCON3EDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LCD_TCON2 Output Phase Control
    #[inline(always)]
    pub fn tcon2edge(&self) -> TCON2EDGE_R {
        TCON2EDGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LCD_TCON1 Output Phase Control
    #[inline(always)]
    pub fn tcon1edge(&self) -> TCON1EDGE_R {
        TCON1EDGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LCD_TCON0 Output Phase Control
    #[inline(always)]
    pub fn tcon0edge(&self) -> TCON0EDGE_R {
        TCON0EDGE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - LCD_DATA Output Phase Control
    #[inline(always)]
    pub fn lcdedge(&self) -> LCDEDGE_R {
        LCDEDGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Correction control
    #[inline(always)]
    pub fn frontgam(&self) -> FRONTGAM_R {
        FRONTGAM_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - LCD_TCON3 Output Phase Control
    #[inline(always)]
    pub fn tcon3edge(&mut self) -> TCON3EDGE_W<OUT_CLKPHASE_SPEC> {
        TCON3EDGE_W::new(self, 3)
    }
    ///Bit 4 - LCD_TCON2 Output Phase Control
    #[inline(always)]
    pub fn tcon2edge(&mut self) -> TCON2EDGE_W<OUT_CLKPHASE_SPEC> {
        TCON2EDGE_W::new(self, 4)
    }
    ///Bit 5 - LCD_TCON1 Output Phase Control
    #[inline(always)]
    pub fn tcon1edge(&mut self) -> TCON1EDGE_W<OUT_CLKPHASE_SPEC> {
        TCON1EDGE_W::new(self, 5)
    }
    ///Bit 6 - LCD_TCON0 Output Phase Control
    #[inline(always)]
    pub fn tcon0edge(&mut self) -> TCON0EDGE_W<OUT_CLKPHASE_SPEC> {
        TCON0EDGE_W::new(self, 6)
    }
    ///Bit 8 - LCD_DATA Output Phase Control
    #[inline(always)]
    pub fn lcdedge(&mut self) -> LCDEDGE_W<OUT_CLKPHASE_SPEC> {
        LCDEDGE_W::new(self, 8)
    }
    ///Bit 12 - Correction control
    #[inline(always)]
    pub fn frontgam(&mut self) -> FRONTGAM_W<OUT_CLKPHASE_SPEC> {
        FRONTGAM_W::new(self, 12)
    }
}
/**Output Control Block Output Phase Control Register

You can [`read`](crate::Reg::read) this register and get [`out_clkphase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_clkphase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_CLKPHASE_SPEC;
impl crate::RegisterSpec for OUT_CLKPHASE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_clkphase::R`](R) reader structure
impl crate::Readable for OUT_CLKPHASE_SPEC {}
///`write(|w| ..)` method takes [`out_clkphase::W`](W) writer structure
impl crate::Writable for OUT_CLKPHASE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OUT_CLKPHASE to value 0
impl crate::Resettable for OUT_CLKPHASE_SPEC {}
