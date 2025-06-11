///Register `SARU%s` reader
pub type R = crate::R<SARU_SPEC>;
///Register `SARU%s` writer
pub type W = crate::W<SARU_SPEC>;
/**7-Bit/10-Bit Address Format Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FS_A {
    ///0: The 7-bit address format is selected.
    _0 = 0,
    ///1: The 10-bit address format is selected.
    _1 = 1,
}
impl From<FS_A> for bool {
    #[inline(always)]
    fn from(variant: FS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FS` reader - 7-Bit/10-Bit Address Format Selection
pub type FS_R = crate::BitReader<FS_A>;
impl FS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FS_A {
        match self.bits {
            false => FS_A::_0,
            true => FS_A::_1,
        }
    }
    ///The 7-bit address format is selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FS_A::_0
    }
    ///The 10-bit address format is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FS_A::_1
    }
}
///Field `FS` writer - 7-Bit/10-Bit Address Format Selection
pub type FS_W<'a, REG> = crate::BitWriter<'a, REG, FS_A>;
impl<'a, REG> FS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The 7-bit address format is selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FS_A::_0)
    }
    ///The 10-bit address format is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FS_A::_1)
    }
}
///Field `SVA8` reader - 10-Bit Address(bit8)
pub type SVA8_R = crate::BitReader;
///Field `SVA8` writer - 10-Bit Address(bit8)
pub type SVA8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SVA9` reader - 10-Bit Address(bit9)
pub type SVA9_R = crate::BitReader;
///Field `SVA9` writer - 10-Bit Address(bit9)
pub type SVA9_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 7-Bit/10-Bit Address Format Selection
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 10-Bit Address(bit8)
    #[inline(always)]
    pub fn sva8(&self) -> SVA8_R {
        SVA8_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 10-Bit Address(bit9)
    #[inline(always)]
    pub fn sva9(&self) -> SVA9_R {
        SVA9_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - 7-Bit/10-Bit Address Format Selection
    #[inline(always)]
    pub fn fs(&mut self) -> FS_W<SARU_SPEC> {
        FS_W::new(self, 0)
    }
    ///Bit 1 - 10-Bit Address(bit8)
    #[inline(always)]
    pub fn sva8(&mut self) -> SVA8_W<SARU_SPEC> {
        SVA8_W::new(self, 1)
    }
    ///Bit 2 - 10-Bit Address(bit9)
    #[inline(always)]
    pub fn sva9(&mut self) -> SVA9_W<SARU_SPEC> {
        SVA9_W::new(self, 2)
    }
}
/**Slave Address Register U%s

You can [`read`](crate::Reg::read) this register and get [`saru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SARU_SPEC;
impl crate::RegisterSpec for SARU_SPEC {
    type Ux = u8;
}
///`read()` method returns [`saru::R`](R) reader structure
impl crate::Readable for SARU_SPEC {}
///`write(|w| ..)` method takes [`saru::W`](W) writer structure
impl crate::Writable for SARU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SARU%s to value 0
impl crate::Resettable for SARU_SPEC {}
