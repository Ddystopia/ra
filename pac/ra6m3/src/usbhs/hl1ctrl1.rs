///Register `HL1CTRL1` reader
pub type R = crate::R<HL1CTRL1_SPEC>;
///Register `HL1CTRL1` writer
pub type W = crate::W<HL1CTRL1_SPEC>;
/**L1 Transition Request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQ_A {
    ///0: This bit is cleared to 0 by hardware when the LPM transaction is completed.
    _0 = 0,
    ///1: Set this bit to 1 when requesting a transition to the L1 state.
    _1 = 1,
}
impl From<L1REQ_A> for bool {
    #[inline(always)]
    fn from(variant: L1REQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L1REQ` reader - L1 Transition Request
pub type L1REQ_R = crate::BitReader<L1REQ_A>;
impl L1REQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1REQ_A {
        match self.bits {
            false => L1REQ_A::_0,
            true => L1REQ_A::_1,
        }
    }
    ///This bit is cleared to 0 by hardware when the LPM transaction is completed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1REQ_A::_0
    }
    ///Set this bit to 1 when requesting a transition to the L1 state.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1REQ_A::_1
    }
}
///Field `L1REQ` writer - L1 Transition Request
pub type L1REQ_W<'a, REG> = crate::BitWriter<'a, REG, L1REQ_A>;
impl<'a, REG> L1REQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit is cleared to 0 by hardware when the LPM transaction is completed.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1REQ_A::_0)
    }
    ///Set this bit to 1 when requesting a transition to the L1 state.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1REQ_A::_1)
    }
}
/**L1 Request Completion Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum L1STATUS_A {
    ///0: ACK received
    _00 = 0,
    ///1: NYET received
    _01 = 1,
    ///2: STALL received
    _10 = 2,
    ///3: Transaction error
    _11 = 3,
}
impl From<L1STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: L1STATUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for L1STATUS_A {
    type Ux = u8;
}
impl crate::IsEnum for L1STATUS_A {}
///Field `L1STATUS` reader - L1 Request Completion Status
pub type L1STATUS_R = crate::FieldReader<L1STATUS_A>;
impl L1STATUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1STATUS_A {
        match self.bits {
            0 => L1STATUS_A::_00,
            1 => L1STATUS_A::_01,
            2 => L1STATUS_A::_10,
            3 => L1STATUS_A::_11,
            _ => unreachable!(),
        }
    }
    ///ACK received
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == L1STATUS_A::_00
    }
    ///NYET received
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == L1STATUS_A::_01
    }
    ///STALL received
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == L1STATUS_A::_10
    }
    ///Transaction error
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == L1STATUS_A::_11
    }
}
impl R {
    ///Bit 0 - L1 Transition Request
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - L1 Request Completion Status
    #[inline(always)]
    pub fn l1status(&self) -> L1STATUS_R {
        L1STATUS_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - L1 Transition Request
    #[inline(always)]
    pub fn l1req(&mut self) -> L1REQ_W<HL1CTRL1_SPEC> {
        L1REQ_W::new(self, 0)
    }
}
/**Host L1 Control Register 1

You can [`read`](crate::Reg::read) this register and get [`hl1ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl1ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HL1CTRL1_SPEC;
impl crate::RegisterSpec for HL1CTRL1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`hl1ctrl1::R`](R) reader structure
impl crate::Readable for HL1CTRL1_SPEC {}
///`write(|w| ..)` method takes [`hl1ctrl1::W`](W) writer structure
impl crate::Writable for HL1CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HL1CTRL1 to value 0
impl crate::Resettable for HL1CTRL1_SPEC {}
