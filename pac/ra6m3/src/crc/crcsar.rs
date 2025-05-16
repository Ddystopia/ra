///Register `CRCSAR` reader
pub type R = crate::R<CRCSAR_SPEC>;
///Register `CRCSAR` writer
pub type W = crate::W<CRCSAR_SPEC>;
/**snoop address bitSet the I/O register address to snoop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CRCSA_A {
    ///3: SCI0.TDR
    _0X0003 = 3,
    ///5: SCI0.RDR
    _0X0005 = 5,
    ///35: SCI1.TDR
    _0X0023 = 35,
    ///37: SCI1.RDR
    _0X0025 = 37,
    ///67: SCI2.TDR
    _0X0043 = 67,
    ///69: SCI2.RDR
    _0X0045 = 69,
    ///99: SCI3.TDR
    _0X0063 = 99,
    ///101: SCI3.RDR
    _0X0065 = 101,
    ///131: SCI4.TDR
    _0X0083 = 131,
    ///133: SCI4.RDR
    _0X0085 = 133,
    ///163: SCI5.TDR
    _0X00A3 = 163,
    ///165: SCI5.RDR
    _0X00A5 = 165,
    ///195: SCI6.TDR
    _0X00C3 = 195,
    ///197: SCI6.RDR
    _0X00C5 = 197,
    ///227: SCI7.TDR
    _0X00E3 = 227,
    ///229: SCI7.RDR
    _0X00E5 = 229,
    ///259: SCI8.TDR
    _0X0103 = 259,
    ///261: SCI8.RDR
    _0X0105 = 261,
    ///291: SCI9.TDR
    _0X0123 = 291,
    ///293: SCI9.RDR
    _0X0125 = 293,
    ///0: Settings other than above are prohibited.
    OTHERS = 0,
}
impl From<CRCSA_A> for u16 {
    #[inline(always)]
    fn from(variant: CRCSA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRCSA_A {
    type Ux = u16;
}
impl crate::IsEnum for CRCSA_A {}
///Field `CRCSA` reader - snoop address bitSet the I/O register address to snoop
pub type CRCSA_R = crate::FieldReader<CRCSA_A>;
impl CRCSA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCSA_A {
        match self.bits {
            3 => CRCSA_A::_0X0003,
            5 => CRCSA_A::_0X0005,
            35 => CRCSA_A::_0X0023,
            37 => CRCSA_A::_0X0025,
            67 => CRCSA_A::_0X0043,
            69 => CRCSA_A::_0X0045,
            99 => CRCSA_A::_0X0063,
            101 => CRCSA_A::_0X0065,
            131 => CRCSA_A::_0X0083,
            133 => CRCSA_A::_0X0085,
            163 => CRCSA_A::_0X00A3,
            165 => CRCSA_A::_0X00A5,
            195 => CRCSA_A::_0X00C3,
            197 => CRCSA_A::_0X00C5,
            227 => CRCSA_A::_0X00E3,
            229 => CRCSA_A::_0X00E5,
            259 => CRCSA_A::_0X0103,
            261 => CRCSA_A::_0X0105,
            291 => CRCSA_A::_0X0123,
            293 => CRCSA_A::_0X0125,
            _ => CRCSA_A::OTHERS,
        }
    }
    ///SCI0.TDR
    #[inline(always)]
    pub fn is_0x0003(&self) -> bool {
        *self == CRCSA_A::_0X0003
    }
    ///SCI0.RDR
    #[inline(always)]
    pub fn is_0x0005(&self) -> bool {
        *self == CRCSA_A::_0X0005
    }
    ///SCI1.TDR
    #[inline(always)]
    pub fn is_0x0023(&self) -> bool {
        *self == CRCSA_A::_0X0023
    }
    ///SCI1.RDR
    #[inline(always)]
    pub fn is_0x0025(&self) -> bool {
        *self == CRCSA_A::_0X0025
    }
    ///SCI2.TDR
    #[inline(always)]
    pub fn is_0x0043(&self) -> bool {
        *self == CRCSA_A::_0X0043
    }
    ///SCI2.RDR
    #[inline(always)]
    pub fn is_0x0045(&self) -> bool {
        *self == CRCSA_A::_0X0045
    }
    ///SCI3.TDR
    #[inline(always)]
    pub fn is_0x0063(&self) -> bool {
        *self == CRCSA_A::_0X0063
    }
    ///SCI3.RDR
    #[inline(always)]
    pub fn is_0x0065(&self) -> bool {
        *self == CRCSA_A::_0X0065
    }
    ///SCI4.TDR
    #[inline(always)]
    pub fn is_0x0083(&self) -> bool {
        *self == CRCSA_A::_0X0083
    }
    ///SCI4.RDR
    #[inline(always)]
    pub fn is_0x0085(&self) -> bool {
        *self == CRCSA_A::_0X0085
    }
    ///SCI5.TDR
    #[inline(always)]
    pub fn is_0x00a3(&self) -> bool {
        *self == CRCSA_A::_0X00A3
    }
    ///SCI5.RDR
    #[inline(always)]
    pub fn is_0x00a5(&self) -> bool {
        *self == CRCSA_A::_0X00A5
    }
    ///SCI6.TDR
    #[inline(always)]
    pub fn is_0x00c3(&self) -> bool {
        *self == CRCSA_A::_0X00C3
    }
    ///SCI6.RDR
    #[inline(always)]
    pub fn is_0x00c5(&self) -> bool {
        *self == CRCSA_A::_0X00C5
    }
    ///SCI7.TDR
    #[inline(always)]
    pub fn is_0x00e3(&self) -> bool {
        *self == CRCSA_A::_0X00E3
    }
    ///SCI7.RDR
    #[inline(always)]
    pub fn is_0x00e5(&self) -> bool {
        *self == CRCSA_A::_0X00E5
    }
    ///SCI8.TDR
    #[inline(always)]
    pub fn is_0x0103(&self) -> bool {
        *self == CRCSA_A::_0X0103
    }
    ///SCI8.RDR
    #[inline(always)]
    pub fn is_0x0105(&self) -> bool {
        *self == CRCSA_A::_0X0105
    }
    ///SCI9.TDR
    #[inline(always)]
    pub fn is_0x0123(&self) -> bool {
        *self == CRCSA_A::_0X0123
    }
    ///SCI9.RDR
    #[inline(always)]
    pub fn is_0x0125(&self) -> bool {
        *self == CRCSA_A::_0X0125
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), CRCSA_A::OTHERS)
    }
}
///Field `CRCSA` writer - snoop address bitSet the I/O register address to snoop
pub type CRCSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, CRCSA_A, crate::Safe>;
impl<'a, REG> CRCSA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///SCI0.TDR
    #[inline(always)]
    pub fn _0x0003(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0003)
    }
    ///SCI0.RDR
    #[inline(always)]
    pub fn _0x0005(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0005)
    }
    ///SCI1.TDR
    #[inline(always)]
    pub fn _0x0023(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0023)
    }
    ///SCI1.RDR
    #[inline(always)]
    pub fn _0x0025(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0025)
    }
    ///SCI2.TDR
    #[inline(always)]
    pub fn _0x0043(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0043)
    }
    ///SCI2.RDR
    #[inline(always)]
    pub fn _0x0045(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0045)
    }
    ///SCI3.TDR
    #[inline(always)]
    pub fn _0x0063(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0063)
    }
    ///SCI3.RDR
    #[inline(always)]
    pub fn _0x0065(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0065)
    }
    ///SCI4.TDR
    #[inline(always)]
    pub fn _0x0083(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0083)
    }
    ///SCI4.RDR
    #[inline(always)]
    pub fn _0x0085(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0085)
    }
    ///SCI5.TDR
    #[inline(always)]
    pub fn _0x00a3(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X00A3)
    }
    ///SCI5.RDR
    #[inline(always)]
    pub fn _0x00a5(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X00A5)
    }
    ///SCI6.TDR
    #[inline(always)]
    pub fn _0x00c3(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X00C3)
    }
    ///SCI6.RDR
    #[inline(always)]
    pub fn _0x00c5(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X00C5)
    }
    ///SCI7.TDR
    #[inline(always)]
    pub fn _0x00e3(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X00E3)
    }
    ///SCI7.RDR
    #[inline(always)]
    pub fn _0x00e5(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X00E5)
    }
    ///SCI8.TDR
    #[inline(always)]
    pub fn _0x0103(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0103)
    }
    ///SCI8.RDR
    #[inline(always)]
    pub fn _0x0105(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0105)
    }
    ///SCI9.TDR
    #[inline(always)]
    pub fn _0x0123(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0123)
    }
    ///SCI9.RDR
    #[inline(always)]
    pub fn _0x0125(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::_0X0125)
    }
    ///Settings other than above are prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSA_A::OTHERS)
    }
}
impl R {
    ///Bits 0:13 - snoop address bitSet the I/O register address to snoop
    #[inline(always)]
    pub fn crcsa(&self) -> CRCSA_R {
        CRCSA_R::new(self.bits & 0x3fff)
    }
}
impl W {
    ///Bits 0:13 - snoop address bitSet the I/O register address to snoop
    #[inline(always)]
    pub fn crcsa(&mut self) -> CRCSA_W<CRCSAR_SPEC> {
        CRCSA_W::new(self, 0)
    }
}
/**Snoop Address Register

You can [`read`](crate::Reg::read) this register and get [`crcsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRCSAR_SPEC;
impl crate::RegisterSpec for CRCSAR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`crcsar::R`](R) reader structure
impl crate::Readable for CRCSAR_SPEC {}
///`write(|w| ..)` method takes [`crcsar::W`](W) writer structure
impl crate::Writable for CRCSAR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCSAR to value 0
impl crate::Resettable for CRCSAR_SPEC {}
