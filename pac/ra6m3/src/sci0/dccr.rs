///Register `DCCR` reader
pub type R = crate::R<DCCR_SPEC>;
///Register `DCCR` writer
pub type W = crate::W<DCCR_SPEC>;
/**Data Compare Match Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMF_A {
    ///0: No matched
    _0 = 0,
    ///1: Matched
    _1 = 1,
}
impl From<DCMF_A> for bool {
    #[inline(always)]
    fn from(variant: DCMF_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DCMF` reader - Data Compare Match Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DCMF_R = crate::BitReader<DCMF_A>;
impl DCMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCMF_A {
        match self.bits {
            false => DCMF_A::_0,
            true => DCMF_A::_1,
        }
    }
    ///No matched
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCMF_A::_0
    }
    ///Matched
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCMF_A::_1
    }
}
///Field `DCMF` writer - Data Compare Match Flag
pub type DCMF_W<'a, REG> = crate::BitWriter0C<'a, REG, DCMF_A>;
impl<'a, REG> DCMF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No matched
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DCMF_A::_0)
    }
    ///Matched
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DCMF_A::_1)
    }
}
/**Data Compare Match Parity Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPER_A {
    ///0: No parity error occurred
    _0 = 0,
    ///1: A parity error has occurred
    _1 = 1,
}
impl From<DPER_A> for bool {
    #[inline(always)]
    fn from(variant: DPER_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DPER` reader - Data Compare Match Parity Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DPER_R = crate::BitReader<DPER_A>;
impl DPER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DPER_A {
        match self.bits {
            false => DPER_A::_0,
            true => DPER_A::_1,
        }
    }
    ///No parity error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPER_A::_0
    }
    ///A parity error has occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPER_A::_1
    }
}
///Field `DPER` writer - Data Compare Match Parity Error Flag
pub type DPER_W<'a, REG> = crate::BitWriter0C<'a, REG, DPER_A>;
impl<'a, REG> DPER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No parity error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DPER_A::_0)
    }
    ///A parity error has occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DPER_A::_1)
    }
}
/**Data Compare Match Framing Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFER_A {
    ///0: No framing error occurred
    _0 = 0,
    ///1: A framing error has occurred
    _1 = 1,
}
impl From<DFER_A> for bool {
    #[inline(always)]
    fn from(variant: DFER_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DFER` reader - Data Compare Match Framing Error Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DFER_R = crate::BitReader<DFER_A>;
impl DFER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFER_A {
        match self.bits {
            false => DFER_A::_0,
            true => DFER_A::_1,
        }
    }
    ///No framing error occurred
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFER_A::_0
    }
    ///A framing error has occurred
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFER_A::_1
    }
}
///Field `DFER` writer - Data Compare Match Framing Error Flag
pub type DFER_W<'a, REG> = crate::BitWriter0C<'a, REG, DFER_A>;
impl<'a, REG> DFER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No framing error occurred
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DFER_A::_0)
    }
    ///A framing error has occurred
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DFER_A::_1)
    }
}
/**ID frame select Bit(Valid only in asynchronous mode(including multi-processor)

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDSEL_A {
    ///0: It's always compared data in spite of the value of the MPB bit.
    _0 = 0,
    ///1: It's compared data when the MPB bit is 1 ( ID frame ) only.
    _1 = 1,
}
impl From<IDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IDSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IDSEL` reader - ID frame select Bit(Valid only in asynchronous mode(including multi-processor)
pub type IDSEL_R = crate::BitReader<IDSEL_A>;
impl IDSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDSEL_A {
        match self.bits {
            false => IDSEL_A::_0,
            true => IDSEL_A::_1,
        }
    }
    ///It's always compared data in spite of the value of the MPB bit.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDSEL_A::_0
    }
    ///It's compared data when the MPB bit is 1 ( ID frame ) only.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDSEL_A::_1
    }
}
///Field `IDSEL` writer - ID frame select Bit(Valid only in asynchronous mode(including multi-processor)
pub type IDSEL_W<'a, REG> = crate::BitWriter<'a, REG, IDSEL_A>;
impl<'a, REG> IDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///It's always compared data in spite of the value of the MPB bit.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IDSEL_A::_0)
    }
    ///It's compared data when the MPB bit is 1 ( ID frame ) only.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IDSEL_A::_1)
    }
}
/**Data Compare Match Enable(Valid only in asynchronous mode(including multi-processor)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCME_A {
    ///0: Address match function is disabled.
    _0 = 0,
    ///1: Address match function is enabled
    _1 = 1,
}
impl From<DCME_A> for bool {
    #[inline(always)]
    fn from(variant: DCME_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DCME` reader - Data Compare Match Enable(Valid only in asynchronous mode(including multi-processor)
pub type DCME_R = crate::BitReader<DCME_A>;
impl DCME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCME_A {
        match self.bits {
            false => DCME_A::_0,
            true => DCME_A::_1,
        }
    }
    ///Address match function is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCME_A::_0
    }
    ///Address match function is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCME_A::_1
    }
}
///Field `DCME` writer - Data Compare Match Enable(Valid only in asynchronous mode(including multi-processor)
pub type DCME_W<'a, REG> = crate::BitWriter<'a, REG, DCME_A>;
impl<'a, REG> DCME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Address match function is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DCME_A::_0)
    }
    ///Address match function is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DCME_A::_1)
    }
}
impl R {
    ///Bit 0 - Data Compare Match Flag
    #[inline(always)]
    pub fn dcmf(&self) -> DCMF_R {
        DCMF_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Data Compare Match Parity Error Flag
    #[inline(always)]
    pub fn dper(&self) -> DPER_R {
        DPER_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data Compare Match Framing Error Flag
    #[inline(always)]
    pub fn dfer(&self) -> DFER_R {
        DFER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - ID frame select Bit(Valid only in asynchronous mode(including multi-processor)
    #[inline(always)]
    pub fn idsel(&self) -> IDSEL_R {
        IDSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Data Compare Match Enable(Valid only in asynchronous mode(including multi-processor)
    #[inline(always)]
    pub fn dcme(&self) -> DCME_R {
        DCME_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Data Compare Match Flag
    #[inline(always)]
    pub fn dcmf(&mut self) -> DCMF_W<DCCR_SPEC> {
        DCMF_W::new(self, 0)
    }
    ///Bit 3 - Data Compare Match Parity Error Flag
    #[inline(always)]
    pub fn dper(&mut self) -> DPER_W<DCCR_SPEC> {
        DPER_W::new(self, 3)
    }
    ///Bit 4 - Data Compare Match Framing Error Flag
    #[inline(always)]
    pub fn dfer(&mut self) -> DFER_W<DCCR_SPEC> {
        DFER_W::new(self, 4)
    }
    ///Bit 6 - ID frame select Bit(Valid only in asynchronous mode(including multi-processor)
    #[inline(always)]
    pub fn idsel(&mut self) -> IDSEL_W<DCCR_SPEC> {
        IDSEL_W::new(self, 6)
    }
    ///Bit 7 - Data Compare Match Enable(Valid only in asynchronous mode(including multi-processor)
    #[inline(always)]
    pub fn dcme(&mut self) -> DCME_W<DCCR_SPEC> {
        DCME_W::new(self, 7)
    }
}
/**Data Compare Match Control Register

You can [`read`](crate::Reg::read) this register and get [`dccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DCCR_SPEC;
impl crate::RegisterSpec for DCCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dccr::R`](R) reader structure
impl crate::Readable for DCCR_SPEC {}
///`write(|w| ..)` method takes [`dccr::W`](W) writer structure
impl crate::Writable for DCCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x19;
}
///`reset()` method sets DCCR to value 0x40
impl crate::Resettable for DCCR_SPEC {
    const RESET_VALUE: u8 = 0x40;
}
