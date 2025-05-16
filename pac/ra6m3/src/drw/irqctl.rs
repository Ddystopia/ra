///Register `IRQCTL` writer
pub type W = crate::W<IRQCTL_SPEC>;
/**ENUMIRQ interrupt mask enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENUMIRQEN_A {
    ///0: disable (mask) ENUMIRQ
    _0 = 0,
    ///1: enable (unmask) ENUMIRQ
    _1 = 1,
}
impl From<ENUMIRQEN_A> for bool {
    #[inline(always)]
    fn from(variant: ENUMIRQEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENUMIRQEN` writer - ENUMIRQ interrupt mask enable
pub type ENUMIRQEN_W<'a, REG> = crate::BitWriter<'a, REG, ENUMIRQEN_A>;
impl<'a, REG> ENUMIRQEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disable (mask) ENUMIRQ
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENUMIRQEN_A::_0)
    }
    ///enable (unmask) ENUMIRQ
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENUMIRQEN_A::_1)
    }
}
/**DLISTIRQ interrupt mask enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLISTIRQEN_A {
    ///0: disable (mask) DLISTIRQ
    _0 = 0,
    ///1: enable (unmask) DLISTIRQ
    _1 = 1,
}
impl From<DLISTIRQEN_A> for bool {
    #[inline(always)]
    fn from(variant: DLISTIRQEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLISTIRQEN` writer - DLISTIRQ interrupt mask enable
pub type DLISTIRQEN_W<'a, REG> = crate::BitWriter<'a, REG, DLISTIRQEN_A>;
impl<'a, REG> DLISTIRQEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disable (mask) DLISTIRQ
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLISTIRQEN_A::_0)
    }
    ///enable (unmask) DLISTIRQ
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLISTIRQEN_A::_1)
    }
}
/**Clear enumeration interrupt ENUMIRQ

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENUMIRQCLR_A {
    ///0: no ENUMIRQCLR clear
    _0 = 0,
    ///1: clear ENUMIRQCLR
    _1 = 1,
}
impl From<ENUMIRQCLR_A> for bool {
    #[inline(always)]
    fn from(variant: ENUMIRQCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENUMIRQCLR` writer - Clear enumeration interrupt ENUMIRQ
pub type ENUMIRQCLR_W<'a, REG> = crate::BitWriter<'a, REG, ENUMIRQCLR_A>;
impl<'a, REG> ENUMIRQCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no ENUMIRQCLR clear
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENUMIRQCLR_A::_0)
    }
    ///clear ENUMIRQCLR
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENUMIRQCLR_A::_1)
    }
}
/**Clear display list interrupt DLISTIRQ

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLISTIRQCLR_A {
    ///0: no DLISTRQCLR clear
    _0 = 0,
    ///1: clear DLISTRQCLR
    _1 = 1,
}
impl From<DLISTIRQCLR_A> for bool {
    #[inline(always)]
    fn from(variant: DLISTIRQCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLISTIRQCLR` writer - Clear display list interrupt DLISTIRQ
pub type DLISTIRQCLR_W<'a, REG> = crate::BitWriter<'a, REG, DLISTIRQCLR_A>;
impl<'a, REG> DLISTIRQCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no DLISTRQCLR clear
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLISTIRQCLR_A::_0)
    }
    ///clear DLISTRQCLR
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLISTIRQCLR_A::_1)
    }
}
/**BUSIRQ interrupt mask enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSIRQEN_A {
    ///0: disable (mask) BUSIRQ
    _0 = 0,
    ///1: enable (unmask) BUSIRQ
    _1 = 1,
}
impl From<BUSIRQEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUSIRQEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSIRQEN` writer - BUSIRQ interrupt mask enable
pub type BUSIRQEN_W<'a, REG> = crate::BitWriter<'a, REG, BUSIRQEN_A>;
impl<'a, REG> BUSIRQEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disable (mask) BUSIRQ
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BUSIRQEN_A::_0)
    }
    ///enable (unmask) BUSIRQ
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BUSIRQEN_A::_1)
    }
}
/**Clear bus error interrupt BUSIRQ

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSIRQCLR_A {
    ///0: no BUSIRQCLR clear
    _0 = 0,
    ///1: clear BUSIRQCLR
    _1 = 1,
}
impl From<BUSIRQCLR_A> for bool {
    #[inline(always)]
    fn from(variant: BUSIRQCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSIRQCLR` writer - Clear bus error interrupt BUSIRQ
pub type BUSIRQCLR_W<'a, REG> = crate::BitWriter<'a, REG, BUSIRQCLR_A>;
impl<'a, REG> BUSIRQCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no BUSIRQCLR clear
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BUSIRQCLR_A::_0)
    }
    ///clear BUSIRQCLR
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BUSIRQCLR_A::_1)
    }
}
impl W {
    ///Bit 0 - ENUMIRQ interrupt mask enable
    #[inline(always)]
    pub fn enumirqen(&mut self) -> ENUMIRQEN_W<IRQCTL_SPEC> {
        ENUMIRQEN_W::new(self, 0)
    }
    ///Bit 1 - DLISTIRQ interrupt mask enable
    #[inline(always)]
    pub fn dlistirqen(&mut self) -> DLISTIRQEN_W<IRQCTL_SPEC> {
        DLISTIRQEN_W::new(self, 1)
    }
    ///Bit 2 - Clear enumeration interrupt ENUMIRQ
    #[inline(always)]
    pub fn enumirqclr(&mut self) -> ENUMIRQCLR_W<IRQCTL_SPEC> {
        ENUMIRQCLR_W::new(self, 2)
    }
    ///Bit 3 - Clear display list interrupt DLISTIRQ
    #[inline(always)]
    pub fn dlistirqclr(&mut self) -> DLISTIRQCLR_W<IRQCTL_SPEC> {
        DLISTIRQCLR_W::new(self, 3)
    }
    ///Bit 4 - BUSIRQ interrupt mask enable
    #[inline(always)]
    pub fn busirqen(&mut self) -> BUSIRQEN_W<IRQCTL_SPEC> {
        BUSIRQEN_W::new(self, 4)
    }
    ///Bit 5 - Clear bus error interrupt BUSIRQ
    #[inline(always)]
    pub fn busirqclr(&mut self) -> BUSIRQCLR_W<IRQCTL_SPEC> {
        BUSIRQCLR_W::new(self, 5)
    }
}
/**Interrupt Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IRQCTL_SPEC;
impl crate::RegisterSpec for IRQCTL_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`irqctl::W`](W) writer structure
impl crate::Writable for IRQCTL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQCTL to value 0
impl crate::Resettable for IRQCTL_SPEC {}
