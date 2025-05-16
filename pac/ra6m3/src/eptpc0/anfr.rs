///Register `ANFR` reader
pub type R = crate::R<ANFR_SPEC>;
///Register `ANFR` writer
pub type W = crate::W<ANFR_SPEC>;
/**leap61This bit is used to set the logical value of the leap61 member of timePropertiesDS.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG0_A {
    ///0: leap61 is set to FALSE.
    _0 = 0,
    ///1: leap61 is set to TRUE.
    _1 = 1,
}
impl From<FLAG0_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG0` reader - leap61This bit is used to set the logical value of the leap61 member of timePropertiesDS.
pub type FLAG0_R = crate::BitReader<FLAG0_A>;
impl FLAG0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG0_A {
        match self.bits {
            false => FLAG0_A::_0,
            true => FLAG0_A::_1,
        }
    }
    ///leap61 is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG0_A::_0
    }
    ///leap61 is set to TRUE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG0_A::_1
    }
}
///Field `FLAG0` writer - leap61This bit is used to set the logical value of the leap61 member of timePropertiesDS.
pub type FLAG0_W<'a, REG> = crate::BitWriter<'a, REG, FLAG0_A>;
impl<'a, REG> FLAG0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///leap61 is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG0_A::_0)
    }
    ///leap61 is set to TRUE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG0_A::_1)
    }
}
/**leap59This bit is used to set the logical value of the leap59 member of timePropertiesDS.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG1_A {
    ///0: leap59 is set to FALSE.
    _0 = 0,
    ///1: leap59 is set to TRUE.
    _1 = 1,
}
impl From<FLAG1_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG1` reader - leap59This bit is used to set the logical value of the leap59 member of timePropertiesDS.
pub type FLAG1_R = crate::BitReader<FLAG1_A>;
impl FLAG1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG1_A {
        match self.bits {
            false => FLAG1_A::_0,
            true => FLAG1_A::_1,
        }
    }
    ///leap59 is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG1_A::_0
    }
    ///leap59 is set to TRUE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG1_A::_1
    }
}
///Field `FLAG1` writer - leap59This bit is used to set the logical value of the leap59 member of timePropertiesDS.
pub type FLAG1_W<'a, REG> = crate::BitWriter<'a, REG, FLAG1_A>;
impl<'a, REG> FLAG1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///leap59 is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG1_A::_0)
    }
    ///leap59 is set to TRUE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG1_A::_1)
    }
}
/**currentUtcOffsetValidThis bit is used to set the logical value of the currentUtcOffsetValid member of timePropertiesDS.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG2_A {
    ///0: currentUtcOffsetValid is set to FALSE.
    _0 = 0,
    ///1: currentUtcOffsetValid is set to TRUE.
    _1 = 1,
}
impl From<FLAG2_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG2` reader - currentUtcOffsetValidThis bit is used to set the logical value of the currentUtcOffsetValid member of timePropertiesDS.
pub type FLAG2_R = crate::BitReader<FLAG2_A>;
impl FLAG2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG2_A {
        match self.bits {
            false => FLAG2_A::_0,
            true => FLAG2_A::_1,
        }
    }
    ///currentUtcOffsetValid is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG2_A::_0
    }
    ///currentUtcOffsetValid is set to TRUE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG2_A::_1
    }
}
///Field `FLAG2` writer - currentUtcOffsetValidThis bit is used to set the logical value of the currentUtcOffsetValid member of timePropertiesDS.
pub type FLAG2_W<'a, REG> = crate::BitWriter<'a, REG, FLAG2_A>;
impl<'a, REG> FLAG2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///currentUtcOffsetValid is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG2_A::_0)
    }
    ///currentUtcOffsetValid is set to TRUE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG2_A::_1)
    }
}
/**ptpTimescaleThis bit is used to set the logical value of the ptpTimescale member of timePropertiesDS.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG3_A {
    ///0: ptpTimescale is set to FALSE.
    _0 = 0,
    ///1: ptpTimescale is set to TRUE.
    _1 = 1,
}
impl From<FLAG3_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG3` reader - ptpTimescaleThis bit is used to set the logical value of the ptpTimescale member of timePropertiesDS.
pub type FLAG3_R = crate::BitReader<FLAG3_A>;
impl FLAG3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG3_A {
        match self.bits {
            false => FLAG3_A::_0,
            true => FLAG3_A::_1,
        }
    }
    ///ptpTimescale is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG3_A::_0
    }
    ///ptpTimescale is set to TRUE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG3_A::_1
    }
}
///Field `FLAG3` writer - ptpTimescaleThis bit is used to set the logical value of the ptpTimescale member of timePropertiesDS.
pub type FLAG3_W<'a, REG> = crate::BitWriter<'a, REG, FLAG3_A>;
impl<'a, REG> FLAG3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ptpTimescale is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG3_A::_0)
    }
    ///ptpTimescale is set to TRUE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG3_A::_1)
    }
}
/**timeTraceableThis bit is used to set the logical value of the timeTraceable member of timePropertiesDS.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG4_A {
    ///0: timeTraceable is set to FALSE.
    _0 = 0,
    ///1: timeTraceable is set to TRUE.
    _1 = 1,
}
impl From<FLAG4_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG4` reader - timeTraceableThis bit is used to set the logical value of the timeTraceable member of timePropertiesDS.
pub type FLAG4_R = crate::BitReader<FLAG4_A>;
impl FLAG4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG4_A {
        match self.bits {
            false => FLAG4_A::_0,
            true => FLAG4_A::_1,
        }
    }
    ///timeTraceable is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG4_A::_0
    }
    ///timeTraceable is set to TRUE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG4_A::_1
    }
}
///Field `FLAG4` writer - timeTraceableThis bit is used to set the logical value of the timeTraceable member of timePropertiesDS.
pub type FLAG4_W<'a, REG> = crate::BitWriter<'a, REG, FLAG4_A>;
impl<'a, REG> FLAG4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///timeTraceable is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG4_A::_0)
    }
    ///timeTraceable is set to TRUE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG4_A::_1)
    }
}
/**frequencyTraceableThis bit is used to set the logical value of the frequencyTraceable member of timePropertiesDS.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG5_A {
    ///0: frequencyTraceable is set to FALSE.
    _0 = 0,
    ///1: frequencyTraceable is set to TRUE.
    _1 = 1,
}
impl From<FLAG5_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG5` reader - frequencyTraceableThis bit is used to set the logical value of the frequencyTraceable member of timePropertiesDS.
pub type FLAG5_R = crate::BitReader<FLAG5_A>;
impl FLAG5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG5_A {
        match self.bits {
            false => FLAG5_A::_0,
            true => FLAG5_A::_1,
        }
    }
    ///frequencyTraceable is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG5_A::_0
    }
    ///frequencyTraceable is set to TRUE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG5_A::_1
    }
}
///Field `FLAG5` writer - frequencyTraceableThis bit is used to set the logical value of the frequencyTraceable member of timePropertiesDS.
pub type FLAG5_W<'a, REG> = crate::BitWriter<'a, REG, FLAG5_A>;
impl<'a, REG> FLAG5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///frequencyTraceable is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG5_A::_0)
    }
    ///frequencyTraceable is set to TRUE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG5_A::_1)
    }
}
/**alternateMasterFlag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG8_A {
    ///0: alternateMasterFlag is set to FALSE.
    _0 = 0,
    ///1: alternateMasterFlag is set to TRUE.
    _1 = 1,
}
impl From<FLAG8_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG8_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG8` reader - alternateMasterFlag
pub type FLAG8_R = crate::BitReader<FLAG8_A>;
impl FLAG8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG8_A {
        match self.bits {
            false => FLAG8_A::_0,
            true => FLAG8_A::_1,
        }
    }
    ///alternateMasterFlag is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG8_A::_0
    }
    ///alternateMasterFlag is set to TRUE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG8_A::_1
    }
}
///Field `FLAG8` writer - alternateMasterFlag
pub type FLAG8_W<'a, REG> = crate::BitWriter<'a, REG, FLAG8_A>;
impl<'a, REG> FLAG8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///alternateMasterFlag is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG8_A::_0)
    }
    ///alternateMasterFlag is set to TRUE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG8_A::_1)
    }
}
/**unicastFlag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG10_A {
    ///0: unicastFlag is set to FALSE.
    _0 = 0,
    ///1: unicastFlag is set to TRUE.
    _1 = 1,
}
impl From<FLAG10_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG10` reader - unicastFlag
pub type FLAG10_R = crate::BitReader<FLAG10_A>;
impl FLAG10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG10_A {
        match self.bits {
            false => FLAG10_A::_0,
            true => FLAG10_A::_1,
        }
    }
    ///unicastFlag is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG10_A::_0
    }
    ///unicastFlag is set to TRUE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG10_A::_1
    }
}
///Field `FLAG10` writer - unicastFlag
pub type FLAG10_W<'a, REG> = crate::BitWriter<'a, REG, FLAG10_A>;
impl<'a, REG> FLAG10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///unicastFlag is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG10_A::_0)
    }
    ///unicastFlag is set to TRUE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG10_A::_1)
    }
}
/**PTP profile Specific 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG13_A {
    ///0: PTP profile Specific 1 is set to FALSE.
    _0 = 0,
    ///1: PTP profile Specific 1 is set to TRUE.
    _1 = 1,
}
impl From<FLAG13_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG13` reader - PTP profile Specific 1
pub type FLAG13_R = crate::BitReader<FLAG13_A>;
impl FLAG13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG13_A {
        match self.bits {
            false => FLAG13_A::_0,
            true => FLAG13_A::_1,
        }
    }
    ///PTP profile Specific 1 is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG13_A::_0
    }
    ///PTP profile Specific 1 is set to TRUE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG13_A::_1
    }
}
///Field `FLAG13` writer - PTP profile Specific 1
pub type FLAG13_W<'a, REG> = crate::BitWriter<'a, REG, FLAG13_A>;
impl<'a, REG> FLAG13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PTP profile Specific 1 is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG13_A::_0)
    }
    ///PTP profile Specific 1 is set to TRUE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG13_A::_1)
    }
}
/**PTP profile Specific 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLAG14_A {
    ///0: PTP profile Specific 2 is set to FALSE.
    _0 = 0,
    ///1: PTP profile Specific 2 is set to TRUE.
    _1 = 1,
}
impl From<FLAG14_A> for bool {
    #[inline(always)]
    fn from(variant: FLAG14_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLAG14` reader - PTP profile Specific 2
pub type FLAG14_R = crate::BitReader<FLAG14_A>;
impl FLAG14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLAG14_A {
        match self.bits {
            false => FLAG14_A::_0,
            true => FLAG14_A::_1,
        }
    }
    ///PTP profile Specific 2 is set to FALSE.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLAG14_A::_0
    }
    ///PTP profile Specific 2 is set to TRUE.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLAG14_A::_1
    }
}
///Field `FLAG14` writer - PTP profile Specific 2
pub type FLAG14_W<'a, REG> = crate::BitWriter<'a, REG, FLAG14_A>;
impl<'a, REG> FLAG14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PTP profile Specific 2 is set to FALSE.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG14_A::_0)
    }
    ///PTP profile Specific 2 is set to TRUE.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLAG14_A::_1)
    }
}
impl R {
    ///Bit 0 - leap61This bit is used to set the logical value of the leap61 member of timePropertiesDS.
    #[inline(always)]
    pub fn flag0(&self) -> FLAG0_R {
        FLAG0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - leap59This bit is used to set the logical value of the leap59 member of timePropertiesDS.
    #[inline(always)]
    pub fn flag1(&self) -> FLAG1_R {
        FLAG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - currentUtcOffsetValidThis bit is used to set the logical value of the currentUtcOffsetValid member of timePropertiesDS.
    #[inline(always)]
    pub fn flag2(&self) -> FLAG2_R {
        FLAG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ptpTimescaleThis bit is used to set the logical value of the ptpTimescale member of timePropertiesDS.
    #[inline(always)]
    pub fn flag3(&self) -> FLAG3_R {
        FLAG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - timeTraceableThis bit is used to set the logical value of the timeTraceable member of timePropertiesDS.
    #[inline(always)]
    pub fn flag4(&self) -> FLAG4_R {
        FLAG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - frequencyTraceableThis bit is used to set the logical value of the frequencyTraceable member of timePropertiesDS.
    #[inline(always)]
    pub fn flag5(&self) -> FLAG5_R {
        FLAG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - alternateMasterFlag
    #[inline(always)]
    pub fn flag8(&self) -> FLAG8_R {
        FLAG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - unicastFlag
    #[inline(always)]
    pub fn flag10(&self) -> FLAG10_R {
        FLAG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - PTP profile Specific 1
    #[inline(always)]
    pub fn flag13(&self) -> FLAG13_R {
        FLAG13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PTP profile Specific 2
    #[inline(always)]
    pub fn flag14(&self) -> FLAG14_R {
        FLAG14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - leap61This bit is used to set the logical value of the leap61 member of timePropertiesDS.
    #[inline(always)]
    pub fn flag0(&mut self) -> FLAG0_W<ANFR_SPEC> {
        FLAG0_W::new(self, 0)
    }
    ///Bit 1 - leap59This bit is used to set the logical value of the leap59 member of timePropertiesDS.
    #[inline(always)]
    pub fn flag1(&mut self) -> FLAG1_W<ANFR_SPEC> {
        FLAG1_W::new(self, 1)
    }
    ///Bit 2 - currentUtcOffsetValidThis bit is used to set the logical value of the currentUtcOffsetValid member of timePropertiesDS.
    #[inline(always)]
    pub fn flag2(&mut self) -> FLAG2_W<ANFR_SPEC> {
        FLAG2_W::new(self, 2)
    }
    ///Bit 3 - ptpTimescaleThis bit is used to set the logical value of the ptpTimescale member of timePropertiesDS.
    #[inline(always)]
    pub fn flag3(&mut self) -> FLAG3_W<ANFR_SPEC> {
        FLAG3_W::new(self, 3)
    }
    ///Bit 4 - timeTraceableThis bit is used to set the logical value of the timeTraceable member of timePropertiesDS.
    #[inline(always)]
    pub fn flag4(&mut self) -> FLAG4_W<ANFR_SPEC> {
        FLAG4_W::new(self, 4)
    }
    ///Bit 5 - frequencyTraceableThis bit is used to set the logical value of the frequencyTraceable member of timePropertiesDS.
    #[inline(always)]
    pub fn flag5(&mut self) -> FLAG5_W<ANFR_SPEC> {
        FLAG5_W::new(self, 5)
    }
    ///Bit 8 - alternateMasterFlag
    #[inline(always)]
    pub fn flag8(&mut self) -> FLAG8_W<ANFR_SPEC> {
        FLAG8_W::new(self, 8)
    }
    ///Bit 10 - unicastFlag
    #[inline(always)]
    pub fn flag10(&mut self) -> FLAG10_W<ANFR_SPEC> {
        FLAG10_W::new(self, 10)
    }
    ///Bit 13 - PTP profile Specific 1
    #[inline(always)]
    pub fn flag13(&mut self) -> FLAG13_W<ANFR_SPEC> {
        FLAG13_W::new(self, 13)
    }
    ///Bit 14 - PTP profile Specific 2
    #[inline(always)]
    pub fn flag14(&mut self) -> FLAG14_W<ANFR_SPEC> {
        FLAG14_W::new(self, 14)
    }
}
/**Announce Message Flag Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`anfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ANFR_SPEC;
impl crate::RegisterSpec for ANFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`anfr::R`](R) reader structure
impl crate::Readable for ANFR_SPEC {}
///`write(|w| ..)` method takes [`anfr::W`](W) writer structure
impl crate::Writable for ANFR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ANFR to value 0
impl crate::Resettable for ANFR_SPEC {}
