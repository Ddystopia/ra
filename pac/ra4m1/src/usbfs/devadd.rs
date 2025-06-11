///Register `DEVADD%s` reader
pub type R = crate::R<DEVADD_SPEC>;
///Register `DEVADD%s` writer
pub type W = crate::W<DEVADD_SPEC>;
/**Transfer Speed of Communication Target Device

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBSPD_A {
    ///0: DEVADDn is not used
    _00 = 0,
    ///1: Low speed
    _01 = 1,
    ///2: Full speed
    _10 = 2,
    ///3: Setting prohibited
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
    ///DEVADDn is not used
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
    ///Setting prohibited
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
    ///DEVADDn is not used
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
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(USBSPD_A::_11)
    }
}
impl R {
    ///Bits 6:7 - Transfer Speed of Communication Target Device
    #[inline(always)]
    pub fn usbspd(&self) -> USBSPD_R {
        USBSPD_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    ///Bits 6:7 - Transfer Speed of Communication Target Device
    #[inline(always)]
    pub fn usbspd(&mut self) -> USBSPD_W<DEVADD_SPEC> {
        USBSPD_W::new(self, 6)
    }
}
/**Device Address %s Configuration Register

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
