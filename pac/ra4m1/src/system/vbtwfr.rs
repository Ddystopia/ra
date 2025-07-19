///Register `VBTWFR` reader
pub type R = crate::R<VBTWFR_SPEC>;
///Register `VBTWFR` writer
pub type W = crate::W<VBTWFR_SPEC>;
/**VBATWIO0 Wakeup Trigger Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH0F_A {
    ///0: No wakeup trigger by the VBATWIO0 pin is generated
    _0 = 0,
    ///1: A wakeup trigger by the VBATWIO0 pin is generated
    _1 = 1,
}
impl From<VCH0F_A> for bool {
    #[inline(always)]
    fn from(variant: VCH0F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `VCH0F` reader - VBATWIO0 Wakeup Trigger Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type VCH0F_R = crate::BitReader<VCH0F_A>;
impl VCH0F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VCH0F_A {
        match self.bits {
            false => VCH0F_A::_0,
            true => VCH0F_A::_1,
        }
    }
    ///No wakeup trigger by the VBATWIO0 pin is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH0F_A::_0
    }
    ///A wakeup trigger by the VBATWIO0 pin is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH0F_A::_1
    }
}
///Field `VCH0F` writer - VBATWIO0 Wakeup Trigger Flag
pub type VCH0F_W<'a, REG> = crate::BitWriter0C<'a, REG, VCH0F_A>;
impl<'a, REG> VCH0F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No wakeup trigger by the VBATWIO0 pin is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0F_A::_0)
    }
    ///A wakeup trigger by the VBATWIO0 pin is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH0F_A::_1)
    }
}
/**VBATWIO1 Wakeup Trigger Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH1F_A {
    ///0: No wakeup trigger by the VBATWIO1 pin is generated
    _0 = 0,
    ///1: A wakeup trigger by the VBATWIO1 pin is generated
    _1 = 1,
}
impl From<VCH1F_A> for bool {
    #[inline(always)]
    fn from(variant: VCH1F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `VCH1F` reader - VBATWIO1 Wakeup Trigger Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type VCH1F_R = crate::BitReader<VCH1F_A>;
impl VCH1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VCH1F_A {
        match self.bits {
            false => VCH1F_A::_0,
            true => VCH1F_A::_1,
        }
    }
    ///No wakeup trigger by the VBATWIO1 pin is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH1F_A::_0
    }
    ///A wakeup trigger by the VBATWIO1 pin is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH1F_A::_1
    }
}
///Field `VCH1F` writer - VBATWIO1 Wakeup Trigger Flag
pub type VCH1F_W<'a, REG> = crate::BitWriter0C<'a, REG, VCH1F_A>;
impl<'a, REG> VCH1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No wakeup trigger by the VBATWIO1 pin is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1F_A::_0)
    }
    ///A wakeup trigger by the VBATWIO1 pin is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH1F_A::_1)
    }
}
/**VBATWIO2 Wakeup Trigger Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCH2F_A {
    ///0: No wakeup trigger by the VBATWIO2 pin is generated
    _0 = 0,
    ///1: A wakeup trigger by the VBATWIO2 pin is generated
    _1 = 1,
}
impl From<VCH2F_A> for bool {
    #[inline(always)]
    fn from(variant: VCH2F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `VCH2F` reader - VBATWIO2 Wakeup Trigger Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type VCH2F_R = crate::BitReader<VCH2F_A>;
impl VCH2F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VCH2F_A {
        match self.bits {
            false => VCH2F_A::_0,
            true => VCH2F_A::_1,
        }
    }
    ///No wakeup trigger by the VBATWIO2 pin is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCH2F_A::_0
    }
    ///A wakeup trigger by the VBATWIO2 pin is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCH2F_A::_1
    }
}
///Field `VCH2F` writer - VBATWIO2 Wakeup Trigger Flag
pub type VCH2F_W<'a, REG> = crate::BitWriter0C<'a, REG, VCH2F_A>;
impl<'a, REG> VCH2F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No wakeup trigger by the VBATWIO2 pin is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2F_A::_0)
    }
    ///A wakeup trigger by the VBATWIO2 pin is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VCH2F_A::_1)
    }
}
/**VBATT RTC-Interval Wakeup Trigger Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRTCIF_A {
    ///0: No wakeup trigger by the RTC interval is generated
    _0 = 0,
    ///1: A wakeup trigger by the RTC interval is generated
    _1 = 1,
}
impl From<VRTCIF_A> for bool {
    #[inline(always)]
    fn from(variant: VRTCIF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `VRTCIF` reader - VBATT RTC-Interval Wakeup Trigger Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type VRTCIF_R = crate::BitReader<VRTCIF_A>;
impl VRTCIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VRTCIF_A {
        match self.bits {
            false => VRTCIF_A::_0,
            true => VRTCIF_A::_1,
        }
    }
    ///No wakeup trigger by the RTC interval is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VRTCIF_A::_0
    }
    ///A wakeup trigger by the RTC interval is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VRTCIF_A::_1
    }
}
///Field `VRTCIF` writer - VBATT RTC-Interval Wakeup Trigger Flag
pub type VRTCIF_W<'a, REG> = crate::BitWriter0C<'a, REG, VRTCIF_A>;
impl<'a, REG> VRTCIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No wakeup trigger by the RTC interval is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VRTCIF_A::_0)
    }
    ///A wakeup trigger by the RTC interval is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VRTCIF_A::_1)
    }
}
/**VBATT RTC-Alarm Wakeup Trigger Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRTCAF_A {
    ///0: No wakeup trigger by the RTC alarm is generated
    _0 = 0,
    ///1: A wakeup trigger by the RTC alarm is generated
    _1 = 1,
}
impl From<VRTCAF_A> for bool {
    #[inline(always)]
    fn from(variant: VRTCAF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `VRTCAF` reader - VBATT RTC-Alarm Wakeup Trigger Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type VRTCAF_R = crate::BitReader<VRTCAF_A>;
impl VRTCAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VRTCAF_A {
        match self.bits {
            false => VRTCAF_A::_0,
            true => VRTCAF_A::_1,
        }
    }
    ///No wakeup trigger by the RTC alarm is generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VRTCAF_A::_0
    }
    ///A wakeup trigger by the RTC alarm is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VRTCAF_A::_1
    }
}
///Field `VRTCAF` writer - VBATT RTC-Alarm Wakeup Trigger Flag
pub type VRTCAF_W<'a, REG> = crate::BitWriter0C<'a, REG, VRTCAF_A>;
impl<'a, REG> VRTCAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No wakeup trigger by the RTC alarm is generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VRTCAF_A::_0)
    }
    ///A wakeup trigger by the RTC alarm is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VRTCAF_A::_1)
    }
}
impl R {
    ///Bit 0 - VBATWIO0 Wakeup Trigger Flag
    #[inline(always)]
    pub fn vch0f(&self) -> VCH0F_R {
        VCH0F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VBATWIO1 Wakeup Trigger Flag
    #[inline(always)]
    pub fn vch1f(&self) -> VCH1F_R {
        VCH1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VBATWIO2 Wakeup Trigger Flag
    #[inline(always)]
    pub fn vch2f(&self) -> VCH2F_R {
        VCH2F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VBATT RTC-Interval Wakeup Trigger Flag
    #[inline(always)]
    pub fn vrtcif(&self) -> VRTCIF_R {
        VRTCIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VBATT RTC-Alarm Wakeup Trigger Flag
    #[inline(always)]
    pub fn vrtcaf(&self) -> VRTCAF_R {
        VRTCAF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VBATWIO0 Wakeup Trigger Flag
    #[inline(always)]
    pub fn vch0f(&mut self) -> VCH0F_W<'_, VBTWFR_SPEC> {
        VCH0F_W::new(self, 0)
    }
    ///Bit 1 - VBATWIO1 Wakeup Trigger Flag
    #[inline(always)]
    pub fn vch1f(&mut self) -> VCH1F_W<'_, VBTWFR_SPEC> {
        VCH1F_W::new(self, 1)
    }
    ///Bit 2 - VBATWIO2 Wakeup Trigger Flag
    #[inline(always)]
    pub fn vch2f(&mut self) -> VCH2F_W<'_, VBTWFR_SPEC> {
        VCH2F_W::new(self, 2)
    }
    ///Bit 3 - VBATT RTC-Interval Wakeup Trigger Flag
    #[inline(always)]
    pub fn vrtcif(&mut self) -> VRTCIF_W<'_, VBTWFR_SPEC> {
        VRTCIF_W::new(self, 3)
    }
    ///Bit 4 - VBATT RTC-Alarm Wakeup Trigger Flag
    #[inline(always)]
    pub fn vrtcaf(&mut self) -> VRTCAF_W<'_, VBTWFR_SPEC> {
        VRTCAF_W::new(self, 4)
    }
}
/**VBATT Wakeup trigger source Flag Register

You can [`read`](crate::Reg::read) this register and get [`vbtwfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTWFR_SPEC;
impl crate::RegisterSpec for VBTWFR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtwfr::R`](R) reader structure
impl crate::Readable for VBTWFR_SPEC {}
///`write(|w| ..)` method takes [`vbtwfr::W`](W) writer structure
impl crate::Writable for VBTWFR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x1f;
}
///`reset()` method sets VBTWFR to value 0
impl crate::Resettable for VBTWFR_SPEC {}
