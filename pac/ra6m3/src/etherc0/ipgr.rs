///Register `IPGR` reader
pub type R = crate::R<IPGR_SPEC>;
///Register `IPGR` writer
pub type W = crate::W<IPGR_SPEC>;
/**Interpacket Gap Range:"16bit time(0x00)"-"140bit time(0x1F)"

Value on reset: 20*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IPG_A {
    ///20: 96 bit time (initial value)
    _14H = 20,
    ///0: (IPGx4+16) bit time
    OTHERS = 0,
}
impl From<IPG_A> for u8 {
    #[inline(always)]
    fn from(variant: IPG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IPG_A {
    type Ux = u8;
}
impl crate::IsEnum for IPG_A {}
///Field `IPG` reader - Interpacket Gap Range:"16bit time(0x00)"-"140bit time(0x1F)"
pub type IPG_R = crate::FieldReader<IPG_A>;
impl IPG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IPG_A {
        match self.bits {
            20 => IPG_A::_14H,
            _ => IPG_A::OTHERS,
        }
    }
    ///96 bit time (initial value)
    #[inline(always)]
    pub fn is_14h(&self) -> bool {
        *self == IPG_A::_14H
    }
    ///(IPGx4+16) bit time
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), IPG_A::OTHERS)
    }
}
///Field `IPG` writer - Interpacket Gap Range:"16bit time(0x00)"-"140bit time(0x1F)"
pub type IPG_W<'a, REG> = crate::FieldWriter<'a, REG, 5, IPG_A, crate::Safe>;
impl<'a, REG> IPG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///96 bit time (initial value)
    #[inline(always)]
    pub fn _14h(self) -> &'a mut crate::W<REG> {
        self.variant(IPG_A::_14H)
    }
    ///(IPGx4+16) bit time
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(IPG_A::OTHERS)
    }
}
impl R {
    ///Bits 0:4 - Interpacket Gap Range:"16bit time(0x00)"-"140bit time(0x1F)"
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - Interpacket Gap Range:"16bit time(0x00)"-"140bit time(0x1F)"
    #[inline(always)]
    pub fn ipg(&mut self) -> IPG_W<IPGR_SPEC> {
        IPG_W::new(self, 0)
    }
}
/**IPG Register

You can [`read`](crate::Reg::read) this register and get [`ipgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IPGR_SPEC;
impl crate::RegisterSpec for IPGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ipgr::R`](R) reader structure
impl crate::Readable for IPGR_SPEC {}
///`write(|w| ..)` method takes [`ipgr::W`](W) writer structure
impl crate::Writable for IPGR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IPGR to value 0x14
impl crate::Resettable for IPGR_SPEC {
    const RESET_VALUE: u32 = 0x14;
}
