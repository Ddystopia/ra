///Register `SPSCR` reader
pub type R = crate::R<SPSCR_SPEC>;
///Register `SPSCR` writer
pub type W = crate::W<SPSCR_SPEC>;
/**RSPI Sequence Length SpecificationThe order in which the SPCMD0 to SPCMD07 registers are to be referenced is changed in accordance with the sequence length that is set in these bits. The relationship among the setting of these bits, sequence length, and SPCMD0 to SPCMD7 registers referenced by the RSPI is shown above. However, the RSPI in slave mode always references SPCMD0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPSLN_A {
    ///0: Length 1 SPDMDx x = 0->0->...
    _000 = 0,
    ///1: Length 2 SPDMDx x = 0->1->0->...
    _001 = 1,
    ///2: Length 3 SPDMDx x = 0->1->2->0->...
    _010 = 2,
    ///3: Length 4 SPDMDx x = 0->1->2->3->0->...
    _011 = 3,
    ///4: Length 5 SPDMDx x = 0->1->2->3->4->0->...
    _100 = 4,
    ///5: Length 6 SPDMDx x = 0->1->2->3->4->5->0->...
    _101 = 5,
    ///6: Length 7 SPDMDx x = 0->1->2->3->4->5->6->0->...
    _110 = 6,
    ///7: Length 8 SPDMDx x = 0->1->2->3->4->5->6->7->0->...
    _111 = 7,
}
impl From<SPSLN_A> for u8 {
    #[inline(always)]
    fn from(variant: SPSLN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPSLN_A {
    type Ux = u8;
}
impl crate::IsEnum for SPSLN_A {}
///Field `SPSLN` reader - RSPI Sequence Length SpecificationThe order in which the SPCMD0 to SPCMD07 registers are to be referenced is changed in accordance with the sequence length that is set in these bits. The relationship among the setting of these bits, sequence length, and SPCMD0 to SPCMD7 registers referenced by the RSPI is shown above. However, the RSPI in slave mode always references SPCMD0.
pub type SPSLN_R = crate::FieldReader<SPSLN_A>;
impl SPSLN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPSLN_A {
        match self.bits {
            0 => SPSLN_A::_000,
            1 => SPSLN_A::_001,
            2 => SPSLN_A::_010,
            3 => SPSLN_A::_011,
            4 => SPSLN_A::_100,
            5 => SPSLN_A::_101,
            6 => SPSLN_A::_110,
            7 => SPSLN_A::_111,
            _ => unreachable!(),
        }
    }
    ///Length 1 SPDMDx x = 0->0->...
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPSLN_A::_000
    }
    ///Length 2 SPDMDx x = 0->1->0->...
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPSLN_A::_001
    }
    ///Length 3 SPDMDx x = 0->1->2->0->...
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPSLN_A::_010
    }
    ///Length 4 SPDMDx x = 0->1->2->3->0->...
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPSLN_A::_011
    }
    ///Length 5 SPDMDx x = 0->1->2->3->4->0->...
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPSLN_A::_100
    }
    ///Length 6 SPDMDx x = 0->1->2->3->4->5->0->...
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPSLN_A::_101
    }
    ///Length 7 SPDMDx x = 0->1->2->3->4->5->6->0->...
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPSLN_A::_110
    }
    ///Length 8 SPDMDx x = 0->1->2->3->4->5->6->7->0->...
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPSLN_A::_111
    }
}
///Field `SPSLN` writer - RSPI Sequence Length SpecificationThe order in which the SPCMD0 to SPCMD07 registers are to be referenced is changed in accordance with the sequence length that is set in these bits. The relationship among the setting of these bits, sequence length, and SPCMD0 to SPCMD7 registers referenced by the RSPI is shown above. However, the RSPI in slave mode always references SPCMD0.
pub type SPSLN_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPSLN_A, crate::Safe>;
impl<'a, REG> SPSLN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Length 1 SPDMDx x = 0->0->...
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SPSLN_A::_000)
    }
    ///Length 2 SPDMDx x = 0->1->0->...
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SPSLN_A::_001)
    }
    ///Length 3 SPDMDx x = 0->1->2->0->...
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SPSLN_A::_010)
    }
    ///Length 4 SPDMDx x = 0->1->2->3->0->...
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SPSLN_A::_011)
    }
    ///Length 5 SPDMDx x = 0->1->2->3->4->0->...
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(SPSLN_A::_100)
    }
    ///Length 6 SPDMDx x = 0->1->2->3->4->5->0->...
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(SPSLN_A::_101)
    }
    ///Length 7 SPDMDx x = 0->1->2->3->4->5->6->0->...
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(SPSLN_A::_110)
    }
    ///Length 8 SPDMDx x = 0->1->2->3->4->5->6->7->0->...
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(SPSLN_A::_111)
    }
}
impl R {
    ///Bits 0:2 - RSPI Sequence Length SpecificationThe order in which the SPCMD0 to SPCMD07 registers are to be referenced is changed in accordance with the sequence length that is set in these bits. The relationship among the setting of these bits, sequence length, and SPCMD0 to SPCMD7 registers referenced by the RSPI is shown above. However, the RSPI in slave mode always references SPCMD0.
    #[inline(always)]
    pub fn spsln(&self) -> SPSLN_R {
        SPSLN_R::new(self.bits & 7)
    }
}
impl W {
    ///Bits 0:2 - RSPI Sequence Length SpecificationThe order in which the SPCMD0 to SPCMD07 registers are to be referenced is changed in accordance with the sequence length that is set in these bits. The relationship among the setting of these bits, sequence length, and SPCMD0 to SPCMD7 registers referenced by the RSPI is shown above. However, the RSPI in slave mode always references SPCMD0.
    #[inline(always)]
    pub fn spsln(&mut self) -> SPSLN_W<SPSCR_SPEC> {
        SPSLN_W::new(self, 0)
    }
}
/**SPI Sequence Control Register

You can [`read`](crate::Reg::read) this register and get [`spscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPSCR_SPEC;
impl crate::RegisterSpec for SPSCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`spscr::R`](R) reader structure
impl crate::Readable for SPSCR_SPEC {}
///`write(|w| ..)` method takes [`spscr::W`](W) writer structure
impl crate::Writable for SPSCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPSCR to value 0
impl crate::Resettable for SPSCR_SPEC {}
