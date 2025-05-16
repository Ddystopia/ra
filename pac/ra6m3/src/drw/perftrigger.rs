///Register `PERFTRIGGER` writer
pub type W = crate::W<PERFTRIGGER_SPEC>;
/**Selects the internal event that will increment PERFCOUNT1 register.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PERFTRIGGER1_A {
    ///0: disable performance counter
    _0X00 = 0,
    ///1: 2D Drawing Engine active cycles
    _0X01 = 1,
    ///2: framebuffer read access
    _0X02 = 2,
    ///3: framebuffer write access
    _0X03 = 3,
    ///4: texture read access
    _0X04 = 4,
    ///5: invisible pixels (enumerated but selected with alpha 0percent)
    _0X05 = 5,
    ///6: invisible pixels while internal FIFO is empty (lost cycles)
    _0X06 = 6,
    ///7: display list reader active cycles
    _0X07 = 7,
    ///8: framebuffer read hits
    _0X08 = 8,
    ///9: framebuffer read misses
    _0X09 = 9,
    ///10: framebuffer write hits
    _0X0A = 10,
    ///11: framebuffer write misses
    _0X0B = 11,
    ///12: texture read hits
    _0X0C = 12,
    ///13: texture read misses
    _0X0D = 13,
    ///31: every clock cycle (for use as timer)
    _0X1F = 31,
    ///14: Setting prohibited
    OTHERS = 14,
}
impl From<PERFTRIGGER1_A> for u16 {
    #[inline(always)]
    fn from(variant: PERFTRIGGER1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PERFTRIGGER1_A {
    type Ux = u16;
}
impl crate::IsEnum for PERFTRIGGER1_A {}
///Field `PERFTRIGGER1` writer - Selects the internal event that will increment PERFCOUNT1 register.
pub type PERFTRIGGER1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, PERFTRIGGER1_A, crate::Safe>;
impl<'a, REG> PERFTRIGGER1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///disable performance counter
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X00)
    }
    ///2D Drawing Engine active cycles
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X01)
    }
    ///framebuffer read access
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X02)
    }
    ///framebuffer write access
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X03)
    }
    ///texture read access
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X04)
    }
    ///invisible pixels (enumerated but selected with alpha 0percent)
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X05)
    }
    ///invisible pixels while internal FIFO is empty (lost cycles)
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X06)
    }
    ///display list reader active cycles
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X07)
    }
    ///framebuffer read hits
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X08)
    }
    ///framebuffer read misses
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X09)
    }
    ///framebuffer write hits
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X0A)
    }
    ///framebuffer write misses
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X0B)
    }
    ///texture read hits
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X0C)
    }
    ///texture read misses
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X0D)
    }
    ///every clock cycle (for use as timer)
    #[inline(always)]
    pub fn _0x1f(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::_0X1F)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER1_A::OTHERS)
    }
}
/**Selects the internal event that will increment PERFCOUNT2 register

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PERFTRIGGER2_A {
    ///0: disable performance counter
    _0X00 = 0,
    ///1: 2D Drawing Engine active cycles
    _0X01 = 1,
    ///2: framebuffer read access
    _0X02 = 2,
    ///3: framebuffer write access
    _0X03 = 3,
    ///4: texture read access
    _0X04 = 4,
    ///5: invisible pixels (enumerated but selected with alpha 0percent)
    _0X05 = 5,
    ///6: invisible pixels while internal FIFO is empty (lost cycles)
    _0X06 = 6,
    ///7: display list reader active cycles
    _0X07 = 7,
    ///8: framebuffer read hits
    _0X08 = 8,
    ///9: framebuffer read misses
    _0X09 = 9,
    ///10: framebuffer write hits
    _0X0A = 10,
    ///11: framebuffer write misses
    _0X0B = 11,
    ///12: texture read hits
    _0X0C = 12,
    ///13: texture read misses
    _0X0D = 13,
    ///31: every clock cycle (for use as timer)
    _0X1F = 31,
    ///14: Setting prohibited
    OTHERS = 14,
}
impl From<PERFTRIGGER2_A> for u16 {
    #[inline(always)]
    fn from(variant: PERFTRIGGER2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PERFTRIGGER2_A {
    type Ux = u16;
}
impl crate::IsEnum for PERFTRIGGER2_A {}
///Field `PERFTRIGGER2` writer - Selects the internal event that will increment PERFCOUNT2 register
pub type PERFTRIGGER2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, PERFTRIGGER2_A, crate::Safe>;
impl<'a, REG> PERFTRIGGER2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///disable performance counter
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X00)
    }
    ///2D Drawing Engine active cycles
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X01)
    }
    ///framebuffer read access
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X02)
    }
    ///framebuffer write access
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X03)
    }
    ///texture read access
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X04)
    }
    ///invisible pixels (enumerated but selected with alpha 0percent)
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X05)
    }
    ///invisible pixels while internal FIFO is empty (lost cycles)
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X06)
    }
    ///display list reader active cycles
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X07)
    }
    ///framebuffer read hits
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X08)
    }
    ///framebuffer read misses
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X09)
    }
    ///framebuffer write hits
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X0A)
    }
    ///framebuffer write misses
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X0B)
    }
    ///texture read hits
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X0C)
    }
    ///texture read misses
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X0D)
    }
    ///every clock cycle (for use as timer)
    #[inline(always)]
    pub fn _0x1f(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::_0X1F)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PERFTRIGGER2_A::OTHERS)
    }
}
impl W {
    ///Bits 0:15 - Selects the internal event that will increment PERFCOUNT1 register.
    #[inline(always)]
    pub fn perftrigger1(&mut self) -> PERFTRIGGER1_W<PERFTRIGGER_SPEC> {
        PERFTRIGGER1_W::new(self, 0)
    }
    ///Bits 16:31 - Selects the internal event that will increment PERFCOUNT2 register
    #[inline(always)]
    pub fn perftrigger2(&mut self) -> PERFTRIGGER2_W<PERFTRIGGER_SPEC> {
        PERFTRIGGER2_W::new(self, 16)
    }
}
/**Performance Counters Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perftrigger::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PERFTRIGGER_SPEC;
impl crate::RegisterSpec for PERFTRIGGER_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`perftrigger::W`](W) writer structure
impl crate::Writable for PERFTRIGGER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PERFTRIGGER to value 0
impl crate::Resettable for PERFTRIGGER_SPEC {}
