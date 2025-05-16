///Register `USBADDR` reader
pub type R = crate::R<USBADDR_SPEC>;
///Register `USBADDR` writer
pub type W = crate::W<USBADDR_SPEC>;
///Field `USBADDR` reader - USB AddressWhen the function controller is selected, these bits indicate the USB address assigned by the host when the SET_ADDRESS request is successfully processed.
pub type USBADDR_R = crate::FieldReader;
///Field `USBADDR` writer - USB AddressWhen the function controller is selected, these bits indicate the USB address assigned by the host when the SET_ADDRESS request is successfully processed.
pub type USBADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Status Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STSRECOV_A {
    ///4: Return to the low-speed state (bits DVSTCTR0.RHST\[2:0\] = 001b;(Recovery when the host controller is selected))
    _0100 = 4,
    ///8: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b;(Recovery when the host controller is selected))
    _1000 = 8,
    ///9: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 001b (Default state);(Recovery when the function controller is selected)
    _1001 = 9,
    ///10: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 010b (Address state);(Recovery when the function controller is selected)
    _1010 = 10,
    ///11: Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 011b (Configured state);(Recovery when the function controller is selected)
    _1011 = 11,
    ///0: Settings other than above are prohibited.
    OTHERS = 0,
}
impl From<STSRECOV_A> for u8 {
    #[inline(always)]
    fn from(variant: STSRECOV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STSRECOV_A {
    type Ux = u8;
}
impl crate::IsEnum for STSRECOV_A {}
///Field `STSRECOV` reader - Status Recovery
pub type STSRECOV_R = crate::FieldReader<STSRECOV_A>;
impl STSRECOV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STSRECOV_A {
        match self.bits {
            4 => STSRECOV_A::_0100,
            8 => STSRECOV_A::_1000,
            9 => STSRECOV_A::_1001,
            10 => STSRECOV_A::_1010,
            11 => STSRECOV_A::_1011,
            _ => STSRECOV_A::OTHERS,
        }
    }
    ///Return to the low-speed state (bits DVSTCTR0.RHST\[2:0\] = 001b;(Recovery when the host controller is selected))
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == STSRECOV_A::_0100
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b;(Recovery when the host controller is selected))
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == STSRECOV_A::_1000
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 001b (Default state);(Recovery when the function controller is selected)
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == STSRECOV_A::_1001
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 010b (Address state);(Recovery when the function controller is selected)
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == STSRECOV_A::_1010
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 011b (Configured state);(Recovery when the function controller is selected)
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == STSRECOV_A::_1011
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), STSRECOV_A::OTHERS)
    }
}
///Field `STSRECOV` writer - Status Recovery
pub type STSRECOV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, STSRECOV_A, crate::Safe>;
impl<'a, REG> STSRECOV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Return to the low-speed state (bits DVSTCTR0.RHST\[2:0\] = 001b;(Recovery when the host controller is selected))
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV_A::_0100)
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b;(Recovery when the host controller is selected))
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV_A::_1000)
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 001b (Default state);(Recovery when the function controller is selected)
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV_A::_1001)
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 010b (Address state);(Recovery when the function controller is selected)
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV_A::_1010)
    }
    ///Return to the full-speed state (bits DVSTCTR0.RHST\[2:0\] = 010b), bits INTSTS0.DVSQ\[2:0\] = 011b (Configured state);(Recovery when the function controller is selected)
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV_A::_1011)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(STSRECOV_A::OTHERS)
    }
}
impl R {
    ///Bits 0:6 - USB AddressWhen the function controller is selected, these bits indicate the USB address assigned by the host when the SET_ADDRESS request is successfully processed.
    #[inline(always)]
    pub fn usbaddr(&self) -> USBADDR_R {
        USBADDR_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:11 - Status Recovery
    #[inline(always)]
    pub fn stsrecov(&self) -> STSRECOV_R {
        STSRECOV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:6 - USB AddressWhen the function controller is selected, these bits indicate the USB address assigned by the host when the SET_ADDRESS request is successfully processed.
    #[inline(always)]
    pub fn usbaddr(&mut self) -> USBADDR_W<USBADDR_SPEC> {
        USBADDR_W::new(self, 0)
    }
    ///Bits 8:11 - Status Recovery
    #[inline(always)]
    pub fn stsrecov(&mut self) -> STSRECOV_W<USBADDR_SPEC> {
        STSRECOV_W::new(self, 8)
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
