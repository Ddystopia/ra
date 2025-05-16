///Register `SYRVLDR` writer
pub type W = crate::W<SYRVLDR_SPEC>;
/**BMC Update

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMUP_A {
    ///0: no effect
    _0 = 0,
    ///1: Setting this bit to 1 leads to simultaneous reflection in the SYNFP module of the values of the registers holding the MasterClock identifying information.
    _1 = 1,
}
impl From<BMUP_A> for bool {
    #[inline(always)]
    fn from(variant: BMUP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BMUP` writer - BMC Update
pub type BMUP_W<'a, REG> = crate::BitWriter<'a, REG, BMUP_A>;
impl<'a, REG> BMUP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BMUP_A::_0)
    }
    ///Setting this bit to 1 leads to simultaneous reflection in the SYNFP module of the values of the registers holding the MasterClock identifying information.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BMUP_A::_1)
    }
}
/**State Update

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STUP_A {
    ///0: no effect
    _0 = 0,
    ///1: Setting this bit to 1 leads to simultaneous reflection in the SYNFP module of the values of the registers related to the reception and transmission of PTP messages.
    _1 = 1,
}
impl From<STUP_A> for bool {
    #[inline(always)]
    fn from(variant: STUP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `STUP` writer - State Update
pub type STUP_W<'a, REG> = crate::BitWriter<'a, REG, STUP_A>;
impl<'a, REG> STUP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(STUP_A::_0)
    }
    ///Setting this bit to 1 leads to simultaneous reflection in the SYNFP module of the values of the registers related to the reception and transmission of PTP messages.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(STUP_A::_1)
    }
}
/**Announce Message Generation Information Update

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANUP_A {
    ///0: no effect
    _0 = 0,
    ///1: Setting this bit to 1 leads to simultaneous reflection in the Announce message generation block of the values of the registers required for the generation of Announce messages.
    _1 = 1,
}
impl From<ANUP_A> for bool {
    #[inline(always)]
    fn from(variant: ANUP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANUP` writer - Announce Message Generation Information Update
pub type ANUP_W<'a, REG> = crate::BitWriter<'a, REG, ANUP_A>;
impl<'a, REG> ANUP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no effect
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANUP_A::_0)
    }
    ///Setting this bit to 1 leads to simultaneous reflection in the Announce message generation block of the values of the registers required for the generation of Announce messages.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANUP_A::_1)
    }
}
impl W {
    ///Bit 0 - BMC Update
    #[inline(always)]
    pub fn bmup(&mut self) -> BMUP_W<SYRVLDR_SPEC> {
        BMUP_W::new(self, 0)
    }
    ///Bit 1 - State Update
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W<SYRVLDR_SPEC> {
        STUP_W::new(self, 1)
    }
    ///Bit 2 - Announce Message Generation Information Update
    #[inline(always)]
    pub fn anup(&mut self) -> ANUP_W<SYRVLDR_SPEC> {
        ANUP_W::new(self, 2)
    }
}
/**SYNFP Register Value Load Directive Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syrvldr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYRVLDR_SPEC;
impl crate::RegisterSpec for SYRVLDR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`syrvldr::W`](W) writer structure
impl crate::Writable for SYRVLDR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYRVLDR to value 0
impl crate::Resettable for SYRVLDR_SPEC {}
