///Register `MSPMPUOAD` reader
pub type R = crate::R<MSPMPUOAD_SPEC>;
///Register `MSPMPUOAD` writer
pub type W = crate::W<MSPMPUOAD_SPEC>;
/**Operation after detection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAD_A {
    ///1: Internal reset.
    _1 = 1,
    ///0: Non-maskable interrupt.
    _0 = 0,
}
impl From<OAD_A> for bool {
    #[inline(always)]
    fn from(variant: OAD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OAD` reader - Operation after detection
pub type OAD_R = crate::BitReader<OAD_A>;
impl OAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OAD_A {
        match self.bits {
            true => OAD_A::_1,
            false => OAD_A::_0,
        }
    }
    ///Internal reset.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAD_A::_1
    }
    ///Non-maskable interrupt.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAD_A::_0
    }
}
///Field `OAD` writer - Operation after detection
pub type OAD_W<'a, REG> = crate::BitWriter<'a, REG, OAD_A>;
impl<'a, REG> OAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Internal reset.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OAD_A::_1)
    }
    ///Non-maskable interrupt.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OAD_A::_0)
    }
}
/**Write Keyword The data written to these bits are not stored.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_A {
    ///165: Writing to the OAD bit is valid, when the KEY bits are written 0xA5.
    _0X_A5 = 165,
    ///0: Writing to the OAD bit is invalid.
    OTHERS = 0,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY_A {
    type Ux = u8;
}
impl crate::IsEnum for KEY_A {}
///Field `KEY` writer - Write Keyword The data written to these bits are not stored.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, KEY_A, crate::Safe>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Writing to the OAD bit is valid, when the KEY bits are written 0xA5.
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::_0X_A5)
    }
    ///Writing to the OAD bit is invalid.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - Operation after detection
    #[inline(always)]
    pub fn oad(&self) -> OAD_R {
        OAD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Operation after detection
    #[inline(always)]
    pub fn oad(&mut self) -> OAD_W<MSPMPUOAD_SPEC> {
        OAD_W::new(self, 0)
    }
    ///Bits 8:15 - Write Keyword The data written to these bits are not stored.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<MSPMPUOAD_SPEC> {
        KEY_W::new(self, 8)
    }
}
/**Stack Pointer Monitor Operation After Detection Register

You can [`read`](crate::Reg::read) this register and get [`mspmpuoad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpuoad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MSPMPUOAD_SPEC;
impl crate::RegisterSpec for MSPMPUOAD_SPEC {
    type Ux = u16;
}
///`read()` method returns [`mspmpuoad::R`](R) reader structure
impl crate::Readable for MSPMPUOAD_SPEC {}
///`write(|w| ..)` method takes [`mspmpuoad::W`](W) writer structure
impl crate::Writable for MSPMPUOAD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSPMPUOAD to value 0
impl crate::Resettable for MSPMPUOAD_SPEC {}
