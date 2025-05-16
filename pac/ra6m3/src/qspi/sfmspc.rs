///Register `SFMSPC` reader
pub type R = crate::R<SFMSPC_SPEC>;
///Register `SFMSPC` writer
pub type W = crate::W<SFMSPC_SPEC>;
/**Selection of SPI protocolNOTE: Serial ROM's SPI protocol is required to be set by software separately.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMSPI_A {
    ///0: Extended SPI protocol
    _00 = 0,
    ///1: Dual SPI protocol
    _01 = 1,
    ///2: Quad SPI protocol
    _10 = 2,
    ///3: Setting prohibited.
    _11 = 3,
}
impl From<SFMSPI_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMSPI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SFMSPI_A {
    type Ux = u8;
}
impl crate::IsEnum for SFMSPI_A {}
///Field `SFMSPI` reader - Selection of SPI protocolNOTE: Serial ROM's SPI protocol is required to be set by software separately.
pub type SFMSPI_R = crate::FieldReader<SFMSPI_A>;
impl SFMSPI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMSPI_A {
        match self.bits {
            0 => SFMSPI_A::_00,
            1 => SFMSPI_A::_01,
            2 => SFMSPI_A::_10,
            3 => SFMSPI_A::_11,
            _ => unreachable!(),
        }
    }
    ///Extended SPI protocol
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SFMSPI_A::_00
    }
    ///Dual SPI protocol
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SFMSPI_A::_01
    }
    ///Quad SPI protocol
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SFMSPI_A::_10
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SFMSPI_A::_11
    }
}
///Field `SFMSPI` writer - Selection of SPI protocolNOTE: Serial ROM's SPI protocol is required to be set by software separately.
pub type SFMSPI_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SFMSPI_A, crate::Safe>;
impl<'a, REG> SFMSPI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Extended SPI protocol
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSPI_A::_00)
    }
    ///Dual SPI protocol
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSPI_A::_01)
    }
    ///Quad SPI protocol
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSPI_A::_10)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSPI_A::_11)
    }
}
/**Selection of the minimum time of input output switch, when Dual SPI protocol or Quad SPI protocol is selected.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMSDE_A {
    ///0: Does not allocate minimum switch time
    _0 = 0,
    ///1: Allocate the minimum switch time equivalent to 1*QSPXLK
    _1 = 1,
}
impl From<SFMSDE_A> for bool {
    #[inline(always)]
    fn from(variant: SFMSDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFMSDE` reader - Selection of the minimum time of input output switch, when Dual SPI protocol or Quad SPI protocol is selected.
pub type SFMSDE_R = crate::BitReader<SFMSDE_A>;
impl SFMSDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFMSDE_A {
        match self.bits {
            false => SFMSDE_A::_0,
            true => SFMSDE_A::_1,
        }
    }
    ///Does not allocate minimum switch time
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMSDE_A::_0
    }
    ///Allocate the minimum switch time equivalent to 1*QSPXLK
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMSDE_A::_1
    }
}
///Field `SFMSDE` writer - Selection of the minimum time of input output switch, when Dual SPI protocol or Quad SPI protocol is selected.
pub type SFMSDE_W<'a, REG> = crate::BitWriter<'a, REG, SFMSDE_A>;
impl<'a, REG> SFMSDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not allocate minimum switch time
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSDE_A::_0)
    }
    ///Allocate the minimum switch time equivalent to 1*QSPXLK
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SFMSDE_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Selection of SPI protocolNOTE: Serial ROM's SPI protocol is required to be set by software separately.
    #[inline(always)]
    pub fn sfmspi(&self) -> SFMSPI_R {
        SFMSPI_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - Selection of the minimum time of input output switch, when Dual SPI protocol or Quad SPI protocol is selected.
    #[inline(always)]
    pub fn sfmsde(&self) -> SFMSDE_R {
        SFMSDE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Selection of SPI protocolNOTE: Serial ROM's SPI protocol is required to be set by software separately.
    #[inline(always)]
    pub fn sfmspi(&mut self) -> SFMSPI_W<SFMSPC_SPEC> {
        SFMSPI_W::new(self, 0)
    }
    ///Bit 4 - Selection of the minimum time of input output switch, when Dual SPI protocol or Quad SPI protocol is selected.
    #[inline(always)]
    pub fn sfmsde(&mut self) -> SFMSDE_W<SFMSPC_SPEC> {
        SFMSDE_W::new(self, 4)
    }
}
/**SPI Protocol Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmspc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmspc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMSPC_SPEC;
impl crate::RegisterSpec for SFMSPC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmspc::R`](R) reader structure
impl crate::Readable for SFMSPC_SPEC {}
///`write(|w| ..)` method takes [`sfmspc::W`](W) writer structure
impl crate::Writable for SFMSPC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMSPC to value 0x10
impl crate::Resettable for SFMSPC_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
