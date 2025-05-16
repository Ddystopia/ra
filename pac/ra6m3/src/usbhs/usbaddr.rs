///Register `USBADDR` reader
pub type R = crate::R<USBADDR_SPEC>;
///Register `USBADDR` writer
pub type W = crate::W<USBADDR_SPEC>;
///Field `USBADDR` reader - USB Address In device controller mode, these flags indicate the USB address assigned by the host when the USBHS processed the SET_ADDRESS request successfully.
pub type USBADDR_R = crate::FieldReader;
/**Status Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STSRECOV0_A {
    ///1: Return to the full-speed state(bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 001b (Default state)(function controller selected)
    _001 = 1,
    ///2: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 010b (Address state)(function controller selected)/ Return to the low-speed state (bitsDVSTCTR0.RHST\[2:0\] = 001b)(host controller is selected)
    _010 = 2,
    ///3: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 011b (Configured state)(function controller selected)
    _011 = 3,
    ///4: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b)(host controller selected)
    _100 = 4,
    ///5: Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b), bits INTSTS0.DVSQ\[2:0\] = 001b (Default state)(function controller selected)
    _101 = 5,
    ///6: Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b), bits INTSTS0.DVSQ\[2:0\] = 010b (Address state)(function controller selected)/ Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b)(host controller selected)
    _110 = 6,
    ///7: Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b), bits INTSTS0.DVSQ\[2:0\] = 011b (Configured state)(function controller selected)
    _111 = 7,
    ///0: Setting prohibited.
    OTHERS = 0,
}
impl From<STSRECOV0_A> for u8 {
    #[inline(always)]
    fn from(variant: STSRECOV0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STSRECOV0_A {
    type Ux = u8;
}
impl crate::IsEnum for STSRECOV0_A {}
///Field `STSRECOV0` reader - Status Recovery
pub type STSRECOV0_R = crate::FieldReader<STSRECOV0_A>;
impl STSRECOV0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STSRECOV0_A {
        match self.bits {
            1 => STSRECOV0_A::_001,
            2 => STSRECOV0_A::_010,
            3 => STSRECOV0_A::_011,
            4 => STSRECOV0_A::_100,
            5 => STSRECOV0_A::_101,
            6 => STSRECOV0_A::_110,
            7 => STSRECOV0_A::_111,
            0 => STSRECOV0_A::OTHERS,
            _ => unreachable!(),
        }
    }
    ///Return to the full-speed state(bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 001b (Default state)(function controller selected)
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == STSRECOV0_A::_001
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 010b (Address state)(function controller selected)/ Return to the low-speed state (bitsDVSTCTR0.RHST\[2:0\] = 001b)(host controller is selected)
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == STSRECOV0_A::_010
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 011b (Configured state)(function controller selected)
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == STSRECOV0_A::_011
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b)(host controller selected)
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == STSRECOV0_A::_100
    }
    ///Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b), bits INTSTS0.DVSQ\[2:0\] = 001b (Default state)(function controller selected)
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == STSRECOV0_A::_101
    }
    ///Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b), bits INTSTS0.DVSQ\[2:0\] = 010b (Address state)(function controller selected)/ Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b)(host controller selected)
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == STSRECOV0_A::_110
    }
    ///Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b), bits INTSTS0.DVSQ\[2:0\] = 011b (Configured state)(function controller selected)
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == STSRECOV0_A::_111
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == STSRECOV0_A::OTHERS
    }
}
///Field `STSRECOV0` writer - Status Recovery
pub type STSRECOV0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, STSRECOV0_A, crate::Safe>;
impl<'a, REG> STSRECOV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Return to the full-speed state(bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 001b (Default state)(function controller selected)
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV0_A::_001)
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 010b (Address state)(function controller selected)/ Return to the low-speed state (bitsDVSTCTR0.RHST\[2:0\] = 001b)(host controller is selected)
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV0_A::_010)
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 011b (Configured state)(function controller selected)
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV0_A::_011)
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b)(host controller selected)
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV0_A::_100)
    }
    ///Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b), bits INTSTS0.DVSQ\[2:0\] = 001b (Default state)(function controller selected)
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV0_A::_101)
    }
    ///Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b), bits INTSTS0.DVSQ\[2:0\] = 010b (Address state)(function controller selected)/ Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b)(host controller selected)
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV0_A::_110)
    }
    ///Return to the high-speed state (bits DVSTCTR0.RHST\[2:0\] = 011b), bits INTSTS0.DVSQ\[2:0\] = 011b (Configured state)(function controller selected)
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV0_A::_111)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV0_A::OTHERS)
    }
}
impl R {
    ///Bits 0:6 - USB Address In device controller mode, these flags indicate the USB address assigned by the host when the USBHS processed the SET_ADDRESS request successfully.
    #[inline(always)]
    pub fn usbaddr(&self) -> USBADDR_R {
        USBADDR_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:10 - Status Recovery
    #[inline(always)]
    pub fn stsrecov0(&self) -> STSRECOV0_R {
        STSRECOV0_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    ///Bits 8:10 - Status Recovery
    #[inline(always)]
    pub fn stsrecov0(&mut self) -> STSRECOV0_W<USBADDR_SPEC> {
        STSRECOV0_W::new(self, 8)
    }
}
/**USB Address Register

You can [`read`](crate::Reg::read) this register and get [`usbaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct USBADDR_SPEC;
impl crate::RegisterSpec for USBADDR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`usbaddr::R`](R) reader structure
impl crate::Readable for USBADDR_SPEC {}
///`write(|w| ..)` method takes [`usbaddr::W`](W) writer structure
impl crate::Writable for USBADDR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBADDR to value 0
impl crate::Resettable for USBADDR_SPEC {}
