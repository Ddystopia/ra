///Register `DEVADD%s` reader
pub type R = crate::R<DEVADD_SPEC>;
///Register `DEVADD%s` writer
pub type W = crate::W<DEVADD_SPEC>;
/**Transfer Speed of Communication Target Device

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBSPD_A {
    ///0: DEVADDx is not used.
    _00 = 0,
    ///1: Low speed
    _01 = 1,
    ///2: Full speed
    _10 = 2,
    ///3: High speed
    _11 = 3,
}
impl From<USBSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: USBSPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USBSPD_A {
    type Ux = u8;
}
impl crate::IsEnum for USBSPD_A {}
///Field `USBSPD` reader - Transfer Speed of Communication Target Device
pub type USBSPD_R = crate::FieldReader<USBSPD_A>;
impl USBSPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBSPD_A {
        match self.bits {
            0 => USBSPD_A::_00,
            1 => USBSPD_A::_01,
            2 => USBSPD_A::_10,
            3 => USBSPD_A::_11,
            _ => unreachable!(),
        }
    }
    ///DEVADDx is not used.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == USBSPD_A::_00
    }
    ///Low speed
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == USBSPD_A::_01
    }
    ///Full speed
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == USBSPD_A::_10
    }
    ///High speed
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == USBSPD_A::_11
    }
}
///Field `USBSPD` writer - Transfer Speed of Communication Target Device
pub type USBSPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USBSPD_A, crate::Safe>;
impl<'a, REG> USBSPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///DEVADDx is not used.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(USBSPD_A::_00)
    }
    ///Low speed
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(USBSPD_A::_01)
    }
    ///Full speed
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(USBSPD_A::_10)
    }
    ///High speed
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(USBSPD_A::_11)
    }
}
/**Communication Target Connecting Hub Port

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HUBPORT_A {
    ///0: Directly connected to the port of the USBHS.
    _000 = 0,
    ///1: Port number of the hub
    OTHERS = 1,
}
impl From<HUBPORT_A> for u8 {
    #[inline(always)]
    fn from(variant: HUBPORT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HUBPORT_A {
    type Ux = u8;
}
impl crate::IsEnum for HUBPORT_A {}
///Field `HUBPORT` reader - Communication Target Connecting Hub Port
pub type HUBPORT_R = crate::FieldReader<HUBPORT_A>;
impl HUBPORT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HUBPORT_A {
        match self.bits {
            0 => HUBPORT_A::_000,
            _ => HUBPORT_A::OTHERS,
        }
    }
    ///Directly connected to the port of the USBHS.
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == HUBPORT_A::_000
    }
    ///Port number of the hub
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), HUBPORT_A::OTHERS)
    }
}
///Field `HUBPORT` writer - Communication Target Connecting Hub Port
pub type HUBPORT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, HUBPORT_A, crate::Safe>;
impl<'a, REG> HUBPORT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Directly connected to the port of the USBHS.
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(HUBPORT_A::_000)
    }
    ///Port number of the hub
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(HUBPORT_A::OTHERS)
    }
}
/**Communication Target Connecting Hub Register

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPPHUB_A {
    ///0: Directly connected to the port of the USBHS.
    _0000 = 0,
    ///1: USB address of the hub
    UPPHUB = 1,
}
impl From<UPPHUB_A> for u8 {
    #[inline(always)]
    fn from(variant: UPPHUB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UPPHUB_A {
    type Ux = u8;
}
impl crate::IsEnum for UPPHUB_A {}
///Field `UPPHUB` reader - Communication Target Connecting Hub Register
pub type UPPHUB_R = crate::FieldReader<UPPHUB_A>;
impl UPPHUB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UPPHUB_A {
        match self.bits {
            0 => UPPHUB_A::_0000,
            _ => UPPHUB_A::UPPHUB,
        }
    }
    ///Directly connected to the port of the USBHS.
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == UPPHUB_A::_0000
    }
    ///USB address of the hub
    #[inline(always)]
    pub fn is_upphub(&self) -> bool {
        matches!(self.variant(), UPPHUB_A::UPPHUB)
    }
}
///Field `UPPHUB` writer - Communication Target Connecting Hub Register
pub type UPPHUB_W<'a, REG> = crate::FieldWriter<'a, REG, 4, UPPHUB_A, crate::Safe>;
impl<'a, REG> UPPHUB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Directly connected to the port of the USBHS.
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(UPPHUB_A::_0000)
    }
    ///USB address of the hub
    #[inline(always)]
    pub fn upphub(self) -> &'a mut crate::W<REG> {
        self.variant(UPPHUB_A::UPPHUB)
    }
}
impl R {
    ///Bits 6:7 - Transfer Speed of Communication Target Device
    #[inline(always)]
    pub fn usbspd(&self) -> USBSPD_R {
        USBSPD_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:10 - Communication Target Connecting Hub Port
    #[inline(always)]
    pub fn hubport(&self) -> HUBPORT_R {
        HUBPORT_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:14 - Communication Target Connecting Hub Register
    #[inline(always)]
    pub fn upphub(&self) -> UPPHUB_R {
        UPPHUB_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 6:7 - Transfer Speed of Communication Target Device
    #[inline(always)]
    pub fn usbspd(&mut self) -> USBSPD_W<DEVADD_SPEC> {
        USBSPD_W::new(self, 6)
    }
    ///Bits 8:10 - Communication Target Connecting Hub Port
    #[inline(always)]
    pub fn hubport(&mut self) -> HUBPORT_W<DEVADD_SPEC> {
        HUBPORT_W::new(self, 8)
    }
    ///Bits 11:14 - Communication Target Connecting Hub Register
    #[inline(always)]
    pub fn upphub(&mut self) -> UPPHUB_W<DEVADD_SPEC> {
        UPPHUB_W::new(self, 11)
    }
}
/**Device Address Configuration Register

You can [`read`](crate::Reg::read) this register and get [`devadd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devadd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DEVADD_SPEC;
impl crate::RegisterSpec for DEVADD_SPEC {
    type Ux = u16;
}
///`read()` method returns [`devadd::R`](R) reader structure
impl crate::Readable for DEVADD_SPEC {}
///`write(|w| ..)` method takes [`devadd::W`](W) writer structure
impl crate::Writable for DEVADD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEVADD%s to value 0
impl crate::Resettable for DEVADD_SPEC {}
