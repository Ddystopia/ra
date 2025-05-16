///Register `PIPEMAXP` reader
pub type R = crate::R<PIPEMAXP_SPEC>;
///Register `PIPEMAXP` writer
pub type W = crate::W<PIPEMAXP_SPEC>;
/**Maximum Packet SizeThese bits specify the maximum data payload (maximum packet size) for the selected pipe.A size of 1h to 40h bytes can be set for PIPE6 to PIPE9.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum MXPS_A {
    ///0: Maximum packet size
    MXPS = 0,
}
impl From<MXPS_A> for u16 {
    #[inline(always)]
    fn from(variant: MXPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MXPS_A {
    type Ux = u16;
}
impl crate::IsEnum for MXPS_A {}
///Field `MXPS` reader - Maximum Packet SizeThese bits specify the maximum data payload (maximum packet size) for the selected pipe.A size of 1h to 40h bytes can be set for PIPE6 to PIPE9.
pub type MXPS_R = crate::FieldReader<MXPS_A>;
impl MXPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MXPS_A {
        match self.bits {
            _ => MXPS_A::MXPS,
        }
    }
    ///Maximum packet size
    #[inline(always)]
    pub fn is_mxps(&self) -> bool {
        matches!(self.variant(), MXPS_A::MXPS)
    }
}
///Field `MXPS` writer - Maximum Packet SizeThese bits specify the maximum data payload (maximum packet size) for the selected pipe.A size of 1h to 40h bytes can be set for PIPE6 to PIPE9.
pub type MXPS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, MXPS_A, crate::Safe>;
impl<'a, REG> MXPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Maximum packet size
    #[inline(always)]
    pub fn mxps(self) -> &'a mut crate::W<REG> {
        self.variant(MXPS_A::MXPS)
    }
}
/**Device SelectThese bits specify the address of the peripheral device when the host controller function is selected.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEVSEL_A {
    ///0: Address=DEVSEL
    DEVSEL = 0,
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
///Field `DEVSEL` reader - Device SelectThese bits specify the address of the peripheral device when the host controller function is selected.
pub type DEVSEL_R = crate::FieldReader<DEVSEL_A>;
impl DEVSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEVSEL_A {
        match self.bits {
            _ => DEVSEL_A::DEVSEL,
        }
    }
    ///Address=DEVSEL
    #[inline(always)]
    pub fn is_devsel(&self) -> bool {
        matches!(self.variant(), DEVSEL_A::DEVSEL)
    }
}
///Field `DEVSEL` writer - Device SelectThese bits specify the address of the peripheral device when the host controller function is selected.
pub type DEVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DEVSEL_A, crate::Safe>;
impl<'a, REG> DEVSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Address=DEVSEL
    #[inline(always)]
    pub fn devsel(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::DEVSEL)
    }
}
impl R {
    ///Bits 0:10 - Maximum Packet SizeThese bits specify the maximum data payload (maximum packet size) for the selected pipe.A size of 1h to 40h bytes can be set for PIPE6 to PIPE9.
    #[inline(always)]
    pub fn mxps(&self) -> MXPS_R {
        MXPS_R::new(self.bits & 0x07ff)
    }
    ///Bits 12:15 - Device SelectThese bits specify the address of the peripheral device when the host controller function is selected.
    #[inline(always)]
    pub fn devsel(&self) -> DEVSEL_R {
        DEVSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:10 - Maximum Packet SizeThese bits specify the maximum data payload (maximum packet size) for the selected pipe.A size of 1h to 40h bytes can be set for PIPE6 to PIPE9.
    #[inline(always)]
    pub fn mxps(&mut self) -> MXPS_W<PIPEMAXP_SPEC> {
        MXPS_W::new(self, 0)
    }
    ///Bits 12:15 - Device SelectThese bits specify the address of the peripheral device when the host controller function is selected.
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
