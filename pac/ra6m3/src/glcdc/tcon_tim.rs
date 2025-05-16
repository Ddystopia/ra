///Register `TCON_TIM` reader
pub type R = crate::R<TCON_TIM_SPEC>;
///Register `TCON_TIM` writer
pub type W = crate::W<TCON_TIM_SPEC>;
/**Horizontal synchronization signal generation reference timingSets the offset from the assertion of the internal horizontal synchronization signal in terms of pixels.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum OFFSET_A {
    ///0: OFFSET+1 pixels. The valid range is 0x000 to 0x3FF.
    OFFSET = 0,
}
impl From<OFFSET_A> for u16 {
    #[inline(always)]
    fn from(variant: OFFSET_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OFFSET_A {
    type Ux = u16;
}
impl crate::IsEnum for OFFSET_A {}
///Field `OFFSET` reader - Horizontal synchronization signal generation reference timingSets the offset from the assertion of the internal horizontal synchronization signal in terms of pixels.
pub type OFFSET_R = crate::FieldReader<OFFSET_A>;
impl OFFSET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OFFSET_A {
        match self.bits {
            _ => OFFSET_A::OFFSET,
        }
    }
    ///OFFSET+1 pixels. The valid range is 0x000 to 0x3FF.
    #[inline(always)]
    pub fn is_offset(&self) -> bool {
        matches!(self.variant(), OFFSET_A::OFFSET)
    }
}
///Field `OFFSET` writer - Horizontal synchronization signal generation reference timingSets the offset from the assertion of the internal horizontal synchronization signal in terms of pixels.
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 11, OFFSET_A, crate::Safe>;
impl<'a, REG> OFFSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///OFFSET+1 pixels. The valid range is 0x000 to 0x3FF.
    #[inline(always)]
    pub fn offset_(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET_A::OFFSET)
    }
}
/**Vertical synchronization signal generation change timing Sets the delay from the assertion of the internal horizontal synchronization signal in terms of pixels.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum HALF_A {
    ///0: HALF pixels. The valid range is 0x000 to 0x3FF.
    HALF = 0,
}
impl From<HALF_A> for u16 {
    #[inline(always)]
    fn from(variant: HALF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HALF_A {
    type Ux = u16;
}
impl crate::IsEnum for HALF_A {}
///Field `HALF` reader - Vertical synchronization signal generation change timing Sets the delay from the assertion of the internal horizontal synchronization signal in terms of pixels.
pub type HALF_R = crate::FieldReader<HALF_A>;
impl HALF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HALF_A {
        match self.bits {
            _ => HALF_A::HALF,
        }
    }
    ///HALF pixels. The valid range is 0x000 to 0x3FF.
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        matches!(self.variant(), HALF_A::HALF)
    }
}
///Field `HALF` writer - Vertical synchronization signal generation change timing Sets the delay from the assertion of the internal horizontal synchronization signal in terms of pixels.
pub type HALF_W<'a, REG> = crate::FieldWriter<'a, REG, 11, HALF_A, crate::Safe>;
impl<'a, REG> HALF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///HALF pixels. The valid range is 0x000 to 0x3FF.
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(HALF_A::HALF)
    }
}
impl R {
    ///Bits 0:10 - Horizontal synchronization signal generation reference timingSets the offset from the assertion of the internal horizontal synchronization signal in terms of pixels.
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Vertical synchronization signal generation change timing Sets the delay from the assertion of the internal horizontal synchronization signal in terms of pixels.
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Horizontal synchronization signal generation reference timingSets the offset from the assertion of the internal horizontal synchronization signal in terms of pixels.
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<TCON_TIM_SPEC> {
        OFFSET_W::new(self, 0)
    }
    ///Bits 16:26 - Vertical synchronization signal generation change timing Sets the delay from the assertion of the internal horizontal synchronization signal in terms of pixels.
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W<TCON_TIM_SPEC> {
        HALF_W::new(self, 16)
    }
}
/**TCON Reference Timing Setting Register

You can [`read`](crate::Reg::read) this register and get [`tcon_tim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_tim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TCON_TIM_SPEC;
impl crate::RegisterSpec for TCON_TIM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tcon_tim::R`](R) reader structure
impl crate::Readable for TCON_TIM_SPEC {}
///`write(|w| ..)` method takes [`tcon_tim::W`](W) writer structure
impl crate::Writable for TCON_TIM_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCON_TIM to value 0
impl crate::Resettable for TCON_TIM_SPEC {}
