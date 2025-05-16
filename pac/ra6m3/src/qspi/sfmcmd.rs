///Register `SFMCMD` reader
pub type R = crate::R<SFMCMD_SPEC>;
///Register `SFMCMD` writer
pub type W = crate::W<SFMCMD_SPEC>;
/**Selection of a mode of communication with the SPI bus

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCOM_A {
    ///0: ROM access mode
    _0 = 0,
    ///1: Direct communication mode
    _1 = 1,
}
impl From<DCOM_A> for bool {
    #[inline(always)]
    fn from(variant: DCOM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DCOM` reader - Selection of a mode of communication with the SPI bus
pub type DCOM_R = crate::BitReader<DCOM_A>;
impl DCOM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCOM_A {
        match self.bits {
            false => DCOM_A::_0,
            true => DCOM_A::_1,
        }
    }
    ///ROM access mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCOM_A::_0
    }
    ///Direct communication mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCOM_A::_1
    }
}
///Field `DCOM` writer - Selection of a mode of communication with the SPI bus
pub type DCOM_W<'a, REG> = crate::BitWriter<'a, REG, DCOM_A>;
impl<'a, REG> DCOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ROM access mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DCOM_A::_0)
    }
    ///Direct communication mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DCOM_A::_1)
    }
}
impl R {
    ///Bit 0 - Selection of a mode of communication with the SPI bus
    #[inline(always)]
    pub fn dcom(&self) -> DCOM_R {
        DCOM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Selection of a mode of communication with the SPI bus
    #[inline(always)]
    pub fn dcom(&mut self) -> DCOM_W<SFMCMD_SPEC> {
        DCOM_W::new(self, 0)
    }
}
/**Communication Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMCMD_SPEC;
impl crate::RegisterSpec for SFMCMD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmcmd::R`](R) reader structure
impl crate::Readable for SFMCMD_SPEC {}
///`write(|w| ..)` method takes [`sfmcmd::W`](W) writer structure
impl crate::Writable for SFMCMD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMCMD to value 0
impl crate::Resettable for SFMCMD_SPEC {}
