///Register `FIDCR%s` reader
pub type R = crate::R<FIDCR_SPEC>;
///Register `FIDCR%s` writer
pub type W = crate::W<FIDCR_SPEC>;
///Field `EID` reader - Extended ID
pub type EID_R = crate::FieldReader<u32>;
///Field `EID` writer - Extended ID
pub type EID_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
///Field `SID` reader - Standard ID
pub type SID_R = crate::FieldReader<u16>;
///Field `SID` writer - Standard ID
pub type SID_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
/**Remote Transmission Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTR_A {
    ///0: Data frame
    _0 = 0,
    ///1: Remote frame
    _1 = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RTR` reader - Remote Transmission Request
pub type RTR_R = crate::BitReader<RTR_A>;
impl RTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::_0,
            true => RTR_A::_1,
        }
    }
    ///Data frame
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTR_A::_0
    }
    ///Remote frame
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTR_A::_1
    }
}
///Field `RTR` writer - Remote Transmission Request
pub type RTR_W<'a, REG> = crate::BitWriter<'a, REG, RTR_A>;
impl<'a, REG> RTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data frame
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RTR_A::_0)
    }
    ///Remote frame
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RTR_A::_1)
    }
}
/**ID Extension

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDE_A {
    ///0: Standard ID
    _0 = 0,
    ///1: Extended ID
    _1 = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IDE` reader - ID Extension
pub type IDE_R = crate::BitReader<IDE_A>;
impl IDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::_0,
            true => IDE_A::_1,
        }
    }
    ///Standard ID
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDE_A::_0
    }
    ///Extended ID
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDE_A::_1
    }
}
///Field `IDE` writer - ID Extension
pub type IDE_W<'a, REG> = crate::BitWriter<'a, REG, IDE_A>;
impl<'a, REG> IDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Standard ID
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IDE_A::_0)
    }
    ///Extended ID
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IDE_A::_1)
    }
}
impl R {
    ///Bits 0:17 - Extended ID
    #[inline(always)]
    pub fn eid(&self) -> EID_R {
        EID_R::new(self.bits & 0x0003_ffff)
    }
    ///Bits 18:28 - Standard ID
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    ///Bit 30 - Remote Transmission Request
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ID Extension
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:17 - Extended ID
    #[inline(always)]
    pub fn eid(&mut self) -> EID_W<FIDCR_SPEC> {
        EID_W::new(self, 0)
    }
    ///Bits 18:28 - Standard ID
    #[inline(always)]
    pub fn sid(&mut self) -> SID_W<FIDCR_SPEC> {
        SID_W::new(self, 18)
    }
    ///Bit 30 - Remote Transmission Request
    #[inline(always)]
    pub fn rtr(&mut self) -> RTR_W<FIDCR_SPEC> {
        RTR_W::new(self, 30)
    }
    ///Bit 31 - ID Extension
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W<FIDCR_SPEC> {
        IDE_W::new(self, 31)
    }
}
/**FIFO Received ID Compare Registers

You can [`read`](crate::Reg::read) this register and get [`fidcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fidcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FIDCR_SPEC;
impl crate::RegisterSpec for FIDCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fidcr::R`](R) reader structure
impl crate::Readable for FIDCR_SPEC {}
///`write(|w| ..)` method takes [`fidcr::W`](W) writer structure
impl crate::Writable for FIDCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIDCR%s to value 0
impl crate::Resettable for FIDCR_SPEC {}
