///Register `BG_SYNC` reader
pub type R = crate::R<BG_SYNC_SPEC>;
///Register `BG_SYNC` writer
pub type W = crate::W<BG_SYNC_SPEC>;
/**Background plane horizontal synchronization signal assertion position on the basis of pixel clock (PXCLK).

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HP_A {
    ///0: Setting prohibited
    _0X0 = 0,
    ///1: (HP)th line (pixels)
    OTHERS = 1,
}
impl From<HP_A> for u8 {
    #[inline(always)]
    fn from(variant: HP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HP_A {
    type Ux = u8;
}
impl crate::IsEnum for HP_A {}
///Field `HP` reader - Background plane horizontal synchronization signal assertion position on the basis of pixel clock (PXCLK).
pub type HP_R = crate::FieldReader<HP_A>;
impl HP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HP_A {
        match self.bits {
            0 => HP_A::_0X0,
            _ => HP_A::OTHERS,
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == HP_A::_0X0
    }
    ///(HP)th line (pixels)
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), HP_A::OTHERS)
    }
}
///Field `HP` writer - Background plane horizontal synchronization signal assertion position on the basis of pixel clock (PXCLK).
pub type HP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HP_A, crate::Safe>;
impl<'a, REG> HP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HP_A::_0X0)
    }
    ///(HP)th line (pixels)
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(HP_A::OTHERS)
    }
}
/**Background plane vertical synchronization signal assertion position on the basis of line.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VP_A {
    ///0: Setting prohibited
    _0X0 = 0,
    ///1: (VP)th line
    OTHERS = 1,
}
impl From<VP_A> for u8 {
    #[inline(always)]
    fn from(variant: VP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VP_A {
    type Ux = u8;
}
impl crate::IsEnum for VP_A {}
///Field `VP` reader - Background plane vertical synchronization signal assertion position on the basis of line.
pub type VP_R = crate::FieldReader<VP_A>;
impl VP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VP_A {
        match self.bits {
            0 => VP_A::_0X0,
            _ => VP_A::OTHERS,
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == VP_A::_0X0
    }
    ///(VP)th line
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), VP_A::OTHERS)
    }
}
///Field `VP` writer - Background plane vertical synchronization signal assertion position on the basis of line.
pub type VP_W<'a, REG> = crate::FieldWriter<'a, REG, 4, VP_A, crate::Safe>;
impl<'a, REG> VP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VP_A::_0X0)
    }
    ///(VP)th line
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(VP_A::OTHERS)
    }
}
impl R {
    ///Bits 0:3 - Background plane horizontal synchronization signal assertion position on the basis of pixel clock (PXCLK).
    #[inline(always)]
    pub fn hp(&self) -> HP_R {
        HP_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 16:19 - Background plane vertical synchronization signal assertion position on the basis of line.
    #[inline(always)]
    pub fn vp(&self) -> VP_R {
        VP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Background plane horizontal synchronization signal assertion position on the basis of pixel clock (PXCLK).
    #[inline(always)]
    pub fn hp(&mut self) -> HP_W<BG_SYNC_SPEC> {
        HP_W::new(self, 0)
    }
    ///Bits 16:19 - Background plane vertical synchronization signal assertion position on the basis of line.
    #[inline(always)]
    pub fn vp(&mut self) -> VP_W<BG_SYNC_SPEC> {
        VP_W::new(self, 16)
    }
}
/**Background Plane Setting Synchronization Position Register

You can [`read`](crate::Reg::read) this register and get [`bg_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BG_SYNC_SPEC;
impl crate::RegisterSpec for BG_SYNC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bg_sync::R`](R) reader structure
impl crate::Readable for BG_SYNC_SPEC {}
///`write(|w| ..)` method takes [`bg_sync::W`](W) writer structure
impl crate::Writable for BG_SYNC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BG_SYNC to value 0x0001_0001
impl crate::Resettable for BG_SYNC_SPEC {
    const RESET_VALUE: u32 = 0x0001_0001;
}
