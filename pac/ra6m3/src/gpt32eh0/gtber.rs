///Register `GTBER` reader
pub type R = crate::R<GTBER_SPEC>;
///Register `GTBER` writer
pub type W = crate::W<GTBER_SPEC>;
/**BD\[3\]: GTDV Buffer Operation DisableBD\[2\]: GTADTR Buffer Operation DisableBD\[1\]: GTPR Buffer Operation DisableBD\[0\]: GTCCR Buffer Operation Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BD_A {
    ///0: Enable buffer operation
    _0 = 0,
    ///1: Disable buffer operation
    _1 = 1,
}
impl From<BD_A> for u8 {
    #[inline(always)]
    fn from(variant: BD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BD_A {
    type Ux = u8;
}
impl crate::IsEnum for BD_A {}
///Field `BD` reader - BD\[3\]: GTDV Buffer Operation DisableBD\[2\]: GTADTR Buffer Operation DisableBD\[1\]: GTPR Buffer Operation DisableBD\[0\]: GTCCR Buffer Operation Disable
pub type BD_R = crate::FieldReader<BD_A>;
impl BD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<BD_A> {
        match self.bits {
            0 => Some(BD_A::_0),
            1 => Some(BD_A::_1),
            _ => None,
        }
    }
    ///Enable buffer operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BD_A::_0
    }
    ///Disable buffer operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BD_A::_1
    }
}
///Field `BD` writer - BD\[3\]: GTDV Buffer Operation DisableBD\[2\]: GTADTR Buffer Operation DisableBD\[1\]: GTPR Buffer Operation DisableBD\[0\]: GTCCR Buffer Operation Disable
pub type BD_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BD_A>;
impl<'a, REG> BD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Enable buffer operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BD_A::_0)
    }
    ///Disable buffer operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BD_A::_1)
    }
}
/**GTCCRA Buffer Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCRA_A {
    ///0: Buffer operation is not performed
    _00 = 0,
    ///1: Single buffer operation (GTCCRA <--> GTCCRC)
    _01 = 1,
    ///2: Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)
    _10 = 2,
    ///3: Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)
    _11 = 3,
}
impl From<CCRA_A> for u8 {
    #[inline(always)]
    fn from(variant: CCRA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CCRA_A {
    type Ux = u8;
}
impl crate::IsEnum for CCRA_A {}
///Field `CCRA` reader - GTCCRA Buffer Operation
pub type CCRA_R = crate::FieldReader<CCRA_A>;
impl CCRA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCRA_A {
        match self.bits {
            0 => CCRA_A::_00,
            1 => CCRA_A::_01,
            2 => CCRA_A::_10,
            3 => CCRA_A::_11,
            _ => unreachable!(),
        }
    }
    ///Buffer operation is not performed
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CCRA_A::_00
    }
    ///Single buffer operation (GTCCRA <--> GTCCRC)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CCRA_A::_01
    }
    ///Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CCRA_A::_10
    }
    ///Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CCRA_A::_11
    }
}
///Field `CCRA` writer - GTCCRA Buffer Operation
pub type CCRA_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CCRA_A, crate::Safe>;
impl<'a, REG> CCRA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Buffer operation is not performed
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CCRA_A::_00)
    }
    ///Single buffer operation (GTCCRA <--> GTCCRC)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CCRA_A::_01)
    }
    ///Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CCRA_A::_10)
    }
    ///Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CCRA_A::_11)
    }
}
/**GTCCRB Buffer Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCRB_A {
    ///0: Buffer operation is not performed
    _00 = 0,
    ///1: Single buffer operation (GTCCRB <--> GTCCRE)
    _01 = 1,
    ///2: Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)
    _10 = 2,
    ///3: Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)
    _11 = 3,
}
impl From<CCRB_A> for u8 {
    #[inline(always)]
    fn from(variant: CCRB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CCRB_A {
    type Ux = u8;
}
impl crate::IsEnum for CCRB_A {}
///Field `CCRB` reader - GTCCRB Buffer Operation
pub type CCRB_R = crate::FieldReader<CCRB_A>;
impl CCRB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCRB_A {
        match self.bits {
            0 => CCRB_A::_00,
            1 => CCRB_A::_01,
            2 => CCRB_A::_10,
            3 => CCRB_A::_11,
            _ => unreachable!(),
        }
    }
    ///Buffer operation is not performed
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CCRB_A::_00
    }
    ///Single buffer operation (GTCCRB <--> GTCCRE)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CCRB_A::_01
    }
    ///Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CCRB_A::_10
    }
    ///Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CCRB_A::_11
    }
}
///Field `CCRB` writer - GTCCRB Buffer Operation
pub type CCRB_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CCRB_A, crate::Safe>;
impl<'a, REG> CCRB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Buffer operation is not performed
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CCRB_A::_00)
    }
    ///Single buffer operation (GTCCRB <--> GTCCRE)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CCRB_A::_01)
    }
    ///Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CCRB_A::_10)
    }
    ///Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CCRB_A::_11)
    }
}
/**GTPR Buffer Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PR_A {
    ///0: Buffer operation is not performed
    _00 = 0,
    ///1: Single buffer operation (GTPBR --> GTPR)
    _01 = 1,
    ///2: Double buffer operation (GTPDBR --> GTPBR --> GTPR)
    _10 = 2,
    ///3: Double buffer operation (GTPDBR --> GTPBR --> GTPR)
    _11 = 3,
}
impl From<PR_A> for u8 {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PR_A {
    type Ux = u8;
}
impl crate::IsEnum for PR_A {}
///Field `PR` reader - GTPR Buffer Operation
pub type PR_R = crate::FieldReader<PR_A>;
impl PR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PR_A {
        match self.bits {
            0 => PR_A::_00,
            1 => PR_A::_01,
            2 => PR_A::_10,
            3 => PR_A::_11,
            _ => unreachable!(),
        }
    }
    ///Buffer operation is not performed
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PR_A::_00
    }
    ///Single buffer operation (GTPBR --> GTPR)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PR_A::_01
    }
    ///Double buffer operation (GTPDBR --> GTPBR --> GTPR)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PR_A::_10
    }
    ///Double buffer operation (GTPDBR --> GTPBR --> GTPR)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PR_A::_11
    }
}
///Field `PR` writer - GTPR Buffer Operation
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PR_A, crate::Safe>;
impl<'a, REG> PR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Buffer operation is not performed
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::_00)
    }
    ///Single buffer operation (GTPBR --> GTPR)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::_01)
    }
    ///Double buffer operation (GTPDBR --> GTPBR --> GTPR)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::_10)
    }
    ///Double buffer operation (GTPDBR --> GTPBR --> GTPR)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::_11)
    }
}
/**GTCCRA and GTCCRB Forcible Buffer OperationThis bit is read as 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRSWT_A {
    ///0: no effect
    _0 = 0,
    ///1: Forcibly performs buffer transfer of GTCCRA and GTCCRB. This bit automatically returns to 0 after the writing of 1.
    _1 = 1,
}
impl From<CCRSWT_A> for bool {
    #[inline(always)]
    fn from(variant: CCRSWT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCRSWT` writer - GTCCRA and GTCCRB Forcible Buffer OperationThis bit is read as 0.
pub type CCRSWT_W<'a, REG> = crate::BitWriter<'a, REG, CCRSWT_A>;
impl<'a, REG> CCRSWT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCRSWT_A::_0)
    }
    ///Forcibly performs buffer transfer of GTCCRA and GTCCRB. This bit automatically returns to 0 after the writing of 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCRSWT_A::_1)
    }
}
/**GTADTRA Buffer Transfer Timing Select in the Triangle wavesNOTE: In the Saw waves, values other than 0 0: Transfer at an underflow (in down-counting) or overflow (in up-counting) is performed.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADTTA_A {
    ///0: No transfer
    _00 = 0,
    ///1: Transfer at crest
    _01 = 1,
    ///2: Transfer at trough
    _10 = 2,
    ///3: Transfer at both crest and trough
    _11 = 3,
}
impl From<ADTTA_A> for u8 {
    #[inline(always)]
    fn from(variant: ADTTA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADTTA_A {
    type Ux = u8;
}
impl crate::IsEnum for ADTTA_A {}
///Field `ADTTA` reader - GTADTRA Buffer Transfer Timing Select in the Triangle wavesNOTE: In the Saw waves, values other than 0 0: Transfer at an underflow (in down-counting) or overflow (in up-counting) is performed.
pub type ADTTA_R = crate::FieldReader<ADTTA_A>;
impl ADTTA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADTTA_A {
        match self.bits {
            0 => ADTTA_A::_00,
            1 => ADTTA_A::_01,
            2 => ADTTA_A::_10,
            3 => ADTTA_A::_11,
            _ => unreachable!(),
        }
    }
    ///No transfer
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADTTA_A::_00
    }
    ///Transfer at crest
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADTTA_A::_01
    }
    ///Transfer at trough
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADTTA_A::_10
    }
    ///Transfer at both crest and trough
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADTTA_A::_11
    }
}
///Field `ADTTA` writer - GTADTRA Buffer Transfer Timing Select in the Triangle wavesNOTE: In the Saw waves, values other than 0 0: Transfer at an underflow (in down-counting) or overflow (in up-counting) is performed.
pub type ADTTA_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADTTA_A, crate::Safe>;
impl<'a, REG> ADTTA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No transfer
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(ADTTA_A::_00)
    }
    ///Transfer at crest
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(ADTTA_A::_01)
    }
    ///Transfer at trough
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(ADTTA_A::_10)
    }
    ///Transfer at both crest and trough
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(ADTTA_A::_11)
    }
}
/**GTADTRA Double Buffer Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTDA_A {
    ///0: Single buffer operation (GTADTBRA --> GTADTRA)
    _0 = 0,
    ///1: Double buffer operation (GTADTDBRA --> GTADTBRA --> GTADTDRA)
    _1 = 1,
}
impl From<ADTDA_A> for bool {
    #[inline(always)]
    fn from(variant: ADTDA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADTDA` reader - GTADTRA Double Buffer Operation
pub type ADTDA_R = crate::BitReader<ADTDA_A>;
impl ADTDA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADTDA_A {
        match self.bits {
            false => ADTDA_A::_0,
            true => ADTDA_A::_1,
        }
    }
    ///Single buffer operation (GTADTBRA --> GTADTRA)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTDA_A::_0
    }
    ///Double buffer operation (GTADTDBRA --> GTADTBRA --> GTADTDRA)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTDA_A::_1
    }
}
///Field `ADTDA` writer - GTADTRA Double Buffer Operation
pub type ADTDA_W<'a, REG> = crate::BitWriter<'a, REG, ADTDA_A>;
impl<'a, REG> ADTDA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single buffer operation (GTADTBRA --> GTADTRA)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADTDA_A::_0)
    }
    ///Double buffer operation (GTADTDBRA --> GTADTBRA --> GTADTDRA)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADTDA_A::_1)
    }
}
/**GTADTRB Buffer Transfer Timing Select in the Triangle wavesNOTE: In the Saw waves, values other than 0 0: Transfer at an underflow (in down-counting) or overflow (in up-counting) is performed.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADTTB_A {
    ///0: No transfer
    _00 = 0,
    ///1: Transfer at crest
    _01 = 1,
    ///2: Transfer at trough
    _10 = 2,
    ///3: Transfer at both crest and trough
    _11 = 3,
}
impl From<ADTTB_A> for u8 {
    #[inline(always)]
    fn from(variant: ADTTB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADTTB_A {
    type Ux = u8;
}
impl crate::IsEnum for ADTTB_A {}
///Field `ADTTB` reader - GTADTRB Buffer Transfer Timing Select in the Triangle wavesNOTE: In the Saw waves, values other than 0 0: Transfer at an underflow (in down-counting) or overflow (in up-counting) is performed.
pub type ADTTB_R = crate::FieldReader<ADTTB_A>;
impl ADTTB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADTTB_A {
        match self.bits {
            0 => ADTTB_A::_00,
            1 => ADTTB_A::_01,
            2 => ADTTB_A::_10,
            3 => ADTTB_A::_11,
            _ => unreachable!(),
        }
    }
    ///No transfer
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADTTB_A::_00
    }
    ///Transfer at crest
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADTTB_A::_01
    }
    ///Transfer at trough
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADTTB_A::_10
    }
    ///Transfer at both crest and trough
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADTTB_A::_11
    }
}
///Field `ADTTB` writer - GTADTRB Buffer Transfer Timing Select in the Triangle wavesNOTE: In the Saw waves, values other than 0 0: Transfer at an underflow (in down-counting) or overflow (in up-counting) is performed.
pub type ADTTB_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADTTB_A, crate::Safe>;
impl<'a, REG> ADTTB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No transfer
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(ADTTB_A::_00)
    }
    ///Transfer at crest
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(ADTTB_A::_01)
    }
    ///Transfer at trough
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(ADTTB_A::_10)
    }
    ///Transfer at both crest and trough
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(ADTTB_A::_11)
    }
}
/**GTADTRB Double Buffer Operation

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTDB_A {
    ///0: Single buffer operation (GTADTBRB --> GTADTRB)
    _0 = 0,
    ///1: Double buffer operation (GTADTDBRB --> GTADTBRB --> GTADTDRB)
    _1 = 1,
}
impl From<ADTDB_A> for bool {
    #[inline(always)]
    fn from(variant: ADTDB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADTDB` reader - GTADTRB Double Buffer Operation
pub type ADTDB_R = crate::BitReader<ADTDB_A>;
impl ADTDB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADTDB_A {
        match self.bits {
            false => ADTDB_A::_0,
            true => ADTDB_A::_1,
        }
    }
    ///Single buffer operation (GTADTBRB --> GTADTRB)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTDB_A::_0
    }
    ///Double buffer operation (GTADTDBRB --> GTADTBRB --> GTADTDRB)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTDB_A::_1
    }
}
///Field `ADTDB` writer - GTADTRB Double Buffer Operation
pub type ADTDB_W<'a, REG> = crate::BitWriter<'a, REG, ADTDB_A>;
impl<'a, REG> ADTDB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single buffer operation (GTADTBRB --> GTADTRB)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADTDB_A::_0)
    }
    ///Double buffer operation (GTADTDBRB --> GTADTBRB --> GTADTDRB)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADTDB_A::_1)
    }
}
impl R {
    ///Bits 0:3 - BD\[3\]: GTDV Buffer Operation DisableBD\[2\]: GTADTR Buffer Operation DisableBD\[1\]: GTPR Buffer Operation DisableBD\[0\]: GTCCR Buffer Operation Disable
    #[inline(always)]
    pub fn bd(&self) -> BD_R {
        BD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 16:17 - GTCCRA Buffer Operation
    #[inline(always)]
    pub fn ccra(&self) -> CCRA_R {
        CCRA_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - GTCCRB Buffer Operation
    #[inline(always)]
    pub fn ccrb(&self) -> CCRB_R {
        CCRB_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - GTPR Buffer Operation
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:25 - GTADTRA Buffer Transfer Timing Select in the Triangle wavesNOTE: In the Saw waves, values other than 0 0: Transfer at an underflow (in down-counting) or overflow (in up-counting) is performed.
    #[inline(always)]
    pub fn adtta(&self) -> ADTTA_R {
        ADTTA_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - GTADTRA Double Buffer Operation
    #[inline(always)]
    pub fn adtda(&self) -> ADTDA_R {
        ADTDA_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 28:29 - GTADTRB Buffer Transfer Timing Select in the Triangle wavesNOTE: In the Saw waves, values other than 0 0: Transfer at an underflow (in down-counting) or overflow (in up-counting) is performed.
    #[inline(always)]
    pub fn adttb(&self) -> ADTTB_R {
        ADTTB_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - GTADTRB Double Buffer Operation
    #[inline(always)]
    pub fn adtdb(&self) -> ADTDB_R {
        ADTDB_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - BD\[3\]: GTDV Buffer Operation DisableBD\[2\]: GTADTR Buffer Operation DisableBD\[1\]: GTPR Buffer Operation DisableBD\[0\]: GTCCR Buffer Operation Disable
    #[inline(always)]
    pub fn bd(&mut self) -> BD_W<GTBER_SPEC> {
        BD_W::new(self, 0)
    }
    ///Bits 16:17 - GTCCRA Buffer Operation
    #[inline(always)]
    pub fn ccra(&mut self) -> CCRA_W<GTBER_SPEC> {
        CCRA_W::new(self, 16)
    }
    ///Bits 18:19 - GTCCRB Buffer Operation
    #[inline(always)]
    pub fn ccrb(&mut self) -> CCRB_W<GTBER_SPEC> {
        CCRB_W::new(self, 18)
    }
    ///Bits 20:21 - GTPR Buffer Operation
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<GTBER_SPEC> {
        PR_W::new(self, 20)
    }
    ///Bit 22 - GTCCRA and GTCCRB Forcible Buffer OperationThis bit is read as 0.
    #[inline(always)]
    pub fn ccrswt(&mut self) -> CCRSWT_W<GTBER_SPEC> {
        CCRSWT_W::new(self, 22)
    }
    ///Bits 24:25 - GTADTRA Buffer Transfer Timing Select in the Triangle wavesNOTE: In the Saw waves, values other than 0 0: Transfer at an underflow (in down-counting) or overflow (in up-counting) is performed.
    #[inline(always)]
    pub fn adtta(&mut self) -> ADTTA_W<GTBER_SPEC> {
        ADTTA_W::new(self, 24)
    }
    ///Bit 26 - GTADTRA Double Buffer Operation
    #[inline(always)]
    pub fn adtda(&mut self) -> ADTDA_W<GTBER_SPEC> {
        ADTDA_W::new(self, 26)
    }
    ///Bits 28:29 - GTADTRB Buffer Transfer Timing Select in the Triangle wavesNOTE: In the Saw waves, values other than 0 0: Transfer at an underflow (in down-counting) or overflow (in up-counting) is performed.
    #[inline(always)]
    pub fn adttb(&mut self) -> ADTTB_W<GTBER_SPEC> {
        ADTTB_W::new(self, 28)
    }
    ///Bit 30 - GTADTRB Double Buffer Operation
    #[inline(always)]
    pub fn adtdb(&mut self) -> ADTDB_W<GTBER_SPEC> {
        ADTDB_W::new(self, 30)
    }
}
/**General PWM Timer Buffer Enable Register

You can [`read`](crate::Reg::read) this register and get [`gtber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTBER_SPEC;
impl crate::RegisterSpec for GTBER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtber::R`](R) reader structure
impl crate::Readable for GTBER_SPEC {}
///`write(|w| ..)` method takes [`gtber::W`](W) writer structure
impl crate::Writable for GTBER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTBER to value 0
impl crate::Resettable for GTBER_SPEC {}
