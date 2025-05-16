///Register `INTENB1` reader
pub type R = crate::R<INTENB1_SPEC>;
///Register `INTENB1` writer
pub type W = crate::W<INTENB1_SPEC>;
/**PDDETINT Detection Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDETINTE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PDDETINTE_A> for bool {
    #[inline(always)]
    fn from(variant: PDDETINTE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDDETINTE` reader - PDDETINT Detection Interrupt Enable
pub type PDDETINTE_R = crate::BitReader<PDDETINTE_A>;
impl PDDETINTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDDETINTE_A {
        match self.bits {
            false => PDDETINTE_A::_0,
            true => PDDETINTE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDDETINTE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDDETINTE_A::_1
    }
}
///Field `PDDETINTE` writer - PDDETINT Detection Interrupt Enable
pub type PDDETINTE_W<'a, REG> = crate::BitWriter<'a, REG, PDDETINTE_A>;
impl<'a, REG> PDDETINTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDDETINTE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDDETINTE_A::_1)
    }
}
/**Setup Transaction Normal Response Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SACKE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<SACKE_A> for bool {
    #[inline(always)]
    fn from(variant: SACKE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SACKE` reader - Setup Transaction Normal Response Interrupt Enable
pub type SACKE_R = crate::BitReader<SACKE_A>;
impl SACKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SACKE_A {
        match self.bits {
            false => SACKE_A::_0,
            true => SACKE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SACKE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SACKE_A::_1
    }
}
///Field `SACKE` writer - Setup Transaction Normal Response Interrupt Enable
pub type SACKE_W<'a, REG> = crate::BitWriter<'a, REG, SACKE_A>;
impl<'a, REG> SACKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SACKE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SACKE_A::_1)
    }
}
/**Setup Transaction Error Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGNE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<SIGNE_A> for bool {
    #[inline(always)]
    fn from(variant: SIGNE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SIGNE` reader - Setup Transaction Error Interrupt Enable
pub type SIGNE_R = crate::BitReader<SIGNE_A>;
impl SIGNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SIGNE_A {
        match self.bits {
            false => SIGNE_A::_0,
            true => SIGNE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SIGNE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SIGNE_A::_1
    }
}
///Field `SIGNE` writer - Setup Transaction Error Interrupt Enable
pub type SIGNE_W<'a, REG> = crate::BitWriter<'a, REG, SIGNE_A>;
impl<'a, REG> SIGNE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SIGNE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SIGNE_A::_1)
    }
}
/**EOF Error Detection Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOFERRE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<EOFERRE_A> for bool {
    #[inline(always)]
    fn from(variant: EOFERRE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOFERRE` reader - EOF Error Detection Interrupt Enable
pub type EOFERRE_R = crate::BitReader<EOFERRE_A>;
impl EOFERRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOFERRE_A {
        match self.bits {
            false => EOFERRE_A::_0,
            true => EOFERRE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOFERRE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOFERRE_A::_1
    }
}
///Field `EOFERRE` writer - EOF Error Detection Interrupt Enable
pub type EOFERRE_W<'a, REG> = crate::BitWriter<'a, REG, EOFERRE_A>;
impl<'a, REG> EOFERRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EOFERRE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EOFERRE_A::_1)
    }
}
/**LPM Transaction End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMENDE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<LPMENDE_A> for bool {
    #[inline(always)]
    fn from(variant: LPMENDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LPMENDE` reader - LPM Transaction End Interrupt Enable
pub type LPMENDE_R = crate::BitReader<LPMENDE_A>;
impl LPMENDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPMENDE_A {
        match self.bits {
            false => LPMENDE_A::_0,
            true => LPMENDE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPMENDE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPMENDE_A::_1
    }
}
///Field `LPMENDE` writer - LPM Transaction End Interrupt Enable
pub type LPMENDE_W<'a, REG> = crate::BitWriter<'a, REG, LPMENDE_A>;
impl<'a, REG> LPMENDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LPMENDE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LPMENDE_A::_1)
    }
}
/**L1 Resume End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1RSMENDE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<L1RSMENDE_A> for bool {
    #[inline(always)]
    fn from(variant: L1RSMENDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L1RSMENDE` reader - L1 Resume End Interrupt Enable
pub type L1RSMENDE_R = crate::BitReader<L1RSMENDE_A>;
impl L1RSMENDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1RSMENDE_A {
        match self.bits {
            false => L1RSMENDE_A::_0,
            true => L1RSMENDE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1RSMENDE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1RSMENDE_A::_1
    }
}
///Field `L1RSMENDE` writer - L1 Resume End Interrupt Enable
pub type L1RSMENDE_W<'a, REG> = crate::BitWriter<'a, REG, L1RSMENDE_A>;
impl<'a, REG> L1RSMENDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(L1RSMENDE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(L1RSMENDE_A::_1)
    }
}
/**Connection Detection Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATTCHE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<ATTCHE_A> for bool {
    #[inline(always)]
    fn from(variant: ATTCHE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ATTCHE` reader - Connection Detection Interrupt Enable
pub type ATTCHE_R = crate::BitReader<ATTCHE_A>;
impl ATTCHE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ATTCHE_A {
        match self.bits {
            false => ATTCHE_A::_0,
            true => ATTCHE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATTCHE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATTCHE_A::_1
    }
}
///Field `ATTCHE` writer - Connection Detection Interrupt Enable
pub type ATTCHE_W<'a, REG> = crate::BitWriter<'a, REG, ATTCHE_A>;
impl<'a, REG> ATTCHE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ATTCHE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ATTCHE_A::_1)
    }
}
/**Disconnection Detection Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCHE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<DTCHE_A> for bool {
    #[inline(always)]
    fn from(variant: DTCHE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DTCHE` reader - Disconnection Detection Interrupt Enable
pub type DTCHE_R = crate::BitReader<DTCHE_A>;
impl DTCHE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTCHE_A {
        match self.bits {
            false => DTCHE_A::_0,
            true => DTCHE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCHE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCHE_A::_1
    }
}
///Field `DTCHE` writer - Disconnection Detection Interrupt Enable
pub type DTCHE_W<'a, REG> = crate::BitWriter<'a, REG, DTCHE_A>;
impl<'a, REG> DTCHE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTCHE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTCHE_A::_1)
    }
}
/**USB Bus Change Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCHGE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<BCHGE_A> for bool {
    #[inline(always)]
    fn from(variant: BCHGE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BCHGE` reader - USB Bus Change Interrupt Enable
pub type BCHGE_R = crate::BitReader<BCHGE_A>;
impl BCHGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BCHGE_A {
        match self.bits {
            false => BCHGE_A::_0,
            true => BCHGE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCHGE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCHGE_A::_1
    }
}
///Field `BCHGE` writer - USB Bus Change Interrupt Enable
pub type BCHGE_W<'a, REG> = crate::BitWriter<'a, REG, BCHGE_A>;
impl<'a, REG> BCHGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BCHGE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BCHGE_A::_1)
    }
}
/**OVRCRE Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRCRE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<OVRCRE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRCRE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRCRE` reader - OVRCRE Interrupt Enable
pub type OVRCRE_R = crate::BitReader<OVRCRE_A>;
impl OVRCRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRCRE_A {
        match self.bits {
            false => OVRCRE_A::_0,
            true => OVRCRE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRCRE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRCRE_A::_1
    }
}
///Field `OVRCRE` writer - OVRCRE Interrupt Enable
pub type OVRCRE_W<'a, REG> = crate::BitWriter<'a, REG, OVRCRE_A>;
impl<'a, REG> OVRCRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRCRE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRCRE_A::_1)
    }
}
impl R {
    ///Bit 0 - PDDETINT Detection Interrupt Enable
    #[inline(always)]
    pub fn pddetinte(&self) -> PDDETINTE_R {
        PDDETINTE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Setup Transaction Normal Response Interrupt Enable
    #[inline(always)]
    pub fn sacke(&self) -> SACKE_R {
        SACKE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Setup Transaction Error Interrupt Enable
    #[inline(always)]
    pub fn signe(&self) -> SIGNE_R {
        SIGNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EOF Error Detection Interrupt Enable
    #[inline(always)]
    pub fn eoferre(&self) -> EOFERRE_R {
        EOFERRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - LPM Transaction End Interrupt Enable
    #[inline(always)]
    pub fn lpmende(&self) -> LPMENDE_R {
        LPMENDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - L1 Resume End Interrupt Enable
    #[inline(always)]
    pub fn l1rsmende(&self) -> L1RSMENDE_R {
        L1RSMENDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Connection Detection Interrupt Enable
    #[inline(always)]
    pub fn attche(&self) -> ATTCHE_R {
        ATTCHE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Disconnection Detection Interrupt Enable
    #[inline(always)]
    pub fn dtche(&self) -> DTCHE_R {
        DTCHE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USB Bus Change Interrupt Enable
    #[inline(always)]
    pub fn bchge(&self) -> BCHGE_R {
        BCHGE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - OVRCRE Interrupt Enable
    #[inline(always)]
    pub fn ovrcre(&self) -> OVRCRE_R {
        OVRCRE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PDDETINT Detection Interrupt Enable
    #[inline(always)]
    pub fn pddetinte(&mut self) -> PDDETINTE_W<INTENB1_SPEC> {
        PDDETINTE_W::new(self, 0)
    }
    ///Bit 4 - Setup Transaction Normal Response Interrupt Enable
    #[inline(always)]
    pub fn sacke(&mut self) -> SACKE_W<INTENB1_SPEC> {
        SACKE_W::new(self, 4)
    }
    ///Bit 5 - Setup Transaction Error Interrupt Enable
    #[inline(always)]
    pub fn signe(&mut self) -> SIGNE_W<INTENB1_SPEC> {
        SIGNE_W::new(self, 5)
    }
    ///Bit 6 - EOF Error Detection Interrupt Enable
    #[inline(always)]
    pub fn eoferre(&mut self) -> EOFERRE_W<INTENB1_SPEC> {
        EOFERRE_W::new(self, 6)
    }
    ///Bit 8 - LPM Transaction End Interrupt Enable
    #[inline(always)]
    pub fn lpmende(&mut self) -> LPMENDE_W<INTENB1_SPEC> {
        LPMENDE_W::new(self, 8)
    }
    ///Bit 9 - L1 Resume End Interrupt Enable
    #[inline(always)]
    pub fn l1rsmende(&mut self) -> L1RSMENDE_W<INTENB1_SPEC> {
        L1RSMENDE_W::new(self, 9)
    }
    ///Bit 11 - Connection Detection Interrupt Enable
    #[inline(always)]
    pub fn attche(&mut self) -> ATTCHE_W<INTENB1_SPEC> {
        ATTCHE_W::new(self, 11)
    }
    ///Bit 12 - Disconnection Detection Interrupt Enable
    #[inline(always)]
    pub fn dtche(&mut self) -> DTCHE_W<INTENB1_SPEC> {
        DTCHE_W::new(self, 12)
    }
    ///Bit 14 - USB Bus Change Interrupt Enable
    #[inline(always)]
    pub fn bchge(&mut self) -> BCHGE_W<INTENB1_SPEC> {
        BCHGE_W::new(self, 14)
    }
    ///Bit 15 - OVRCRE Interrupt Enable
    #[inline(always)]
    pub fn ovrcre(&mut self) -> OVRCRE_W<INTENB1_SPEC> {
        OVRCRE_W::new(self, 15)
    }
}
/**Interrupt Enable Register 1

You can [`read`](crate::Reg::read) this register and get [`intenb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTENB1_SPEC;
impl crate::RegisterSpec for INTENB1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`intenb1::R`](R) reader structure
impl crate::Readable for INTENB1_SPEC {}
///`write(|w| ..)` method takes [`intenb1::W`](W) writer structure
impl crate::Writable for INTENB1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTENB1 to value 0
impl crate::Resettable for INTENB1_SPEC {}
