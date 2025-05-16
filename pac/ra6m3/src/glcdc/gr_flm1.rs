///Register `GR%s_FLM1` reader
pub type R = crate::R<GR_FLM1_SPEC>;
///Register `GR%s_FLM1` writer
pub type W = crate::W<GR_FLM1_SPEC>;
/**Burst transfer control for graphics data (frame buffer data)access

Value on reset: 3*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BSTMD_A {
    ///3: 16-beat increment burst transfer (64-byte boundary)
    _11 = 3,
    ///0: Setting prohibited.
    OTHERS = 0,
}
impl From<BSTMD_A> for u8 {
    #[inline(always)]
    fn from(variant: BSTMD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BSTMD_A {
    type Ux = u8;
}
impl crate::IsEnum for BSTMD_A {}
///Field `BSTMD` reader - Burst transfer control for graphics data (frame buffer data)access
pub type BSTMD_R = crate::FieldReader<BSTMD_A>;
impl BSTMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSTMD_A {
        match self.bits {
            3 => BSTMD_A::_11,
            _ => BSTMD_A::OTHERS,
        }
    }
    ///16-beat increment burst transfer (64-byte boundary)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BSTMD_A::_11
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), BSTMD_A::OTHERS)
    }
}
///Field `BSTMD` writer - Burst transfer control for graphics data (frame buffer data)access
pub type BSTMD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BSTMD_A, crate::Safe>;
impl<'a, REG> BSTMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///16-beat increment burst transfer (64-byte boundary)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(BSTMD_A::_11)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(BSTMD_A::OTHERS)
    }
}
impl R {
    ///Bits 0:1 - Burst transfer control for graphics data (frame buffer data)access
    #[inline(always)]
    pub fn bstmd(&self) -> BSTMD_R {
        BSTMD_R::new((self.bits & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Burst transfer control for graphics data (frame buffer data)access
    #[inline(always)]
    pub fn bstmd(&mut self) -> BSTMD_W<GR_FLM1_SPEC> {
        BSTMD_W::new(self, 0)
    }
}
/**Graphics %s Frame Buffer Control Register 1

You can [`read`](crate::Reg::read) this register and get [`gr_flm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_flm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_FLM1_SPEC;
impl crate::RegisterSpec for GR_FLM1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_flm1::R`](R) reader structure
impl crate::Readable for GR_FLM1_SPEC {}
///`write(|w| ..)` method takes [`gr_flm1::W`](W) writer structure
impl crate::Writable for GR_FLM1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_FLM1 to value 0x03
impl crate::Resettable for GR_FLM1_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
