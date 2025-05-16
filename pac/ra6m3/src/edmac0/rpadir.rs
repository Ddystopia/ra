///Register `RPADIR` reader
pub type R = crate::R<RPADIR_SPEC>;
///Register `RPADIR` writer
pub type W = crate::W<RPADIR_SPEC>;
/**Padding Slot

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PADR_A {
    ///0: Padding is inserted at the head of received data.
    _00H = 0,
    ///1: Padding is inserted between the (PADR)th byte and (PADR+1)th byte of received data.
    OTHERS = 1,
}
impl From<PADR_A> for u8 {
    #[inline(always)]
    fn from(variant: PADR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PADR_A {
    type Ux = u8;
}
impl crate::IsEnum for PADR_A {}
///Field `PADR` reader - Padding Slot
pub type PADR_R = crate::FieldReader<PADR_A>;
impl PADR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PADR_A {
        match self.bits {
            0 => PADR_A::_00H,
            _ => PADR_A::OTHERS,
        }
    }
    ///Padding is inserted at the head of received data.
    #[inline(always)]
    pub fn is_00h(&self) -> bool {
        *self == PADR_A::_00H
    }
    ///Padding is inserted between the (PADR)th byte and (PADR+1)th byte of received data.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), PADR_A::OTHERS)
    }
}
///Field `PADR` writer - Padding Slot
pub type PADR_W<'a, REG> = crate::FieldWriter<'a, REG, 6, PADR_A, crate::Safe>;
impl<'a, REG> PADR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Padding is inserted at the head of received data.
    #[inline(always)]
    pub fn _00h(self) -> &'a mut crate::W<REG> {
        self.variant(PADR_A::_00H)
    }
    ///Padding is inserted between the (PADR)th byte and (PADR+1)th byte of received data.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PADR_A::OTHERS)
    }
}
/**Padding Size

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PADS_A {
    ///0: No padding is inserted.
    _00 = 0,
    ///1: 1 byte is inserted.
    _01 = 1,
    ///2: 2 bytes are inserted.
    _10 = 2,
    ///3: 3 bytes are inserted.
    _11 = 3,
}
impl From<PADS_A> for u8 {
    #[inline(always)]
    fn from(variant: PADS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PADS_A {
    type Ux = u8;
}
impl crate::IsEnum for PADS_A {}
///Field `PADS` reader - Padding Size
pub type PADS_R = crate::FieldReader<PADS_A>;
impl PADS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PADS_A {
        match self.bits {
            0 => PADS_A::_00,
            1 => PADS_A::_01,
            2 => PADS_A::_10,
            3 => PADS_A::_11,
            _ => unreachable!(),
        }
    }
    ///No padding is inserted.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PADS_A::_00
    }
    ///1 byte is inserted.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PADS_A::_01
    }
    ///2 bytes are inserted.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PADS_A::_10
    }
    ///3 bytes are inserted.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PADS_A::_11
    }
}
///Field `PADS` writer - Padding Size
pub type PADS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PADS_A, crate::Safe>;
impl<'a, REG> PADS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No padding is inserted.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(PADS_A::_00)
    }
    ///1 byte is inserted.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(PADS_A::_01)
    }
    ///2 bytes are inserted.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(PADS_A::_10)
    }
    ///3 bytes are inserted.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(PADS_A::_11)
    }
}
impl R {
    ///Bits 0:5 - Padding Slot
    #[inline(always)]
    pub fn padr(&self) -> PADR_R {
        PADR_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 16:17 - Padding Size
    #[inline(always)]
    pub fn pads(&self) -> PADS_R {
        PADS_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    ///Bits 0:5 - Padding Slot
    #[inline(always)]
    pub fn padr(&mut self) -> PADR_W<RPADIR_SPEC> {
        PADR_W::new(self, 0)
    }
    ///Bits 16:17 - Padding Size
    #[inline(always)]
    pub fn pads(&mut self) -> PADS_W<RPADIR_SPEC> {
        PADS_W::new(self, 16)
    }
}
/**Receive Data Padding Insert Register

You can [`read`](crate::Reg::read) this register and get [`rpadir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpadir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RPADIR_SPEC;
impl crate::RegisterSpec for RPADIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rpadir::R`](R) reader structure
impl crate::Readable for RPADIR_SPEC {}
///`write(|w| ..)` method takes [`rpadir::W`](W) writer structure
impl crate::Writable for RPADIR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RPADIR to value 0
impl crate::Resettable for RPADIR_SPEC {}
