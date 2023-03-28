#[doc = "Register `CCIPR2` reader"]
pub struct R(crate::R<CCIPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR2` writer"]
pub struct W(crate::W<CCIPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CCIPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C4SEL` reader - I2C4 clock source selection"]
pub type I2C4SEL_R = crate::FieldReader<u8, I2C4SEL_A>;
#[doc = "I2C4 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C4SEL_A {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
}
impl From<I2C4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C4SEL_A) -> Self {
        variant as _
    }
}
impl I2C4SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C4SEL_A> {
        match self.bits {
            0 => Some(I2C4SEL_A::Pclk),
            1 => Some(I2C4SEL_A::Sysclk),
            2 => Some(I2C4SEL_A::Hsi16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pclk`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == I2C4SEL_A::Pclk
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C4SEL_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C4SEL_A::Hsi16
    }
}
#[doc = "Field `I2C4SEL` writer - I2C4 clock source selection"]
pub type I2C4SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, I2C4SEL_A, 2, O>;
impl<'a, const O: u8> I2C4SEL_W<'a, O> {
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(I2C4SEL_A::Pclk)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C4SEL_A::Sysclk)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C4SEL_A::Hsi16)
    }
}
#[doc = "Field `DFSDMSEL` reader - Digital filter for sigma delta modulator kernel clock source selection"]
pub type DFSDMSEL_R = crate::BitReader<DFSDMSEL_A>;
#[doc = "Digital filter for sigma delta modulator kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDMSEL_A {
    #[doc = "0: APB2 clock (PCLK2) selected as DFSDM kernel clock"]
    Pclk2 = 0,
    #[doc = "1: System clock selected as DFSDM kernel clock"]
    Sysclk = 1,
}
impl From<DFSDMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDMSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DFSDMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDMSEL_A {
        match self.bits {
            false => DFSDMSEL_A::Pclk2,
            true => DFSDMSEL_A::Sysclk,
        }
    }
    #[doc = "Checks if the value of the field is `Pclk2`"]
    #[inline(always)]
    pub fn is_pclk2(&self) -> bool {
        *self == DFSDMSEL_A::Pclk2
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == DFSDMSEL_A::Sysclk
    }
}
#[doc = "Field `DFSDMSEL` writer - Digital filter for sigma delta modulator kernel clock source selection"]
pub type DFSDMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR2_SPEC, DFSDMSEL_A, O>;
impl<'a, const O: u8> DFSDMSEL_W<'a, O> {
    #[doc = "APB2 clock (PCLK2) selected as DFSDM kernel clock"]
    #[inline(always)]
    pub fn pclk2(self) -> &'a mut W {
        self.variant(DFSDMSEL_A::Pclk2)
    }
    #[doc = "System clock selected as DFSDM kernel clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(DFSDMSEL_A::Sysclk)
    }
}
#[doc = "Field `ADFSDMSEL` reader - Digital filter for sigma delta modulator audio clock source selection"]
pub type ADFSDMSEL_R = crate::FieldReader<u8, ADFSDMSEL_A>;
#[doc = "Digital filter for sigma delta modulator audio clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADFSDMSEL_A {
    #[doc = "0: SAI1clock selected as DFSDM audio clock"]
    Sai1 = 0,
    #[doc = "1: HSI clock selected as DFSDM audio clock"]
    Hsi = 1,
    #[doc = "2: MSI clock selected as DFSDM audio clock"]
    Msi = 2,
}
impl From<ADFSDMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADFSDMSEL_A) -> Self {
        variant as _
    }
}
impl ADFSDMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADFSDMSEL_A> {
        match self.bits {
            0 => Some(ADFSDMSEL_A::Sai1),
            1 => Some(ADFSDMSEL_A::Hsi),
            2 => Some(ADFSDMSEL_A::Msi),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Sai1`"]
    #[inline(always)]
    pub fn is_sai1(&self) -> bool {
        *self == ADFSDMSEL_A::Sai1
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == ADFSDMSEL_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Msi`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == ADFSDMSEL_A::Msi
    }
}
#[doc = "Field `ADFSDMSEL` writer - Digital filter for sigma delta modulator audio clock source selection"]
pub type ADFSDMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, ADFSDMSEL_A, 2, O>;
impl<'a, const O: u8> ADFSDMSEL_W<'a, O> {
    #[doc = "SAI1clock selected as DFSDM audio clock"]
    #[inline(always)]
    pub fn sai1(self) -> &'a mut W {
        self.variant(ADFSDMSEL_A::Sai1)
    }
    #[doc = "HSI clock selected as DFSDM audio clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(ADFSDMSEL_A::Hsi)
    }
    #[doc = "MSI clock selected as DFSDM audio clock"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(ADFSDMSEL_A::Msi)
    }
}
#[doc = "Field `SAI1SEL` reader - SAI1 clock source selection"]
pub type SAI1SEL_R = crate::FieldReader<u8, SAI1SEL_A>;
#[doc = "SAI1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL_A {
    #[doc = "0: PLLSAI1CLK clock is selected as SAIx clock"]
    Pllsai1clk = 0,
    #[doc = "1: PLLSAI2CLK clock is selected as SAIx clock"]
    Pllsai2clk = 1,
    #[doc = "2: PLLSAI3CLK clock is selected as SAIx clock"]
    Pllsai3clk = 2,
    #[doc = "3: External clock SAIx_EXTCLK clock selected as SAIx clock"]
    Sai2Extclk = 3,
    #[doc = "4: HSI clock selected as SAIx clock"]
    Hsi = 4,
}
impl From<SAI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL_A) -> Self {
        variant as _
    }
}
impl SAI1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAI1SEL_A> {
        match self.bits {
            0 => Some(SAI1SEL_A::Pllsai1clk),
            1 => Some(SAI1SEL_A::Pllsai2clk),
            2 => Some(SAI1SEL_A::Pllsai3clk),
            3 => Some(SAI1SEL_A::Sai2Extclk),
            4 => Some(SAI1SEL_A::Hsi),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pllsai1clk`"]
    #[inline(always)]
    pub fn is_pllsai1clk(&self) -> bool {
        *self == SAI1SEL_A::Pllsai1clk
    }
    #[doc = "Checks if the value of the field is `Pllsai2clk`"]
    #[inline(always)]
    pub fn is_pllsai2clk(&self) -> bool {
        *self == SAI1SEL_A::Pllsai2clk
    }
    #[doc = "Checks if the value of the field is `Pllsai3clk`"]
    #[inline(always)]
    pub fn is_pllsai3clk(&self) -> bool {
        *self == SAI1SEL_A::Pllsai3clk
    }
    #[doc = "Checks if the value of the field is `Sai2Extclk`"]
    #[inline(always)]
    pub fn is_sai2_extclk(&self) -> bool {
        *self == SAI1SEL_A::Sai2Extclk
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SAI1SEL_A::Hsi
    }
}
#[doc = "Field `SAI1SEL` writer - SAI1 clock source selection"]
pub type SAI1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, SAI1SEL_A, 3, O>;
impl<'a, const O: u8> SAI1SEL_W<'a, O> {
    #[doc = "PLLSAI1CLK clock is selected as SAIx clock"]
    #[inline(always)]
    pub fn pllsai1clk(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Pllsai1clk)
    }
    #[doc = "PLLSAI2CLK clock is selected as SAIx clock"]
    #[inline(always)]
    pub fn pllsai2clk(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Pllsai2clk)
    }
    #[doc = "PLLSAI3CLK clock is selected as SAIx clock"]
    #[inline(always)]
    pub fn pllsai3clk(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Pllsai3clk)
    }
    #[doc = "External clock SAIx_EXTCLK clock selected as SAIx clock"]
    #[inline(always)]
    pub fn sai2_extclk(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Sai2Extclk)
    }
    #[doc = "HSI clock selected as SAIx clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Hsi)
    }
}
#[doc = "Field `SAI2SEL` reader - SAI2 clock source selection"]
pub use SAI1SEL_R as SAI2SEL_R;
#[doc = "Field `SAI2SEL` writer - SAI2 clock source selection"]
pub use SAI1SEL_W as SAI2SEL_W;
#[doc = "Field `DSISEL` reader - clock selection"]
pub type DSISEL_R = crate::BitReader<DSISEL_A>;
#[doc = "clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSISEL_A {
    #[doc = "0: DSI-PHY is selected as DSI byte lane clock source (usual case)"]
    Dsiphy = 0,
    #[doc = "1: PLLDSICLK is selected as DSI byte lane clock source, used in case DSI PLL and DSIPHY are off (low-power mode)"]
    Plldsiclk = 1,
}
impl From<DSISEL_A> for bool {
    #[inline(always)]
    fn from(variant: DSISEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DSISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSISEL_A {
        match self.bits {
            false => DSISEL_A::Dsiphy,
            true => DSISEL_A::Plldsiclk,
        }
    }
    #[doc = "Checks if the value of the field is `Dsiphy`"]
    #[inline(always)]
    pub fn is_dsiphy(&self) -> bool {
        *self == DSISEL_A::Dsiphy
    }
    #[doc = "Checks if the value of the field is `Plldsiclk`"]
    #[inline(always)]
    pub fn is_plldsiclk(&self) -> bool {
        *self == DSISEL_A::Plldsiclk
    }
}
#[doc = "Field `DSISEL` writer - clock selection"]
pub type DSISEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR2_SPEC, DSISEL_A, O>;
impl<'a, const O: u8> DSISEL_W<'a, O> {
    #[doc = "DSI-PHY is selected as DSI byte lane clock source (usual case)"]
    #[inline(always)]
    pub fn dsiphy(self) -> &'a mut W {
        self.variant(DSISEL_A::Dsiphy)
    }
    #[doc = "PLLDSICLK is selected as DSI byte lane clock source, used in case DSI PLL and DSIPHY are off (low-power mode)"]
    #[inline(always)]
    pub fn plldsiclk(self) -> &'a mut W {
        self.variant(DSISEL_A::Plldsiclk)
    }
}
#[doc = "Field `SDMMCSEL` reader - SDMMC clock selection"]
pub type SDMMCSEL_R = crate::BitReader<SDMMCSEL_A>;
#[doc = "SDMMC clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMCSEL_A {
    #[doc = "0: 48 MHz clock is selected as SDMMC kernel clock"]
    Hsi48 = 0,
    #[doc = "1: PLLSAI3CLK is selected as SDMMC kernel clock, used in case higher frequency than 48MHz is needed (for SDR50 mode)"]
    Pllsai3clk = 1,
}
impl From<SDMMCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMCSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMMCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMCSEL_A {
        match self.bits {
            false => SDMMCSEL_A::Hsi48,
            true => SDMMCSEL_A::Pllsai3clk,
        }
    }
    #[doc = "Checks if the value of the field is `Hsi48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == SDMMCSEL_A::Hsi48
    }
    #[doc = "Checks if the value of the field is `Pllsai3clk`"]
    #[inline(always)]
    pub fn is_pllsai3clk(&self) -> bool {
        *self == SDMMCSEL_A::Pllsai3clk
    }
}
#[doc = "Field `SDMMCSEL` writer - SDMMC clock selection"]
pub type SDMMCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR2_SPEC, SDMMCSEL_A, O>;
impl<'a, const O: u8> SDMMCSEL_W<'a, O> {
    #[doc = "48 MHz clock is selected as SDMMC kernel clock"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(SDMMCSEL_A::Hsi48)
    }
    #[doc = "PLLSAI3CLK is selected as SDMMC kernel clock, used in case higher frequency than 48MHz is needed (for SDR50 mode)"]
    #[inline(always)]
    pub fn pllsai3clk(self) -> &'a mut W {
        self.variant(SDMMCSEL_A::Pllsai3clk)
    }
}
#[doc = "Field `PLLSAI2DIVR` reader - division factor for LTDC clock"]
pub type PLLSAI2DIVR_R = crate::FieldReader<u8, PLLSAI2DIVR_A>;
#[doc = "division factor for LTDC clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAI2DIVR_A {
    #[doc = "0: PLLSAI2DIVR = /2"]
    Div2 = 0,
    #[doc = "1: PLLSAI2DIVR = /4"]
    Div4 = 1,
    #[doc = "2: PLLSAI2DIVR = /8"]
    Div8 = 2,
    #[doc = "3: PLLSAI2DIVR = /16"]
    Div16 = 3,
}
impl From<PLLSAI2DIVR_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI2DIVR_A) -> Self {
        variant as _
    }
}
impl PLLSAI2DIVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2DIVR_A {
        match self.bits {
            0 => PLLSAI2DIVR_A::Div2,
            1 => PLLSAI2DIVR_A::Div4,
            2 => PLLSAI2DIVR_A::Div8,
            3 => PLLSAI2DIVR_A::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI2DIVR_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI2DIVR_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI2DIVR_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAI2DIVR_A::Div16
    }
}
#[doc = "Field `PLLSAI2DIVR` writer - division factor for LTDC clock"]
pub type PLLSAI2DIVR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR2_SPEC, u8, PLLSAI2DIVR_A, 2, O>;
impl<'a, const O: u8> PLLSAI2DIVR_W<'a, O> {
    #[doc = "PLLSAI2DIVR = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAI2DIVR_A::Div2)
    }
    #[doc = "PLLSAI2DIVR = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAI2DIVR_A::Div4)
    }
    #[doc = "PLLSAI2DIVR = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAI2DIVR_A::Div8)
    }
    #[doc = "PLLSAI2DIVR = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAI2DIVR_A::Div16)
    }
}
#[doc = "Field `OSPISEL` reader - Octospi clock source selection"]
pub type OSPISEL_R = crate::FieldReader<u8, OSPISEL_A>;
#[doc = "Octospi clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPISEL_A {
    #[doc = "0: System clock selected as OctoSPI kernel clock"]
    Sysclk = 0,
    #[doc = "1: MSI clock selected as OctoSPI kernel clock"]
    Msi = 1,
    #[doc = "2: PLL48M1CLK clock selected as OctoSPI kernel clock"]
    Pll48m1clk = 2,
}
impl From<OSPISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPISEL_A) -> Self {
        variant as _
    }
}
impl OSPISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSPISEL_A> {
        match self.bits {
            0 => Some(OSPISEL_A::Sysclk),
            1 => Some(OSPISEL_A::Msi),
            2 => Some(OSPISEL_A::Pll48m1clk),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == OSPISEL_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Msi`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == OSPISEL_A::Msi
    }
    #[doc = "Checks if the value of the field is `Pll48m1clk`"]
    #[inline(always)]
    pub fn is_pll48m1clk(&self) -> bool {
        *self == OSPISEL_A::Pll48m1clk
    }
}
#[doc = "Field `OSPISEL` writer - Octospi clock source selection"]
pub type OSPISEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR2_SPEC, u8, OSPISEL_A, 2, O>;
impl<'a, const O: u8> OSPISEL_W<'a, O> {
    #[doc = "System clock selected as OctoSPI kernel clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(OSPISEL_A::Sysclk)
    }
    #[doc = "MSI clock selected as OctoSPI kernel clock"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(OSPISEL_A::Msi)
    }
    #[doc = "PLL48M1CLK clock selected as OctoSPI kernel clock"]
    #[inline(always)]
    pub fn pll48m1clk(self) -> &'a mut W {
        self.variant(OSPISEL_A::Pll48m1clk)
    }
}
impl R {
    #[doc = "Bits 0:1 - I2C4 clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Digital filter for sigma delta modulator kernel clock source selection"]
    #[inline(always)]
    pub fn dfsdmsel(&self) -> DFSDMSEL_R {
        DFSDMSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Digital filter for sigma delta modulator audio clock source selection"]
    #[inline(always)]
    pub fn adfsdmsel(&self) -> ADFSDMSEL_R {
        ADFSDMSEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - clock selection"]
    #[inline(always)]
    pub fn dsisel(&self) -> DSISEL_R {
        DSISEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - SDMMC clock selection"]
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - division factor for LTDC clock"]
    #[inline(always)]
    pub fn pllsai2divr(&self) -> PLLSAI2DIVR_R {
        PLLSAI2DIVR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Octospi clock source selection"]
    #[inline(always)]
    pub fn ospisel(&self) -> OSPISEL_R {
        OSPISEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2C4 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<0> {
        I2C4SEL_W::new(self)
    }
    #[doc = "Bit 2 - Digital filter for sigma delta modulator kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmsel(&mut self) -> DFSDMSEL_W<2> {
        DFSDMSEL_W::new(self)
    }
    #[doc = "Bits 3:4 - Digital filter for sigma delta modulator audio clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adfsdmsel(&mut self) -> ADFSDMSEL_W<3> {
        ADFSDMSEL_W::new(self)
    }
    #[doc = "Bits 5:7 - SAI1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<5> {
        SAI1SEL_W::new(self)
    }
    #[doc = "Bits 8:10 - SAI2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<8> {
        SAI2SEL_W::new(self)
    }
    #[doc = "Bit 12 - clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn dsisel(&mut self) -> DSISEL_W<12> {
        DSISEL_W::new(self)
    }
    #[doc = "Bit 14 - SDMMC clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W<14> {
        SDMMCSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - division factor for LTDC clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2divr(&mut self) -> PLLSAI2DIVR_W<16> {
        PLLSAI2DIVR_W::new(self)
    }
    #[doc = "Bits 20:21 - Octospi clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ospisel(&mut self) -> OSPISEL_W<20> {
        OSPISEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripherals independent clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr2](index.html) module"]
pub struct CCIPR2_SPEC;
impl crate::RegisterSpec for CCIPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr2::R](R) reader structure"]
impl crate::Readable for CCIPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr2::W](W) writer structure"]
impl crate::Writable for CCIPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCIPR2 to value 0"]
impl crate::Resettable for CCIPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
