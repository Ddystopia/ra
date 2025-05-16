///Register `FCFTR` reader
pub type R = crate::R<FCFTR_SPEC>;
///Register `FCFTR` writer
pub type W = crate::W<FCFTR_SPEC>;
/**Receive FIFO Data PAUSE Output Threshold(When (RFDO+1)x256-32 bytes of data is stored in the receive FIFO.)

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFDO_A {
    ///0: When 224 ( 256 - 32) bytes of data is stored in the receive FIFO.
    _000 = 0,
    ///1: When 480 ( 512 - 32) bytes of data is stored in the receive FIFO.
    _001 = 1,
    ///2: When 736 ( 768 - 32) bytes of data is stored in the receive FIFO.
    _010 = 2,
    ///3: When 992 (1024 - 32) bytes of data is stored in the receive FIFO.
    _011 = 3,
    ///4: When 1248 (1280 - 32) bytes of data is stored in the receive FIFO.
    _100 = 4,
    ///5: When 1504 (1536 - 32) bytes of data is stored in the receive FIFO.
    _101 = 5,
    ///6: When 1760 (1792 - 32) bytes of data is stored in the receive FIFO.
    _110 = 6,
    ///7: When 2016 (2048 - 32) bytes of data is stored in the receive FIFO.
    _111 = 7,
}
impl From<RFDO_A> for u8 {
    #[inline(always)]
    fn from(variant: RFDO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RFDO_A {
    type Ux = u8;
}
impl crate::IsEnum for RFDO_A {}
///Field `RFDO` reader - Receive FIFO Data PAUSE Output Threshold(When (RFDO+1)x256-32 bytes of data is stored in the receive FIFO.)
pub type RFDO_R = crate::FieldReader<RFDO_A>;
impl RFDO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFDO_A {
        match self.bits {
            0 => RFDO_A::_000,
            1 => RFDO_A::_001,
            2 => RFDO_A::_010,
            3 => RFDO_A::_011,
            4 => RFDO_A::_100,
            5 => RFDO_A::_101,
            6 => RFDO_A::_110,
            7 => RFDO_A::_111,
            _ => unreachable!(),
        }
    }
    ///When 224 ( 256 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RFDO_A::_000
    }
    ///When 480 ( 512 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RFDO_A::_001
    }
    ///When 736 ( 768 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RFDO_A::_010
    }
    ///When 992 (1024 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RFDO_A::_011
    }
    ///When 1248 (1280 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RFDO_A::_100
    }
    ///When 1504 (1536 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RFDO_A::_101
    }
    ///When 1760 (1792 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RFDO_A::_110
    }
    ///When 2016 (2048 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RFDO_A::_111
    }
}
///Field `RFDO` writer - Receive FIFO Data PAUSE Output Threshold(When (RFDO+1)x256-32 bytes of data is stored in the receive FIFO.)
pub type RFDO_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RFDO_A, crate::Safe>;
impl<'a, REG> RFDO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///When 224 ( 256 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(RFDO_A::_000)
    }
    ///When 480 ( 512 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(RFDO_A::_001)
    }
    ///When 736 ( 768 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(RFDO_A::_010)
    }
    ///When 992 (1024 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(RFDO_A::_011)
    }
    ///When 1248 (1280 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(RFDO_A::_100)
    }
    ///When 1504 (1536 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(RFDO_A::_101)
    }
    ///When 1760 (1792 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(RFDO_A::_110)
    }
    ///When 2016 (2048 - 32) bytes of data is stored in the receive FIFO.
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(RFDO_A::_111)
    }
}
/**Receive FIFO Frame PAUSE Output Threshold(When ((RFFO+1)x2) receive frames have been stored in the receive FIFO.)

Value on reset: 7*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFFO_A {
    ///0: When 2 receive frames have been stored in the receive FIFO.
    _000 = 0,
    ///1: When 4 receive frames have been stored in the receive FIFO.
    _001 = 1,
    ///2: When 6 receive frames have been stored in the receive FIFO.
    _010 = 2,
    ///3: When 8 receive frames have been stored in the receive FIFO.
    _011 = 3,
    ///4: When 10 receive frames have been stored in the receive FIFO.
    _100 = 4,
    ///5: When 12 receive frames have been stored in the receive FIFO.
    _101 = 5,
    ///6: When 14 receive frames have been stored in the receive FIFO.
    _110 = 6,
    ///7: When 16 receive frames have been stored in the receive FIFO.
    _111 = 7,
}
impl From<RFFO_A> for u8 {
    #[inline(always)]
    fn from(variant: RFFO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RFFO_A {
    type Ux = u8;
}
impl crate::IsEnum for RFFO_A {}
///Field `RFFO` reader - Receive FIFO Frame PAUSE Output Threshold(When ((RFFO+1)x2) receive frames have been stored in the receive FIFO.)
pub type RFFO_R = crate::FieldReader<RFFO_A>;
impl RFFO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFFO_A {
        match self.bits {
            0 => RFFO_A::_000,
            1 => RFFO_A::_001,
            2 => RFFO_A::_010,
            3 => RFFO_A::_011,
            4 => RFFO_A::_100,
            5 => RFFO_A::_101,
            6 => RFFO_A::_110,
            7 => RFFO_A::_111,
            _ => unreachable!(),
        }
    }
    ///When 2 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RFFO_A::_000
    }
    ///When 4 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RFFO_A::_001
    }
    ///When 6 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RFFO_A::_010
    }
    ///When 8 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RFFO_A::_011
    }
    ///When 10 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RFFO_A::_100
    }
    ///When 12 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RFFO_A::_101
    }
    ///When 14 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RFFO_A::_110
    }
    ///When 16 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RFFO_A::_111
    }
}
///Field `RFFO` writer - Receive FIFO Frame PAUSE Output Threshold(When ((RFFO+1)x2) receive frames have been stored in the receive FIFO.)
pub type RFFO_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RFFO_A, crate::Safe>;
impl<'a, REG> RFFO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///When 2 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(RFFO_A::_000)
    }
    ///When 4 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(RFFO_A::_001)
    }
    ///When 6 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(RFFO_A::_010)
    }
    ///When 8 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(RFFO_A::_011)
    }
    ///When 10 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(RFFO_A::_100)
    }
    ///When 12 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(RFFO_A::_101)
    }
    ///When 14 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(RFFO_A::_110)
    }
    ///When 16 receive frames have been stored in the receive FIFO.
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(RFFO_A::_111)
    }
}
impl R {
    ///Bits 0:2 - Receive FIFO Data PAUSE Output Threshold(When (RFDO+1)x256-32 bytes of data is stored in the receive FIFO.)
    #[inline(always)]
    pub fn rfdo(&self) -> RFDO_R {
        RFDO_R::new((self.bits & 7) as u8)
    }
    ///Bits 16:18 - Receive FIFO Frame PAUSE Output Threshold(When ((RFFO+1)x2) receive frames have been stored in the receive FIFO.)
    #[inline(always)]
    pub fn rffo(&self) -> RFFO_R {
        RFFO_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Receive FIFO Data PAUSE Output Threshold(When (RFDO+1)x256-32 bytes of data is stored in the receive FIFO.)
    #[inline(always)]
    pub fn rfdo(&mut self) -> RFDO_W<FCFTR_SPEC> {
        RFDO_W::new(self, 0)
    }
    ///Bits 16:18 - Receive FIFO Frame PAUSE Output Threshold(When ((RFFO+1)x2) receive frames have been stored in the receive FIFO.)
    #[inline(always)]
    pub fn rffo(&mut self) -> RFFO_W<FCFTR_SPEC> {
        RFFO_W::new(self, 16)
    }
}
/**Flow Control Start FIFO Threshold Setting Register

You can [`read`](crate::Reg::read) this register and get [`fcftr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcftr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FCFTR_SPEC;
impl crate::RegisterSpec for FCFTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fcftr::R`](R) reader structure
impl crate::Readable for FCFTR_SPEC {}
///`write(|w| ..)` method takes [`fcftr::W`](W) writer structure
impl crate::Writable for FCFTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCFTR to value 0x0007_0007
impl crate::Resettable for FCFTR_SPEC {
    const RESET_VALUE: u32 = 0x0007_0007;
}
