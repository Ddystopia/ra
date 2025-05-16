///Register `PIPESEL` reader
pub type R = crate::R<PIPESEL_SPEC>;
///Register `PIPESEL` writer
pub type W = crate::W<PIPESEL_SPEC>;
/**Pipe Window Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PIPESEL_A {
    ///0: No pipe selected
    _0000 = 0,
    ///1: PIPE1
    _0001 = 1,
    ///2: PIPE2
    _0010 = 2,
    ///3: PIPE3
    _0011 = 3,
    ///4: PIPE4
    _0100 = 4,
    ///5: PIPE5
    _0101 = 5,
    ///6: PIPE6
    _0110 = 6,
    ///7: PIPE7
    _0111 = 7,
    ///8: PIPE8
    _1000 = 8,
    ///9: PIPE9
    _1001 = 9,
    ///10: Settings prohibited.
    OTHERS = 10,
}
impl From<PIPESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PIPESEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PIPESEL_A {
    type Ux = u8;
}
impl crate::IsEnum for PIPESEL_A {}
///Field `PIPESEL` reader - Pipe Window Select
pub type PIPESEL_R = crate::FieldReader<PIPESEL_A>;
impl PIPESEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPESEL_A {
        match self.bits {
            0 => PIPESEL_A::_0000,
            1 => PIPESEL_A::_0001,
            2 => PIPESEL_A::_0010,
            3 => PIPESEL_A::_0011,
            4 => PIPESEL_A::_0100,
            5 => PIPESEL_A::_0101,
            6 => PIPESEL_A::_0110,
            7 => PIPESEL_A::_0111,
            8 => PIPESEL_A::_1000,
            9 => PIPESEL_A::_1001,
            _ => PIPESEL_A::OTHERS,
        }
    }
    ///No pipe selected
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == PIPESEL_A::_0000
    }
    ///PIPE1
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == PIPESEL_A::_0001
    }
    ///PIPE2
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == PIPESEL_A::_0010
    }
    ///PIPE3
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == PIPESEL_A::_0011
    }
    ///PIPE4
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PIPESEL_A::_0100
    }
    ///PIPE5
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PIPESEL_A::_0101
    }
    ///PIPE6
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PIPESEL_A::_0110
    }
    ///PIPE7
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PIPESEL_A::_0111
    }
    ///PIPE8
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PIPESEL_A::_1000
    }
    ///PIPE9
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PIPESEL_A::_1001
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), PIPESEL_A::OTHERS)
    }
}
///Field `PIPESEL` writer - Pipe Window Select
pub type PIPESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PIPESEL_A, crate::Safe>;
impl<'a, REG> PIPESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pipe selected
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0000)
    }
    ///PIPE1
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0001)
    }
    ///PIPE2
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0010)
    }
    ///PIPE3
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0011)
    }
    ///PIPE4
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0100)
    }
    ///PIPE5
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0101)
    }
    ///PIPE6
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0110)
    }
    ///PIPE7
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0111)
    }
    ///PIPE8
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_1000)
    }
    ///PIPE9
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_1001)
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::OTHERS)
    }
}
impl R {
    ///Bits 0:3 - Pipe Window Select
    #[inline(always)]
    pub fn pipesel(&self) -> PIPESEL_R {
        PIPESEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Pipe Window Select
    #[inline(always)]
    pub fn pipesel(&mut self) -> PIPESEL_W<PIPESEL_SPEC> {
        PIPESEL_W::new(self, 0)
    }
}
/**Pipe Window Select Register

You can [`read`](crate::Reg::read) this register and get [`pipesel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipesel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIPESEL_SPEC;
impl crate::RegisterSpec for PIPESEL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pipesel::R`](R) reader structure
impl crate::Readable for PIPESEL_SPEC {}
///`write(|w| ..)` method takes [`pipesel::W`](W) writer structure
impl crate::Writable for PIPESEL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPESEL to value 0
impl crate::Resettable for PIPESEL_SPEC {}
