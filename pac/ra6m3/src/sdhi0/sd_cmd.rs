///Register `SD_CMD` reader
pub type R = crate::R<SD_CMD_SPEC>;
///Register `SD_CMD` writer
pub type W = crate::W<SD_CMD_SPEC>;
///Field `CMDIDX` reader - Command IndexThese bits specify Command Format\[45:40\] (command index).\[Examples\]CMD6: SD_CMD\[7:0\] = 8'b00_000110CMD18: SD_CMD\[7:0\] = 8'b00_010010ACMD13: SD_CMD\[7:0\] = 8'b01_001101
pub type CMDIDX_R = crate::FieldReader;
///Field `CMDIDX` writer - Command IndexThese bits specify Command Format\[45:40\] (command index).\[Examples\]CMD6: SD_CMD\[7:0\] = 8'b00_000110CMD18: SD_CMD\[7:0\] = 8'b00_010010ACMD13: SD_CMD\[7:0\] = 8'b01_001101
pub type CMDIDX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
/**Command Type Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACMD_A {
    ///0: CMD
    _00 = 0,
    ///1: ACMD
    _01 = 1,
    ///2: Setting prohibited
    OTHERS = 2,
}
impl From<ACMD_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACMD_A {
    type Ux = u8;
}
impl crate::IsEnum for ACMD_A {}
///Field `ACMD` reader - Command Type Select
pub type ACMD_R = crate::FieldReader<ACMD_A>;
impl ACMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACMD_A {
        match self.bits {
            0 => ACMD_A::_00,
            1 => ACMD_A::_01,
            _ => ACMD_A::OTHERS,
        }
    }
    ///CMD
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ACMD_A::_00
    }
    ///ACMD
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ACMD_A::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), ACMD_A::OTHERS)
    }
}
///Field `ACMD` writer - Command Type Select
pub type ACMD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ACMD_A, crate::Safe>;
impl<'a, REG> ACMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CMD
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(ACMD_A::_00)
    }
    ///ACMD
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(ACMD_A::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(ACMD_A::OTHERS)
    }
}
/**Mode/Response TypeNOTE: As some commands cannot be used in normal mode, see section 1.4.10, Example of SD_CMD Register Setting to select mode/response type.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSPTP_A {
    ///0: Normal mode The response type and the transfer mode are selected by SD_CMD\[7:0\], and the SD_CMD\[15:11\] setting is disabled.
    _000 = 0,
    ///3: Expansion mode and no response
    _011 = 3,
    ///4: Expansion mode and R1, R5, R6, or R7 response
    _100 = 4,
    ///5: Expansion mode and R1b response
    _101 = 5,
    ///6: Expansion mode and R2 response
    _110 = 6,
    ///7: Expansion mode and R3 or R4 response
    _111 = 7,
    ///1: Settings prohibited.
    OTHERS = 1,
}
impl From<RSPTP_A> for u8 {
    #[inline(always)]
    fn from(variant: RSPTP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSPTP_A {
    type Ux = u8;
}
impl crate::IsEnum for RSPTP_A {}
///Field `RSPTP` reader - Mode/Response TypeNOTE: As some commands cannot be used in normal mode, see section 1.4.10, Example of SD_CMD Register Setting to select mode/response type.
pub type RSPTP_R = crate::FieldReader<RSPTP_A>;
impl RSPTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSPTP_A {
        match self.bits {
            0 => RSPTP_A::_000,
            3 => RSPTP_A::_011,
            4 => RSPTP_A::_100,
            5 => RSPTP_A::_101,
            6 => RSPTP_A::_110,
            7 => RSPTP_A::_111,
            _ => RSPTP_A::OTHERS,
        }
    }
    ///Normal mode The response type and the transfer mode are selected by SD_CMD\[7:0\], and the SD_CMD\[15:11\] setting is disabled.
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RSPTP_A::_000
    }
    ///Expansion mode and no response
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RSPTP_A::_011
    }
    ///Expansion mode and R1, R5, R6, or R7 response
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RSPTP_A::_100
    }
    ///Expansion mode and R1b response
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RSPTP_A::_101
    }
    ///Expansion mode and R2 response
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RSPTP_A::_110
    }
    ///Expansion mode and R3 or R4 response
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RSPTP_A::_111
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), RSPTP_A::OTHERS)
    }
}
///Field `RSPTP` writer - Mode/Response TypeNOTE: As some commands cannot be used in normal mode, see section 1.4.10, Example of SD_CMD Register Setting to select mode/response type.
pub type RSPTP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RSPTP_A, crate::Safe>;
impl<'a, REG> RSPTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal mode The response type and the transfer mode are selected by SD_CMD\[7:0\], and the SD_CMD\[15:11\] setting is disabled.
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTP_A::_000)
    }
    ///Expansion mode and no response
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTP_A::_011)
    }
    ///Expansion mode and R1, R5, R6, or R7 response
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTP_A::_100)
    }
    ///Expansion mode and R1b response
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTP_A::_101)
    }
    ///Expansion mode and R2 response
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTP_A::_110)
    }
    ///Expansion mode and R3 or R4 response
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTP_A::_111)
    }
    ///Settings prohibited.
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTP_A::OTHERS)
    }
}
/**Data Mode (Command Type)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDTP_A {
    ///0: Command does not include data transfer (bc, bcr, or ac)
    _0 = 0,
    ///1: Command includes data transfer (adtc)
    _1 = 1,
}
impl From<CMDTP_A> for bool {
    #[inline(always)]
    fn from(variant: CMDTP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDTP` reader - Data Mode (Command Type)
pub type CMDTP_R = crate::BitReader<CMDTP_A>;
impl CMDTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMDTP_A {
        match self.bits {
            false => CMDTP_A::_0,
            true => CMDTP_A::_1,
        }
    }
    ///Command does not include data transfer (bc, bcr, or ac)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDTP_A::_0
    }
    ///Command includes data transfer (adtc)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDTP_A::_1
    }
}
///Field `CMDTP` writer - Data Mode (Command Type)
pub type CMDTP_W<'a, REG> = crate::BitWriter<'a, REG, CMDTP_A>;
impl<'a, REG> CMDTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Command does not include data transfer (bc, bcr, or ac)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTP_A::_0)
    }
    ///Command includes data transfer (adtc)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMDTP_A::_1)
    }
}
/**Write/Read Mode (enabled when the command with data is handled)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDRW_A {
    ///0: Write (SD/MMC host interface -> SD card/MMC)
    _0 = 0,
    ///1: Read (SD/MMC host interface <- SD card/MMC)
    _1 = 1,
}
impl From<CMDRW_A> for bool {
    #[inline(always)]
    fn from(variant: CMDRW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDRW` reader - Write/Read Mode (enabled when the command with data is handled)
pub type CMDRW_R = crate::BitReader<CMDRW_A>;
impl CMDRW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMDRW_A {
        match self.bits {
            false => CMDRW_A::_0,
            true => CMDRW_A::_1,
        }
    }
    ///Write (SD/MMC host interface -> SD card/MMC)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDRW_A::_0
    }
    ///Read (SD/MMC host interface <- SD card/MMC)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDRW_A::_1
    }
}
///Field `CMDRW` writer - Write/Read Mode (enabled when the command with data is handled)
pub type CMDRW_W<'a, REG> = crate::BitWriter<'a, REG, CMDRW_A>;
impl<'a, REG> CMDRW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write (SD/MMC host interface -> SD card/MMC)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMDRW_A::_0)
    }
    ///Read (SD/MMC host interface <- SD card/MMC)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMDRW_A::_1)
    }
}
/**Single/Multiple Block Transfer (enabled when the command with data is handled)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRSTP_A {
    ///0: Single block transfer
    _0 = 0,
    ///1: Multiple block transfer
    _1 = 1,
}
impl From<TRSTP_A> for bool {
    #[inline(always)]
    fn from(variant: TRSTP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TRSTP` reader - Single/Multiple Block Transfer (enabled when the command with data is handled)
pub type TRSTP_R = crate::BitReader<TRSTP_A>;
impl TRSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRSTP_A {
        match self.bits {
            false => TRSTP_A::_0,
            true => TRSTP_A::_1,
        }
    }
    ///Single block transfer
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRSTP_A::_0
    }
    ///Multiple block transfer
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRSTP_A::_1
    }
}
///Field `TRSTP` writer - Single/Multiple Block Transfer (enabled when the command with data is handled)
pub type TRSTP_W<'a, REG> = crate::BitWriter<'a, REG, TRSTP_A>;
impl<'a, REG> TRSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single block transfer
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TRSTP_A::_0)
    }
    ///Multiple block transfer
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TRSTP_A::_1)
    }
}
/**Multiple Block Transfer Mode (enabled at multiple block transfer)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD12AT_A {
    ///0: CMD12 is automatically issued at multiple block transfer.
    _00 = 0,
    ///1: CMD12 is not automatically issued at multiple block transfer.
    _01 = 1,
    ///2: Setting prohibited
    _10 = 2,
    ///3: Setting prohibited
    _11 = 3,
}
impl From<CMD12AT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD12AT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMD12AT_A {
    type Ux = u8;
}
impl crate::IsEnum for CMD12AT_A {}
///Field `CMD12AT` reader - Multiple Block Transfer Mode (enabled at multiple block transfer)
pub type CMD12AT_R = crate::FieldReader<CMD12AT_A>;
impl CMD12AT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMD12AT_A {
        match self.bits {
            0 => CMD12AT_A::_00,
            1 => CMD12AT_A::_01,
            2 => CMD12AT_A::_10,
            3 => CMD12AT_A::_11,
            _ => unreachable!(),
        }
    }
    ///CMD12 is automatically issued at multiple block transfer.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMD12AT_A::_00
    }
    ///CMD12 is not automatically issued at multiple block transfer.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMD12AT_A::_01
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMD12AT_A::_10
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMD12AT_A::_11
    }
}
///Field `CMD12AT` writer - Multiple Block Transfer Mode (enabled at multiple block transfer)
pub type CMD12AT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CMD12AT_A, crate::Safe>;
impl<'a, REG> CMD12AT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CMD12 is automatically issued at multiple block transfer.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CMD12AT_A::_00)
    }
    ///CMD12 is not automatically issued at multiple block transfer.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CMD12AT_A::_01)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CMD12AT_A::_10)
    }
    ///Setting prohibited
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CMD12AT_A::_11)
    }
}
impl R {
    ///Bits 0:5 - Command IndexThese bits specify Command Format\[45:40\] (command index).\[Examples\]CMD6: SD_CMD\[7:0\] = 8'b00_000110CMD18: SD_CMD\[7:0\] = 8'b00_010010ACMD13: SD_CMD\[7:0\] = 8'b01_001101
    #[inline(always)]
    pub fn cmdidx(&self) -> CMDIDX_R {
        CMDIDX_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - Command Type Select
    #[inline(always)]
    pub fn acmd(&self) -> ACMD_R {
        ACMD_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:10 - Mode/Response TypeNOTE: As some commands cannot be used in normal mode, see section 1.4.10, Example of SD_CMD Register Setting to select mode/response type.
    #[inline(always)]
    pub fn rsptp(&self) -> RSPTP_R {
        RSPTP_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - Data Mode (Command Type)
    #[inline(always)]
    pub fn cmdtp(&self) -> CMDTP_R {
        CMDTP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Write/Read Mode (enabled when the command with data is handled)
    #[inline(always)]
    pub fn cmdrw(&self) -> CMDRW_R {
        CMDRW_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Single/Multiple Block Transfer (enabled when the command with data is handled)
    #[inline(always)]
    pub fn trstp(&self) -> TRSTP_R {
        TRSTP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Multiple Block Transfer Mode (enabled at multiple block transfer)
    #[inline(always)]
    pub fn cmd12at(&self) -> CMD12AT_R {
        CMD12AT_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    ///Bits 0:5 - Command IndexThese bits specify Command Format\[45:40\] (command index).\[Examples\]CMD6: SD_CMD\[7:0\] = 8'b00_000110CMD18: SD_CMD\[7:0\] = 8'b00_010010ACMD13: SD_CMD\[7:0\] = 8'b01_001101
    #[inline(always)]
    pub fn cmdidx(&mut self) -> CMDIDX_W<SD_CMD_SPEC> {
        CMDIDX_W::new(self, 0)
    }
    ///Bits 6:7 - Command Type Select
    #[inline(always)]
    pub fn acmd(&mut self) -> ACMD_W<SD_CMD_SPEC> {
        ACMD_W::new(self, 6)
    }
    ///Bits 8:10 - Mode/Response TypeNOTE: As some commands cannot be used in normal mode, see section 1.4.10, Example of SD_CMD Register Setting to select mode/response type.
    #[inline(always)]
    pub fn rsptp(&mut self) -> RSPTP_W<SD_CMD_SPEC> {
        RSPTP_W::new(self, 8)
    }
    ///Bit 11 - Data Mode (Command Type)
    #[inline(always)]
    pub fn cmdtp(&mut self) -> CMDTP_W<SD_CMD_SPEC> {
        CMDTP_W::new(self, 11)
    }
    ///Bit 12 - Write/Read Mode (enabled when the command with data is handled)
    #[inline(always)]
    pub fn cmdrw(&mut self) -> CMDRW_W<SD_CMD_SPEC> {
        CMDRW_W::new(self, 12)
    }
    ///Bit 13 - Single/Multiple Block Transfer (enabled when the command with data is handled)
    #[inline(always)]
    pub fn trstp(&mut self) -> TRSTP_W<SD_CMD_SPEC> {
        TRSTP_W::new(self, 13)
    }
    ///Bits 14:15 - Multiple Block Transfer Mode (enabled at multiple block transfer)
    #[inline(always)]
    pub fn cmd12at(&mut self) -> CMD12AT_W<SD_CMD_SPEC> {
        CMD12AT_W::new(self, 14)
    }
}
/**Command Type Register

You can [`read`](crate::Reg::read) this register and get [`sd_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_CMD_SPEC;
impl crate::RegisterSpec for SD_CMD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_cmd::R`](R) reader structure
impl crate::Readable for SD_CMD_SPEC {}
///`write(|w| ..)` method takes [`sd_cmd::W`](W) writer structure
impl crate::Writable for SD_CMD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_CMD to value 0
impl crate::Resettable for SD_CMD_SPEC {}
