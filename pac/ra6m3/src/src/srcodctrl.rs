///Register `SRCODCTRL` reader
pub type R = crate::R<SRCODCTRL_SPEC>;
///Register `SRCODCTRL` writer
pub type W = crate::W<SRCODCTRL_SPEC>;
/**Output FIFO Data Trigger Number

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OFTRG_A {
    ///0: 1
    _00 = 0,
    ///1: 4
    _01 = 1,
    ///2: 8
    _10 = 2,
    ///3: 12
    _11 = 3,
}
impl From<OFTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: OFTRG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OFTRG_A {
    type Ux = u8;
}
impl crate::IsEnum for OFTRG_A {}
///Field `OFTRG` reader - Output FIFO Data Trigger Number
pub type OFTRG_R = crate::FieldReader<OFTRG_A>;
impl OFTRG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OFTRG_A {
        match self.bits {
            0 => OFTRG_A::_00,
            1 => OFTRG_A::_01,
            2 => OFTRG_A::_10,
            3 => OFTRG_A::_11,
            _ => unreachable!(),
        }
    }
    ///1
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OFTRG_A::_00
    }
    ///4
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OFTRG_A::_01
    }
    ///8
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OFTRG_A::_10
    }
    ///12
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OFTRG_A::_11
    }
}
///Field `OFTRG` writer - Output FIFO Data Trigger Number
pub type OFTRG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OFTRG_A, crate::Safe>;
impl<'a, REG> OFTRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(OFTRG_A::_00)
    }
    ///4
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(OFTRG_A::_01)
    }
    ///8
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(OFTRG_A::_10)
    }
    ///12
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(OFTRG_A::_11)
    }
}
/**Output Data FIFO Full Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OEN_A {
    ///0: Output data FIFO full interrupt is disabled.
    _0 = 0,
    ///1: Output data FIFO full interrupt is enabled.
    _1 = 1,
}
impl From<OEN_A> for bool {
    #[inline(always)]
    fn from(variant: OEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OEN` reader - Output Data FIFO Full Interrupt Enable
pub type OEN_R = crate::BitReader<OEN_A>;
impl OEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OEN_A {
        match self.bits {
            false => OEN_A::_0,
            true => OEN_A::_1,
        }
    }
    ///Output data FIFO full interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OEN_A::_0
    }
    ///Output data FIFO full interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OEN_A::_1
    }
}
///Field `OEN` writer - Output Data FIFO Full Interrupt Enable
pub type OEN_W<'a, REG> = crate::BitWriter<'a, REG, OEN_A>;
impl<'a, REG> OEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output data FIFO full interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OEN_A::_0)
    }
    ///Output data FIFO full interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OEN_A::_1)
    }
}
/**Output Data Endian

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OED_A {
    ///0: Endian formats are the same between the chip and input data.
    _0 = 0,
    ///1: Endian formats are different between the chip and input data.
    _1 = 1,
}
impl From<OED_A> for bool {
    #[inline(always)]
    fn from(variant: OED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OED` reader - Output Data Endian
pub type OED_R = crate::BitReader<OED_A>;
impl OED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OED_A {
        match self.bits {
            false => OED_A::_0,
            true => OED_A::_1,
        }
    }
    ///Endian formats are the same between the chip and input data.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OED_A::_0
    }
    ///Endian formats are different between the chip and input data.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OED_A::_1
    }
}
///Field `OED` writer - Output Data Endian
pub type OED_W<'a, REG> = crate::BitWriter<'a, REG, OED_A>;
impl<'a, REG> OED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Endian formats are the same between the chip and input data.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OED_A::_0)
    }
    ///Endian formats are different between the chip and input data.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OED_A::_1)
    }
}
/**Output Data Channel Exchange

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCH_A {
    ///0: Does not exchange the channels (the same order as data input)
    _0 = 0,
    ///1: Exchanges the channels (the opposite order from data input)
    _1 = 1,
}
impl From<OCH_A> for bool {
    #[inline(always)]
    fn from(variant: OCH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OCH` reader - Output Data Channel Exchange
pub type OCH_R = crate::BitReader<OCH_A>;
impl OCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OCH_A {
        match self.bits {
            false => OCH_A::_0,
            true => OCH_A::_1,
        }
    }
    ///Does not exchange the channels (the same order as data input)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCH_A::_0
    }
    ///Exchanges the channels (the opposite order from data input)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCH_A::_1
    }
}
///Field `OCH` writer - Output Data Channel Exchange
pub type OCH_W<'a, REG> = crate::BitWriter<'a, REG, OCH_A>;
impl<'a, REG> OCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not exchange the channels (the same order as data input)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OCH_A::_0)
    }
    ///Exchanges the channels (the opposite order from data input)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OCH_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Output FIFO Data Trigger Number
    #[inline(always)]
    pub fn oftrg(&self) -> OFTRG_R {
        OFTRG_R::new((self.bits & 3) as u8)
    }
    ///Bit 8 - Output Data FIFO Full Interrupt Enable
    #[inline(always)]
    pub fn oen(&self) -> OEN_R {
        OEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output Data Endian
    #[inline(always)]
    pub fn oed(&self) -> OED_R {
        OED_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output Data Channel Exchange
    #[inline(always)]
    pub fn och(&self) -> OCH_R {
        OCH_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Output FIFO Data Trigger Number
    #[inline(always)]
    pub fn oftrg(&mut self) -> OFTRG_W<SRCODCTRL_SPEC> {
        OFTRG_W::new(self, 0)
    }
    ///Bit 8 - Output Data FIFO Full Interrupt Enable
    #[inline(always)]
    pub fn oen(&mut self) -> OEN_W<SRCODCTRL_SPEC> {
        OEN_W::new(self, 8)
    }
    ///Bit 9 - Output Data Endian
    #[inline(always)]
    pub fn oed(&mut self) -> OED_W<SRCODCTRL_SPEC> {
        OED_W::new(self, 9)
    }
    ///Bit 10 - Output Data Channel Exchange
    #[inline(always)]
    pub fn och(&mut self) -> OCH_W<SRCODCTRL_SPEC> {
        OCH_W::new(self, 10)
    }
}
/**Output Data Control Register

You can [`read`](crate::Reg::read) this register and get [`srcodctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcodctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRCODCTRL_SPEC;
impl crate::RegisterSpec for SRCODCTRL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`srcodctrl::R`](R) reader structure
impl crate::Readable for SRCODCTRL_SPEC {}
///`write(|w| ..)` method takes [`srcodctrl::W`](W) writer structure
impl crate::Writable for SRCODCTRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRCODCTRL to value 0
impl crate::Resettable for SRCODCTRL_SPEC {}
