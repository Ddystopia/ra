///Register `MB%s_DL` reader
pub type R = crate::R<MB_DL_SPEC>;
///Register `MB%s_DL` writer
pub type W = crate::W<MB_DL_SPEC>;
/**Data Length Code

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLC_A {
    ///0: Data length = 0 byte
    _0000 = 0,
    ///1: Data length = 1 byte
    _0001 = 1,
    ///2: Data length = 2 bytes
    _0010 = 2,
    ///3: Data length = 3 bytes
    _0011 = 3,
    ///4: Data length = 4 bytes
    _0100 = 4,
    ///5: Data length = 5 bytes
    _0101 = 5,
    ///6: Data length = 6 bytes
    _0110 = 6,
    ///7: Data length = 7 bytes
    _0111 = 7,
    ///8: Data length = 8 bytes
    OTHERS = 8,
}
impl From<DLC_A> for u8 {
    #[inline(always)]
    fn from(variant: DLC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DLC_A {
    type Ux = u8;
}
impl crate::IsEnum for DLC_A {}
///Field `DLC` reader - Data Length Code
pub type DLC_R = crate::FieldReader<DLC_A>;
impl DLC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLC_A {
        match self.bits {
            0 => DLC_A::_0000,
            1 => DLC_A::_0001,
            2 => DLC_A::_0010,
            3 => DLC_A::_0011,
            4 => DLC_A::_0100,
            5 => DLC_A::_0101,
            6 => DLC_A::_0110,
            7 => DLC_A::_0111,
            _ => DLC_A::OTHERS,
        }
    }
    ///Data length = 0 byte
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DLC_A::_0000
    }
    ///Data length = 1 byte
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DLC_A::_0001
    }
    ///Data length = 2 bytes
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DLC_A::_0010
    }
    ///Data length = 3 bytes
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DLC_A::_0011
    }
    ///Data length = 4 bytes
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DLC_A::_0100
    }
    ///Data length = 5 bytes
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DLC_A::_0101
    }
    ///Data length = 6 bytes
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == DLC_A::_0110
    }
    ///Data length = 7 bytes
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == DLC_A::_0111
    }
    ///Data length = 8 bytes
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), DLC_A::OTHERS)
    }
}
///Field `DLC` writer - Data Length Code
pub type DLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DLC_A, crate::Safe>;
impl<'a, REG> DLC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Data length = 0 byte
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(DLC_A::_0000)
    }
    ///Data length = 1 byte
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(DLC_A::_0001)
    }
    ///Data length = 2 bytes
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(DLC_A::_0010)
    }
    ///Data length = 3 bytes
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(DLC_A::_0011)
    }
    ///Data length = 4 bytes
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(DLC_A::_0100)
    }
    ///Data length = 5 bytes
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(DLC_A::_0101)
    }
    ///Data length = 6 bytes
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(DLC_A::_0110)
    }
    ///Data length = 7 bytes
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(DLC_A::_0111)
    }
    ///Data length = 8 bytes
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(DLC_A::OTHERS)
    }
}
impl R {
    ///Bits 0:3 - Data Length Code
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Data Length Code
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W<MB_DL_SPEC> {
        DLC_W::new(self, 0)
    }
}
/**Mailbox Register

You can [`read`](crate::Reg::read) this register and get [`mb_dl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_dl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MB_DL_SPEC;
impl crate::RegisterSpec for MB_DL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`mb_dl::R`](R) reader structure
impl crate::Readable for MB_DL_SPEC {}
///`write(|w| ..)` method takes [`mb_dl::W`](W) writer structure
impl crate::Writable for MB_DL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MB%s_DL to value 0
impl crate::Resettable for MB_DL_SPEC {}
