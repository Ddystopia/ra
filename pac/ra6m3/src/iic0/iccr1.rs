///Register `ICCR1` reader
pub type R = crate::R<ICCR1_SPEC>;
///Register `ICCR1` writer
pub type W = crate::W<ICCR1_SPEC>;
/**SDA Line Monitor

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDAI_A {
    ///0: SDAn line is low.
    _0 = 0,
    ///1: SDAn line is high.
    _1 = 1,
}
impl From<SDAI_A> for bool {
    #[inline(always)]
    fn from(variant: SDAI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDAI` reader - SDA Line Monitor
pub type SDAI_R = crate::BitReader<SDAI_A>;
impl SDAI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDAI_A {
        match self.bits {
            false => SDAI_A::_0,
            true => SDAI_A::_1,
        }
    }
    ///SDAn line is low.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDAI_A::_0
    }
    ///SDAn line is high.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDAI_A::_1
    }
}
/**SCL Line Monitor

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLI_A {
    ///0: SCLn line is low.
    _0 = 0,
    ///1: SCLn line is high.
    _1 = 1,
}
impl From<SCLI_A> for bool {
    #[inline(always)]
    fn from(variant: SCLI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SCLI` reader - SCL Line Monitor
pub type SCLI_R = crate::BitReader<SCLI_A>;
impl SCLI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCLI_A {
        match self.bits {
            false => SCLI_A::_0,
            true => SCLI_A::_1,
        }
    }
    ///SCLn line is low.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCLI_A::_0
    }
    ///SCLn line is high.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCLI_A::_1
    }
}
/**SDA Output Control/Monitor

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDAO_A {
    ///0: (Read)The RIIC has driven the SDAn pin low. / (Write)The RIIC drives the SDAn pin low.
    _0 = 0,
    ///1: (Read)The RIIC has released the SDAn pin./ (Write)The RIIC releases the SDAn pin.
    _1 = 1,
}
impl From<SDAO_A> for bool {
    #[inline(always)]
    fn from(variant: SDAO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SDAO` reader - SDA Output Control/Monitor
pub type SDAO_R = crate::BitReader<SDAO_A>;
impl SDAO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDAO_A {
        match self.bits {
            false => SDAO_A::_0,
            true => SDAO_A::_1,
        }
    }
    ///(Read)The RIIC has driven the SDAn pin low. / (Write)The RIIC drives the SDAn pin low.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDAO_A::_0
    }
    ///(Read)The RIIC has released the SDAn pin./ (Write)The RIIC releases the SDAn pin.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDAO_A::_1
    }
}
///Field `SDAO` writer - SDA Output Control/Monitor
pub type SDAO_W<'a, REG> = crate::BitWriter<'a, REG, SDAO_A>;
impl<'a, REG> SDAO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///(Read)The RIIC has driven the SDAn pin low. / (Write)The RIIC drives the SDAn pin low.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SDAO_A::_0)
    }
    ///(Read)The RIIC has released the SDAn pin./ (Write)The RIIC releases the SDAn pin.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SDAO_A::_1)
    }
}
/**SCL Output Control/Monitor

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLO_A {
    ///0: (Read)The RIIC has driven the SCLn pin low. / (Write)The RIIC drives the SCLn pin low.
    _0 = 0,
    ///1: (Read)The RIIC has released the SCLn pin. / (Write)The RIIC releases the SCLn pin.
    _1 = 1,
}
impl From<SCLO_A> for bool {
    #[inline(always)]
    fn from(variant: SCLO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SCLO` reader - SCL Output Control/Monitor
pub type SCLO_R = crate::BitReader<SCLO_A>;
impl SCLO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCLO_A {
        match self.bits {
            false => SCLO_A::_0,
            true => SCLO_A::_1,
        }
    }
    ///(Read)The RIIC has driven the SCLn pin low. / (Write)The RIIC drives the SCLn pin low.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCLO_A::_0
    }
    ///(Read)The RIIC has released the SCLn pin. / (Write)The RIIC releases the SCLn pin.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCLO_A::_1
    }
}
///Field `SCLO` writer - SCL Output Control/Monitor
pub type SCLO_W<'a, REG> = crate::BitWriter<'a, REG, SCLO_A>;
impl<'a, REG> SCLO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///(Read)The RIIC has driven the SCLn pin low. / (Write)The RIIC drives the SCLn pin low.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SCLO_A::_0)
    }
    ///(Read)The RIIC has released the SCLn pin. / (Write)The RIIC releases the SCLn pin.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SCLO_A::_1)
    }
}
/**SCLO/SDAO Write Protect

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOWP_A {
    ///0: Enables a value to be written in SCLO bit and SDAO bit.
    _0 = 0,
    ///1: Disables a value to be written in SCLO bit and SDAO bit.
    _1 = 1,
}
impl From<SOWP_A> for bool {
    #[inline(always)]
    fn from(variant: SOWP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SOWP` writer - SCLO/SDAO Write Protect
pub type SOWP_W<'a, REG> = crate::BitWriter<'a, REG, SOWP_A>;
impl<'a, REG> SOWP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables a value to be written in SCLO bit and SDAO bit.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SOWP_A::_0)
    }
    ///Disables a value to be written in SCLO bit and SDAO bit.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SOWP_A::_1)
    }
}
/**Extra SCL Clock Cycle Output

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLO_A {
    ///0: Does not output an extra SCL clock cycle.
    _0 = 0,
    ///1: Outputs an extra SCL clock cycle.
    _1 = 1,
}
impl From<CLO_A> for bool {
    #[inline(always)]
    fn from(variant: CLO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CLO` reader - Extra SCL Clock Cycle Output
pub type CLO_R = crate::BitReader<CLO_A>;
impl CLO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLO_A {
        match self.bits {
            false => CLO_A::_0,
            true => CLO_A::_1,
        }
    }
    ///Does not output an extra SCL clock cycle.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLO_A::_0
    }
    ///Outputs an extra SCL clock cycle.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLO_A::_1
    }
}
///Field `CLO` writer - Extra SCL Clock Cycle Output
pub type CLO_W<'a, REG> = crate::BitWriter<'a, REG, CLO_A>;
impl<'a, REG> CLO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Does not output an extra SCL clock cycle.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CLO_A::_0)
    }
    ///Outputs an extra SCL clock cycle.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CLO_A::_1)
    }
}
/**I2C Bus Interface Internal ResetNote:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information).

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICRST_A {
    ///0: Releases the RIIC reset or internal reset.
    _0 = 0,
    ///1: Initiates the RIIC reset or internal reset.
    _1 = 1,
}
impl From<IICRST_A> for bool {
    #[inline(always)]
    fn from(variant: IICRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IICRST` reader - I2C Bus Interface Internal ResetNote:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information).
pub type IICRST_R = crate::BitReader<IICRST_A>;
impl IICRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IICRST_A {
        match self.bits {
            false => IICRST_A::_0,
            true => IICRST_A::_1,
        }
    }
    ///Releases the RIIC reset or internal reset.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICRST_A::_0
    }
    ///Initiates the RIIC reset or internal reset.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICRST_A::_1
    }
}
///Field `IICRST` writer - I2C Bus Interface Internal ResetNote:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information).
pub type IICRST_W<'a, REG> = crate::BitWriter<'a, REG, IICRST_A>;
impl<'a, REG> IICRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Releases the RIIC reset or internal reset.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IICRST_A::_0)
    }
    ///Initiates the RIIC reset or internal reset.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IICRST_A::_1)
    }
}
/**I2C Bus Interface Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICE_A {
    ///0: Disable (SCLn and SDAn pins in inactive state)
    _0 = 0,
    ///1: Enable (SCLn and SDAn pins in active state)
    _1 = 1,
}
impl From<ICE_A> for bool {
    #[inline(always)]
    fn from(variant: ICE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ICE` reader - I2C Bus Interface Enable
pub type ICE_R = crate::BitReader<ICE_A>;
impl ICE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICE_A {
        match self.bits {
            false => ICE_A::_0,
            true => ICE_A::_1,
        }
    }
    ///Disable (SCLn and SDAn pins in inactive state)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICE_A::_0
    }
    ///Enable (SCLn and SDAn pins in active state)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICE_A::_1
    }
}
///Field `ICE` writer - I2C Bus Interface Enable
pub type ICE_W<'a, REG> = crate::BitWriter<'a, REG, ICE_A>;
impl<'a, REG> ICE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable (SCLn and SDAn pins in inactive state)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ICE_A::_0)
    }
    ///Enable (SCLn and SDAn pins in active state)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ICE_A::_1)
    }
}
impl R {
    ///Bit 0 - SDA Line Monitor
    #[inline(always)]
    pub fn sdai(&self) -> SDAI_R {
        SDAI_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SCL Line Monitor
    #[inline(always)]
    pub fn scli(&self) -> SCLI_R {
        SCLI_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SDA Output Control/Monitor
    #[inline(always)]
    pub fn sdao(&self) -> SDAO_R {
        SDAO_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SCL Output Control/Monitor
    #[inline(always)]
    pub fn sclo(&self) -> SCLO_R {
        SCLO_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Extra SCL Clock Cycle Output
    #[inline(always)]
    pub fn clo(&self) -> CLO_R {
        CLO_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - I2C Bus Interface Internal ResetNote:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information).
    #[inline(always)]
    pub fn iicrst(&self) -> IICRST_R {
        IICRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C Bus Interface Enable
    #[inline(always)]
    pub fn ice(&self) -> ICE_R {
        ICE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - SDA Output Control/Monitor
    #[inline(always)]
    pub fn sdao(&mut self) -> SDAO_W<ICCR1_SPEC> {
        SDAO_W::new(self, 2)
    }
    ///Bit 3 - SCL Output Control/Monitor
    #[inline(always)]
    pub fn sclo(&mut self) -> SCLO_W<ICCR1_SPEC> {
        SCLO_W::new(self, 3)
    }
    ///Bit 4 - SCLO/SDAO Write Protect
    #[inline(always)]
    pub fn sowp(&mut self) -> SOWP_W<ICCR1_SPEC> {
        SOWP_W::new(self, 4)
    }
    ///Bit 5 - Extra SCL Clock Cycle Output
    #[inline(always)]
    pub fn clo(&mut self) -> CLO_W<ICCR1_SPEC> {
        CLO_W::new(self, 5)
    }
    ///Bit 6 - I2C Bus Interface Internal ResetNote:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information).
    #[inline(always)]
    pub fn iicrst(&mut self) -> IICRST_W<ICCR1_SPEC> {
        IICRST_W::new(self, 6)
    }
    ///Bit 7 - I2C Bus Interface Enable
    #[inline(always)]
    pub fn ice(&mut self) -> ICE_W<ICCR1_SPEC> {
        ICE_W::new(self, 7)
    }
}
/**I2C Bus Control Register 1

You can [`read`](crate::Reg::read) this register and get [`iccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICCR1_SPEC;
impl crate::RegisterSpec for ICCR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`iccr1::R`](R) reader structure
impl crate::Readable for ICCR1_SPEC {}
///`write(|w| ..)` method takes [`iccr1::W`](W) writer structure
impl crate::Writable for ICCR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICCR1 to value 0x1f
impl crate::Resettable for ICCR1_SPEC {
    const RESET_VALUE: u8 = 0x1f;
}
