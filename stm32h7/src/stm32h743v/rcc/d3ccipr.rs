#[doc = "Register `D3CCIPR` reader"]
pub struct R(crate::R<D3CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3CCIPR` writer"]
pub struct W(crate::W<D3CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3CCIPR_SPEC>;
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
impl From<crate::W<D3CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPUART1SEL` reader - LPUART1 kernel clock source selection"]
pub type LPUART1SEL_R = crate::FieldReader<u8, LPUART1SEL_A>;
#[doc = "LPUART1 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPUART1SEL_A {
    #[doc = "0: rcc_pclk_d3 selected as peripheral clock"]
    RccPclkD3 = 0,
    #[doc = "1: pll2_q selected as peripheral clock"]
    Pll2Q = 1,
    #[doc = "2: pll3_q selected as peripheral clock"]
    Pll3Q = 2,
    #[doc = "3: hsi_ker selected as peripheral clock"]
    HsiKer = 3,
    #[doc = "4: csi_ker selected as peripheral clock"]
    CsiKer = 4,
    #[doc = "5: LSE selected as peripheral clock"]
    Lse = 5,
}
impl From<LPUART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL_A) -> Self {
        variant as _
    }
}
impl LPUART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPUART1SEL_A> {
        match self.bits {
            0 => Some(LPUART1SEL_A::RccPclkD3),
            1 => Some(LPUART1SEL_A::Pll2Q),
            2 => Some(LPUART1SEL_A::Pll3Q),
            3 => Some(LPUART1SEL_A::HsiKer),
            4 => Some(LPUART1SEL_A::CsiKer),
            5 => Some(LPUART1SEL_A::Lse),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RccPclkD3`"]
    #[inline(always)]
    pub fn is_rcc_pclk_d3(&self) -> bool {
        *self == LPUART1SEL_A::RccPclkD3
    }
    #[doc = "Checks if the value of the field is `Pll2Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == LPUART1SEL_A::Pll2Q
    }
    #[doc = "Checks if the value of the field is `Pll3Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == LPUART1SEL_A::Pll3Q
    }
    #[doc = "Checks if the value of the field is `HsiKer`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == LPUART1SEL_A::HsiKer
    }
    #[doc = "Checks if the value of the field is `CsiKer`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == LPUART1SEL_A::CsiKer
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPUART1SEL_A::Lse
    }
}
#[doc = "Field `LPUART1SEL` writer - LPUART1 kernel clock source selection"]
pub type LPUART1SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D3CCIPR_SPEC, u8, LPUART1SEL_A, 3, O>;
impl<'a, const O: u8> LPUART1SEL_W<'a, O> {
    #[doc = "rcc_pclk_d3 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk_d3(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::RccPclkD3)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Pll2Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Pll3Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::HsiKer)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::CsiKer)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Lse)
    }
}
#[doc = "Field `I2C4SEL` reader - I2C4 kernel clock source selection"]
pub type I2C4SEL_R = crate::FieldReader<u8, I2C4SEL_A>;
#[doc = "I2C4 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C4SEL_A {
    #[doc = "0: rcc_pclk4 selected as peripheral clock"]
    RccPclk4 = 0,
    #[doc = "1: pll3_r selected as peripheral clock"]
    Pll3R = 1,
    #[doc = "2: hsi_ker selected as peripheral clock"]
    HsiKer = 2,
    #[doc = "3: csi_ker selected as peripheral clock"]
    CsiKer = 3,
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
    pub fn variant(&self) -> I2C4SEL_A {
        match self.bits {
            0 => I2C4SEL_A::RccPclk4,
            1 => I2C4SEL_A::Pll3R,
            2 => I2C4SEL_A::HsiKer,
            3 => I2C4SEL_A::CsiKer,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RccPclk4`"]
    #[inline(always)]
    pub fn is_rcc_pclk4(&self) -> bool {
        *self == I2C4SEL_A::RccPclk4
    }
    #[doc = "Checks if the value of the field is `Pll3R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == I2C4SEL_A::Pll3R
    }
    #[doc = "Checks if the value of the field is `HsiKer`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I2C4SEL_A::HsiKer
    }
    #[doc = "Checks if the value of the field is `CsiKer`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == I2C4SEL_A::CsiKer
    }
}
#[doc = "Field `I2C4SEL` writer - I2C4 kernel clock source selection"]
pub type I2C4SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, D3CCIPR_SPEC, u8, I2C4SEL_A, 2, O>;
impl<'a, const O: u8> I2C4SEL_W<'a, O> {
    #[doc = "rcc_pclk4 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk4(self) -> &'a mut W {
        self.variant(I2C4SEL_A::RccPclk4)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(I2C4SEL_A::Pll3R)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(I2C4SEL_A::HsiKer)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(I2C4SEL_A::CsiKer)
    }
}
#[doc = "Field `LPTIM2SEL` reader - LPTIM2 kernel clock source selection"]
pub type LPTIM2SEL_R = crate::FieldReader<u8, LPTIM2SEL_A>;
#[doc = "LPTIM2 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM2SEL_A {
    #[doc = "0: rcc_pclk4 selected as peripheral clock"]
    RccPclk4 = 0,
    #[doc = "1: pll2_p selected as peripheral clock"]
    Pll2P = 1,
    #[doc = "2: pll3_r selected as peripheral clock"]
    Pll3R = 2,
    #[doc = "3: LSE selected as peripheral clock"]
    Lse = 3,
    #[doc = "4: LSI selected as peripheral clock"]
    Lsi = 4,
    #[doc = "5: PER selected as peripheral clock"]
    Per = 5,
}
impl From<LPTIM2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM2SEL_A) -> Self {
        variant as _
    }
}
impl LPTIM2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPTIM2SEL_A> {
        match self.bits {
            0 => Some(LPTIM2SEL_A::RccPclk4),
            1 => Some(LPTIM2SEL_A::Pll2P),
            2 => Some(LPTIM2SEL_A::Pll3R),
            3 => Some(LPTIM2SEL_A::Lse),
            4 => Some(LPTIM2SEL_A::Lsi),
            5 => Some(LPTIM2SEL_A::Per),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RccPclk4`"]
    #[inline(always)]
    pub fn is_rcc_pclk4(&self) -> bool {
        *self == LPTIM2SEL_A::RccPclk4
    }
    #[doc = "Checks if the value of the field is `Pll2P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == LPTIM2SEL_A::Pll2P
    }
    #[doc = "Checks if the value of the field is `Pll3R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == LPTIM2SEL_A::Pll3R
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM2SEL_A::Lse
    }
    #[doc = "Checks if the value of the field is `Lsi`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM2SEL_A::Lsi
    }
    #[doc = "Checks if the value of the field is `Per`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == LPTIM2SEL_A::Per
    }
}
#[doc = "Field `LPTIM2SEL` writer - LPTIM2 kernel clock source selection"]
pub type LPTIM2SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D3CCIPR_SPEC, u8, LPTIM2SEL_A, 3, O>;
impl<'a, const O: u8> LPTIM2SEL_W<'a, O> {
    #[doc = "rcc_pclk4 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk4(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::RccPclk4)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::Pll2P)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::Pll3R)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::Lse)
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::Lsi)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::Per)
    }
}
#[doc = "Field `LPTIM345SEL` reader - LPTIM3,4,5 kernel clock source selection"]
pub use LPTIM2SEL_R as LPTIM345SEL_R;
#[doc = "Field `LPTIM345SEL` writer - LPTIM3,4,5 kernel clock source selection"]
pub use LPTIM2SEL_W as LPTIM345SEL_W;
#[doc = "Field `ADCSEL` reader - SAR ADC kernel clock source selection"]
pub type ADCSEL_R = crate::FieldReader<u8, ADCSEL_A>;
#[doc = "SAR ADC kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSEL_A {
    #[doc = "0: pll2_p selected as peripheral clock"]
    Pll2P = 0,
    #[doc = "1: pll3_r selected as peripheral clock"]
    Pll3R = 1,
    #[doc = "2: PER selected as peripheral clock"]
    Per = 2,
}
impl From<ADCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSEL_A) -> Self {
        variant as _
    }
}
impl ADCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCSEL_A> {
        match self.bits {
            0 => Some(ADCSEL_A::Pll2P),
            1 => Some(ADCSEL_A::Pll3R),
            2 => Some(ADCSEL_A::Per),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pll2P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == ADCSEL_A::Pll2P
    }
    #[doc = "Checks if the value of the field is `Pll3R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == ADCSEL_A::Pll3R
    }
    #[doc = "Checks if the value of the field is `Per`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == ADCSEL_A::Per
    }
}
#[doc = "Field `ADCSEL` writer - SAR ADC kernel clock source selection"]
pub type ADCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, D3CCIPR_SPEC, u8, ADCSEL_A, 2, O>;
impl<'a, const O: u8> ADCSEL_W<'a, O> {
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(ADCSEL_A::Pll2P)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(ADCSEL_A::Pll3R)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(ADCSEL_A::Per)
    }
}
#[doc = "Field `SAI4ASEL` reader - Sub-Block A of SAI4 kernel clock source selection"]
pub type SAI4ASEL_R = crate::FieldReader<u8, SAI4ASEL_A>;
#[doc = "Sub-Block A of SAI4 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI4ASEL_A {
    #[doc = "0: pll1_q selected as peripheral clock"]
    Pll1Q = 0,
    #[doc = "1: pll2_p selected as peripheral clock"]
    Pll2P = 1,
    #[doc = "2: pll3_p selected as peripheral clock"]
    Pll3P = 2,
    #[doc = "3: i2s_ckin selected as peripheral clock"]
    I2sCkin = 3,
    #[doc = "4: PER selected as peripheral clock"]
    Per = 4,
}
impl From<SAI4ASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI4ASEL_A) -> Self {
        variant as _
    }
}
impl SAI4ASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAI4ASEL_A> {
        match self.bits {
            0 => Some(SAI4ASEL_A::Pll1Q),
            1 => Some(SAI4ASEL_A::Pll2P),
            2 => Some(SAI4ASEL_A::Pll3P),
            3 => Some(SAI4ASEL_A::I2sCkin),
            4 => Some(SAI4ASEL_A::Per),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pll1Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SAI4ASEL_A::Pll1Q
    }
    #[doc = "Checks if the value of the field is `Pll2P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SAI4ASEL_A::Pll2P
    }
    #[doc = "Checks if the value of the field is `Pll3P`"]
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SAI4ASEL_A::Pll3P
    }
    #[doc = "Checks if the value of the field is `I2sCkin`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI4ASEL_A::I2sCkin
    }
    #[doc = "Checks if the value of the field is `Per`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == SAI4ASEL_A::Per
    }
}
#[doc = "Field `SAI4ASEL` writer - Sub-Block A of SAI4 kernel clock source selection"]
pub type SAI4ASEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D3CCIPR_SPEC, u8, SAI4ASEL_A, 3, O>;
impl<'a, const O: u8> SAI4ASEL_W<'a, O> {
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SAI4ASEL_A::Pll1Q)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(SAI4ASEL_A::Pll2P)
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut W {
        self.variant(SAI4ASEL_A::Pll3P)
    }
    #[doc = "i2s_ckin selected as peripheral clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI4ASEL_A::I2sCkin)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(SAI4ASEL_A::Per)
    }
}
#[doc = "Field `SAI4BSEL` reader - Sub-Block B of SAI4 kernel clock source selection"]
pub use SAI4ASEL_R as SAI4BSEL_R;
#[doc = "Field `SAI4BSEL` writer - Sub-Block B of SAI4 kernel clock source selection"]
pub use SAI4ASEL_W as SAI4BSEL_W;
#[doc = "Field `SPI6SEL` reader - SPI6 kernel clock source selection"]
pub type SPI6SEL_R = crate::FieldReader<u8, SPI6SEL_A>;
#[doc = "SPI6 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI6SEL_A {
    #[doc = "0: rcc_pclk4 selected as peripheral clock"]
    RccPclk4 = 0,
    #[doc = "1: pll2_q selected as peripheral clock"]
    Pll2Q = 1,
    #[doc = "2: pll3_q selected as peripheral clock"]
    Pll3Q = 2,
    #[doc = "3: hsi_ker selected as peripheral clock"]
    HsiKer = 3,
    #[doc = "4: csi_ker selected as peripheral clock"]
    CsiKer = 4,
    #[doc = "5: HSE selected as peripheral clock"]
    Hse = 5,
}
impl From<SPI6SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI6SEL_A) -> Self {
        variant as _
    }
}
impl SPI6SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI6SEL_A> {
        match self.bits {
            0 => Some(SPI6SEL_A::RccPclk4),
            1 => Some(SPI6SEL_A::Pll2Q),
            2 => Some(SPI6SEL_A::Pll3Q),
            3 => Some(SPI6SEL_A::HsiKer),
            4 => Some(SPI6SEL_A::CsiKer),
            5 => Some(SPI6SEL_A::Hse),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RccPclk4`"]
    #[inline(always)]
    pub fn is_rcc_pclk4(&self) -> bool {
        *self == SPI6SEL_A::RccPclk4
    }
    #[doc = "Checks if the value of the field is `Pll2Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == SPI6SEL_A::Pll2Q
    }
    #[doc = "Checks if the value of the field is `Pll3Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == SPI6SEL_A::Pll3Q
    }
    #[doc = "Checks if the value of the field is `HsiKer`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPI6SEL_A::HsiKer
    }
    #[doc = "Checks if the value of the field is `CsiKer`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == SPI6SEL_A::CsiKer
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SPI6SEL_A::Hse
    }
}
#[doc = "Field `SPI6SEL` writer - SPI6 kernel clock source selection"]
pub type SPI6SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D3CCIPR_SPEC, u8, SPI6SEL_A, 3, O>;
impl<'a, const O: u8> SPI6SEL_W<'a, O> {
    #[doc = "rcc_pclk4 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk4(self) -> &'a mut W {
        self.variant(SPI6SEL_A::RccPclk4)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(SPI6SEL_A::Pll2Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(SPI6SEL_A::Pll3Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SPI6SEL_A::HsiKer)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(SPI6SEL_A::CsiKer)
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SPI6SEL_A::Hse)
    }
}
impl R {
    #[doc = "Bits 0:2 - LPUART1 kernel clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - I2C4 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - LPTIM2 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - LPTIM3,4,5 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim345sel(&self) -> LPTIM345SEL_R {
        LPTIM345SEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - SAR ADC kernel clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 21:23 - Sub-Block A of SAI4 kernel clock source selection"]
    #[inline(always)]
    pub fn sai4asel(&self) -> SAI4ASEL_R {
        SAI4ASEL_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Sub-Block B of SAI4 kernel clock source selection"]
    #[inline(always)]
    pub fn sai4bsel(&self) -> SAI4BSEL_R {
        SAI4BSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - SPI6 kernel clock source selection"]
    #[inline(always)]
    pub fn spi6sel(&self) -> SPI6SEL_R {
        SPI6SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LPUART1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<0> {
        LPUART1SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - I2C4 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<8> {
        I2C4SEL_W::new(self)
    }
    #[doc = "Bits 10:12 - LPTIM2 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<10> {
        LPTIM2SEL_W::new(self)
    }
    #[doc = "Bits 13:15 - LPTIM3,4,5 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim345sel(&mut self) -> LPTIM345SEL_W<13> {
        LPTIM345SEL_W::new(self)
    }
    #[doc = "Bits 16:17 - SAR ADC kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<16> {
        ADCSEL_W::new(self)
    }
    #[doc = "Bits 21:23 - Sub-Block A of SAI4 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai4asel(&mut self) -> SAI4ASEL_W<21> {
        SAI4ASEL_W::new(self)
    }
    #[doc = "Bits 24:26 - Sub-Block B of SAI4 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai4bsel(&mut self) -> SAI4BSEL_W<24> {
        SAI4BSEL_W::new(self)
    }
    #[doc = "Bits 28:30 - SPI6 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi6sel(&mut self) -> SPI6SEL_W<28> {
        SPI6SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Domain 3 Kernel Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3ccipr](index.html) module"]
pub struct D3CCIPR_SPEC;
impl crate::RegisterSpec for D3CCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3ccipr::R](R) reader structure"]
impl crate::Readable for D3CCIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3ccipr::W](W) writer structure"]
impl crate::Writable for D3CCIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3CCIPR to value 0"]
impl crate::Resettable for D3CCIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
