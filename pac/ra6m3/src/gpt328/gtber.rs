///Register `GTBER` reader
pub type R = crate::R<GTBER_SPEC>;
///Register `GTBER` writer
pub type W = crate::W<GTBER_SPEC>;
/**GTCCR Buffer Operation Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BD2_A {
    ///0: Enable buffer operation
    _0 = 0,
    ///1: Disable buffer operation.
    _1 = 1,
}
impl From<BD2_A> for bool {
    #[inline(always)]
    fn from(variant: BD2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BD2` reader - GTCCR Buffer Operation Disable
pub type BD2_R = crate::BitReader<BD2_A>;
impl BD2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BD2_A {
        match self.bits {
            false => BD2_A::_0,
            true => BD2_A::_1,
        }
    }
    ///Enable buffer operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BD2_A::_0
    }
    ///Disable buffer operation.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BD2_A::_1
    }
}
///Field `BD2` writer - GTCCR Buffer Operation Disable
pub type BD2_W<'a, REG> = crate::BitWriter<'a, REG, BD2_A>;
impl<'a, REG> BD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable buffer operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BD2_A::_0)
    }
    ///Disable buffer operation.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BD2_A::_1)
    }
}
/**GTPR Buffer Operation Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BD1_A {
    ///0: Enable buffer operation
    _0 = 0,
    ///1: Disable buffer operation.
    _1 = 1,
}
impl From<BD1_A> for bool {
    #[inline(always)]
    fn from(variant: BD1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BD1` reader - GTPR Buffer Operation Disable
pub type BD1_R = crate::BitReader<BD1_A>;
impl BD1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BD1_A {
        match self.bits {
            false => BD1_A::_0,
            true => BD1_A::_1,
        }
    }
    ///Enable buffer operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BD1_A::_0
    }
    ///Disable buffer operation.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BD1_A::_1
    }
}
///Field `BD1` writer - GTPR Buffer Operation Disable
pub type BD1_W<'a, REG> = crate::BitWriter<'a, REG, BD1_A>;
impl<'a, REG> BD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable buffer operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BD1_A::_0)
    }
    ///Disable buffer operation.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BD1_A::_1)
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
    ///2: Setting prohibited
    OTHERS = 2,
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
            _ => PR_A::OTHERS,
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
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), PR_A::OTHERS)
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
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::OTHERS)
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
impl R {
    ///Bit 0 - GTCCR Buffer Operation Disable
    #[inline(always)]
    pub fn bd2(&self) -> BD2_R {
        BD2_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTPR Buffer Operation Disable
    #[inline(always)]
    pub fn bd1(&self) -> BD1_R {
        BD1_R::new(((self.bits >> 1) & 1) != 0)
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
}
impl W {
    ///Bit 0 - GTCCR Buffer Operation Disable
    #[inline(always)]
    pub fn bd2(&mut self) -> BD2_W<GTBER_SPEC> {
        BD2_W::new(self, 0)
    }
    ///Bit 1 - GTPR Buffer Operation Disable
    #[inline(always)]
    pub fn bd1(&mut self) -> BD1_W<GTBER_SPEC> {
        BD1_W::new(self, 1)
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
