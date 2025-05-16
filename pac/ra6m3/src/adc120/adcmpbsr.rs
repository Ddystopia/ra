///Register `ADCMPBSR` reader
pub type R = crate::R<ADCMPBSR_SPEC>;
///Register `ADCMPBSR` writer
pub type W = crate::W<ADCMPBSR_SPEC>;
/**Compare window B flag.It is a status flag that shows the comparative result of CH (AN000-AN007,AN016-AN020, temperature sensor, and internal reference voltage) made the object of window B relation condition.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTB_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTB_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTB_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTB` reader - Compare window B flag.It is a status flag that shows the comparative result of CH (AN000-AN007,AN016-AN020, temperature sensor, and internal reference voltage) made the object of window B relation condition.

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTB_R = crate::BitReader<CMPSTB_A>;
impl CMPSTB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTB_A {
        match self.bits {
            false => CMPSTB_A::_0,
            true => CMPSTB_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTB_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTB_A::_1
    }
}
///Field `CMPSTB` writer - Compare window B flag.It is a status flag that shows the comparative result of CH (AN000-AN007,AN016-AN020, temperature sensor, and internal reference voltage) made the object of window B relation condition.
pub type CMPSTB_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTB_A>;
impl<'a, REG> CMPSTB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTB_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTB_A::_1)
    }
}
impl R {
    ///Bit 0 - Compare window B flag.It is a status flag that shows the comparative result of CH (AN000-AN007,AN016-AN020, temperature sensor, and internal reference voltage) made the object of window B relation condition.
    #[inline(always)]
    pub fn cmpstb(&self) -> CMPSTB_R {
        CMPSTB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Compare window B flag.It is a status flag that shows the comparative result of CH (AN000-AN007,AN016-AN020, temperature sensor, and internal reference voltage) made the object of window B relation condition.
    #[inline(always)]
    pub fn cmpstb(&mut self) -> CMPSTB_W<ADCMPBSR_SPEC> {
        CMPSTB_W::new(self, 0)
    }
}
/**A/D Compare Function Window B Status Register

You can [`read`](crate::Reg::read) this register and get [`adcmpbsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpbsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPBSR_SPEC;
impl crate::RegisterSpec for ADCMPBSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adcmpbsr::R`](R) reader structure
impl crate::Readable for ADCMPBSR_SPEC {}
///`write(|w| ..)` method takes [`adcmpbsr::W`](W) writer structure
impl crate::Writable for ADCMPBSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x01;
}
///`reset()` method sets ADCMPBSR to value 0
impl crate::Resettable for ADCMPBSR_SPEC {}
