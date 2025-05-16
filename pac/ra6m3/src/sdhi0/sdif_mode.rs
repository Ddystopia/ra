///Register `SDIF_MODE` reader
pub type R = crate::R<SDIF_MODE_SPEC>;
///Register `SDIF_MODE` writer
pub type W = crate::W<SDIF_MODE_SPEC>;
/**CRC Check Mask (for MMC test commands)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOCHKCR_A {
    ///0: CRC check is valid
    _0 = 0,
    ///1: CRC check is invalid(CRC16 value is ignored when read and CRC Status value is ignored when write)
    _1 = 1,
}
impl From<NOCHKCR_A> for bool {
    #[inline(always)]
    fn from(variant: NOCHKCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NOCHKCR` reader - CRC Check Mask (for MMC test commands)
pub type NOCHKCR_R = crate::BitReader<NOCHKCR_A>;
impl NOCHKCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NOCHKCR_A {
        match self.bits {
            false => NOCHKCR_A::_0,
            true => NOCHKCR_A::_1,
        }
    }
    ///CRC check is valid
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NOCHKCR_A::_0
    }
    ///CRC check is invalid(CRC16 value is ignored when read and CRC Status value is ignored when write)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NOCHKCR_A::_1
    }
}
///Field `NOCHKCR` writer - CRC Check Mask (for MMC test commands)
pub type NOCHKCR_W<'a, REG> = crate::BitWriter<'a, REG, NOCHKCR_A>;
impl<'a, REG> NOCHKCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC check is valid
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NOCHKCR_A::_0)
    }
    ///CRC check is invalid(CRC16 value is ignored when read and CRC Status value is ignored when write)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NOCHKCR_A::_1)
    }
}
impl R {
    ///Bit 8 - CRC Check Mask (for MMC test commands)
    #[inline(always)]
    pub fn nochkcr(&self) -> NOCHKCR_R {
        NOCHKCR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 8 - CRC Check Mask (for MMC test commands)
    #[inline(always)]
    pub fn nochkcr(&mut self) -> NOCHKCR_W<SDIF_MODE_SPEC> {
        NOCHKCR_W::new(self, 8)
    }
}
/**SD Interface Mode Setting Register

You can [`read`](crate::Reg::read) this register and get [`sdif_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdif_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDIF_MODE_SPEC;
impl crate::RegisterSpec for SDIF_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sdif_mode::R`](R) reader structure
impl crate::Readable for SDIF_MODE_SPEC {}
///`write(|w| ..)` method takes [`sdif_mode::W`](W) writer structure
impl crate::Writable for SDIF_MODE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDIF_MODE to value 0
impl crate::Resettable for SDIF_MODE_SPEC {}
