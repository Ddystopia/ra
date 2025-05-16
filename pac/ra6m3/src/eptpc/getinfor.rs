///Register `GETINFOR` reader
pub type R = crate::R<GETINFOR_SPEC>;
///Register `GETINFOR` writer
pub type W = crate::W<GETINFOR_SPEC>;
/**Information Retention ControlNOTE: Once information fetching is directed, values of various statistical information read before completion of information fetching are not guaranteed.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INFO_A {
    ///0: Has no effects.(write) / Information retention is completed.(read)
    _0 = 0,
    ///1: Information is retained.(write) / Processing for information retention is in progress.(read)
    _1 = 1,
}
impl From<INFO_A> for bool {
    #[inline(always)]
    fn from(variant: INFO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INFO` reader - Information Retention ControlNOTE: Once information fetching is directed, values of various statistical information read before completion of information fetching are not guaranteed.
pub type INFO_R = crate::BitReader<INFO_A>;
impl INFO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INFO_A {
        match self.bits {
            false => INFO_A::_0,
            true => INFO_A::_1,
        }
    }
    ///Has no effects.(write) / Information retention is completed.(read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INFO_A::_0
    }
    ///Information is retained.(write) / Processing for information retention is in progress.(read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INFO_A::_1
    }
}
///Field `INFO` writer - Information Retention ControlNOTE: Once information fetching is directed, values of various statistical information read before completion of information fetching are not guaranteed.
pub type INFO_W<'a, REG> = crate::BitWriter<'a, REG, INFO_A>;
impl<'a, REG> INFO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Has no effects.(write) / Information retention is completed.(read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INFO_A::_0)
    }
    ///Information is retained.(write) / Processing for information retention is in progress.(read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INFO_A::_1)
    }
}
impl R {
    ///Bit 0 - Information Retention ControlNOTE: Once information fetching is directed, values of various statistical information read before completion of information fetching are not guaranteed.
    #[inline(always)]
    pub fn info(&self) -> INFO_R {
        INFO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Information Retention ControlNOTE: Once information fetching is directed, values of various statistical information read before completion of information fetching are not guaranteed.
    #[inline(always)]
    pub fn info(&mut self) -> INFO_W<GETINFOR_SPEC> {
        INFO_W::new(self, 0)
    }
}
/**Statistical Information Retention Control Register

You can [`read`](crate::Reg::read) this register and get [`getinfor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`getinfor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GETINFOR_SPEC;
impl crate::RegisterSpec for GETINFOR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`getinfor::R`](R) reader structure
impl crate::Readable for GETINFOR_SPEC {}
///`write(|w| ..)` method takes [`getinfor::W`](W) writer structure
impl crate::Writable for GETINFOR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GETINFOR to value 0
impl crate::Resettable for GETINFOR_SPEC {}
