///Register `PIPEMAXP` reader
pub type R = crate::R<PIPEMAXP_SPEC>;
///Register `PIPEMAXP` writer
pub type W = crate::W<PIPEMAXP_SPEC>;
///Field `MXPS` reader - Maximum Packet SizePIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h)PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \[8:7\] and \[2:0\] are not provided.)PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \[8:7\] are not provided.)
pub type MXPS_R = crate::FieldReader<u16>;
///Field `MXPS` writer - Maximum Packet SizePIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h)PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \[8:7\] and \[2:0\] are not provided.)PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \[8:7\] are not provided.)
pub type MXPS_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
/**Device Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEVSEL_A {
    ///0: Address 0000
    _0000 = 0,
    ///1: Address 0001
    _0001 = 1,
    ///2: Address 0010
    _0010 = 2,
    ///3: Address 0011
    _0011 = 3,
    ///4: Address 0100
    _0100 = 4,
    ///5: Address 0101
    _0101 = 5,
    ///6: Settings prohibited.
    OTHERS = 6,
}
impl From<DEVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DEVSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for DEVSEL_A {}
///Field `DEVSEL` reader - Device Select
pub type DEVSEL_R = crate::FieldReader<DEVSEL_A>;
impl DEVSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEVSEL_A {
        match self.bits {
            0 => DEVSEL_A::_0000,
            1 => DEVSEL_A::_0001,
            2 => DEVSEL_A::_0010,
            3 => DEVSEL_A::_0011,
            4 => DEVSEL_A::_0100,
            5 => DEVSEL_A::_0101,
            _ => DEVSEL_A::OTHERS,
        }
    }
    ///Address 0000
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DEVSEL_A::_0000
    }
    ///Address 0001
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DEVSEL_A::_0001
    }
    ///Address 0010
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DEVSEL_A::_0010
    }
    ///Address 0011
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DEVSEL_A::_0011
    }
    ///Address 0100
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DEVSEL_A::_0100
    }
    ///Address 0101
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DEVSEL_A::_0101
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), DEVSEL_A::OTHERS)
    }
}
///Field `DEVSEL` writer - Device Select
pub type DEVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DEVSEL_A, crate::Safe>;
impl<'a, REG> DEVSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Address 0000
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0000)
    }
    ///Address 0001
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0001)
    }
    ///Address 0010
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0010)
    }
    ///Address 0011
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0011)
    }
    ///Address 0100
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0100)
    }
    ///Address 0101
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0101)
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::OTHERS)
    }
}
impl R {
    ///Bits 0:8 - Maximum Packet SizePIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h)PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \[8:7\] and \[2:0\] are not provided.)PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \[8:7\] are not provided.)
    #[inline(always)]
    pub fn mxps(&self) -> MXPS_R {
        MXPS_R::new(self.bits & 0x01ff)
    }
    ///Bits 12:15 - Device Select
    #[inline(always)]
    pub fn devsel(&self) -> DEVSEL_R {
        DEVSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:8 - Maximum Packet SizePIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h)PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \[8:7\] and \[2:0\] are not provided.)PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \[8:7\] are not provided.)
    #[inline(always)]
    pub fn mxps(&mut self) -> MXPS_W<PIPEMAXP_SPEC> {
        MXPS_W::new(self, 0)
    }
    ///Bits 12:15 - Device Select
    #[inline(always)]
    pub fn devsel(&mut self) -> DEVSEL_W<PIPEMAXP_SPEC> {
        DEVSEL_W::new(self, 12)
    }
}
/**Pipe Maximum Packet Size Register

You can [`read`](crate::Reg::read) this register and get [`pipemaxp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipemaxp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIPEMAXP_SPEC;
impl crate::RegisterSpec for PIPEMAXP_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pipemaxp::R`](R) reader structure
impl crate::Readable for PIPEMAXP_SPEC {}
///`write(|w| ..)` method takes [`pipemaxp::W`](W) writer structure
impl crate::Writable for PIPEMAXP_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPEMAXP to value 0
impl crate::Resettable for PIPEMAXP_SPEC {}
