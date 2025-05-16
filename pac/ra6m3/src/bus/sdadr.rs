///Register `SDADR` reader
pub type R = crate::R<SDADR_SPEC>;
///Register `SDADR` writer
pub type W = crate::W<SDADR_SPEC>;
/**Address Multiplex Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MXC_A {
    ///0: 8-bit shift
    _00 = 0,
    ///1: 9-bit shift
    _01 = 1,
    ///2: 10-bit shift
    _10 = 2,
    ///3: 11-bit shift
    _11 = 3,
}
impl From<MXC_A> for u8 {
    #[inline(always)]
    fn from(variant: MXC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MXC_A {
    type Ux = u8;
}
impl crate::IsEnum for MXC_A {}
///Field `MXC` reader - Address Multiplex Select
pub type MXC_R = crate::FieldReader<MXC_A>;
impl MXC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MXC_A {
        match self.bits {
            0 => MXC_A::_00,
            1 => MXC_A::_01,
            2 => MXC_A::_10,
            3 => MXC_A::_11,
            _ => unreachable!(),
        }
    }
    ///8-bit shift
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MXC_A::_00
    }
    ///9-bit shift
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MXC_A::_01
    }
    ///10-bit shift
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MXC_A::_10
    }
    ///11-bit shift
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MXC_A::_11
    }
}
///Field `MXC` writer - Address Multiplex Select
pub type MXC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MXC_A, crate::Safe>;
impl<'a, REG> MXC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8-bit shift
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(MXC_A::_00)
    }
    ///9-bit shift
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(MXC_A::_01)
    }
    ///10-bit shift
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(MXC_A::_10)
    }
    ///11-bit shift
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(MXC_A::_11)
    }
}
impl R {
    ///Bits 0:1 - Address Multiplex Select
    #[inline(always)]
    pub fn mxc(&self) -> MXC_R {
        MXC_R::new(self.bits & 3)
    }
}
impl W {
    ///Bits 0:1 - Address Multiplex Select
    #[inline(always)]
    pub fn mxc(&mut self) -> MXC_W<SDADR_SPEC> {
        MXC_W::new(self, 0)
    }
}
/**SDRAM Address Register

You can [`read`](crate::Reg::read) this register and get [`sdadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDADR_SPEC;
impl crate::RegisterSpec for SDADR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sdadr::R`](R) reader structure
impl crate::Readable for SDADR_SPEC {}
///`write(|w| ..)` method takes [`sdadr::W`](W) writer structure
impl crate::Writable for SDADR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDADR to value 0
impl crate::Resettable for SDADR_SPEC {}
