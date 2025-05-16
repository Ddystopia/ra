///Register `MMSFR` reader
pub type R = crate::R<MMSFR_SPEC>;
///Register `MMSFR` writer
pub type W = crate::W<MMSFR_SPEC>;
///Field `MEMMIRADDR` reader - Specifies the memory mirror address.NOTE: A value cannot be set in the low-order 7 bits. These bits are fixed to 0.
pub type MEMMIRADDR_R = crate::FieldReader<u16>;
///Field `MEMMIRADDR` writer - Specifies the memory mirror address.NOTE: A value cannot be set in the low-order 7 bits. These bits are fixed to 0.
pub type MEMMIRADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
/**MMSFR Key Code

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_A {
    ///219: Writing to the MEMMIRADDR bits are valid, when the KEY bits are written 0xDB.
    _0X_DB = 219,
    ///0: Writing to the MEMMIRADDR bits are invalid.
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
///Field `KEY` writer - MMSFR Key Code
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, KEY_A, crate::Safe>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Writing to the MEMMIRADDR bits are valid, when the KEY bits are written 0xDB.
    #[inline(always)]
    pub fn _0x_db(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::_0X_DB)
    }
    ///Writing to the MEMMIRADDR bits are invalid.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::OTHERS)
    }
}
impl R {
    ///Bits 7:22 - Specifies the memory mirror address.NOTE: A value cannot be set in the low-order 7 bits. These bits are fixed to 0.
    #[inline(always)]
    pub fn memmiraddr(&self) -> MEMMIRADDR_R {
        MEMMIRADDR_R::new(((self.bits >> 7) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 7:22 - Specifies the memory mirror address.NOTE: A value cannot be set in the low-order 7 bits. These bits are fixed to 0.
    #[inline(always)]
    pub fn memmiraddr(&mut self) -> MEMMIRADDR_W<MMSFR_SPEC> {
        MEMMIRADDR_W::new(self, 7)
    }
    ///Bits 24:31 - MMSFR Key Code
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<MMSFR_SPEC> {
        KEY_W::new(self, 24)
    }
}
/**MemMirror Special Function Register

You can [`read`](crate::Reg::read) this register and get [`mmsfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmsfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MMSFR_SPEC;
impl crate::RegisterSpec for MMSFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mmsfr::R`](R) reader structure
impl crate::Readable for MMSFR_SPEC {}
///`write(|w| ..)` method takes [`mmsfr::W`](W) writer structure
impl crate::Writable for MMSFR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMSFR to value 0
impl crate::Resettable for MMSFR_SPEC {}
