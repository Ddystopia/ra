///Register `DELSR%s` reader
pub type R = crate::R<DELSR_SPEC>;
///Register `DELSR%s` writer
pub type W = crate::W<DELSR_SPEC>;
/**DMAC Event Link Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DELS_A {
    ///0: Nothing is selected.
    _0X000 = 0,
    ///1: See Event Table
    OTHERS = 1,
}
impl From<DELS_A> for u16 {
    #[inline(always)]
    fn from(variant: DELS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DELS_A {
    type Ux = u16;
}
impl crate::IsEnum for DELS_A {}
///Field `DELS` reader - DMAC Event Link Select
pub type DELS_R = crate::FieldReader<DELS_A>;
impl DELS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DELS_A {
        match self.bits {
            0 => DELS_A::_0X000,
            _ => DELS_A::OTHERS,
        }
    }
    ///Nothing is selected.
    #[inline(always)]
    pub fn is_0x000(&self) -> bool {
        *self == DELS_A::_0X000
    }
    ///See Event Table
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), DELS_A::OTHERS)
    }
}
///Field `DELS` writer - DMAC Event Link Select
pub type DELS_W<'a, REG> = crate::FieldWriter<'a, REG, 9, DELS_A, crate::Safe>;
impl<'a, REG> DELS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Nothing is selected.
    #[inline(always)]
    pub fn _0x000(self) -> &'a mut crate::W<REG> {
        self.variant(DELS_A::_0X000)
    }
    ///See Event Table
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(DELS_A::OTHERS)
    }
}
/**Interrupt Status Flag for DMAC

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IR_A {
    ///0: No interrupt request is generated
    _0 = 0,
    ///1: An interrupt request is generated ( "1" write to the IR bit is prohibited. )
    _1 = 1,
}
impl From<IR_A> for bool {
    #[inline(always)]
    fn from(variant: IR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IR` reader - Interrupt Status Flag for DMAC
pub type IR_R = crate::BitReader<IR_A>;
impl IR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IR_A {
        match self.bits {
            false => IR_A::_0,
            true => IR_A::_1,
        }
    }
    ///No interrupt request is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IR_A::_0
    }
    ///An interrupt request is generated ( "1" write to the IR bit is prohibited. )
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IR_A::_1
    }
}
///Field `IR` writer - Interrupt Status Flag for DMAC
pub type IR_W<'a, REG> = crate::BitWriter<'a, REG, IR_A>;
impl<'a, REG> IR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No interrupt request is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IR_A::_0)
    }
    ///An interrupt request is generated ( "1" write to the IR bit is prohibited. )
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IR_A::_1)
    }
}
impl R {
    ///Bits 0:8 - DMAC Event Link Select
    #[inline(always)]
    pub fn dels(&self) -> DELS_R {
        DELS_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 16 - Interrupt Status Flag for DMAC
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bits 0:8 - DMAC Event Link Select
    #[inline(always)]
    pub fn dels(&mut self) -> DELS_W<DELSR_SPEC> {
        DELS_W::new(self, 0)
    }
    ///Bit 16 - Interrupt Status Flag for DMAC
    #[inline(always)]
    pub fn ir(&mut self) -> IR_W<DELSR_SPEC> {
        IR_W::new(self, 16)
    }
}
/**DMAC Event Link Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`delsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DELSR_SPEC;
impl crate::RegisterSpec for DELSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`delsr::R`](R) reader structure
impl crate::Readable for DELSR_SPEC {}
///`write(|w| ..)` method takes [`delsr::W`](W) writer structure
impl crate::Writable for DELSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DELSR%s to value 0
impl crate::Resettable for DELSR_SPEC {}
