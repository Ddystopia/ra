///Register `SCKDIVCR2` reader
pub type R = crate::R<SCKDIVCR2_SPEC>;
///Register `SCKDIVCR2` writer
pub type W = crate::W<SCKDIVCR2_SPEC>;
/**USB Clock (UCLK) Select

Value on reset: 4*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCK_A {
    ///2: /3
    _010 = 2,
    ///3: /4
    _011 = 3,
    ///4: /5
    _100 = 4,
    ///0: Setting prohibited
    OTHERS = 0,
}
impl From<UCK_A> for u8 {
    #[inline(always)]
    fn from(variant: UCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCK_A {
    type Ux = u8;
}
impl crate::IsEnum for UCK_A {}
///Field `UCK` reader - USB Clock (UCLK) Select
pub type UCK_R = crate::FieldReader<UCK_A>;
impl UCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UCK_A {
        match self.bits {
            2 => UCK_A::_010,
            3 => UCK_A::_011,
            4 => UCK_A::_100,
            _ => UCK_A::OTHERS,
        }
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == UCK_A::_010
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == UCK_A::_011
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == UCK_A::_100
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), UCK_A::OTHERS)
    }
}
///Field `UCK` writer - USB Clock (UCLK) Select
pub type UCK_W<'a, REG> = crate::FieldWriter<'a, REG, 3, UCK_A, crate::Safe>;
impl<'a, REG> UCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/3"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(UCK_A::_010)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(UCK_A::_011)
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(UCK_A::_100)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(UCK_A::OTHERS)
    }
}
impl R {
    ///Bits 4:6 - USB Clock (UCLK) Select
    #[inline(always)]
    pub fn uck(&self) -> UCK_R {
        UCK_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    ///Bits 4:6 - USB Clock (UCLK) Select
    #[inline(always)]
    pub fn uck(&mut self) -> UCK_W<SCKDIVCR2_SPEC> {
        UCK_W::new(self, 4)
    }
}
/**System Clock Division Control Register 2

You can [`read`](crate::Reg::read) this register and get [`sckdivcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckdivcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCKDIVCR2_SPEC;
impl crate::RegisterSpec for SCKDIVCR2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sckdivcr2::R`](R) reader structure
impl crate::Readable for SCKDIVCR2_SPEC {}
///`write(|w| ..)` method takes [`sckdivcr2::W`](W) writer structure
impl crate::Writable for SCKDIVCR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCKDIVCR2 to value 0x40
impl crate::Resettable for SCKDIVCR2_SPEC {
    const RESET_VALUE: u8 = 0x40;
}
