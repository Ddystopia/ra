///Register `SMR_SMCI` reader
pub type R = crate::R<SMR_SMCI_SPEC>;
///Register `SMR_SMCI` writer
pub type W = crate::W<SMR_SMCI_SPEC>;
/**Clock Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKS_A {
    ///0: PCLK clock
    _00 = 0,
    ///1: PCLK/4 clock
    _01 = 1,
    ///2: PCLK/16 clock
    _10 = 2,
    ///3: PCLK/64 clock
    _11 = 3,
}
impl From<CKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKS_A {
    type Ux = u8;
}
impl crate::IsEnum for CKS_A {}
///Field `CKS` reader - Clock Select
pub type CKS_R = crate::FieldReader<CKS_A>;
impl CKS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKS_A {
        match self.bits {
            0 => CKS_A::_00,
            1 => CKS_A::_01,
            2 => CKS_A::_10,
            3 => CKS_A::_11,
            _ => unreachable!(),
        }
    }
    ///PCLK clock
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CKS_A::_00
    }
    ///PCLK/4 clock
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CKS_A::_01
    }
    ///PCLK/16 clock
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CKS_A::_10
    }
    ///PCLK/64 clock
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CKS_A::_11
    }
}
///Field `CKS` writer - Clock Select
pub type CKS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKS_A, crate::Safe>;
impl<'a, REG> CKS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK clock
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_00)
    }
    ///PCLK/4 clock
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_01)
    }
    ///PCLK/16 clock
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_10)
    }
    ///PCLK/64 clock
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CKS_A::_11)
    }
}
/**Stop Bit Length(Valid only in asynchronous mode)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCP_A {
    ///0: 93 clock cycles(S=93) (SCMR.BCP2=0) / 32 clock cycles(S=32) (SCMR.BCP2=1)
    _00 = 0,
    ///1: 128 clock cycles(S=128) (SCMR.BCP2=0) / 64 clock cycles(S=64) (SCMR.BCP2=1)
    _01 = 1,
    ///2: 186 clock cycles(S=186) (SCMR.BCP2=0) / 372 clock cycles(S=372) (SCMR.BCP2=1)
    _10 = 2,
    ///3: 512 clock cycles(S=512) (SCMR.BCP2=0) / 256 clock cycles(S=256) (SCMR.BCP2=1)
    _11 = 3,
}
impl From<BCP_A> for u8 {
    #[inline(always)]
    fn from(variant: BCP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BCP_A {
    type Ux = u8;
}
impl crate::IsEnum for BCP_A {}
///Field `BCP` reader - Stop Bit Length(Valid only in asynchronous mode)
pub type BCP_R = crate::FieldReader<BCP_A>;
impl BCP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BCP_A {
        match self.bits {
            0 => BCP_A::_00,
            1 => BCP_A::_01,
            2 => BCP_A::_10,
            3 => BCP_A::_11,
            _ => unreachable!(),
        }
    }
    ///93 clock cycles(S=93) (SCMR.BCP2=0) / 32 clock cycles(S=32) (SCMR.BCP2=1)
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BCP_A::_00
    }
    ///128 clock cycles(S=128) (SCMR.BCP2=0) / 64 clock cycles(S=64) (SCMR.BCP2=1)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BCP_A::_01
    }
    ///186 clock cycles(S=186) (SCMR.BCP2=0) / 372 clock cycles(S=372) (SCMR.BCP2=1)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BCP_A::_10
    }
    ///512 clock cycles(S=512) (SCMR.BCP2=0) / 256 clock cycles(S=256) (SCMR.BCP2=1)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BCP_A::_11
    }
}
///Field `BCP` writer - Stop Bit Length(Valid only in asynchronous mode)
pub type BCP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BCP_A, crate::Safe>;
impl<'a, REG> BCP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///93 clock cycles(S=93) (SCMR.BCP2=0) / 32 clock cycles(S=32) (SCMR.BCP2=1)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(BCP_A::_00)
    }
    ///128 clock cycles(S=128) (SCMR.BCP2=0) / 64 clock cycles(S=64) (SCMR.BCP2=1)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(BCP_A::_01)
    }
    ///186 clock cycles(S=186) (SCMR.BCP2=0) / 372 clock cycles(S=372) (SCMR.BCP2=1)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(BCP_A::_10)
    }
    ///512 clock cycles(S=512) (SCMR.BCP2=0) / 256 clock cycles(S=256) (SCMR.BCP2=1)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(BCP_A::_11)
    }
}
/**Parity Mode (Valid only when the PE bit is 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM_A {
    ///0: Selects even parity
    _0 = 0,
    ///1: Selects odd parity
    _1 = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PM` reader - Parity Mode (Valid only when the PE bit is 1)
pub type PM_R = crate::BitReader<PM_A>;
impl PM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::_0,
            true => PM_A::_1,
        }
    }
    ///Selects even parity
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PM_A::_0
    }
    ///Selects odd parity
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PM_A::_1
    }
}
///Field `PM` writer - Parity Mode (Valid only when the PE bit is 1)
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG, PM_A>;
impl<'a, REG> PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selects even parity
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::_0)
    }
    ///Selects odd parity
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::_1)
    }
}
/**Parity Enable(Valid only in asynchronous mode)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    ///0: Setting Prohibited
    _0 = 0,
    ///1: Set this bit to 1 in smart card interface mode.
    _1 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PE` reader - Parity Enable(Valid only in asynchronous mode)
pub type PE_R = crate::BitReader<PE_A>;
impl PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::_0,
            true => PE_A::_1,
        }
    }
    ///Setting Prohibited
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PE_A::_0
    }
    ///Set this bit to 1 in smart card interface mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PE_A::_1
    }
}
///Field `PE` writer - Parity Enable(Valid only in asynchronous mode)
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG, PE_A>;
impl<'a, REG> PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting Prohibited
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::_0)
    }
    ///Set this bit to 1 in smart card interface mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::_1)
    }
}
/**Block Transfer Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK_A {
    ///0: Normal mode operation
    _0 = 0,
    ///1: Block transfer mode operation
    _1 = 1,
}
impl From<BLK_A> for bool {
    #[inline(always)]
    fn from(variant: BLK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BLK` reader - Block Transfer Mode
pub type BLK_R = crate::BitReader<BLK_A>;
impl BLK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BLK_A {
        match self.bits {
            false => BLK_A::_0,
            true => BLK_A::_1,
        }
    }
    ///Normal mode operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLK_A::_0
    }
    ///Block transfer mode operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLK_A::_1
    }
}
///Field `BLK` writer - Block Transfer Mode
pub type BLK_W<'a, REG> = crate::BitWriter<'a, REG, BLK_A>;
impl<'a, REG> BLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BLK_A::_0)
    }
    ///Block transfer mode operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BLK_A::_1)
    }
}
/**GSM Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GM_A {
    ///0: Normal mode operation
    _0 = 0,
    ///1: GSM mode operation
    _1 = 1,
}
impl From<GM_A> for bool {
    #[inline(always)]
    fn from(variant: GM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GM` reader - GSM Mode
pub type GM_R = crate::BitReader<GM_A>;
impl GM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GM_A {
        match self.bits {
            false => GM_A::_0,
            true => GM_A::_1,
        }
    }
    ///Normal mode operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GM_A::_0
    }
    ///GSM mode operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GM_A::_1
    }
}
///Field `GM` writer - GSM Mode
pub type GM_W<'a, REG> = crate::BitWriter<'a, REG, GM_A>;
impl<'a, REG> GM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GM_A::_0)
    }
    ///GSM mode operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GM_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Clock Select
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(self.bits & 3)
    }
    ///Bits 2:3 - Stop Bit Length(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn bcp(&self) -> BCP_R {
        BCP_R::new((self.bits >> 2) & 3)
    }
    ///Bit 4 - Parity Mode (Valid only when the PE bit is 1)
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Parity Enable(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Block Transfer Mode
    #[inline(always)]
    pub fn blk(&self) -> BLK_R {
        BLK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GSM Mode
    #[inline(always)]
    pub fn gm(&self) -> GM_R {
        GM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Clock Select
    #[inline(always)]
    pub fn cks(&mut self) -> CKS_W<SMR_SMCI_SPEC> {
        CKS_W::new(self, 0)
    }
    ///Bits 2:3 - Stop Bit Length(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn bcp(&mut self) -> BCP_W<SMR_SMCI_SPEC> {
        BCP_W::new(self, 2)
    }
    ///Bit 4 - Parity Mode (Valid only when the PE bit is 1)
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<SMR_SMCI_SPEC> {
        PM_W::new(self, 4)
    }
    ///Bit 5 - Parity Enable(Valid only in asynchronous mode)
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<SMR_SMCI_SPEC> {
        PE_W::new(self, 5)
    }
    ///Bit 6 - Block Transfer Mode
    #[inline(always)]
    pub fn blk(&mut self) -> BLK_W<SMR_SMCI_SPEC> {
        BLK_W::new(self, 6)
    }
    ///Bit 7 - GSM Mode
    #[inline(always)]
    pub fn gm(&mut self) -> GM_W<SMR_SMCI_SPEC> {
        GM_W::new(self, 7)
    }
}
/**Serial mode register (SCMR.SMIF = 1)

You can [`read`](crate::Reg::read) this register and get [`smr_smci::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr_smci::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SMR_SMCI_SPEC;
impl crate::RegisterSpec for SMR_SMCI_SPEC {
    type Ux = u8;
}
///`read()` method returns [`smr_smci::R`](R) reader structure
impl crate::Readable for SMR_SMCI_SPEC {}
///`write(|w| ..)` method takes [`smr_smci::W`](W) writer structure
impl crate::Writable for SMR_SMCI_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMR_SMCI to value 0
impl crate::Resettable for SMR_SMCI_SPEC {}
