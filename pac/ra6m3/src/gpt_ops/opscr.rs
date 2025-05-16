///Register `OPSCR` reader
pub type R = crate::R<OPSCR_SPEC>;
///Register `OPSCR` writer
pub type W = crate::W<OPSCR_SPEC>;
///Field `UF` reader - Input Phase Soft Setting WFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1.
pub type UF_R = crate::BitReader;
///Field `UF` writer - Input Phase Soft Setting WFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1.
pub type UF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VF` reader - Input Phase Soft Setting VFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1.
pub type VF_R = crate::BitReader;
///Field `VF` writer - Input Phase Soft Setting VFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1.
pub type VF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WF` reader - Input Phase Soft Setting UFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1.
pub type WF_R = crate::BitReader;
///Field `WF` writer - Input Phase Soft Setting UFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1.
pub type WF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `U` reader - Input U-Phase MonitorThis bit monitors the state of the input phase.OPSCR.FB=0:External input monitoring by PCLKOPSCR.FB=1:Software settings (UF/VF/WF)
pub type U_R = crate::BitReader;
///Field `V` reader - Input V-Phase MonitorThis bit monitors the state of the input phase.OPSCR.FB=0:External input monitoring by PCLKOPSCR.FB=1:Software settings (UF/VF/WF)
pub type V_R = crate::BitReader;
///Field `W` reader - Input W-Phase MonitorThis bit monitors the state of the input phase.OPSCR.FB=0:External input monitoring by PCLKOPSCR.FB=1:Software settings (UF/VF/WF)
pub type W_R = crate::BitReader;
/**Enable-Phase Output Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    ///0: Not Output(Hi-Z external terminals).
    _0 = 0,
    ///1: Output
    _1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Enable-Phase Output Control
pub type EN_R = crate::BitReader<EN_A>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::_0,
            true => EN_A::_1,
        }
    }
    ///Not Output(Hi-Z external terminals).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_A::_0
    }
    ///Output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_A::_1
    }
}
///Field `EN` writer - Enable-Phase Output Control
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not Output(Hi-Z external terminals).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::_0)
    }
    ///Output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::_1)
    }
}
/**External Feedback Signal EnableThis bit selects the input phase from the software settings and external input.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FB_A {
    ///0: Select the external input.
    _0 = 0,
    ///1: Select the soft setting(OPSCR.UF, VF, WF).
    _1 = 1,
}
impl From<FB_A> for bool {
    #[inline(always)]
    fn from(variant: FB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FB` reader - External Feedback Signal EnableThis bit selects the input phase from the software settings and external input.
pub type FB_R = crate::BitReader<FB_A>;
impl FB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FB_A {
        match self.bits {
            false => FB_A::_0,
            true => FB_A::_1,
        }
    }
    ///Select the external input.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FB_A::_0
    }
    ///Select the soft setting(OPSCR.UF, VF, WF).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FB_A::_1
    }
}
///Field `FB` writer - External Feedback Signal EnableThis bit selects the input phase from the software settings and external input.
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG, FB_A>;
impl<'a, REG> FB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Select the external input.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FB_A::_0)
    }
    ///Select the soft setting(OPSCR.UF, VF, WF).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FB_A::_1)
    }
}
/**Positive-Phase Output (P) Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P_A {
    ///0: Level signal output
    _0 = 0,
    ///1: PWM signal output (PWM of GPT0)
    _1 = 1,
}
impl From<P_A> for bool {
    #[inline(always)]
    fn from(variant: P_A) -> Self {
        variant as u8 != 0
    }
}
///Field `P` reader - Positive-Phase Output (P) Control
pub type P_R = crate::BitReader<P_A>;
impl P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> P_A {
        match self.bits {
            false => P_A::_0,
            true => P_A::_1,
        }
    }
    ///Level signal output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P_A::_0
    }
    ///PWM signal output (PWM of GPT0)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P_A::_1
    }
}
///Field `P` writer - Positive-Phase Output (P) Control
pub type P_W<'a, REG> = crate::BitWriter<'a, REG, P_A>;
impl<'a, REG> P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Level signal output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(P_A::_0)
    }
    ///PWM signal output (PWM of GPT0)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(P_A::_1)
    }
}
/**Negative-Phase Output (N) Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_A {
    ///0: Level signal output
    _0 = 0,
    ///1: PWM signal output (PWM of GPT0)
    _1 = 1,
}
impl From<N_A> for bool {
    #[inline(always)]
    fn from(variant: N_A) -> Self {
        variant as u8 != 0
    }
}
///Field `N` reader - Negative-Phase Output (N) Control
pub type N_R = crate::BitReader<N_A>;
impl N_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_A {
        match self.bits {
            false => N_A::_0,
            true => N_A::_1,
        }
    }
    ///Level signal output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == N_A::_0
    }
    ///PWM signal output (PWM of GPT0)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == N_A::_1
    }
}
///Field `N` writer - Negative-Phase Output (N) Control
pub type N_W<'a, REG> = crate::BitWriter<'a, REG, N_A>;
impl<'a, REG> N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Level signal output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(N_A::_0)
    }
    ///PWM signal output (PWM of GPT0)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(N_A::_1)
    }
}
/**Invert-Phase Output Control

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    ///0: Positive Logic (Active High)output
    _0 = 0,
    ///1: Negative Logic (Active Low)output
    _1 = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INV` reader - Invert-Phase Output Control
pub type INV_R = crate::BitReader<INV_A>;
impl INV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::_0,
            true => INV_A::_1,
        }
    }
    ///Positive Logic (Active High)output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV_A::_0
    }
    ///Negative Logic (Active Low)output
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV_A::_1
    }
}
///Field `INV` writer - Invert-Phase Output Control
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG, INV_A>;
impl<'a, REG> INV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Positive Logic (Active High)output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(INV_A::_0)
    }
    ///Negative Logic (Active Low)output
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(INV_A::_1)
    }
}
/**Output phase rotation direction reversal

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RV_A {
    ///0: U/V/W-Phase output
    _0 = 0,
    ///1: Output to reverse the V / W-phase
    _1 = 1,
}
impl From<RV_A> for bool {
    #[inline(always)]
    fn from(variant: RV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RV` reader - Output phase rotation direction reversal
pub type RV_R = crate::BitReader<RV_A>;
impl RV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RV_A {
        match self.bits {
            false => RV_A::_0,
            true => RV_A::_1,
        }
    }
    ///U/V/W-Phase output
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RV_A::_0
    }
    ///Output to reverse the V / W-phase
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RV_A::_1
    }
}
///Field `RV` writer - Output phase rotation direction reversal
pub type RV_W<'a, REG> = crate::BitWriter<'a, REG, RV_A>;
impl<'a, REG> RV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///U/V/W-Phase output
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RV_A::_0)
    }
    ///Output to reverse the V / W-phase
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RV_A::_1)
    }
}
/**Input phase alignment

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN_A {
    ///0: Input phase is aligned to PCLK.
    _0 = 0,
    ///1: Input phase is aligned PWM.
    _1 = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALIGN` reader - Input phase alignment
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
impl ALIGN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::_0,
            true => ALIGN_A::_1,
        }
    }
    ///Input phase is aligned to PCLK.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALIGN_A::_0
    }
    ///Input phase is aligned PWM.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALIGN_A::_1
    }
}
///Field `ALIGN` writer - Input phase alignment
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG, ALIGN_A>;
impl<'a, REG> ALIGN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input phase is aligned to PCLK.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN_A::_0)
    }
    ///Input phase is aligned PWM.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN_A::_1)
    }
}
/**Output disabled source selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GRP_A {
    ///0: Select Group A output disable source
    _00 = 0,
    ///1: Select Group B output disable source
    _01 = 1,
    ///2: Select Group C output disable source
    _10 = 2,
    ///3: Select Group D output disable source
    _11 = 3,
}
impl From<GRP_A> for u8 {
    #[inline(always)]
    fn from(variant: GRP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GRP_A {
    type Ux = u8;
}
impl crate::IsEnum for GRP_A {}
///Field `GRP` reader - Output disabled source selection
pub type GRP_R = crate::FieldReader<GRP_A>;
impl GRP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GRP_A {
        match self.bits {
            0 => GRP_A::_00,
            1 => GRP_A::_01,
            2 => GRP_A::_10,
            3 => GRP_A::_11,
            _ => unreachable!(),
        }
    }
    ///Select Group A output disable source
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == GRP_A::_00
    }
    ///Select Group B output disable source
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == GRP_A::_01
    }
    ///Select Group C output disable source
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == GRP_A::_10
    }
    ///Select Group D output disable source
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == GRP_A::_11
    }
}
///Field `GRP` writer - Output disabled source selection
pub type GRP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, GRP_A, crate::Safe>;
impl<'a, REG> GRP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select Group A output disable source
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(GRP_A::_00)
    }
    ///Select Group B output disable source
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(GRP_A::_01)
    }
    ///Select Group C output disable source
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(GRP_A::_10)
    }
    ///Select Group D output disable source
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(GRP_A::_11)
    }
}
/**Group output disable function

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GODF_A {
    ///0: This bit function is ignored.
    _0 = 0,
    ///1: Group disable will clear OPSCR.EN Bit.
    _1 = 1,
}
impl From<GODF_A> for bool {
    #[inline(always)]
    fn from(variant: GODF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GODF` reader - Group output disable function
pub type GODF_R = crate::BitReader<GODF_A>;
impl GODF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GODF_A {
        match self.bits {
            false => GODF_A::_0,
            true => GODF_A::_1,
        }
    }
    ///This bit function is ignored.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GODF_A::_0
    }
    ///Group disable will clear OPSCR.EN Bit.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GODF_A::_1
    }
}
///Field `GODF` writer - Group output disable function
pub type GODF_W<'a, REG> = crate::BitWriter<'a, REG, GODF_A>;
impl<'a, REG> GODF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit function is ignored.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(GODF_A::_0)
    }
    ///Group disable will clear OPSCR.EN Bit.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(GODF_A::_1)
    }
}
/**External Input Noise Filter Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFEN_A {
    ///0: Do not use a noise filter to the external input.
    _0 = 0,
    ///1: Use a noise filter to the external input.
    _1 = 1,
}
impl From<NFEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NFEN` reader - External Input Noise Filter Enable
pub type NFEN_R = crate::BitReader<NFEN_A>;
impl NFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFEN_A {
        match self.bits {
            false => NFEN_A::_0,
            true => NFEN_A::_1,
        }
    }
    ///Do not use a noise filter to the external input.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFEN_A::_0
    }
    ///Use a noise filter to the external input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFEN_A::_1
    }
}
///Field `NFEN` writer - External Input Noise Filter Enable
pub type NFEN_W<'a, REG> = crate::BitWriter<'a, REG, NFEN_A>;
impl<'a, REG> NFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not use a noise filter to the external input.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NFEN_A::_0)
    }
    ///Use a noise filter to the external input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NFEN_A::_1)
    }
}
/**External Input Noise Filter Clock selectionNoise filter sampling clock setting of the external input.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCS_A {
    ///0: PCLK/1
    _00 = 0,
    ///1: PCLK/4
    _01 = 1,
    ///2: PCLK/16
    _10 = 2,
    ///3: PCLK/64
    _11 = 3,
}
impl From<NFCS_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NFCS_A {
    type Ux = u8;
}
impl crate::IsEnum for NFCS_A {}
///Field `NFCS` reader - External Input Noise Filter Clock selectionNoise filter sampling clock setting of the external input.
pub type NFCS_R = crate::FieldReader<NFCS_A>;
impl NFCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NFCS_A {
        match self.bits {
            0 => NFCS_A::_00,
            1 => NFCS_A::_01,
            2 => NFCS_A::_10,
            3 => NFCS_A::_11,
            _ => unreachable!(),
        }
    }
    ///PCLK/1
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCS_A::_00
    }
    ///PCLK/4
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCS_A::_01
    }
    ///PCLK/16
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCS_A::_10
    }
    ///PCLK/64
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCS_A::_11
    }
}
///Field `NFCS` writer - External Input Noise Filter Clock selectionNoise filter sampling clock setting of the external input.
pub type NFCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, NFCS_A, crate::Safe>;
impl<'a, REG> NFCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK/1
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_00)
    }
    ///PCLK/4
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_01)
    }
    ///PCLK/16
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_10)
    }
    ///PCLK/64
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(NFCS_A::_11)
    }
}
impl R {
    ///Bit 0 - Input Phase Soft Setting WFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1.
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Input Phase Soft Setting VFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1.
    #[inline(always)]
    pub fn vf(&self) -> VF_R {
        VF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Input Phase Soft Setting UFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1.
    #[inline(always)]
    pub fn wf(&self) -> WF_R {
        WF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Input U-Phase MonitorThis bit monitors the state of the input phase.OPSCR.FB=0:External input monitoring by PCLKOPSCR.FB=1:Software settings (UF/VF/WF)
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Input V-Phase MonitorThis bit monitors the state of the input phase.OPSCR.FB=0:External input monitoring by PCLKOPSCR.FB=1:Software settings (UF/VF/WF)
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Input W-Phase MonitorThis bit monitors the state of the input phase.OPSCR.FB=0:External input monitoring by PCLKOPSCR.FB=1:Software settings (UF/VF/WF)
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Enable-Phase Output Control
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - External Feedback Signal EnableThis bit selects the input phase from the software settings and external input.
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Positive-Phase Output (P) Control
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Negative-Phase Output (N) Control
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Invert-Phase Output Control
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Output phase rotation direction reversal
    #[inline(always)]
    pub fn rv(&self) -> RV_R {
        RV_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Input phase alignment
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 24:25 - Output disabled source selection
    #[inline(always)]
    pub fn grp(&self) -> GRP_R {
        GRP_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - Group output disable function
    #[inline(always)]
    pub fn godf(&self) -> GODF_R {
        GODF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 29 - External Input Noise Filter Enable
    #[inline(always)]
    pub fn nfen(&self) -> NFEN_R {
        NFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - External Input Noise Filter Clock selectionNoise filter sampling clock setting of the external input.
    #[inline(always)]
    pub fn nfcs(&self) -> NFCS_R {
        NFCS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Input Phase Soft Setting WFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1.
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W<OPSCR_SPEC> {
        UF_W::new(self, 0)
    }
    ///Bit 1 - Input Phase Soft Setting VFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1.
    #[inline(always)]
    pub fn vf(&mut self) -> VF_W<OPSCR_SPEC> {
        VF_W::new(self, 1)
    }
    ///Bit 2 - Input Phase Soft Setting UFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1.
    #[inline(always)]
    pub fn wf(&mut self) -> WF_W<OPSCR_SPEC> {
        WF_W::new(self, 2)
    }
    ///Bit 8 - Enable-Phase Output Control
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<OPSCR_SPEC> {
        EN_W::new(self, 8)
    }
    ///Bit 16 - External Feedback Signal EnableThis bit selects the input phase from the software settings and external input.
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<OPSCR_SPEC> {
        FB_W::new(self, 16)
    }
    ///Bit 17 - Positive-Phase Output (P) Control
    #[inline(always)]
    pub fn p(&mut self) -> P_W<OPSCR_SPEC> {
        P_W::new(self, 17)
    }
    ///Bit 18 - Negative-Phase Output (N) Control
    #[inline(always)]
    pub fn n(&mut self) -> N_W<OPSCR_SPEC> {
        N_W::new(self, 18)
    }
    ///Bit 19 - Invert-Phase Output Control
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W<OPSCR_SPEC> {
        INV_W::new(self, 19)
    }
    ///Bit 20 - Output phase rotation direction reversal
    #[inline(always)]
    pub fn rv(&mut self) -> RV_W<OPSCR_SPEC> {
        RV_W::new(self, 20)
    }
    ///Bit 21 - Input phase alignment
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<OPSCR_SPEC> {
        ALIGN_W::new(self, 21)
    }
    ///Bits 24:25 - Output disabled source selection
    #[inline(always)]
    pub fn grp(&mut self) -> GRP_W<OPSCR_SPEC> {
        GRP_W::new(self, 24)
    }
    ///Bit 26 - Group output disable function
    #[inline(always)]
    pub fn godf(&mut self) -> GODF_W<OPSCR_SPEC> {
        GODF_W::new(self, 26)
    }
    ///Bit 29 - External Input Noise Filter Enable
    #[inline(always)]
    pub fn nfen(&mut self) -> NFEN_W<OPSCR_SPEC> {
        NFEN_W::new(self, 29)
    }
    ///Bits 30:31 - External Input Noise Filter Clock selectionNoise filter sampling clock setting of the external input.
    #[inline(always)]
    pub fn nfcs(&mut self) -> NFCS_W<OPSCR_SPEC> {
        NFCS_W::new(self, 30)
    }
}
/**Output Phase Switching Control Register

You can [`read`](crate::Reg::read) this register and get [`opscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OPSCR_SPEC;
impl crate::RegisterSpec for OPSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`opscr::R`](R) reader structure
impl crate::Readable for OPSCR_SPEC {}
///`write(|w| ..)` method takes [`opscr::W`](W) writer structure
impl crate::Writable for OPSCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPSCR to value 0
impl crate::Resettable for OPSCR_SPEC {}
