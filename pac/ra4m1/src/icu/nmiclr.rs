///Register `NMICLR` reader
pub type R = crate::R<NMICLR_SPEC>;
///Register `NMICLR` writer
pub type W = crate::W<NMICLR_SPEC>;
/**IWDT Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTCLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the NMISR.IWDTST flag.
    _1 = 1,
}
impl From<IWDTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDTCLR` writer - IWDT Clear
pub type IWDTCLR_W<'a, REG> = crate::BitWriter<'a, REG, IWDTCLR_A>;
impl<'a, REG> IWDTCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IWDTCLR_A::_0)
    }
    ///Clear the NMISR.IWDTST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IWDTCLR_A::_1)
    }
}
/**WDT Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the NMISR.WDTST flag.
    _1 = 1,
}
impl From<WDTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WDTCLR` writer - WDT Clear
pub type WDTCLR_W<'a, REG> = crate::BitWriter<'a, REG, WDTCLR_A>;
impl<'a, REG> WDTCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCLR_A::_0)
    }
    ///Clear the NMISR.WDTST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCLR_A::_1)
    }
}
/**LVD1 Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1CLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the NMISR.LVD1ST flag.
    _1 = 1,
}
impl From<LVD1CLR_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1CLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD1CLR` writer - LVD1 Clear
pub type LVD1CLR_W<'a, REG> = crate::BitWriter<'a, REG, LVD1CLR_A>;
impl<'a, REG> LVD1CLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1CLR_A::_0)
    }
    ///Clear the NMISR.LVD1ST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1CLR_A::_1)
    }
}
/**LVD2 Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2CLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the NMISR.LVD2ST flag.
    _1 = 1,
}
impl From<LVD2CLR_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2CLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD2CLR` writer - LVD2 Clear
pub type LVD2CLR_W<'a, REG> = crate::BitWriter<'a, REG, LVD2CLR_A>;
impl<'a, REG> LVD2CLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2CLR_A::_0)
    }
    ///Clear the NMISR.LVD2ST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2CLR_A::_1)
    }
}
/**VBATT Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATTCLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the NMISR.VBATTST flag.
    _1 = 1,
}
impl From<VBATTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: VBATTCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VBATTCLR` writer - VBATT Clear
pub type VBATTCLR_W<'a, REG> = crate::BitWriter<'a, REG, VBATTCLR_A>;
impl<'a, REG> VBATTCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBATTCLR_A::_0)
    }
    ///Clear the NMISR.VBATTST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBATTCLR_A::_1)
    }
}
/**OST Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTCLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the NMISR.OSTST flag.
    _1 = 1,
}
impl From<OSTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: OSTCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OSTCLR` writer - OST Clear
pub type OSTCLR_W<'a, REG> = crate::BitWriter<'a, REG, OSTCLR_A>;
impl<'a, REG> OSTCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTCLR_A::_0)
    }
    ///Clear the NMISR.OSTST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTCLR_A::_1)
    }
}
/**NMI Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMICLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the NMISR.NMIST flag.
    _1 = 1,
}
impl From<NMICLR_A> for bool {
    #[inline(always)]
    fn from(variant: NMICLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NMICLR` writer - NMI Clear
pub type NMICLR_W<'a, REG> = crate::BitWriter<'a, REG, NMICLR_A>;
impl<'a, REG> NMICLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NMICLR_A::_0)
    }
    ///Clear the NMISR.NMIST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NMICLR_A::_1)
    }
}
/**SRAM Parity Error Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPECLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the NMISR.RPEST flag.
    _1 = 1,
}
impl From<RPECLR_A> for bool {
    #[inline(always)]
    fn from(variant: RPECLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPECLR` writer - SRAM Parity Error Clear
pub type RPECLR_W<'a, REG> = crate::BitWriter<'a, REG, RPECLR_A>;
impl<'a, REG> RPECLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPECLR_A::_0)
    }
    ///Clear the NMISR.RPEST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPECLR_A::_1)
    }
}
/**SRAM ECC Error Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECCCLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the NMISR.RECCST flag.
    _1 = 1,
}
impl From<RECCCLR_A> for bool {
    #[inline(always)]
    fn from(variant: RECCCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RECCCLR` writer - SRAM ECC Error Clear
pub type RECCCLR_W<'a, REG> = crate::BitWriter<'a, REG, RECCCLR_A>;
impl<'a, REG> RECCCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RECCCLR_A::_0)
    }
    ///Clear the NMISR.RECCST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RECCCLR_A::_1)
    }
}
/**Bus Slave Error Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSSCLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the NMISR.BUSSST flag.
    _1 = 1,
}
impl From<BUSSCLR_A> for bool {
    #[inline(always)]
    fn from(variant: BUSSCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSSCLR` writer - Bus Slave Error Clear
pub type BUSSCLR_W<'a, REG> = crate::BitWriter<'a, REG, BUSSCLR_A>;
impl<'a, REG> BUSSCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BUSSCLR_A::_0)
    }
    ///Clear the NMISR.BUSSST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BUSSCLR_A::_1)
    }
}
/**Bus Master Error Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSMCLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the NMISR.BUSMST flag.
    _1 = 1,
}
impl From<BUSMCLR_A> for bool {
    #[inline(always)]
    fn from(variant: BUSMCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSMCLR` writer - Bus Master Error Clear
pub type BUSMCLR_W<'a, REG> = crate::BitWriter<'a, REG, BUSMCLR_A>;
impl<'a, REG> BUSMCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BUSMCLR_A::_0)
    }
    ///Clear the NMISR.BUSMST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BUSMCLR_A::_1)
    }
}
/**CPU Stack Pointer Monitor Interrupt Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPECLR_A {
    ///0: No effect.
    _0 = 0,
    ///1: Clear the NMISR.SPEST flag.
    _1 = 1,
}
impl From<SPECLR_A> for bool {
    #[inline(always)]
    fn from(variant: SPECLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPECLR` writer - CPU Stack Pointer Monitor Interrupt Clear
pub type SPECLR_W<'a, REG> = crate::BitWriter<'a, REG, SPECLR_A>;
impl<'a, REG> SPECLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPECLR_A::_0)
    }
    ///Clear the NMISR.SPEST flag.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPECLR_A::_1)
    }
}
impl W {
    ///Bit 0 - IWDT Clear
    #[inline(always)]
    pub fn iwdtclr(&mut self) -> IWDTCLR_W<'_, NMICLR_SPEC> {
        IWDTCLR_W::new(self, 0)
    }
    ///Bit 1 - WDT Clear
    #[inline(always)]
    pub fn wdtclr(&mut self) -> WDTCLR_W<'_, NMICLR_SPEC> {
        WDTCLR_W::new(self, 1)
    }
    ///Bit 2 - LVD1 Clear
    #[inline(always)]
    pub fn lvd1clr(&mut self) -> LVD1CLR_W<'_, NMICLR_SPEC> {
        LVD1CLR_W::new(self, 2)
    }
    ///Bit 3 - LVD2 Clear
    #[inline(always)]
    pub fn lvd2clr(&mut self) -> LVD2CLR_W<'_, NMICLR_SPEC> {
        LVD2CLR_W::new(self, 3)
    }
    ///Bit 4 - VBATT Clear
    #[inline(always)]
    pub fn vbattclr(&mut self) -> VBATTCLR_W<'_, NMICLR_SPEC> {
        VBATTCLR_W::new(self, 4)
    }
    ///Bit 6 - OST Clear
    #[inline(always)]
    pub fn ostclr(&mut self) -> OSTCLR_W<'_, NMICLR_SPEC> {
        OSTCLR_W::new(self, 6)
    }
    ///Bit 7 - NMI Clear
    #[inline(always)]
    pub fn nmiclr(&mut self) -> NMICLR_W<'_, NMICLR_SPEC> {
        NMICLR_W::new(self, 7)
    }
    ///Bit 8 - SRAM Parity Error Clear
    #[inline(always)]
    pub fn rpeclr(&mut self) -> RPECLR_W<'_, NMICLR_SPEC> {
        RPECLR_W::new(self, 8)
    }
    ///Bit 9 - SRAM ECC Error Clear
    #[inline(always)]
    pub fn reccclr(&mut self) -> RECCCLR_W<'_, NMICLR_SPEC> {
        RECCCLR_W::new(self, 9)
    }
    ///Bit 10 - Bus Slave Error Clear
    #[inline(always)]
    pub fn bussclr(&mut self) -> BUSSCLR_W<'_, NMICLR_SPEC> {
        BUSSCLR_W::new(self, 10)
    }
    ///Bit 11 - Bus Master Error Clear
    #[inline(always)]
    pub fn busmclr(&mut self) -> BUSMCLR_W<'_, NMICLR_SPEC> {
        BUSMCLR_W::new(self, 11)
    }
    ///Bit 12 - CPU Stack Pointer Monitor Interrupt Clear
    #[inline(always)]
    pub fn speclr(&mut self) -> SPECLR_W<'_, NMICLR_SPEC> {
        SPECLR_W::new(self, 12)
    }
}
/**Non-Maskable Interrupt Status Clear Register

You can [`read`](crate::Reg::read) this register and get [`nmiclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NMICLR_SPEC;
impl crate::RegisterSpec for NMICLR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`nmiclr::R`](R) reader structure
impl crate::Readable for NMICLR_SPEC {}
///`write(|w| ..)` method takes [`nmiclr::W`](W) writer structure
impl crate::Writable for NMICLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NMICLR to value 0
impl crate::Resettable for NMICLR_SPEC {}
