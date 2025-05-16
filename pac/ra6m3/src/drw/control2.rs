///Register `CONTROL2` writer
pub type W = crate::W<CONTROL2_SPEC>;
/**Pixel source is a pattern color (blend of COLOR1 and COLOR2 depending on PATTERN and pattern index)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PATTERNENABLE_A {
    ///0: disabled pattern
    _0 = 0,
    ///1: enabled pattern
    _1 = 1,
}
impl From<PATTERNENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PATTERNENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PATTERNENABLE` writer - Pixel source is a pattern color (blend of COLOR1 and COLOR2 depending on PATTERN and pattern index)
pub type PATTERNENABLE_W<'a, REG> = crate::BitWriter<'a, REG, PATTERNENABLE_A>;
impl<'a, REG> PATTERNENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled pattern
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PATTERNENABLE_A::_0)
    }
    ///enabled pattern
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PATTERNENABLE_A::_1)
    }
}
/**Pixel source is read from texture and used as an alpha to blend between COLOR1 and COLOR2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTUREENABLE_A {
    ///0: disabled texture
    _0 = 0,
    ///1: enabled texture
    _1 = 1,
}
impl From<TEXTUREENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TEXTUREENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEXTUREENABLE` writer - Pixel source is read from texture and used as an alpha to blend between COLOR1 and COLOR2
pub type TEXTUREENABLE_W<'a, REG> = crate::BitWriter<'a, REG, TEXTUREENABLE_A>;
impl<'a, REG> TEXTUREENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///disabled texture
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTUREENABLE_A::_0)
    }
    ///enabled texture
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTUREENABLE_A::_1)
    }
}
///Field `PATTERNSOURCEL5` writer - Limiter 5 is used as pattern index instead of the default U limiter.Limiter 5 can be combined with limiter 6 to form a quadratic limiter which can be used to make quadratic pattern functions to draw radial patterns.
pub type PATTERNSOURCEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Alpha blend mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USEACB_A {
    ///0: use WRITEALPHA\[1:0\] mode
    _0 = 0,
    ///1: use full alpha channel blending mode
    _1 = 1,
}
impl From<USEACB_A> for bool {
    #[inline(always)]
    fn from(variant: USEACB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USEACB` writer - Alpha blend mode
pub type USEACB_W<'a, REG> = crate::BitWriter<'a, REG, USEACB_A>;
impl<'a, REG> USEACB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///use WRITEALPHA\[1:0\] mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USEACB_A::_0)
    }
    ///use full alpha channel blending mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USEACB_A::_1)
    }
}
///Field `READFORMAT32` writer - Bit 4 and 3 of the texture buffer format.See READFORMAT above for description
pub type READFORMAT32_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Blend source factor for alpha channel in alpha channel blending mode (USEACB = 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSFA_A {
    ///0: use 1.0 as blend source factor for alpha channel
    _0 = 0,
    ///1: use alpha as blend source factor for alpha channel
    _1 = 1,
}
impl From<BSFA_A> for bool {
    #[inline(always)]
    fn from(variant: BSFA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSFA` writer - Blend source factor for alpha channel in alpha channel blending mode (USEACB = 1)
pub type BSFA_W<'a, REG> = crate::BitWriter<'a, REG, BSFA_A>;
impl<'a, REG> BSFA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///use 1.0 as blend source factor for alpha channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSFA_A::_0)
    }
    ///use alpha as blend source factor for alpha channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSFA_A::_1)
    }
}
/**Blend destinetion factor for alpha channel in alpha channel blending mode (USEACB = 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDFA_A {
    ///0: use 1.0 as blend destination factor for alpha channel
    _0 = 0,
    ///1: use alpha as blend destination factor for alpha channel
    _1 = 1,
}
impl From<BDFA_A> for bool {
    #[inline(always)]
    fn from(variant: BDFA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BDFA` writer - Blend destinetion factor for alpha channel in alpha channel blending mode (USEACB = 1)
pub type BDFA_W<'a, REG> = crate::BitWriter<'a, REG, BDFA_A>;
impl<'a, REG> BDFA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///use 1.0 as blend destination factor for alpha channel
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BDFA_A::_0)
    }
    ///use alpha as blend destination factor for alpha channel
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BDFA_A::_1)
    }
}
///Field `WRITEFORMAT2` writer - Bit 3 of framebuffer pixel formatSee WRITEFORMAT above description.
pub type WRITEFORMAT2_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Blend source factorsrc factor is alpha (factor is 1 per default)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSF_A {
    ///0: use 1.0 as blend source factor
    _0 = 0,
    ///1: use alpha as blend source factor
    _1 = 1,
}
impl From<BSF_A> for bool {
    #[inline(always)]
    fn from(variant: BSF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSF` writer - Blend source factorsrc factor is alpha (factor is 1 per default)
pub type BSF_W<'a, REG> = crate::BitWriter<'a, REG, BSF_A>;
impl<'a, REG> BSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///use 1.0 as blend source factor
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSF_A::_0)
    }
    ///use alpha as blend source factor
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSF_A::_1)
    }
}
/**Blend destination factordst factor is alpha (factor is 1 per default)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDF_A {
    ///0: use 1.0 as blend destination factor
    _0 = 0,
    ///1: use alpha as blend destination factor
    _1 = 1,
}
impl From<BDF_A> for bool {
    #[inline(always)]
    fn from(variant: BDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BDF` writer - Blend destination factordst factor is alpha (factor is 1 per default)
pub type BDF_W<'a, REG> = crate::BitWriter<'a, REG, BDF_A>;
impl<'a, REG> BDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///use 1.0 as blend destination factor
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BDF_A::_0)
    }
    ///use alpha as blend destination factor
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BDF_A::_1)
    }
}
/**Blend source factor is invertedsrc factor will be inverted (meaning 1-a or 1-1 depending on BSF)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSI_A {
    ///0: use blend factor as specified through BSF
    _0 = 0,
    ///1: invert blend source factor (1-x)
    _1 = 1,
}
impl From<BSI_A> for bool {
    #[inline(always)]
    fn from(variant: BSI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSI` writer - Blend source factor is invertedsrc factor will be inverted (meaning 1-a or 1-1 depending on BSF)
pub type BSI_W<'a, REG> = crate::BitWriter<'a, REG, BSI_A>;
impl<'a, REG> BSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///use blend factor as specified through BSF
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSI_A::_0)
    }
    ///invert blend source factor (1-x)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSI_A::_1)
    }
}
/**Blend destination factor is inverteddst factor will be inverted (meaning 1-a or 1-1 depending on BDF)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDI_A {
    ///0: use blend factor as specified through BDF
    _0 = 0,
    ///1: invert blend destinationfactor (1-x)
    _1 = 1,
}
impl From<BDI_A> for bool {
    #[inline(always)]
    fn from(variant: BDI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BDI` writer - Blend destination factor is inverteddst factor will be inverted (meaning 1-a or 1-1 depending on BDF)
pub type BDI_W<'a, REG> = crate::BitWriter<'a, REG, BDI_A>;
impl<'a, REG> BDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///use blend factor as specified through BDF
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BDI_A::_0)
    }
    ///invert blend destinationfactor (1-x)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BDI_A::_1)
    }
}
/**Blend color 2 instead of framebuffer pixel

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BC2_A {
    ///0: use pixel from framebuffer as destination (DST)
    _0 = 0,
    ///1: use color 2 as destination (DST)
    _1 = 1,
}
impl From<BC2_A> for bool {
    #[inline(always)]
    fn from(variant: BC2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BC2` writer - Blend color 2 instead of framebuffer pixel
pub type BC2_W<'a, REG> = crate::BitWriter<'a, REG, BC2_A>;
impl<'a, REG> BC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///use pixel from framebuffer as destination (DST)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BC2_A::_0)
    }
    ///use color 2 as destination (DST)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BC2_A::_1)
    }
}
/**Calculating U limiter outside use textureThe bit describes what happens if the U limiter (x direction in texture space) calculates a U value outside of the used texture

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTURECLAMPX_A {
    ///0: Texture wrap mode: The integer part of the calculated value from the u limiter is anded with TEXUMASK. This results in a repetition of the texture in x/u direction.
    _0 = 0,
    ///1: Texture clamp mode: The texture color at the border of the texture is taken. This results in a repetition of the texture border color in x/u direction.
    _1 = 1,
}
impl From<TEXTURECLAMPX_A> for bool {
    #[inline(always)]
    fn from(variant: TEXTURECLAMPX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEXTURECLAMPX` writer - Calculating U limiter outside use textureThe bit describes what happens if the U limiter (x direction in texture space) calculates a U value outside of the used texture
pub type TEXTURECLAMPX_W<'a, REG> = crate::BitWriter<'a, REG, TEXTURECLAMPX_A>;
impl<'a, REG> TEXTURECLAMPX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Texture wrap mode: The integer part of the calculated value from the u limiter is anded with TEXUMASK. This results in a repetition of the texture in x/u direction.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTURECLAMPX_A::_0)
    }
    ///Texture clamp mode: The texture color at the border of the texture is taken. This results in a repetition of the texture border color in x/u direction.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTURECLAMPX_A::_1)
    }
}
/**Calculating V limiter outside use textureThe bit describes what happens if the V limiter (y direction in texture space) calculates a V value outside of the used texture

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTURECLAMPY_A {
    ///0: Texture wrap mode: The integer part of the calculated value from the v limiter is anded with TEXVMASK. This results in a repetition of the texture in y/v direction.
    _0 = 0,
    ///1: Texture clamp mode: The texture color at the border of the texture is taken. This results in a repetition of the texture border color in y/v direction.
    _1 = 1,
}
impl From<TEXTURECLAMPY_A> for bool {
    #[inline(always)]
    fn from(variant: TEXTURECLAMPY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEXTURECLAMPY` writer - Calculating V limiter outside use textureThe bit describes what happens if the V limiter (y direction in texture space) calculates a V value outside of the used texture
pub type TEXTURECLAMPY_W<'a, REG> = crate::BitWriter<'a, REG, TEXTURECLAMPY_A>;
impl<'a, REG> TEXTURECLAMPY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Texture wrap mode: The integer part of the calculated value from the v limiter is anded with TEXVMASK. This results in a repetition of the texture in y/v direction.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTURECLAMPY_A::_0)
    }
    ///Texture clamp mode: The texture color at the border of the texture is taken. This results in a repetition of the texture border color in y/v direction.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTURECLAMPY_A::_1)
    }
}
/**Linear filtering on texture U axis

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTUREFILTERX_A {
    ///0: no filtering on texture U axis
    _0 = 0,
    ///1: linear filtering on texture U axis
    _1 = 1,
}
impl From<TEXTUREFILTERX_A> for bool {
    #[inline(always)]
    fn from(variant: TEXTUREFILTERX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEXTUREFILTERX` writer - Linear filtering on texture U axis
pub type TEXTUREFILTERX_W<'a, REG> = crate::BitWriter<'a, REG, TEXTUREFILTERX_A>;
impl<'a, REG> TEXTUREFILTERX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no filtering on texture U axis
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTUREFILTERX_A::_0)
    }
    ///linear filtering on texture U axis
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTUREFILTERX_A::_1)
    }
}
/**Linear filtering on texture V axis

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTUREFILTERY_A {
    ///0: no filtering on texture V axis
    _0 = 0,
    ///1: linear filtering on texture V axis
    _1 = 1,
}
impl From<TEXTUREFILTERY_A> for bool {
    #[inline(always)]
    fn from(variant: TEXTUREFILTERY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEXTUREFILTERY` writer - Linear filtering on texture V axis
pub type TEXTUREFILTERY_W<'a, REG> = crate::BitWriter<'a, REG, TEXTUREFILTERY_A>;
impl<'a, REG> TEXTUREFILTERY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///no filtering on texture V axis
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTUREFILTERY_A::_0)
    }
    ///linear filtering on texture V axis
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TEXTUREFILTERY_A::_1)
    }
}
/**Pixel format of the texture buffer{READFORMAT32,READFORMAT10}0000: 8 bpp a(8)0001: 16 bpp RGB(565)0010: 32 bpp aRGB(8888)0011: 16 bpp aRGB(4444)0100: 16 bpp aRGB(1555)0101: 8 bpp aCLUT(44) 4 bit alpha and 4 bit indexed color1001: 8 bpp CLUT(8)/I(8), 8 bit indexed color/luminance1010: 4 bpp CLUT(4)/I(4), 4 bit indexed color/luminance1011: 2 bpp CLUT(2)/I(2), 2 bit indexed color/luminance 1100: 1 bpp CLUT(1)/I(1), 1 bit indexed color/luminance

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum READFORMAT10_A {
    ///0: 8 bpp a(8) (READFORMAT32=00) / 16 bpp aRGB(1555) (READFORMAT32=01) / 1 bpp CLUT(1)/I(1), 1 bit indexed color/luminance (READFORMAT32=11)
    _00 = 0,
    ///1: 16 bpp RGB(565) (READFORMAT32=00) / 8 bpp aCLUT(44) 4 bit alpha and 4 bit indexed color (READFORMAT32=01) / 8 bpp CLUT(8)/I(8), 8 bit indexed color/luminance (READFORMAT32=10)
    _01 = 1,
    ///2: 32 bpp aRGB(8888) (READFORMAT32=00) / 4 bpp CLUT(4)/I(4), 4 bit indexed color/luminance (READFORMAT32=10)
    _10 = 2,
    ///3: 16 bpp aRGB(4444) (READFORMAT32=00) / 2 bpp CLUT(2)/I(2), 2 bit indexed color/luminance (READFORMAT32=10)
    _11 = 3,
}
impl From<READFORMAT10_A> for u8 {
    #[inline(always)]
    fn from(variant: READFORMAT10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for READFORMAT10_A {
    type Ux = u8;
}
impl crate::IsEnum for READFORMAT10_A {}
///Field `READFORMAT10` writer - Pixel format of the texture buffer{READFORMAT32,READFORMAT10}0000: 8 bpp a(8)0001: 16 bpp RGB(565)0010: 32 bpp aRGB(8888)0011: 16 bpp aRGB(4444)0100: 16 bpp aRGB(1555)0101: 8 bpp aCLUT(44) 4 bit alpha and 4 bit indexed color1001: 8 bpp CLUT(8)/I(8), 8 bit indexed color/luminance1010: 4 bpp CLUT(4)/I(4), 4 bit indexed color/luminance1011: 2 bpp CLUT(2)/I(2), 2 bit indexed color/luminance 1100: 1 bpp CLUT(1)/I(1), 1 bit indexed color/luminance
pub type READFORMAT10_W<'a, REG> = crate::FieldWriter<'a, REG, 2, READFORMAT10_A, crate::Safe>;
impl<'a, REG> READFORMAT10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bpp a(8) (READFORMAT32=00) / 16 bpp aRGB(1555) (READFORMAT32=01) / 1 bpp CLUT(1)/I(1), 1 bit indexed color/luminance (READFORMAT32=11)
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(READFORMAT10_A::_00)
    }
    ///16 bpp RGB(565) (READFORMAT32=00) / 8 bpp aCLUT(44) 4 bit alpha and 4 bit indexed color (READFORMAT32=01) / 8 bpp CLUT(8)/I(8), 8 bit indexed color/luminance (READFORMAT32=10)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(READFORMAT10_A::_01)
    }
    ///32 bpp aRGB(8888) (READFORMAT32=00) / 4 bpp CLUT(4)/I(4), 4 bit indexed color/luminance (READFORMAT32=10)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(READFORMAT10_A::_10)
    }
    ///16 bpp aRGB(4444) (READFORMAT32=00) / 2 bpp CLUT(2)/I(2), 2 bit indexed color/luminance (READFORMAT32=10)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(READFORMAT10_A::_11)
    }
}
/**Pixel format of the framebuffer

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRITEFORMAT10_A {
    ///0: 8bpp a(8)0
    _00 = 0,
    ///1: 16bpp RGB(565)
    _01 = 1,
    ///2: 32bpp aRGB(8888)
    _10 = 2,
    ///3: 16bpp aRGB(4444)
    _11 = 3,
}
impl From<WRITEFORMAT10_A> for u8 {
    #[inline(always)]
    fn from(variant: WRITEFORMAT10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WRITEFORMAT10_A {
    type Ux = u8;
}
impl crate::IsEnum for WRITEFORMAT10_A {}
///Field `WRITEFORMAT10` writer - Pixel format of the framebuffer
pub type WRITEFORMAT10_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WRITEFORMAT10_A, crate::Safe>;
impl<'a, REG> WRITEFORMAT10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8bpp a(8)0
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(WRITEFORMAT10_A::_00)
    }
    ///16bpp RGB(565)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(WRITEFORMAT10_A::_01)
    }
    ///32bpp aRGB(8888)
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(WRITEFORMAT10_A::_10)
    }
    ///16bpp aRGB(4444)
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(WRITEFORMAT10_A::_11)
    }
}
/**Writeback alpha source for framebufferSet the 'alpha source' for the framebuffer(USEACB = 0)Blend alpha in color 2 instead of framebuffer alpha((USEACB = 1))In not alpha channel blending mode (USEACB = 0):Set the 'alpha source' for the framebuffer.In alpha channel blending mode (USEACB = 1):Blend alpha in color 2 instead of framebuffer alpha00B: BC2A = 1: use alpha from framebuffer as destination (DST_A)else: BC2A = 0: use alpha in color 2 as destination (DST_A)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRITEALPHA_A {
    ///0: use alpha from color 2
    _00 = 0,
    ///1: use source alpha (pixel coverage)
    _01 = 1,
    ///2: use 0.0 as alpha
    _10 = 2,
    ///3: use alpha from framebuffer
    _11 = 3,
}
impl From<WRITEALPHA_A> for u8 {
    #[inline(always)]
    fn from(variant: WRITEALPHA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WRITEALPHA_A {
    type Ux = u8;
}
impl crate::IsEnum for WRITEALPHA_A {}
///Field `WRITEALPHA` writer - Writeback alpha source for framebufferSet the 'alpha source' for the framebuffer(USEACB = 0)Blend alpha in color 2 instead of framebuffer alpha((USEACB = 1))In not alpha channel blending mode (USEACB = 0):Set the 'alpha source' for the framebuffer.In alpha channel blending mode (USEACB = 1):Blend alpha in color 2 instead of framebuffer alpha00B: BC2A = 1: use alpha from framebuffer as destination (DST_A)else: BC2A = 0: use alpha in color 2 as destination (DST_A)
pub type WRITEALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WRITEALPHA_A, crate::Safe>;
impl<'a, REG> WRITEALPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///use alpha from color 2
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(WRITEALPHA_A::_00)
    }
    ///use source alpha (pixel coverage)
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(WRITEALPHA_A::_01)
    }
    ///use 0.0 as alpha
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(WRITEALPHA_A::_10)
    }
    ///use alpha from framebuffer
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(WRITEALPHA_A::_11)
    }
}
/**RLE enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLEENABLE_A {
    ///0: RLE disabled
    _0 = 0,
    ///1: RLE enabled
    _1 = 1,
}
impl From<RLEENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: RLEENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RLEENABLE` writer - RLE enable
pub type RLEENABLE_W<'a, REG> = crate::BitWriter<'a, REG, RLEENABLE_A>;
impl<'a, REG> RLEENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RLE disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RLEENABLE_A::_0)
    }
    ///RLE enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RLEENABLE_A::_1)
    }
}
/**CLUT enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLUTENABLE_A {
    ///0: CLUT disabled
    _0 = 0,
    ///1: CLUT enabled
    _1 = 1,
}
impl From<CLUTENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CLUTENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CLUTENABLE` writer - CLUT enable
pub type CLUTENABLE_W<'a, REG> = crate::BitWriter<'a, REG, CLUTENABLE_A>;
impl<'a, REG> CLUTENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CLUT disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CLUTENABLE_A::_0)
    }
    ///CLUT enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CLUTENABLE_A::_1)
    }
}
/**color keying enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLKEYENABLE_A {
    ///0: color keying disabled
    _0 = 0,
    ///1: color keying enabled
    _1 = 1,
}
impl From<COLKEYENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: COLKEYENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COLKEYENABLE` writer - color keying enable
pub type COLKEYENABLE_W<'a, REG> = crate::BitWriter<'a, REG, COLKEYENABLE_A>;
impl<'a, REG> COLKEYENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///color keying disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(COLKEYENABLE_A::_0)
    }
    ///color keying enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(COLKEYENABLE_A::_1)
    }
}
/**Format of the CLUT

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLUTFORMAT_A {
    ///0: aRGB(8888)
    _0 = 0,
    ///1: RGB(565)
    _1 = 1,
}
impl From<CLUTFORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: CLUTFORMAT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CLUTFORMAT` writer - Format of the CLUT
pub type CLUTFORMAT_W<'a, REG> = crate::BitWriter<'a, REG, CLUTFORMAT_A>;
impl<'a, REG> CLUTFORMAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///aRGB(8888)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CLUTFORMAT_A::_0)
    }
    ///RGB(565)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CLUTFORMAT_A::_1)
    }
}
/**Blend source factor inverted in alpha channel (USEACB = 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSIA_A {
    ///0: use blend factor as specified through BSFA
    _0 = 0,
    ///1: invert blend source factor (1-x)
    _1 = 1,
}
impl From<BSIA_A> for bool {
    #[inline(always)]
    fn from(variant: BSIA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSIA` writer - Blend source factor inverted in alpha channel (USEACB = 1)
pub type BSIA_W<'a, REG> = crate::BitWriter<'a, REG, BSIA_A>;
impl<'a, REG> BSIA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///use blend factor as specified through BSFA
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSIA_A::_0)
    }
    ///invert blend source factor (1-x)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSIA_A::_1)
    }
}
/**Blend destination factor inverted in alpha channel (USEACB = 1)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDIA_A {
    ///0: use blend factor as specified through BDFA
    _0 = 0,
    ///1: invert blend destination factor (1-x)
    _1 = 1,
}
impl From<BDIA_A> for bool {
    #[inline(always)]
    fn from(variant: BDIA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BDIA` writer - Blend destination factor inverted in alpha channel (USEACB = 1)
pub type BDIA_W<'a, REG> = crate::BitWriter<'a, REG, BDIA_A>;
impl<'a, REG> BDIA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///use blend factor as specified through BDFA
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BDIA_A::_0)
    }
    ///invert blend destination factor (1-x)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BDIA_A::_1)
    }
}
/**Texel width for RLE unit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RLEPIXELWIDTH_A {
    ///0: 1 byte per texel
    _00 = 0,
    ///1: 2 byte per texel
    _01 = 1,
    ///2: 3 byte per texel
    _10 = 2,
    ///3: 4 byte per texel
    _11 = 3,
}
impl From<RLEPIXELWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: RLEPIXELWIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RLEPIXELWIDTH_A {
    type Ux = u8;
}
impl crate::IsEnum for RLEPIXELWIDTH_A {}
///Field `RLEPIXELWIDTH` writer - Texel width for RLE unit
pub type RLEPIXELWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RLEPIXELWIDTH_A, crate::Safe>;
impl<'a, REG> RLEPIXELWIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 byte per texel
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(RLEPIXELWIDTH_A::_00)
    }
    ///2 byte per texel
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(RLEPIXELWIDTH_A::_01)
    }
    ///3 byte per texel
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(RLEPIXELWIDTH_A::_10)
    }
    ///4 byte per texel
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(RLEPIXELWIDTH_A::_11)
    }
}
impl W {
    ///Bit 0 - Pixel source is a pattern color (blend of COLOR1 and COLOR2 depending on PATTERN and pattern index)
    #[inline(always)]
    pub fn patternenable(&mut self) -> PATTERNENABLE_W<CONTROL2_SPEC> {
        PATTERNENABLE_W::new(self, 0)
    }
    ///Bit 1 - Pixel source is read from texture and used as an alpha to blend between COLOR1 and COLOR2
    #[inline(always)]
    pub fn textureenable(&mut self) -> TEXTUREENABLE_W<CONTROL2_SPEC> {
        TEXTUREENABLE_W::new(self, 1)
    }
    ///Bit 2 - Limiter 5 is used as pattern index instead of the default U limiter.Limiter 5 can be combined with limiter 6 to form a quadratic limiter which can be used to make quadratic pattern functions to draw radial patterns.
    #[inline(always)]
    pub fn patternsourcel5(&mut self) -> PATTERNSOURCEL5_W<CONTROL2_SPEC> {
        PATTERNSOURCEL5_W::new(self, 2)
    }
    ///Bit 3 - Alpha blend mode
    #[inline(always)]
    pub fn useacb(&mut self) -> USEACB_W<CONTROL2_SPEC> {
        USEACB_W::new(self, 3)
    }
    ///Bits 4:5 - Bit 4 and 3 of the texture buffer format.See READFORMAT above for description
    #[inline(always)]
    pub fn readformat32(&mut self) -> READFORMAT32_W<CONTROL2_SPEC> {
        READFORMAT32_W::new(self, 4)
    }
    ///Bit 6 - Blend source factor for alpha channel in alpha channel blending mode (USEACB = 1)
    #[inline(always)]
    pub fn bsfa(&mut self) -> BSFA_W<CONTROL2_SPEC> {
        BSFA_W::new(self, 6)
    }
    ///Bit 7 - Blend destinetion factor for alpha channel in alpha channel blending mode (USEACB = 1)
    #[inline(always)]
    pub fn bdfa(&mut self) -> BDFA_W<CONTROL2_SPEC> {
        BDFA_W::new(self, 7)
    }
    ///Bit 8 - Bit 3 of framebuffer pixel formatSee WRITEFORMAT above description.
    #[inline(always)]
    pub fn writeformat2(&mut self) -> WRITEFORMAT2_W<CONTROL2_SPEC> {
        WRITEFORMAT2_W::new(self, 8)
    }
    ///Bit 9 - Blend source factorsrc factor is alpha (factor is 1 per default)
    #[inline(always)]
    pub fn bsf(&mut self) -> BSF_W<CONTROL2_SPEC> {
        BSF_W::new(self, 9)
    }
    ///Bit 10 - Blend destination factordst factor is alpha (factor is 1 per default)
    #[inline(always)]
    pub fn bdf(&mut self) -> BDF_W<CONTROL2_SPEC> {
        BDF_W::new(self, 10)
    }
    ///Bit 11 - Blend source factor is invertedsrc factor will be inverted (meaning 1-a or 1-1 depending on BSF)
    #[inline(always)]
    pub fn bsi(&mut self) -> BSI_W<CONTROL2_SPEC> {
        BSI_W::new(self, 11)
    }
    ///Bit 12 - Blend destination factor is inverteddst factor will be inverted (meaning 1-a or 1-1 depending on BDF)
    #[inline(always)]
    pub fn bdi(&mut self) -> BDI_W<CONTROL2_SPEC> {
        BDI_W::new(self, 12)
    }
    ///Bit 13 - Blend color 2 instead of framebuffer pixel
    #[inline(always)]
    pub fn bc2(&mut self) -> BC2_W<CONTROL2_SPEC> {
        BC2_W::new(self, 13)
    }
    ///Bit 14 - Calculating U limiter outside use textureThe bit describes what happens if the U limiter (x direction in texture space) calculates a U value outside of the used texture
    #[inline(always)]
    pub fn textureclampx(&mut self) -> TEXTURECLAMPX_W<CONTROL2_SPEC> {
        TEXTURECLAMPX_W::new(self, 14)
    }
    ///Bit 15 - Calculating V limiter outside use textureThe bit describes what happens if the V limiter (y direction in texture space) calculates a V value outside of the used texture
    #[inline(always)]
    pub fn textureclampy(&mut self) -> TEXTURECLAMPY_W<CONTROL2_SPEC> {
        TEXTURECLAMPY_W::new(self, 15)
    }
    ///Bit 16 - Linear filtering on texture U axis
    #[inline(always)]
    pub fn texturefilterx(&mut self) -> TEXTUREFILTERX_W<CONTROL2_SPEC> {
        TEXTUREFILTERX_W::new(self, 16)
    }
    ///Bit 17 - Linear filtering on texture V axis
    #[inline(always)]
    pub fn texturefiltery(&mut self) -> TEXTUREFILTERY_W<CONTROL2_SPEC> {
        TEXTUREFILTERY_W::new(self, 17)
    }
    ///Bits 18:19 - Pixel format of the texture buffer{READFORMAT32,READFORMAT10}0000: 8 bpp a(8)0001: 16 bpp RGB(565)0010: 32 bpp aRGB(8888)0011: 16 bpp aRGB(4444)0100: 16 bpp aRGB(1555)0101: 8 bpp aCLUT(44) 4 bit alpha and 4 bit indexed color1001: 8 bpp CLUT(8)/I(8), 8 bit indexed color/luminance1010: 4 bpp CLUT(4)/I(4), 4 bit indexed color/luminance1011: 2 bpp CLUT(2)/I(2), 2 bit indexed color/luminance 1100: 1 bpp CLUT(1)/I(1), 1 bit indexed color/luminance
    #[inline(always)]
    pub fn readformat10(&mut self) -> READFORMAT10_W<CONTROL2_SPEC> {
        READFORMAT10_W::new(self, 18)
    }
    ///Bits 20:21 - Pixel format of the framebuffer
    #[inline(always)]
    pub fn writeformat10(&mut self) -> WRITEFORMAT10_W<CONTROL2_SPEC> {
        WRITEFORMAT10_W::new(self, 20)
    }
    ///Bits 22:23 - Writeback alpha source for framebufferSet the 'alpha source' for the framebuffer(USEACB = 0)Blend alpha in color 2 instead of framebuffer alpha((USEACB = 1))In not alpha channel blending mode (USEACB = 0):Set the 'alpha source' for the framebuffer.In alpha channel blending mode (USEACB = 1):Blend alpha in color 2 instead of framebuffer alpha00B: BC2A = 1: use alpha from framebuffer as destination (DST_A)else: BC2A = 0: use alpha in color 2 as destination (DST_A)
    #[inline(always)]
    pub fn writealpha(&mut self) -> WRITEALPHA_W<CONTROL2_SPEC> {
        WRITEALPHA_W::new(self, 22)
    }
    ///Bit 24 - RLE enable
    #[inline(always)]
    pub fn rleenable(&mut self) -> RLEENABLE_W<CONTROL2_SPEC> {
        RLEENABLE_W::new(self, 24)
    }
    ///Bit 25 - CLUT enable
    #[inline(always)]
    pub fn clutenable(&mut self) -> CLUTENABLE_W<CONTROL2_SPEC> {
        CLUTENABLE_W::new(self, 25)
    }
    ///Bit 26 - color keying enable
    #[inline(always)]
    pub fn colkeyenable(&mut self) -> COLKEYENABLE_W<CONTROL2_SPEC> {
        COLKEYENABLE_W::new(self, 26)
    }
    ///Bit 27 - Format of the CLUT
    #[inline(always)]
    pub fn clutformat(&mut self) -> CLUTFORMAT_W<CONTROL2_SPEC> {
        CLUTFORMAT_W::new(self, 27)
    }
    ///Bit 28 - Blend source factor inverted in alpha channel (USEACB = 1)
    #[inline(always)]
    pub fn bsia(&mut self) -> BSIA_W<CONTROL2_SPEC> {
        BSIA_W::new(self, 28)
    }
    ///Bit 29 - Blend destination factor inverted in alpha channel (USEACB = 1)
    #[inline(always)]
    pub fn bdia(&mut self) -> BDIA_W<CONTROL2_SPEC> {
        BDIA_W::new(self, 29)
    }
    ///Bits 30:31 - Texel width for RLE unit
    #[inline(always)]
    pub fn rlepixelwidth(&mut self) -> RLEPIXELWIDTH_W<CONTROL2_SPEC> {
        RLEPIXELWIDTH_W::new(self, 30)
    }
}
/**Surface Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONTROL2_SPEC;
impl crate::RegisterSpec for CONTROL2_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`control2::W`](W) writer structure
impl crate::Writable for CONTROL2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONTROL2 to value 0
impl crate::Resettable for CONTROL2_SPEC {}
