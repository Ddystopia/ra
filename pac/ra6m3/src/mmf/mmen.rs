///Register `MMEN` reader
pub type R = crate::R<MMEN_SPEC>;
///Register `MMEN` writer
pub type W = crate::W<MMEN_SPEC>;
/**Memory Mirror Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    ///1: Memory Mirror Function is enabled.
    _1 = 1,
    ///0: Memory Mirror Function is disabled.
    _0 = 0,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Memory Mirror Function Enable
pub type EN_R = crate::BitReader<EN_A>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            true => EN_A::_1,
            false => EN_A::_0,
        }
    }
    ///Memory Mirror Function is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_A::_1
    }
    ///Memory Mirror Function is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_A::_0
    }
}
///Field `EN` writer - Memory Mirror Function Enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Memory Mirror Function is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::_1)
    }
    ///Memory Mirror Function is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::_0)
    }
}
/**MMEN Key Code

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_A {
    ///219: Writing to the EN bit is valid, when the KEY bits are written 0xDB.
    _0X_DB = 219,
    ///0: Writing to the EN bit is invalid.
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
///Field `KEY` writer - MMEN Key Code
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, KEY_A, crate::Safe>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Writing to the EN bit is valid, when the KEY bits are written 0xDB.
    #[inline(always)]
    pub fn _0x_db(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::_0X_DB)
    }
    ///Writing to the EN bit is invalid.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::OTHERS)
    }
}
impl R {
    ///Bit 0 - Memory Mirror Function Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Memory Mirror Function Enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<MMEN_SPEC> {
        EN_W::new(self, 0)
    }
    ///Bits 24:31 - MMEN Key Code
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<MMEN_SPEC> {
        KEY_W::new(self, 24)
    }
}
/**MemMirror Enable Register

You can [`read`](crate::Reg::read) this register and get [`mmen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MMEN_SPEC;
impl crate::RegisterSpec for MMEN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mmen::R`](R) reader structure
impl crate::Readable for MMEN_SPEC {}
///`write(|w| ..)` method takes [`mmen::W`](W) writer structure
impl crate::Writable for MMEN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMEN to value 0
impl crate::Resettable for MMEN_SPEC {}
