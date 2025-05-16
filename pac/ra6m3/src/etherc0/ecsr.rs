///Register `ECSR` reader
pub type R = crate::R<ECSR_SPEC>;
///Register `ECSR` writer
pub type W = crate::W<ECSR_SPEC>;
/**False Carrier Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICD_A {
    ///0: PHY-LSI has not detected a false carrier on the line.
    _0 = 0,
    ///1: PHY-LSI has detected a false carrier on the line.
    _1 = 1,
}
impl From<ICD_A> for bool {
    #[inline(always)]
    fn from(variant: ICD_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `ICD` reader - False Carrier Detect Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type ICD_R = crate::BitReader<ICD_A>;
impl ICD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICD_A {
        match self.bits {
            false => ICD_A::_0,
            true => ICD_A::_1,
        }
    }
    ///PHY-LSI has not detected a false carrier on the line.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICD_A::_0
    }
    ///PHY-LSI has detected a false carrier on the line.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICD_A::_1
    }
}
///Field `ICD` writer - False Carrier Detect Flag
pub type ICD_W<'a, REG> = crate::BitWriter1C<'a, REG, ICD_A>;
impl<'a, REG> ICD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PHY-LSI has not detected a false carrier on the line.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ICD_A::_0)
    }
    ///PHY-LSI has detected a false carrier on the line.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ICD_A::_1)
    }
}
/**Magic Packet Detect Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPD_A {
    ///0: Magic Packet has not been detected.
    _0 = 0,
    ///1: Magic Packet has been detected.
    _1 = 1,
}
impl From<MPD_A> for bool {
    #[inline(always)]
    fn from(variant: MPD_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `MPD` reader - Magic Packet Detect Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type MPD_R = crate::BitReader<MPD_A>;
impl MPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MPD_A {
        match self.bits {
            false => MPD_A::_0,
            true => MPD_A::_1,
        }
    }
    ///Magic Packet has not been detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPD_A::_0
    }
    ///Magic Packet has been detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPD_A::_1
    }
}
///Field `MPD` writer - Magic Packet Detect Flag
pub type MPD_W<'a, REG> = crate::BitWriter1C<'a, REG, MPD_A>;
impl<'a, REG> MPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Magic Packet has not been detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MPD_A::_0)
    }
    ///Magic Packet has been detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MPD_A::_1)
    }
}
/**LCHNG Link Signal Change Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCHNG_A {
    ///0: Change in the ETn_LINKSTA signal has not been detected.
    _0 = 0,
    ///1: Change in the ETn_LINKSTA signal has been detected (high to low, or low to high).
    _1 = 1,
}
impl From<LCHNG_A> for bool {
    #[inline(always)]
    fn from(variant: LCHNG_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `LCHNG` reader - LCHNG Link Signal Change Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type LCHNG_R = crate::BitReader<LCHNG_A>;
impl LCHNG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCHNG_A {
        match self.bits {
            false => LCHNG_A::_0,
            true => LCHNG_A::_1,
        }
    }
    ///Change in the ETn_LINKSTA signal has not been detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCHNG_A::_0
    }
    ///Change in the ETn_LINKSTA signal has been detected (high to low, or low to high).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCHNG_A::_1
    }
}
///Field `LCHNG` writer - LCHNG Link Signal Change Flag
pub type LCHNG_W<'a, REG> = crate::BitWriter1C<'a, REG, LCHNG_A>;
impl<'a, REG> LCHNG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Change in the ETn_LINKSTA signal has not been detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LCHNG_A::_0)
    }
    ///Change in the ETn_LINKSTA signal has been detected (high to low, or low to high).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LCHNG_A::_1)
    }
}
/**PAUSE Frame Retransmit Over Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSRTO_A {
    ///0: PAUSE frame retransmit count has not reached the upper limit.
    _0 = 0,
    ///1: PAUSE frame retransmit count has reached the upper limit.
    _1 = 1,
}
impl From<PSRTO_A> for bool {
    #[inline(always)]
    fn from(variant: PSRTO_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PSRTO` reader - PAUSE Frame Retransmit Over Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PSRTO_R = crate::BitReader<PSRTO_A>;
impl PSRTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSRTO_A {
        match self.bits {
            false => PSRTO_A::_0,
            true => PSRTO_A::_1,
        }
    }
    ///PAUSE frame retransmit count has not reached the upper limit.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSRTO_A::_0
    }
    ///PAUSE frame retransmit count has reached the upper limit.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSRTO_A::_1
    }
}
///Field `PSRTO` writer - PAUSE Frame Retransmit Over Flag
pub type PSRTO_W<'a, REG> = crate::BitWriter1C<'a, REG, PSRTO_A>;
impl<'a, REG> PSRTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PAUSE frame retransmit count has not reached the upper limit.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSRTO_A::_0)
    }
    ///PAUSE frame retransmit count has reached the upper limit.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSRTO_A::_1)
    }
}
/**Continuous Broadcast Frame Reception Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFR_A {
    ///0: Continuous reception of broadcast frames has not been detected.
    _0 = 0,
    ///1: Continuous reception of broadcast frames has been detected.
    _1 = 1,
}
impl From<BFR_A> for bool {
    #[inline(always)]
    fn from(variant: BFR_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `BFR` reader - Continuous Broadcast Frame Reception Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type BFR_R = crate::BitReader<BFR_A>;
impl BFR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BFR_A {
        match self.bits {
            false => BFR_A::_0,
            true => BFR_A::_1,
        }
    }
    ///Continuous reception of broadcast frames has not been detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFR_A::_0
    }
    ///Continuous reception of broadcast frames has been detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFR_A::_1
    }
}
///Field `BFR` writer - Continuous Broadcast Frame Reception Flag
pub type BFR_W<'a, REG> = crate::BitWriter1C<'a, REG, BFR_A>;
impl<'a, REG> BFR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Continuous reception of broadcast frames has not been detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BFR_A::_0)
    }
    ///Continuous reception of broadcast frames has been detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BFR_A::_1)
    }
}
impl R {
    ///Bit 0 - False Carrier Detect Flag
    #[inline(always)]
    pub fn icd(&self) -> ICD_R {
        ICD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Magic Packet Detect Flag
    #[inline(always)]
    pub fn mpd(&self) -> MPD_R {
        MPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LCHNG Link Signal Change Flag
    #[inline(always)]
    pub fn lchng(&self) -> LCHNG_R {
        LCHNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - PAUSE Frame Retransmit Over Flag
    #[inline(always)]
    pub fn psrto(&self) -> PSRTO_R {
        PSRTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Continuous Broadcast Frame Reception Flag
    #[inline(always)]
    pub fn bfr(&self) -> BFR_R {
        BFR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - False Carrier Detect Flag
    #[inline(always)]
    pub fn icd(&mut self) -> ICD_W<ECSR_SPEC> {
        ICD_W::new(self, 0)
    }
    ///Bit 1 - Magic Packet Detect Flag
    #[inline(always)]
    pub fn mpd(&mut self) -> MPD_W<ECSR_SPEC> {
        MPD_W::new(self, 1)
    }
    ///Bit 2 - LCHNG Link Signal Change Flag
    #[inline(always)]
    pub fn lchng(&mut self) -> LCHNG_W<ECSR_SPEC> {
        LCHNG_W::new(self, 2)
    }
    ///Bit 4 - PAUSE Frame Retransmit Over Flag
    #[inline(always)]
    pub fn psrto(&mut self) -> PSRTO_W<ECSR_SPEC> {
        PSRTO_W::new(self, 4)
    }
    ///Bit 5 - Continuous Broadcast Frame Reception Flag
    #[inline(always)]
    pub fn bfr(&mut self) -> BFR_W<ECSR_SPEC> {
        BFR_W::new(self, 5)
    }
}
/**ETHERC Status Register

You can [`read`](crate::Reg::read) this register and get [`ecsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECSR_SPEC;
impl crate::RegisterSpec for ECSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ecsr::R`](R) reader structure
impl crate::Readable for ECSR_SPEC {}
///`write(|w| ..)` method takes [`ecsr::W`](W) writer structure
impl crate::Writable for ECSR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x37;
}
///`reset()` method sets ECSR to value 0
impl crate::Resettable for ECSR_SPEC {}
