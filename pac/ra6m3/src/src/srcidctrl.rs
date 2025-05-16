///Register `SRCIDCTRL` reader
pub type R = crate::R<SRCIDCTRL_SPEC>;
///Register `SRCIDCTRL` writer
pub type W = crate::W<SRCIDCTRL_SPEC>;
/**Input FIFO Data Triggering Number

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IFTRG_A {
    ///0: 0
    _00 = 0,
    ///1: 2
    _01 = 1,
    ///2: 4
    _10 = 2,
    ///3: 6
    _11 = 3,
}
impl From<IFTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: IFTRG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IFTRG_A {
    type Ux = u8;
}
impl crate::IsEnum for IFTRG_A {}
///Field `IFTRG` reader - Input FIFO Data Triggering Number
pub type IFTRG_R = crate::FieldReader<IFTRG_A>;
impl IFTRG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IFTRG_A {
        match self.bits {
            0 => IFTRG_A::_00,
            1 => IFTRG_A::_01,
            2 => IFTRG_A::_10,
            3 => IFTRG_A::_11,
            _ => unreachable!(),
        }
    }
    ///0
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IFTRG_A::_00
    }
    ///2
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IFTRG_A::_01
    }
    ///4
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IFTRG_A::_10
    }
    ///6
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IFTRG_A::_11
    }
}
///Field `IFTRG` writer - Input FIFO Data Triggering Number
pub type IFTRG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IFTRG_A, crate::Safe>;
impl<'a, REG> IFTRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///0
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(IFTRG_A::_00)
    }
    ///2
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(IFTRG_A::_01)
    }
    ///4
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(IFTRG_A::_10)
    }
    ///6
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(IFTRG_A::_11)
    }
}
/**Input FIFO Empty Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN_A {
    ///0: Input FIFO empty interrupt is disabled.
    _0 = 0,
    ///1: Input FIFO empty interrupt is enabled.
    _1 = 1,
}
impl From<IEN_A> for bool {
    #[inline(always)]
    fn from(variant: IEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IEN` reader - Input FIFO Empty Interrupt Enable
pub type IEN_R = crate::BitReader<IEN_A>;
impl IEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IEN_A {
        match self.bits {
            false => IEN_A::_0,
            true => IEN_A::_1,
        }
    }
    ///Input FIFO empty interrupt is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEN_A::_0
    }
    ///Input FIFO empty interrupt is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEN_A::_1
    }
}
///Field `IEN` writer - Input FIFO Empty Interrupt Enable
pub type IEN_W<'a, REG> = crate::BitWriter<'a, REG, IEN_A>;
impl<'a, REG> IEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input FIFO empty interrupt is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IEN_A::_0)
    }
    ///Input FIFO empty interrupt is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IEN_A::_1)
    }
}
/**Input Data Endian

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IED_A {
    ///0: Endian formats 1 are the same between the CPU and input data.
    _0 = 0,
    ///1: Endian formats 1 are different between the CPU and input data.
    _1 = 1,
}
impl From<IED_A> for bool {
    #[inline(always)]
    fn from(variant: IED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IED` reader - Input Data Endian
pub type IED_R = crate::BitReader<IED_A>;
impl IED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IED_A {
        match self.bits {
            false => IED_A::_0,
            true => IED_A::_1,
        }
    }
    ///Endian formats 1 are the same between the CPU and input data.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IED_A::_0
    }
    ///Endian formats 1 are different between the CPU and input data.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IED_A::_1
    }
}
///Field `IED` writer - Input Data Endian
pub type IED_W<'a, REG> = crate::BitWriter<'a, REG, IED_A>;
impl<'a, REG> IED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Endian formats 1 are the same between the CPU and input data.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IED_A::_0)
    }
    ///Endian formats 1 are different between the CPU and input data.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IED_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Input FIFO Data Triggering Number
    #[inline(always)]
    pub fn iftrg(&self) -> IFTRG_R {
        IFTRG_R::new((self.bits & 3) as u8)
    }
    ///Bit 8 - Input FIFO Empty Interrupt Enable
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Input Data Endian
    #[inline(always)]
    pub fn ied(&self) -> IED_R {
        IED_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Input FIFO Data Triggering Number
    #[inline(always)]
    pub fn iftrg(&mut self) -> IFTRG_W<SRCIDCTRL_SPEC> {
        IFTRG_W::new(self, 0)
    }
    ///Bit 8 - Input FIFO Empty Interrupt Enable
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W<SRCIDCTRL_SPEC> {
        IEN_W::new(self, 8)
    }
    ///Bit 9 - Input Data Endian
    #[inline(always)]
    pub fn ied(&mut self) -> IED_W<SRCIDCTRL_SPEC> {
        IED_W::new(self, 9)
    }
}
/**Input Data Control Register

You can [`read`](crate::Reg::read) this register and get [`srcidctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcidctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRCIDCTRL_SPEC;
impl crate::RegisterSpec for SRCIDCTRL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`srcidctrl::R`](R) reader structure
impl crate::Readable for SRCIDCTRL_SPEC {}
///`write(|w| ..)` method takes [`srcidctrl::W`](W) writer structure
impl crate::Writable for SRCIDCTRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRCIDCTRL to value 0
impl crate::Resettable for SRCIDCTRL_SPEC {}
