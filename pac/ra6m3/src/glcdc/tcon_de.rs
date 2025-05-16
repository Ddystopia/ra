///Register `TCON_DE` reader
pub type R = crate::R<TCON_DE_SPEC>;
///Register `TCON_DE` writer
pub type W = crate::W<TCON_DE_SPEC>;
/**DE signal polarity inversion control.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    ///1: Inverted
    _1 = 1,
    ///0: Not inverted
    _0 = 0,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INV` reader - DE signal polarity inversion control.
pub type INV_R = crate::BitReader<INV_A>;
impl INV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INV_A {
        match self.bits {
            true => INV_A::_1,
            false => INV_A::_0,
        }
    }
    ///Inverted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV_A::_1
    }
    ///Not inverted
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV_A::_0
    }
}
///Field `INV` writer - DE signal polarity inversion control.
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG, INV_A>;
impl<'a, REG> INV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Inverted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INV_A::_1)
    }
    ///Not inverted
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INV_A::_0)
    }
}
impl R {
    ///Bit 0 - DE signal polarity inversion control.
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DE signal polarity inversion control.
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W<TCON_DE_SPEC> {
        INV_W::new(self, 0)
    }
}
/**TCON Data Enable Polarity Setting Register

You can [`read`](crate::Reg::read) this register and get [`tcon_de::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_de::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TCON_DE_SPEC;
impl crate::RegisterSpec for TCON_DE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tcon_de::R`](R) reader structure
impl crate::Readable for TCON_DE_SPEC {}
///`write(|w| ..)` method takes [`tcon_de::W`](W) writer structure
impl crate::Writable for TCON_DE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCON_DE to value 0
impl crate::Resettable for TCON_DE_SPEC {}
