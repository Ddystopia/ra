///Register `ECSR` reader
pub type R = crate::R<ECSR_SPEC>;
///Register `ECSR` writer
pub type W = crate::W<ECSR_SPEC>;
/**Stuff Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEF_A {
    ///0: No stuff error detected
    _0 = 0,
    ///1: Stuff error detected
    _1 = 1,
}
impl From<SEF_A> for bool {
    #[inline(always)]
    fn from(variant: SEF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SEF` reader - Stuff Error Flag
pub type SEF_R = crate::BitReader<SEF_A>;
impl SEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SEF_A {
        match self.bits {
            false => SEF_A::_0,
            true => SEF_A::_1,
        }
    }
    ///No stuff error detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEF_A::_0
    }
    ///Stuff error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEF_A::_1
    }
}
///Field `SEF` writer - Stuff Error Flag
pub type SEF_W<'a, REG> = crate::BitWriter<'a, REG, SEF_A>;
impl<'a, REG> SEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No stuff error detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SEF_A::_0)
    }
    ///Stuff error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SEF_A::_1)
    }
}
/**Form Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEF_A {
    ///0: No form error detected
    _0 = 0,
    ///1: Form error detected
    _1 = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FEF` reader - Form Error Flag
pub type FEF_R = crate::BitReader<FEF_A>;
impl FEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::_0,
            true => FEF_A::_1,
        }
    }
    ///No form error detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEF_A::_0
    }
    ///Form error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEF_A::_1
    }
}
///Field `FEF` writer - Form Error Flag
pub type FEF_W<'a, REG> = crate::BitWriter<'a, REG, FEF_A>;
impl<'a, REG> FEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No form error detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FEF_A::_0)
    }
    ///Form error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FEF_A::_1)
    }
}
/**ACK Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AEF_A {
    ///0: No ACK error detected
    _0 = 0,
    ///1: ACK error detected
    _1 = 1,
}
impl From<AEF_A> for bool {
    #[inline(always)]
    fn from(variant: AEF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AEF` reader - ACK Error Flag
pub type AEF_R = crate::BitReader<AEF_A>;
impl AEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AEF_A {
        match self.bits {
            false => AEF_A::_0,
            true => AEF_A::_1,
        }
    }
    ///No ACK error detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AEF_A::_0
    }
    ///ACK error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AEF_A::_1
    }
}
///Field `AEF` writer - ACK Error Flag
pub type AEF_W<'a, REG> = crate::BitWriter<'a, REG, AEF_A>;
impl<'a, REG> AEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No ACK error detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AEF_A::_0)
    }
    ///ACK error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AEF_A::_1)
    }
}
/**CRC Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEF_A {
    ///0: No CRC error detected
    _0 = 0,
    ///1: CRC error detected
    _1 = 1,
}
impl From<CEF_A> for bool {
    #[inline(always)]
    fn from(variant: CEF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CEF` reader - CRC Error Flag
pub type CEF_R = crate::BitReader<CEF_A>;
impl CEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CEF_A {
        match self.bits {
            false => CEF_A::_0,
            true => CEF_A::_1,
        }
    }
    ///No CRC error detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEF_A::_0
    }
    ///CRC error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEF_A::_1
    }
}
///Field `CEF` writer - CRC Error Flag
pub type CEF_W<'a, REG> = crate::BitWriter<'a, REG, CEF_A>;
impl<'a, REG> CEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No CRC error detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CEF_A::_0)
    }
    ///CRC error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CEF_A::_1)
    }
}
/**Bit Error (recessive) Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BE1F_A {
    ///0: No bit error (recessive) detected
    _0 = 0,
    ///1: Bit error (recessive) detected
    _1 = 1,
}
impl From<BE1F_A> for bool {
    #[inline(always)]
    fn from(variant: BE1F_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BE1F` reader - Bit Error (recessive) Flag
pub type BE1F_R = crate::BitReader<BE1F_A>;
impl BE1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BE1F_A {
        match self.bits {
            false => BE1F_A::_0,
            true => BE1F_A::_1,
        }
    }
    ///No bit error (recessive) detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BE1F_A::_0
    }
    ///Bit error (recessive) detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BE1F_A::_1
    }
}
///Field `BE1F` writer - Bit Error (recessive) Flag
pub type BE1F_W<'a, REG> = crate::BitWriter<'a, REG, BE1F_A>;
impl<'a, REG> BE1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No bit error (recessive) detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BE1F_A::_0)
    }
    ///Bit error (recessive) detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BE1F_A::_1)
    }
}
/**Bit Error (dominant) Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BE0F_A {
    ///0: No bit error (dominant) detected
    _0 = 0,
    ///1: Bit error (dominant) detected
    _1 = 1,
}
impl From<BE0F_A> for bool {
    #[inline(always)]
    fn from(variant: BE0F_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BE0F` reader - Bit Error (dominant) Flag
pub type BE0F_R = crate::BitReader<BE0F_A>;
impl BE0F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BE0F_A {
        match self.bits {
            false => BE0F_A::_0,
            true => BE0F_A::_1,
        }
    }
    ///No bit error (dominant) detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BE0F_A::_0
    }
    ///Bit error (dominant) detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BE0F_A::_1
    }
}
///Field `BE0F` writer - Bit Error (dominant) Flag
pub type BE0F_W<'a, REG> = crate::BitWriter<'a, REG, BE0F_A>;
impl<'a, REG> BE0F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No bit error (dominant) detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BE0F_A::_0)
    }
    ///Bit error (dominant) detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BE0F_A::_1)
    }
}
/**ACK Delimiter Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADEF_A {
    ///0: No ACK delimiter error detected
    _0 = 0,
    ///1: ACK delimiter error detected
    _1 = 1,
}
impl From<ADEF_A> for bool {
    #[inline(always)]
    fn from(variant: ADEF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADEF` reader - ACK Delimiter Error Flag
pub type ADEF_R = crate::BitReader<ADEF_A>;
impl ADEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADEF_A {
        match self.bits {
            false => ADEF_A::_0,
            true => ADEF_A::_1,
        }
    }
    ///No ACK delimiter error detected
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADEF_A::_0
    }
    ///ACK delimiter error detected
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADEF_A::_1
    }
}
///Field `ADEF` writer - ACK Delimiter Error Flag
pub type ADEF_W<'a, REG> = crate::BitWriter<'a, REG, ADEF_A>;
impl<'a, REG> ADEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No ACK delimiter error detected
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADEF_A::_0)
    }
    ///ACK delimiter error detected
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADEF_A::_1)
    }
}
/**Error Display Mode Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDPM_A {
    ///0: Output of first detected error code
    _0 = 0,
    ///1: Output of accumulated error code
    _1 = 1,
}
impl From<EDPM_A> for bool {
    #[inline(always)]
    fn from(variant: EDPM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EDPM` reader - Error Display Mode Select
pub type EDPM_R = crate::BitReader<EDPM_A>;
impl EDPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EDPM_A {
        match self.bits {
            false => EDPM_A::_0,
            true => EDPM_A::_1,
        }
    }
    ///Output of first detected error code
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDPM_A::_0
    }
    ///Output of accumulated error code
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDPM_A::_1
    }
}
///Field `EDPM` writer - Error Display Mode Select
pub type EDPM_W<'a, REG> = crate::BitWriter<'a, REG, EDPM_A>;
impl<'a, REG> EDPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output of first detected error code
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EDPM_A::_0)
    }
    ///Output of accumulated error code
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EDPM_A::_1)
    }
}
impl R {
    ///Bit 0 - Stuff Error Flag
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Form Error Flag
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ACK Error Flag
    #[inline(always)]
    pub fn aef(&self) -> AEF_R {
        AEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CRC Error Flag
    #[inline(always)]
    pub fn cef(&self) -> CEF_R {
        CEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Bit Error (recessive) Flag
    #[inline(always)]
    pub fn be1f(&self) -> BE1F_R {
        BE1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Bit Error (dominant) Flag
    #[inline(always)]
    pub fn be0f(&self) -> BE0F_R {
        BE0F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ACK Delimiter Error Flag
    #[inline(always)]
    pub fn adef(&self) -> ADEF_R {
        ADEF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Error Display Mode Select
    #[inline(always)]
    pub fn edpm(&self) -> EDPM_R {
        EDPM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Stuff Error Flag
    #[inline(always)]
    pub fn sef(&mut self) -> SEF_W<ECSR_SPEC> {
        SEF_W::new(self, 0)
    }
    ///Bit 1 - Form Error Flag
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W<ECSR_SPEC> {
        FEF_W::new(self, 1)
    }
    ///Bit 2 - ACK Error Flag
    #[inline(always)]
    pub fn aef(&mut self) -> AEF_W<ECSR_SPEC> {
        AEF_W::new(self, 2)
    }
    ///Bit 3 - CRC Error Flag
    #[inline(always)]
    pub fn cef(&mut self) -> CEF_W<ECSR_SPEC> {
        CEF_W::new(self, 3)
    }
    ///Bit 4 - Bit Error (recessive) Flag
    #[inline(always)]
    pub fn be1f(&mut self) -> BE1F_W<ECSR_SPEC> {
        BE1F_W::new(self, 4)
    }
    ///Bit 5 - Bit Error (dominant) Flag
    #[inline(always)]
    pub fn be0f(&mut self) -> BE0F_W<ECSR_SPEC> {
        BE0F_W::new(self, 5)
    }
    ///Bit 6 - ACK Delimiter Error Flag
    #[inline(always)]
    pub fn adef(&mut self) -> ADEF_W<ECSR_SPEC> {
        ADEF_W::new(self, 6)
    }
    ///Bit 7 - Error Display Mode Select
    #[inline(always)]
    pub fn edpm(&mut self) -> EDPM_W<ECSR_SPEC> {
        EDPM_W::new(self, 7)
    }
}
/**Error Code Store Register

You can [`read`](crate::Reg::read) this register and get [`ecsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECSR_SPEC;
impl crate::RegisterSpec for ECSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ecsr::R`](R) reader structure
impl crate::Readable for ECSR_SPEC {}
///`write(|w| ..)` method takes [`ecsr::W`](W) writer structure
impl crate::Writable for ECSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECSR to value 0
impl crate::Resettable for ECSR_SPEC {}
