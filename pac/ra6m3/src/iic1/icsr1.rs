///Register `ICSR1` reader
pub type R = crate::R<ICSR1_SPEC>;
///Register `ICSR1` writer
pub type W = crate::W<ICSR1_SPEC>;
/**Slave Address 0 Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AAS0_A {
    ///0: Slave address 0 is not detected.
    _0 = 0,
    ///1: Slave address 0 is detected.
    _1 = 1,
}
impl From<AAS0_A> for bool {
    #[inline(always)]
    fn from(variant: AAS0_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `AAS0` reader - Slave Address 0 Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type AAS0_R = crate::BitReader<AAS0_A>;
impl AAS0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AAS0_A {
        match self.bits {
            false => AAS0_A::_0,
            true => AAS0_A::_1,
        }
    }
    ///Slave address 0 is not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AAS0_A::_0
    }
    ///Slave address 0 is detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AAS0_A::_1
    }
}
///Field `AAS0` writer - Slave Address 0 Detection Flag
pub type AAS0_W<'a, REG> = crate::BitWriter0C<'a, REG, AAS0_A>;
impl<'a, REG> AAS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave address 0 is not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AAS0_A::_0)
    }
    ///Slave address 0 is detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AAS0_A::_1)
    }
}
/**Slave Address 1 Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AAS1_A {
    ///0: Slave address 1 is not detected.
    _0 = 0,
    ///1: Slave address 1 is detected.
    _1 = 1,
}
impl From<AAS1_A> for bool {
    #[inline(always)]
    fn from(variant: AAS1_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `AAS1` reader - Slave Address 1 Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type AAS1_R = crate::BitReader<AAS1_A>;
impl AAS1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AAS1_A {
        match self.bits {
            false => AAS1_A::_0,
            true => AAS1_A::_1,
        }
    }
    ///Slave address 1 is not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AAS1_A::_0
    }
    ///Slave address 1 is detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AAS1_A::_1
    }
}
///Field `AAS1` writer - Slave Address 1 Detection Flag
pub type AAS1_W<'a, REG> = crate::BitWriter0C<'a, REG, AAS1_A>;
impl<'a, REG> AAS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave address 1 is not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AAS1_A::_0)
    }
    ///Slave address 1 is detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AAS1_A::_1)
    }
}
/**Slave Address 2 Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AAS2_A {
    ///0: Slave address 2 is not detected.
    _0 = 0,
    ///1: Slave address 2 is detected
    _1 = 1,
}
impl From<AAS2_A> for bool {
    #[inline(always)]
    fn from(variant: AAS2_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `AAS2` reader - Slave Address 2 Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type AAS2_R = crate::BitReader<AAS2_A>;
impl AAS2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AAS2_A {
        match self.bits {
            false => AAS2_A::_0,
            true => AAS2_A::_1,
        }
    }
    ///Slave address 2 is not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AAS2_A::_0
    }
    ///Slave address 2 is detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AAS2_A::_1
    }
}
///Field `AAS2` writer - Slave Address 2 Detection Flag
pub type AAS2_W<'a, REG> = crate::BitWriter0C<'a, REG, AAS2_A>;
impl<'a, REG> AAS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave address 2 is not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AAS2_A::_0)
    }
    ///Slave address 2 is detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AAS2_A::_1)
    }
}
/**General Call Address Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCA_A {
    ///0: General call address is not detected.
    _0 = 0,
    ///1: General call address is detected.
    _1 = 1,
}
impl From<GCA_A> for bool {
    #[inline(always)]
    fn from(variant: GCA_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `GCA` reader - General Call Address Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type GCA_R = crate::BitReader<GCA_A>;
impl GCA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GCA_A {
        match self.bits {
            false => GCA_A::_0,
            true => GCA_A::_1,
        }
    }
    ///General call address is not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GCA_A::_0
    }
    ///General call address is detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GCA_A::_1
    }
}
///Field `GCA` writer - General Call Address Detection Flag
pub type GCA_W<'a, REG> = crate::BitWriter0C<'a, REG, GCA_A>;
impl<'a, REG> GCA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///General call address is not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GCA_A::_0)
    }
    ///General call address is detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GCA_A::_1)
    }
}
/**Device-ID Address Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DID_A {
    ///0: Device-ID command is not detected.
    _0 = 0,
    ///1: Device-ID command is detected.
    _1 = 1,
}
impl From<DID_A> for bool {
    #[inline(always)]
    fn from(variant: DID_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DID` reader - Device-ID Address Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DID_R = crate::BitReader<DID_A>;
impl DID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DID_A {
        match self.bits {
            false => DID_A::_0,
            true => DID_A::_1,
        }
    }
    ///Device-ID command is not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DID_A::_0
    }
    ///Device-ID command is detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DID_A::_1
    }
}
///Field `DID` writer - Device-ID Address Detection Flag
pub type DID_W<'a, REG> = crate::BitWriter0C<'a, REG, DID_A>;
impl<'a, REG> DID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Device-ID command is not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DID_A::_0)
    }
    ///Device-ID command is detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DID_A::_1)
    }
}
/**Host Address Detection Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOA_A {
    ///0: Host address is not detected.
    _0 = 0,
    ///1: Host address is detected.
    _1 = 1,
}
impl From<HOA_A> for bool {
    #[inline(always)]
    fn from(variant: HOA_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `HOA` reader - Host Address Detection Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type HOA_R = crate::BitReader<HOA_A>;
impl HOA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HOA_A {
        match self.bits {
            false => HOA_A::_0,
            true => HOA_A::_1,
        }
    }
    ///Host address is not detected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOA_A::_0
    }
    ///Host address is detected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOA_A::_1
    }
}
///Field `HOA` writer - Host Address Detection Flag
pub type HOA_W<'a, REG> = crate::BitWriter0C<'a, REG, HOA_A>;
impl<'a, REG> HOA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Host address is not detected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(HOA_A::_0)
    }
    ///Host address is detected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(HOA_A::_1)
    }
}
impl R {
    ///Bit 0 - Slave Address 0 Detection Flag
    #[inline(always)]
    pub fn aas0(&self) -> AAS0_R {
        AAS0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Slave Address 1 Detection Flag
    #[inline(always)]
    pub fn aas1(&self) -> AAS1_R {
        AAS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Slave Address 2 Detection Flag
    #[inline(always)]
    pub fn aas2(&self) -> AAS2_R {
        AAS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - General Call Address Detection Flag
    #[inline(always)]
    pub fn gca(&self) -> GCA_R {
        GCA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Device-ID Address Detection Flag
    #[inline(always)]
    pub fn did(&self) -> DID_R {
        DID_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Host Address Detection Flag
    #[inline(always)]
    pub fn hoa(&self) -> HOA_R {
        HOA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Slave Address 0 Detection Flag
    #[inline(always)]
    pub fn aas0(&mut self) -> AAS0_W<ICSR1_SPEC> {
        AAS0_W::new(self, 0)
    }
    ///Bit 1 - Slave Address 1 Detection Flag
    #[inline(always)]
    pub fn aas1(&mut self) -> AAS1_W<ICSR1_SPEC> {
        AAS1_W::new(self, 1)
    }
    ///Bit 2 - Slave Address 2 Detection Flag
    #[inline(always)]
    pub fn aas2(&mut self) -> AAS2_W<ICSR1_SPEC> {
        AAS2_W::new(self, 2)
    }
    ///Bit 3 - General Call Address Detection Flag
    #[inline(always)]
    pub fn gca(&mut self) -> GCA_W<ICSR1_SPEC> {
        GCA_W::new(self, 3)
    }
    ///Bit 5 - Device-ID Address Detection Flag
    #[inline(always)]
    pub fn did(&mut self) -> DID_W<ICSR1_SPEC> {
        DID_W::new(self, 5)
    }
    ///Bit 7 - Host Address Detection Flag
    #[inline(always)]
    pub fn hoa(&mut self) -> HOA_W<ICSR1_SPEC> {
        HOA_W::new(self, 7)
    }
}
/**I2C Bus Status Register 1

You can [`read`](crate::Reg::read) this register and get [`icsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICSR1_SPEC;
impl crate::RegisterSpec for ICSR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`icsr1::R`](R) reader structure
impl crate::Readable for ICSR1_SPEC {}
///`write(|w| ..)` method takes [`icsr1::W`](W) writer structure
impl crate::Writable for ICSR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xaf;
}
///`reset()` method sets ICSR1 to value 0
impl crate::Resettable for ICSR1_SPEC {}
