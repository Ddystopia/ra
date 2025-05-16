///Register `SMPUFBIU` reader
pub type R = crate::R<SMPUFBIU_SPEC>;
///Register `SMPUFBIU` writer
pub type W = crate::W<SMPUFBIU_SPEC>;
/**CPU Read protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP_CPU_A {
    ///0: CPU read of memory protection is disabled.
    _0 = 0,
    ///1: CPU read of memory protection is enabled.
    _1 = 1,
}
impl From<RP_CPU_A> for bool {
    #[inline(always)]
    fn from(variant: RP_CPU_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RP_CPU` reader - CPU Read protection
pub type RP_CPU_R = crate::BitReader<RP_CPU_A>;
impl RP_CPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RP_CPU_A {
        match self.bits {
            false => RP_CPU_A::_0,
            true => RP_CPU_A::_1,
        }
    }
    ///CPU read of memory protection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RP_CPU_A::_0
    }
    ///CPU read of memory protection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RP_CPU_A::_1
    }
}
///Field `RP_CPU` writer - CPU Read protection
pub type RP_CPU_W<'a, REG> = crate::BitWriter<'a, REG, RP_CPU_A>;
impl<'a, REG> RP_CPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CPU read of memory protection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RP_CPU_A::_0)
    }
    ///CPU read of memory protection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RP_CPU_A::_1)
    }
}
/**CPU Write protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_CPU_A {
    ///0: CPU write of memory protection is disabled.
    _0 = 0,
    ///1: CPU write of memory protection is enabled.
    _1 = 1,
}
impl From<WP_CPU_A> for bool {
    #[inline(always)]
    fn from(variant: WP_CPU_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WP_CPU` reader - CPU Write protection
pub type WP_CPU_R = crate::BitReader<WP_CPU_A>;
impl WP_CPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP_CPU_A {
        match self.bits {
            false => WP_CPU_A::_0,
            true => WP_CPU_A::_1,
        }
    }
    ///CPU write of memory protection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_CPU_A::_0
    }
    ///CPU write of memory protection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_CPU_A::_1
    }
}
///Field `WP_CPU` writer - CPU Write protection
pub type WP_CPU_W<'a, REG> = crate::BitWriter<'a, REG, WP_CPU_A>;
impl<'a, REG> WP_CPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CPU write of memory protection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WP_CPU_A::_0)
    }
    ///CPU write of memory protection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WP_CPU_A::_1)
    }
}
/**Master Group A Read protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP_GRPA_A {
    ///0: Master group A read of memory protection is disabled.
    _0 = 0,
    ///1: Master group A read of memory protection is enabled.
    _1 = 1,
}
impl From<RP_GRPA_A> for bool {
    #[inline(always)]
    fn from(variant: RP_GRPA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RP_GRPA` reader - Master Group A Read protection
pub type RP_GRPA_R = crate::BitReader<RP_GRPA_A>;
impl RP_GRPA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RP_GRPA_A {
        match self.bits {
            false => RP_GRPA_A::_0,
            true => RP_GRPA_A::_1,
        }
    }
    ///Master group A read of memory protection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RP_GRPA_A::_0
    }
    ///Master group A read of memory protection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RP_GRPA_A::_1
    }
}
///Field `RP_GRPA` writer - Master Group A Read protection
pub type RP_GRPA_W<'a, REG> = crate::BitWriter<'a, REG, RP_GRPA_A>;
impl<'a, REG> RP_GRPA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master group A read of memory protection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RP_GRPA_A::_0)
    }
    ///Master group A read of memory protection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RP_GRPA_A::_1)
    }
}
/**Master Group A Write protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_GRPA_A {
    ///0: Master group A write of memory protection is disabled.
    _0 = 0,
    ///1: Master group A write of memory protection is enabled.
    _1 = 1,
}
impl From<WP_GRPA_A> for bool {
    #[inline(always)]
    fn from(variant: WP_GRPA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WP_GRPA` reader - Master Group A Write protection
pub type WP_GRPA_R = crate::BitReader<WP_GRPA_A>;
impl WP_GRPA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP_GRPA_A {
        match self.bits {
            false => WP_GRPA_A::_0,
            true => WP_GRPA_A::_1,
        }
    }
    ///Master group A write of memory protection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_GRPA_A::_0
    }
    ///Master group A write of memory protection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_GRPA_A::_1
    }
}
///Field `WP_GRPA` writer - Master Group A Write protection
pub type WP_GRPA_W<'a, REG> = crate::BitWriter<'a, REG, WP_GRPA_A>;
impl<'a, REG> WP_GRPA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master group A write of memory protection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WP_GRPA_A::_0)
    }
    ///Master group A write of memory protection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WP_GRPA_A::_1)
    }
}
/**Master Group B Read protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP_GRPB_A {
    ///0: Master group B read of memory protection is disabled.
    _0 = 0,
    ///1: Master group B read of memory protection is enabled.
    _1 = 1,
}
impl From<RP_GRPB_A> for bool {
    #[inline(always)]
    fn from(variant: RP_GRPB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RP_GRPB` reader - Master Group B Read protection
pub type RP_GRPB_R = crate::BitReader<RP_GRPB_A>;
impl RP_GRPB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RP_GRPB_A {
        match self.bits {
            false => RP_GRPB_A::_0,
            true => RP_GRPB_A::_1,
        }
    }
    ///Master group B read of memory protection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RP_GRPB_A::_0
    }
    ///Master group B read of memory protection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RP_GRPB_A::_1
    }
}
///Field `RP_GRPB` writer - Master Group B Read protection
pub type RP_GRPB_W<'a, REG> = crate::BitWriter<'a, REG, RP_GRPB_A>;
impl<'a, REG> RP_GRPB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master group B read of memory protection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RP_GRPB_A::_0)
    }
    ///Master group B read of memory protection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RP_GRPB_A::_1)
    }
}
/**Master Group B Write protection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_GRPB_A {
    ///0: Master group B write of memory protection is disabled.
    _0 = 0,
    ///1: Master group B write of memory protection is enabled.
    _1 = 1,
}
impl From<WP_GRPB_A> for bool {
    #[inline(always)]
    fn from(variant: WP_GRPB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WP_GRPB` reader - Master Group B Write protection
pub type WP_GRPB_R = crate::BitReader<WP_GRPB_A>;
impl WP_GRPB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP_GRPB_A {
        match self.bits {
            false => WP_GRPB_A::_0,
            true => WP_GRPB_A::_1,
        }
    }
    ///Master group B write of memory protection is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_GRPB_A::_0
    }
    ///Master group B write of memory protection is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_GRPB_A::_1
    }
}
///Field `WP_GRPB` writer - Master Group B Write protection
pub type WP_GRPB_W<'a, REG> = crate::BitWriter<'a, REG, WP_GRPB_A>;
impl<'a, REG> WP_GRPB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master group B write of memory protection is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WP_GRPB_A::_0)
    }
    ///Master group B write of memory protection is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WP_GRPB_A::_1)
    }
}
/**Master Group C Read protection

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP_GRPC_A {
    ///0: Setting prohibited
    _0 = 0,
    ///1: Master group C read of memory protection is enabled. The write value should always be 1.
    _1 = 1,
}
impl From<RP_GRPC_A> for bool {
    #[inline(always)]
    fn from(variant: RP_GRPC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RP_GRPC` reader - Master Group C Read protection
pub type RP_GRPC_R = crate::BitReader<RP_GRPC_A>;
impl RP_GRPC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RP_GRPC_A {
        match self.bits {
            false => RP_GRPC_A::_0,
            true => RP_GRPC_A::_1,
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RP_GRPC_A::_0
    }
    ///Master group C read of memory protection is enabled. The write value should always be 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RP_GRPC_A::_1
    }
}
///Field `RP_GRPC` writer - Master Group C Read protection
pub type RP_GRPC_W<'a, REG> = crate::BitWriter<'a, REG, RP_GRPC_A>;
impl<'a, REG> RP_GRPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RP_GRPC_A::_0)
    }
    ///Master group C read of memory protection is enabled. The write value should always be 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RP_GRPC_A::_1)
    }
}
/**Master Group C Write protection

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_GRPC_A {
    ///0: Setting prohibited
    _0 = 0,
    ///1: Master group C write of memory protection is enabled. The write value should always be 1.
    _1 = 1,
}
impl From<WP_GRPC_A> for bool {
    #[inline(always)]
    fn from(variant: WP_GRPC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WP_GRPC` reader - Master Group C Write protection
pub type WP_GRPC_R = crate::BitReader<WP_GRPC_A>;
impl WP_GRPC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WP_GRPC_A {
        match self.bits {
            false => WP_GRPC_A::_0,
            true => WP_GRPC_A::_1,
        }
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_GRPC_A::_0
    }
    ///Master group C write of memory protection is enabled. The write value should always be 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_GRPC_A::_1
    }
}
///Field `WP_GRPC` writer - Master Group C Write protection
pub type WP_GRPC_W<'a, REG> = crate::BitWriter<'a, REG, WP_GRPC_A>;
impl<'a, REG> WP_GRPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting prohibited
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WP_GRPC_A::_0)
    }
    ///Master group C write of memory protection is enabled. The write value should always be 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WP_GRPC_A::_1)
    }
}
impl R {
    ///Bit 0 - CPU Read protection
    #[inline(always)]
    pub fn rp_cpu(&self) -> RP_CPU_R {
        RP_CPU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU Write protection
    #[inline(always)]
    pub fn wp_cpu(&self) -> WP_CPU_R {
        WP_CPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Master Group A Read protection
    #[inline(always)]
    pub fn rp_grpa(&self) -> RP_GRPA_R {
        RP_GRPA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Master Group A Write protection
    #[inline(always)]
    pub fn wp_grpa(&self) -> WP_GRPA_R {
        WP_GRPA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Master Group B Read protection
    #[inline(always)]
    pub fn rp_grpb(&self) -> RP_GRPB_R {
        RP_GRPB_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Master Group B Write protection
    #[inline(always)]
    pub fn wp_grpb(&self) -> WP_GRPB_R {
        WP_GRPB_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Master Group C Read protection
    #[inline(always)]
    pub fn rp_grpc(&self) -> RP_GRPC_R {
        RP_GRPC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Master Group C Write protection
    #[inline(always)]
    pub fn wp_grpc(&self) -> WP_GRPC_R {
        WP_GRPC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CPU Read protection
    #[inline(always)]
    pub fn rp_cpu(&mut self) -> RP_CPU_W<SMPUFBIU_SPEC> {
        RP_CPU_W::new(self, 0)
    }
    ///Bit 1 - CPU Write protection
    #[inline(always)]
    pub fn wp_cpu(&mut self) -> WP_CPU_W<SMPUFBIU_SPEC> {
        WP_CPU_W::new(self, 1)
    }
    ///Bit 2 - Master Group A Read protection
    #[inline(always)]
    pub fn rp_grpa(&mut self) -> RP_GRPA_W<SMPUFBIU_SPEC> {
        RP_GRPA_W::new(self, 2)
    }
    ///Bit 3 - Master Group A Write protection
    #[inline(always)]
    pub fn wp_grpa(&mut self) -> WP_GRPA_W<SMPUFBIU_SPEC> {
        WP_GRPA_W::new(self, 3)
    }
    ///Bit 4 - Master Group B Read protection
    #[inline(always)]
    pub fn rp_grpb(&mut self) -> RP_GRPB_W<SMPUFBIU_SPEC> {
        RP_GRPB_W::new(self, 4)
    }
    ///Bit 5 - Master Group B Write protection
    #[inline(always)]
    pub fn wp_grpb(&mut self) -> WP_GRPB_W<SMPUFBIU_SPEC> {
        WP_GRPB_W::new(self, 5)
    }
    ///Bit 6 - Master Group C Read protection
    #[inline(always)]
    pub fn rp_grpc(&mut self) -> RP_GRPC_W<SMPUFBIU_SPEC> {
        RP_GRPC_W::new(self, 6)
    }
    ///Bit 7 - Master Group C Write protection
    #[inline(always)]
    pub fn wp_grpc(&mut self) -> WP_GRPC_W<SMPUFBIU_SPEC> {
        WP_GRPC_W::new(self, 7)
    }
}
/**Access Control Register for FBIU

You can [`read`](crate::Reg::read) this register and get [`smpufbiu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpufbiu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SMPUFBIU_SPEC;
impl crate::RegisterSpec for SMPUFBIU_SPEC {
    type Ux = u16;
}
///`read()` method returns [`smpufbiu::R`](R) reader structure
impl crate::Readable for SMPUFBIU_SPEC {}
///`write(|w| ..)` method takes [`smpufbiu::W`](W) writer structure
impl crate::Writable for SMPUFBIU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPUFBIU to value 0xc0
impl crate::Resettable for SMPUFBIU_SPEC {
    const RESET_VALUE: u16 = 0xc0;
}
