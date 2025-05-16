///Register `EDMR` reader
pub type R = crate::R<EDMR_SPEC>;
///Register `EDMR` writer
pub type W = crate::W<EDMR_SPEC>;
/**Software Reset

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWR_A {
    ///0: no effect.
    _0 = 0,
    ///1: the corresponding channels of the EDMAC and ETHERC are reset. Registers TDLAR, RDLAR, RMFCR, TFUCR, and RFOCR are not reset.
    _1 = 1,
}
impl From<SWR_A> for bool {
    #[inline(always)]
    fn from(variant: SWR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SWR` writer - Software Reset
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG, SWR_A>;
impl<'a, REG> SWR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SWR_A::_0)
    }
    ///the corresponding channels of the EDMAC and ETHERC are reset. Registers TDLAR, RDLAR, RMFCR, TFUCR, and RFOCR are not reset.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SWR_A::_1)
    }
}
/**Transmit/Receive DescriptorLength

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DL_A {
    ///0: 16 bytes
    _00 = 0,
    ///1: 32 bytes
    _01 = 1,
    ///2: 64 bytes
    _10 = 2,
    ///3: 16 bytes
    _11 = 3,
}
impl From<DL_A> for u8 {
    #[inline(always)]
    fn from(variant: DL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DL_A {
    type Ux = u8;
}
impl crate::IsEnum for DL_A {}
///Field `DL` reader - Transmit/Receive DescriptorLength
pub type DL_R = crate::FieldReader<DL_A>;
impl DL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DL_A {
        match self.bits {
            0 => DL_A::_00,
            1 => DL_A::_01,
            2 => DL_A::_10,
            3 => DL_A::_11,
            _ => unreachable!(),
        }
    }
    ///16 bytes
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DL_A::_00
    }
    ///32 bytes
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DL_A::_01
    }
    ///64 bytes
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DL_A::_10
    }
    ///16 bytes
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DL_A::_11
    }
}
///Field `DL` writer - Transmit/Receive DescriptorLength
pub type DL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DL_A, crate::Safe>;
impl<'a, REG> DL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///16 bytes
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DL_A::_00)
    }
    ///32 bytes
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DL_A::_01)
    }
    ///64 bytes
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(DL_A::_10)
    }
    ///16 bytes
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(DL_A::_11)
    }
}
/**Big Endian Mode/Little Endian ModeNOTE: This setting applies to data for the transmit/receive buffer. It does not apply to transmit/receive descriptors and registers.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DE_A {
    ///0: Big endian mode
    _0 = 0,
    ///1: Little endian mode
    _1 = 1,
}
impl From<DE_A> for bool {
    #[inline(always)]
    fn from(variant: DE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DE` reader - Big Endian Mode/Little Endian ModeNOTE: This setting applies to data for the transmit/receive buffer. It does not apply to transmit/receive descriptors and registers.
pub type DE_R = crate::BitReader<DE_A>;
impl DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DE_A {
        match self.bits {
            false => DE_A::_0,
            true => DE_A::_1,
        }
    }
    ///Big endian mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DE_A::_0
    }
    ///Little endian mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DE_A::_1
    }
}
///Field `DE` writer - Big Endian Mode/Little Endian ModeNOTE: This setting applies to data for the transmit/receive buffer. It does not apply to transmit/receive descriptors and registers.
pub type DE_W<'a, REG> = crate::BitWriter<'a, REG, DE_A>;
impl<'a, REG> DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Big endian mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DE_A::_0)
    }
    ///Little endian mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DE_A::_1)
    }
}
impl R {
    ///Bits 4:5 - Transmit/Receive DescriptorLength
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Big Endian Mode/Little Endian ModeNOTE: This setting applies to data for the transmit/receive buffer. It does not apply to transmit/receive descriptors and registers.
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Software Reset
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W<EDMR_SPEC> {
        SWR_W::new(self, 0)
    }
    ///Bits 4:5 - Transmit/Receive DescriptorLength
    #[inline(always)]
    pub fn dl(&mut self) -> DL_W<EDMR_SPEC> {
        DL_W::new(self, 4)
    }
    ///Bit 6 - Big Endian Mode/Little Endian ModeNOTE: This setting applies to data for the transmit/receive buffer. It does not apply to transmit/receive descriptors and registers.
    #[inline(always)]
    pub fn de(&mut self) -> DE_W<EDMR_SPEC> {
        DE_W::new(self, 6)
    }
}
/**EDMAC Mode Register

You can [`read`](crate::Reg::read) this register and get [`edmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EDMR_SPEC;
impl crate::RegisterSpec for EDMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`edmr::R`](R) reader structure
impl crate::Readable for EDMR_SPEC {}
///`write(|w| ..)` method takes [`edmr::W`](W) writer structure
impl crate::Writable for EDMR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EDMR to value 0
impl crate::Resettable for EDMR_SPEC {}
