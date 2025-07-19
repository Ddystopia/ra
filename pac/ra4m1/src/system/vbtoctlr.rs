///Register `VBTOCTLR` reader
pub type R = crate::R<VBTOCTLR_SPEC>;
///Register `VBTOCTLR` writer
pub type W = crate::W<VBTOCTLR_SPEC>;
/**VBATT Wakeup I/O 0 Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH0OEN_A {
    ///0: VBATWIO0 output disabled
    _0 = 0,
    ///1: VBATWIO0 output enabled
    _1 = 1,
}
impl From<VCH0OEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH0OEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VCH0OEN` reader - VBATT Wakeup I/O 0 Output Enable
pub type VCH0OEN_R = crate::BitReader<VCH0OEN_A>;
impl VCH0OEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VCH0OEN_A {
        match self.bits {
            false => VCH0OEN_A::_0,
            true => VCH0OEN_A::_1,
        }
    }
    ///VBATWIO0 output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH0OEN_A::_0
    }
    ///VBATWIO0 output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH0OEN_A::_1
    }
}
///Field `VCH0OEN` writer - VBATT Wakeup I/O 0 Output Enable
pub type VCH0OEN_W<'a, REG> = crate::BitWriter<'a, REG, VCH0OEN_A>;
impl<'a, REG> VCH0OEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATWIO0 output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0OEN_A::_0)
    }
    ///VBATWIO0 output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0OEN_A::_1)
    }
}
/**VBATT Wakeup I/O 1 Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH1OEN_A {
    ///0: VBATWIO1 output disabled
    _0 = 0,
    ///1: VBATWIO1 output enabled
    _1 = 1,
}
impl From<VCH1OEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH1OEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VCH1OEN` reader - VBATT Wakeup I/O 1 Output Enable
pub type VCH1OEN_R = crate::BitReader<VCH1OEN_A>;
impl VCH1OEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VCH1OEN_A {
        match self.bits {
            false => VCH1OEN_A::_0,
            true => VCH1OEN_A::_1,
        }
    }
    ///VBATWIO1 output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH1OEN_A::_0
    }
    ///VBATWIO1 output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH1OEN_A::_1
    }
}
///Field `VCH1OEN` writer - VBATT Wakeup I/O 1 Output Enable
pub type VCH1OEN_W<'a, REG> = crate::BitWriter<'a, REG, VCH1OEN_A>;
impl<'a, REG> VCH1OEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATWIO1 output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1OEN_A::_0)
    }
    ///VBATWIO1 output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1OEN_A::_1)
    }
}
/**VBATT Wakeup I/O 2 Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH2OEN_A {
    ///0: VBATWIO2 output disabled
    _0 = 0,
    ///1: VBATWIO2 output enabled
    _1 = 1,
}
impl From<VCH2OEN_A> for bool {
    #[inline(always)]
    fn from(variant: VCH2OEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VCH2OEN` reader - VBATT Wakeup I/O 2 Output Enable
pub type VCH2OEN_R = crate::BitReader<VCH2OEN_A>;
impl VCH2OEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VCH2OEN_A {
        match self.bits {
            false => VCH2OEN_A::_0,
            true => VCH2OEN_A::_1,
        }
    }
    ///VBATWIO2 output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH2OEN_A::_0
    }
    ///VBATWIO2 output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH2OEN_A::_1
    }
}
///Field `VCH2OEN` writer - VBATT Wakeup I/O 2 Output Enable
pub type VCH2OEN_W<'a, REG> = crate::BitWriter<'a, REG, VCH2OEN_A>;
impl<'a, REG> VCH2OEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATWIO2 output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2OEN_A::_0)
    }
    ///VBATWIO2 output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2OEN_A::_1)
    }
}
/**VBATT Wakeup I/O 0 Output Level Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOUT0LSEL_A {
    ///0: Output L before VBATT wakeup trigger
    _0 = 0,
    ///1: Output H before VBATT wakeup trigger
    _1 = 1,
}
impl From<VOUT0LSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VOUT0LSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VOUT0LSEL` reader - VBATT Wakeup I/O 0 Output Level Selection
pub type VOUT0LSEL_R = crate::BitReader<VOUT0LSEL_A>;
impl VOUT0LSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VOUT0LSEL_A {
        match self.bits {
            false => VOUT0LSEL_A::_0,
            true => VOUT0LSEL_A::_1,
        }
    }
    ///Output L before VBATT wakeup trigger
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VOUT0LSEL_A::_0
    }
    ///Output H before VBATT wakeup trigger
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VOUT0LSEL_A::_1
    }
}
///Field `VOUT0LSEL` writer - VBATT Wakeup I/O 0 Output Level Selection
pub type VOUT0LSEL_W<'a, REG> = crate::BitWriter<'a, REG, VOUT0LSEL_A>;
impl<'a, REG> VOUT0LSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output L before VBATT wakeup trigger
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VOUT0LSEL_A::_0)
    }
    ///Output H before VBATT wakeup trigger
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VOUT0LSEL_A::_1)
    }
}
/**VBATT Wakeup I/O 1 Output Level Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOUT1LSEL_A {
    ///0: Output L before VBATT wake up trigger
    _0 = 0,
    ///1: Output H before VBATT wake up trigger
    _1 = 1,
}
impl From<VOUT1LSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VOUT1LSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VOUT1LSEL` reader - VBATT Wakeup I/O 1 Output Level Selection
pub type VOUT1LSEL_R = crate::BitReader<VOUT1LSEL_A>;
impl VOUT1LSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VOUT1LSEL_A {
        match self.bits {
            false => VOUT1LSEL_A::_0,
            true => VOUT1LSEL_A::_1,
        }
    }
    ///Output L before VBATT wake up trigger
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VOUT1LSEL_A::_0
    }
    ///Output H before VBATT wake up trigger
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VOUT1LSEL_A::_1
    }
}
///Field `VOUT1LSEL` writer - VBATT Wakeup I/O 1 Output Level Selection
pub type VOUT1LSEL_W<'a, REG> = crate::BitWriter<'a, REG, VOUT1LSEL_A>;
impl<'a, REG> VOUT1LSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output L before VBATT wake up trigger
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VOUT1LSEL_A::_0)
    }
    ///Output H before VBATT wake up trigger
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VOUT1LSEL_A::_1)
    }
}
/**VBATT Wakeup I/O 2 Output Level Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOUT2LSEL_A {
    ///0: Output L before VBATT wake up trigger
    _0 = 0,
    ///1: Output H before VBATT wake up trigger
    _1 = 1,
}
impl From<VOUT2LSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VOUT2LSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VOUT2LSEL` reader - VBATT Wakeup I/O 2 Output Level Selection
pub type VOUT2LSEL_R = crate::BitReader<VOUT2LSEL_A>;
impl VOUT2LSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VOUT2LSEL_A {
        match self.bits {
            false => VOUT2LSEL_A::_0,
            true => VOUT2LSEL_A::_1,
        }
    }
    ///Output L before VBATT wake up trigger
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VOUT2LSEL_A::_0
    }
    ///Output H before VBATT wake up trigger
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VOUT2LSEL_A::_1
    }
}
///Field `VOUT2LSEL` writer - VBATT Wakeup I/O 2 Output Level Selection
pub type VOUT2LSEL_W<'a, REG> = crate::BitWriter<'a, REG, VOUT2LSEL_A>;
impl<'a, REG> VOUT2LSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output L before VBATT wake up trigger
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VOUT2LSEL_A::_0)
    }
    ///Output H before VBATT wake up trigger
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VOUT2LSEL_A::_1)
    }
}
impl R {
    ///Bit 0 - VBATT Wakeup I/O 0 Output Enable
    #[inline(always)]
    pub fn vch0oen(&self) -> VCH0OEN_R {
        VCH0OEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VBATT Wakeup I/O 1 Output Enable
    #[inline(always)]
    pub fn vch1oen(&self) -> VCH1OEN_R {
        VCH1OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VBATT Wakeup I/O 2 Output Enable
    #[inline(always)]
    pub fn vch2oen(&self) -> VCH2OEN_R {
        VCH2OEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VBATT Wakeup I/O 0 Output Level Selection
    #[inline(always)]
    pub fn vout0lsel(&self) -> VOUT0LSEL_R {
        VOUT0LSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VBATT Wakeup I/O 1 Output Level Selection
    #[inline(always)]
    pub fn vout1lsel(&self) -> VOUT1LSEL_R {
        VOUT1LSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - VBATT Wakeup I/O 2 Output Level Selection
    #[inline(always)]
    pub fn vout2lsel(&self) -> VOUT2LSEL_R {
        VOUT2LSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VBATT Wakeup I/O 0 Output Enable
    #[inline(always)]
    pub fn vch0oen(&mut self) -> VCH0OEN_W<'_, VBTOCTLR_SPEC> {
        VCH0OEN_W::new(self, 0)
    }
    ///Bit 1 - VBATT Wakeup I/O 1 Output Enable
    #[inline(always)]
    pub fn vch1oen(&mut self) -> VCH1OEN_W<'_, VBTOCTLR_SPEC> {
        VCH1OEN_W::new(self, 1)
    }
    ///Bit 2 - VBATT Wakeup I/O 2 Output Enable
    #[inline(always)]
    pub fn vch2oen(&mut self) -> VCH2OEN_W<'_, VBTOCTLR_SPEC> {
        VCH2OEN_W::new(self, 2)
    }
    ///Bit 3 - VBATT Wakeup I/O 0 Output Level Selection
    #[inline(always)]
    pub fn vout0lsel(&mut self) -> VOUT0LSEL_W<'_, VBTOCTLR_SPEC> {
        VOUT0LSEL_W::new(self, 3)
    }
    ///Bit 4 - VBATT Wakeup I/O 1 Output Level Selection
    #[inline(always)]
    pub fn vout1lsel(&mut self) -> VOUT1LSEL_W<'_, VBTOCTLR_SPEC> {
        VOUT1LSEL_W::new(self, 4)
    }
    ///Bit 5 - VBATT Wakeup I/O 2 Output Level Selection
    #[inline(always)]
    pub fn vout2lsel(&mut self) -> VOUT2LSEL_W<'_, VBTOCTLR_SPEC> {
        VOUT2LSEL_W::new(self, 5)
    }
}
/**VBATT Output Control Register

You can [`read`](crate::Reg::read) this register and get [`vbtoctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtoctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTOCTLR_SPEC;
impl crate::RegisterSpec for VBTOCTLR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtoctlr::R`](R) reader structure
impl crate::Readable for VBTOCTLR_SPEC {}
///`write(|w| ..)` method takes [`vbtoctlr::W`](W) writer structure
impl crate::Writable for VBTOCTLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTOCTLR to value 0
impl crate::Resettable for VBTOCTLR_SPEC {}
