///Register `SDTR` reader
pub type R = crate::R<SDTR_SPEC>;
///Register `SDTR` writer
pub type W = crate::W<SDTR_SPEC>;
/**SDRAMC Column Latency

Value on reset: 2*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CL_A {
    ///1: 1 cycle
    _001 = 1,
    ///2: 2 cycles
    _010 = 2,
    ///3: 3 cycles
    _011 = 3,
    ///0: Setting prohibited
    OTHERS = 0,
}
impl From<CL_A> for u8 {
    #[inline(always)]
    fn from(variant: CL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CL_A {
    type Ux = u8;
}
impl crate::IsEnum for CL_A {}
///Field `CL` reader - SDRAMC Column Latency
pub type CL_R = crate::FieldReader<CL_A>;
impl CL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CL_A {
        match self.bits {
            1 => CL_A::_001,
            2 => CL_A::_010,
            3 => CL_A::_011,
            _ => CL_A::OTHERS,
        }
    }
    ///1 cycle
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CL_A::_001
    }
    ///2 cycles
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CL_A::_010
    }
    ///3 cycles
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CL_A::_011
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CL_A::OTHERS)
    }
}
///Field `CL` writer - SDRAMC Column Latency
pub type CL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CL_A, crate::Safe>;
impl<'a, REG> CL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 cycle
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(CL_A::_001)
    }
    ///2 cycles
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(CL_A::_010)
    }
    ///3 cycles
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(CL_A::_011)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CL_A::OTHERS)
    }
}
/**Write Recovery Interval

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WR_A {
    ///0: 1 cycle
    _0 = 0,
    ///1: 2 cycles
    _1 = 1,
}
impl From<WR_A> for bool {
    #[inline(always)]
    fn from(variant: WR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WR` reader - Write Recovery Interval
pub type WR_R = crate::BitReader<WR_A>;
impl WR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WR_A {
        match self.bits {
            false => WR_A::_0,
            true => WR_A::_1,
        }
    }
    ///1 cycle
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WR_A::_0
    }
    ///2 cycles
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WR_A::_1
    }
}
///Field `WR` writer - Write Recovery Interval
pub type WR_W<'a, REG> = crate::BitWriter<'a, REG, WR_A>;
impl<'a, REG> WR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///1 cycle
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WR_A::_0)
    }
    ///2 cycles
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WR_A::_1)
    }
}
///Field `RP` reader - Row Precharge Interval ( RP+1 cycles )
pub type RP_R = crate::FieldReader;
///Field `RP` writer - Row Precharge Interval ( RP+1 cycles )
pub type RP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RCD` reader - Row Column Latency ( RCD+1 cycles )
pub type RCD_R = crate::FieldReader;
///Field `RCD` writer - Row Column Latency ( RCD+1 cycles )
pub type RCD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Row Active Interval

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAS_A {
    ///0: 1 cycle
    _000 = 0,
    ///1: 2 cycles
    _001 = 1,
    ///2: 3 cycles
    _010 = 2,
    ///3: 4 cycles
    _011 = 3,
    ///4: 5 cycles
    _100 = 4,
    ///5: 6 cycles
    _101 = 5,
    ///6: 7 cycles
    _110 = 6,
    ///7: Setting prohibited
    _111 = 7,
}
impl From<RAS_A> for u8 {
    #[inline(always)]
    fn from(variant: RAS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RAS_A {
    type Ux = u8;
}
impl crate::IsEnum for RAS_A {}
///Field `RAS` reader - Row Active Interval
pub type RAS_R = crate::FieldReader<RAS_A>;
impl RAS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RAS_A {
        match self.bits {
            0 => RAS_A::_000,
            1 => RAS_A::_001,
            2 => RAS_A::_010,
            3 => RAS_A::_011,
            4 => RAS_A::_100,
            5 => RAS_A::_101,
            6 => RAS_A::_110,
            7 => RAS_A::_111,
            _ => unreachable!(),
        }
    }
    ///1 cycle
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RAS_A::_000
    }
    ///2 cycles
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RAS_A::_001
    }
    ///3 cycles
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RAS_A::_010
    }
    ///4 cycles
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RAS_A::_011
    }
    ///5 cycles
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RAS_A::_100
    }
    ///6 cycles
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RAS_A::_101
    }
    ///7 cycles
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RAS_A::_110
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RAS_A::_111
    }
}
///Field `RAS` writer - Row Active Interval
pub type RAS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RAS_A, crate::Safe>;
impl<'a, REG> RAS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 cycle
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(RAS_A::_000)
    }
    ///2 cycles
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(RAS_A::_001)
    }
    ///3 cycles
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(RAS_A::_010)
    }
    ///4 cycles
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(RAS_A::_011)
    }
    ///5 cycles
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(RAS_A::_100)
    }
    ///6 cycles
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(RAS_A::_101)
    }
    ///7 cycles
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(RAS_A::_110)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(RAS_A::_111)
    }
}
impl R {
    ///Bits 0:2 - SDRAMC Column Latency
    #[inline(always)]
    pub fn cl(&self) -> CL_R {
        CL_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - Write Recovery Interval
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:11 - Row Precharge Interval ( RP+1 cycles )
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:13 - Row Column Latency ( RCD+1 cycles )
    #[inline(always)]
    pub fn rcd(&self) -> RCD_R {
        RCD_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:18 - Row Active Interval
    #[inline(always)]
    pub fn ras(&self) -> RAS_R {
        RAS_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - SDRAMC Column Latency
    #[inline(always)]
    pub fn cl(&mut self) -> CL_W<SDTR_SPEC> {
        CL_W::new(self, 0)
    }
    ///Bit 8 - Write Recovery Interval
    #[inline(always)]
    pub fn wr(&mut self) -> WR_W<SDTR_SPEC> {
        WR_W::new(self, 8)
    }
    ///Bits 9:11 - Row Precharge Interval ( RP+1 cycles )
    #[inline(always)]
    pub fn rp(&mut self) -> RP_W<SDTR_SPEC> {
        RP_W::new(self, 9)
    }
    ///Bits 12:13 - Row Column Latency ( RCD+1 cycles )
    #[inline(always)]
    pub fn rcd(&mut self) -> RCD_W<SDTR_SPEC> {
        RCD_W::new(self, 12)
    }
    ///Bits 16:18 - Row Active Interval
    #[inline(always)]
    pub fn ras(&mut self) -> RAS_W<SDTR_SPEC> {
        RAS_W::new(self, 16)
    }
}
/**SDRAM Timing Register

You can [`read`](crate::Reg::read) this register and get [`sdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDTR_SPEC;
impl crate::RegisterSpec for SDTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sdtr::R`](R) reader structure
impl crate::Readable for SDTR_SPEC {}
///`write(|w| ..)` method takes [`sdtr::W`](W) writer structure
impl crate::Writable for SDTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDTR to value 0x02
impl crate::Resettable for SDTR_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
