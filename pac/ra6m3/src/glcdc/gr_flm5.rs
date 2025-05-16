///Register `GR%s_FLM5` reader
pub type R = crate::R<GR_FLM5_SPEC>;
///Register `GR%s_FLM5` writer
pub type W = crate::W<GR_FLM5_SPEC>;
/**Number of data transfer times per line for accessing graphics data (frame buffer data), where one transfer is defined as 16-beat burst access (64-byte boundary)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DATANUM_A {
    ///0: DATAUM+1 times.
    DATAUM = 0,
}
impl From<DATANUM_A> for u16 {
    #[inline(always)]
    fn from(variant: DATANUM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATANUM_A {
    type Ux = u16;
}
impl crate::IsEnum for DATANUM_A {}
///Field `DATANUM` reader - Number of data transfer times per line for accessing graphics data (frame buffer data), where one transfer is defined as 16-beat burst access (64-byte boundary)
pub type DATANUM_R = crate::FieldReader<DATANUM_A>;
impl DATANUM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DATANUM_A {
        match self.bits {
            _ => DATANUM_A::DATAUM,
        }
    }
    ///DATAUM+1 times.
    #[inline(always)]
    pub fn is_dataum(&self) -> bool {
        matches!(self.variant(), DATANUM_A::DATAUM)
    }
}
///Field `DATANUM` writer - Number of data transfer times per line for accessing graphics data (frame buffer data), where one transfer is defined as 16-beat burst access (64-byte boundary)
pub type DATANUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, DATANUM_A, crate::Safe>;
impl<'a, REG> DATANUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///DATAUM+1 times.
    #[inline(always)]
    pub fn dataum(self) -> &'a mut crate::W<REG> {
        self.variant(DATANUM_A::DATAUM)
    }
}
/**Number of lines per frame for accessing graphics data (frame buffer data).

Value on reset: 15*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum LNNUM_A {
    ///0: LNNUM lines. The valid range is 0x00F to 0x3FF.
    LNNUM = 0,
}
impl From<LNNUM_A> for u16 {
    #[inline(always)]
    fn from(variant: LNNUM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LNNUM_A {
    type Ux = u16;
}
impl crate::IsEnum for LNNUM_A {}
///Field `LNNUM` reader - Number of lines per frame for accessing graphics data (frame buffer data).
pub type LNNUM_R = crate::FieldReader<LNNUM_A>;
impl LNNUM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LNNUM_A {
        match self.bits {
            _ => LNNUM_A::LNNUM,
        }
    }
    ///LNNUM lines. The valid range is 0x00F to 0x3FF.
    #[inline(always)]
    pub fn is_lnnum(&self) -> bool {
        matches!(self.variant(), LNNUM_A::LNNUM)
    }
}
///Field `LNNUM` writer - Number of lines per frame for accessing graphics data (frame buffer data).
pub type LNNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 11, LNNUM_A, crate::Safe>;
impl<'a, REG> LNNUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///LNNUM lines. The valid range is 0x00F to 0x3FF.
    #[inline(always)]
    pub fn lnnum(self) -> &'a mut crate::W<REG> {
        self.variant(LNNUM_A::LNNUM)
    }
}
impl R {
    ///Bits 0:15 - Number of data transfer times per line for accessing graphics data (frame buffer data), where one transfer is defined as 16-beat burst access (64-byte boundary)
    #[inline(always)]
    pub fn datanum(&self) -> DATANUM_R {
        DATANUM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:26 - Number of lines per frame for accessing graphics data (frame buffer data).
    #[inline(always)]
    pub fn lnnum(&self) -> LNNUM_R {
        LNNUM_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Number of data transfer times per line for accessing graphics data (frame buffer data), where one transfer is defined as 16-beat burst access (64-byte boundary)
    #[inline(always)]
    pub fn datanum(&mut self) -> DATANUM_W<GR_FLM5_SPEC> {
        DATANUM_W::new(self, 0)
    }
    ///Bits 16:26 - Number of lines per frame for accessing graphics data (frame buffer data).
    #[inline(always)]
    pub fn lnnum(&mut self) -> LNNUM_W<GR_FLM5_SPEC> {
        LNNUM_W::new(self, 16)
    }
}
/**Graphics %s Frame Buffer Control Register 5

You can [`read`](crate::Reg::read) this register and get [`gr_flm5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_flm5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_FLM5_SPEC;
impl crate::RegisterSpec for GR_FLM5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_flm5::R`](R) reader structure
impl crate::Readable for GR_FLM5_SPEC {}
///`write(|w| ..)` method takes [`gr_flm5::W`](W) writer structure
impl crate::Writable for GR_FLM5_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_FLM5 to value 0x000f_0000
impl crate::Resettable for GR_FLM5_SPEC {
    const RESET_VALUE: u32 = 0x000f_0000;
}
