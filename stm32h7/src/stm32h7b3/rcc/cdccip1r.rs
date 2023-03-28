#[doc = "Register `CDCCIP1R` reader"]
pub struct R(crate::R<CDCCIP1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDCCIP1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDCCIP1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDCCIP1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDCCIP1R` writer"]
pub struct W(crate::W<CDCCIP1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDCCIP1R_SPEC>;
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
impl From<crate::W<CDCCIP1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDCCIP1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAI1SEL` reader - SAI1 and DFSDM1 kernel Aclk clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it isnot be possible to switch to another clock. Refer to for additional information. Note: DFSDM1 clock source selection is done by DFSDM1SEL. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
pub type SAI1SEL_R = crate::FieldReader<u8, SAI1SEL_A>;
#[doc = "SAI1 and DFSDM1 kernel Aclk clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it isnot be possible to switch to another clock. Refer to for additional information. Note: DFSDM1 clock source selection is done by DFSDM1SEL. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL_A {
    #[doc = "0: pll1_q selected as peripheral clock"]
    Pll1Q = 0,
    #[doc = "1: pll2_p selected as peripheral clock"]
    Pll2P = 1,
    #[doc = "2: pll3_p selected as peripheral clock"]
    Pll3P = 2,
    #[doc = "3: I2S_CKIN selected as peripheral clock"]
    I2sCkin = 3,
    #[doc = "4: PER selected as peripheral clock"]
    Per = 4,
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
            0 => Some(SAI1SEL_A::Pll1Q),
            1 => Some(SAI1SEL_A::Pll2P),
            2 => Some(SAI1SEL_A::Pll3P),
            3 => Some(SAI1SEL_A::I2sCkin),
            4 => Some(SAI1SEL_A::Per),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pll1Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SAI1SEL_A::Pll1Q
    }
    #[doc = "Checks if the value of the field is `Pll2P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SAI1SEL_A::Pll2P
    }
    #[doc = "Checks if the value of the field is `Pll3P`"]
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SAI1SEL_A::Pll3P
    }
    #[doc = "Checks if the value of the field is `I2sCkin`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1SEL_A::I2sCkin
    }
    #[doc = "Checks if the value of the field is `Per`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == SAI1SEL_A::Per
    }
}
#[doc = "Field `SAI1SEL` writer - SAI1 and DFSDM1 kernel Aclk clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it isnot be possible to switch to another clock. Refer to for additional information. Note: DFSDM1 clock source selection is done by DFSDM1SEL. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
pub type SAI1SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CDCCIP1R_SPEC, u8, SAI1SEL_A, 3, O>;
impl<'a, const O: u8> SAI1SEL_W<'a, O> {
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Pll1Q)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Pll2P)
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Pll3P)
    }
    #[doc = "I2S_CKIN selected as peripheral clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1SEL_A::I2sCkin)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Per)
    }
}
#[doc = "Field `SAI2ASEL` reader - SAI2 kernel clock source A selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the SPDIFRX (see )."]
pub type SAI2ASEL_R = crate::FieldReader<u8, SAI2ASEL_A>;
#[doc = "SAI2 kernel clock source A selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the SPDIFRX (see ).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI2ASEL_A {
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
impl From<SAI2ASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI2ASEL_A) -> Self {
        variant as _
    }
}
impl SAI2ASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAI2ASEL_A> {
        match self.bits {
            0 => Some(SAI2ASEL_A::Pll1Q),
            1 => Some(SAI2ASEL_A::Pll2P),
            2 => Some(SAI2ASEL_A::Pll3P),
            3 => Some(SAI2ASEL_A::I2sCkin),
            4 => Some(SAI2ASEL_A::Per),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pll1Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SAI2ASEL_A::Pll1Q
    }
    #[doc = "Checks if the value of the field is `Pll2P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SAI2ASEL_A::Pll2P
    }
    #[doc = "Checks if the value of the field is `Pll3P`"]
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SAI2ASEL_A::Pll3P
    }
    #[doc = "Checks if the value of the field is `I2sCkin`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI2ASEL_A::I2sCkin
    }
    #[doc = "Checks if the value of the field is `Per`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == SAI2ASEL_A::Per
    }
}
#[doc = "Field `SAI2ASEL` writer - SAI2 kernel clock source A selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the SPDIFRX (see )."]
pub type SAI2ASEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CDCCIP1R_SPEC, u8, SAI2ASEL_A, 3, O>;
impl<'a, const O: u8> SAI2ASEL_W<'a, O> {
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SAI2ASEL_A::Pll1Q)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(SAI2ASEL_A::Pll2P)
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut W {
        self.variant(SAI2ASEL_A::Pll3P)
    }
    #[doc = "i2s_ckin selected as peripheral clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI2ASEL_A::I2sCkin)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(SAI2ASEL_A::Per)
    }
}
#[doc = "Field `SPI123SEL` reader - SPI/I2S1,2 and 3 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
pub use SAI1SEL_R as SPI123SEL_R;
#[doc = "Field `SPI123SEL` writer - SPI/I2S1,2 and 3 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
pub use SAI1SEL_W as SPI123SEL_W;
#[doc = "Field `SAI2BSEL` reader - SAI2 kernel clock B source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the spdifrx (see )."]
pub use SAI2ASEL_R as SAI2BSEL_R;
#[doc = "Field `SAI2BSEL` writer - SAI2 kernel clock B source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the spdifrx (see )."]
pub use SAI2ASEL_W as SAI2BSEL_W;
#[doc = "Field `SPI45SEL` reader - SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI45SEL_R = crate::FieldReader<u8, SPI45SEL_A>;
#[doc = "SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI45SEL_A {
    #[doc = "0: APB clock selected as peripheral clock"]
    Apb = 0,
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
impl From<SPI45SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI45SEL_A) -> Self {
        variant as _
    }
}
impl SPI45SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI45SEL_A> {
        match self.bits {
            0 => Some(SPI45SEL_A::Apb),
            1 => Some(SPI45SEL_A::Pll2Q),
            2 => Some(SPI45SEL_A::Pll3Q),
            3 => Some(SPI45SEL_A::HsiKer),
            4 => Some(SPI45SEL_A::CsiKer),
            5 => Some(SPI45SEL_A::Hse),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Apb`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == SPI45SEL_A::Apb
    }
    #[doc = "Checks if the value of the field is `Pll2Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == SPI45SEL_A::Pll2Q
    }
    #[doc = "Checks if the value of the field is `Pll3Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == SPI45SEL_A::Pll3Q
    }
    #[doc = "Checks if the value of the field is `HsiKer`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPI45SEL_A::HsiKer
    }
    #[doc = "Checks if the value of the field is `CsiKer`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == SPI45SEL_A::CsiKer
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SPI45SEL_A::Hse
    }
}
#[doc = "Field `SPI45SEL` writer - SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI45SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CDCCIP1R_SPEC, u8, SPI45SEL_A, 3, O>;
impl<'a, const O: u8> SPI45SEL_W<'a, O> {
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(SPI45SEL_A::Apb)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(SPI45SEL_A::Pll2Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(SPI45SEL_A::Pll3Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SPI45SEL_A::HsiKer)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(SPI45SEL_A::CsiKer)
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SPI45SEL_A::Hse)
    }
}
#[doc = "Field `SPDIFRXSEL` reader - SPDIFRX kernel clock source selection"]
pub type SPDIFRXSEL_R = crate::FieldReader<u8, SPDIFRXSEL_A>;
#[doc = "SPDIFRX kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPDIFRXSEL_A {
    #[doc = "0: pll1_q selected as peripheral clock"]
    Pll1Q = 0,
    #[doc = "1: pll2_r selected as peripheral clock"]
    Pll2R = 1,
    #[doc = "2: pll3_r selected as peripheral clock"]
    Pll3R = 2,
    #[doc = "3: hsi_ker selected as peripheral clock"]
    HsiKer = 3,
}
impl From<SPDIFRXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDIFRXSEL_A) -> Self {
        variant as _
    }
}
impl SPDIFRXSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDIFRXSEL_A {
        match self.bits {
            0 => SPDIFRXSEL_A::Pll1Q,
            1 => SPDIFRXSEL_A::Pll2R,
            2 => SPDIFRXSEL_A::Pll3R,
            3 => SPDIFRXSEL_A::HsiKer,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pll1Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SPDIFRXSEL_A::Pll1Q
    }
    #[doc = "Checks if the value of the field is `Pll2R`"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == SPDIFRXSEL_A::Pll2R
    }
    #[doc = "Checks if the value of the field is `Pll3R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == SPDIFRXSEL_A::Pll3R
    }
    #[doc = "Checks if the value of the field is `HsiKer`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPDIFRXSEL_A::HsiKer
    }
}
#[doc = "Field `SPDIFRXSEL` writer - SPDIFRX kernel clock source selection"]
pub type SPDIFRXSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CDCCIP1R_SPEC, u8, SPDIFRXSEL_A, 2, O>;
impl<'a, const O: u8> SPDIFRXSEL_W<'a, O> {
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SPDIFRXSEL_A::Pll1Q)
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut W {
        self.variant(SPDIFRXSEL_A::Pll2R)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(SPDIFRXSEL_A::Pll3R)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SPDIFRXSEL_A::HsiKer)
    }
}
#[doc = "Field `DFSDM1SEL` reader - DFSDM1 kernel clock Clk source selection Set and reset by software. Note: the DFSDM1 Aclk clock source selection is done by SAI1SEL (see )."]
pub type DFSDM1SEL_R = crate::BitReader<DFSDM1SEL_A>;
#[doc = "DFSDM1 kernel clock Clk source selection Set and reset by software. Note: the DFSDM1 Aclk clock source selection is done by SAI1SEL (see ).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1SEL_A {
    #[doc = "0: rcc_pclk2 selected as peripheral clock"]
    RccPclk2 = 0,
    #[doc = "1: System clock selected as peripheral clock"]
    Sys = 1,
}
impl From<DFSDM1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DFSDM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDM1SEL_A {
        match self.bits {
            false => DFSDM1SEL_A::RccPclk2,
            true => DFSDM1SEL_A::Sys,
        }
    }
    #[doc = "Checks if the value of the field is `RccPclk2`"]
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        *self == DFSDM1SEL_A::RccPclk2
    }
    #[doc = "Checks if the value of the field is `Sys`"]
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        *self == DFSDM1SEL_A::Sys
    }
}
#[doc = "Field `DFSDM1SEL` writer - DFSDM1 kernel clock Clk source selection Set and reset by software. Note: the DFSDM1 Aclk clock source selection is done by SAI1SEL (see )."]
pub type DFSDM1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDCCIP1R_SPEC, DFSDM1SEL_A, O>;
impl<'a, const O: u8> DFSDM1SEL_W<'a, O> {
    #[doc = "rcc_pclk2 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk2(self) -> &'a mut W {
        self.variant(DFSDM1SEL_A::RccPclk2)
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn sys(self) -> &'a mut W {
        self.variant(DFSDM1SEL_A::Sys)
    }
}
#[doc = "Field `FDCANSEL` reader - FDCAN kernel clock source selection Set and reset by software."]
pub type FDCANSEL_R = crate::FieldReader<u8, FDCANSEL_A>;
#[doc = "FDCAN kernel clock source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDCANSEL_A {
    #[doc = "0: HSE selected as peripheral clock"]
    Hse = 0,
    #[doc = "1: pll1_q selected as peripheral clock"]
    Pll1Q = 1,
    #[doc = "2: pll2_q selected as peripheral clock"]
    Pll2Q = 2,
}
impl From<FDCANSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FDCANSEL_A) -> Self {
        variant as _
    }
}
impl FDCANSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FDCANSEL_A> {
        match self.bits {
            0 => Some(FDCANSEL_A::Hse),
            1 => Some(FDCANSEL_A::Pll1Q),
            2 => Some(FDCANSEL_A::Pll2Q),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == FDCANSEL_A::Hse
    }
    #[doc = "Checks if the value of the field is `Pll1Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FDCANSEL_A::Pll1Q
    }
    #[doc = "Checks if the value of the field is `Pll2Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == FDCANSEL_A::Pll2Q
    }
}
#[doc = "Field `FDCANSEL` writer - FDCAN kernel clock source selection Set and reset by software."]
pub type FDCANSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CDCCIP1R_SPEC, u8, FDCANSEL_A, 2, O>;
impl<'a, const O: u8> FDCANSEL_W<'a, O> {
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(FDCANSEL_A::Hse)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(FDCANSEL_A::Pll1Q)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(FDCANSEL_A::Pll2Q)
    }
}
#[doc = "Field `SWPMISEL` reader - SWPMI kernel clock source selection Set and reset by software."]
pub type SWPMISEL_R = crate::BitReader<SWPMISEL_A>;
#[doc = "SWPMI kernel clock source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWPMISEL_A {
    #[doc = "0: pclk selected as peripheral clock"]
    Pclk = 0,
    #[doc = "1: hsi_ker selected as peripheral clock"]
    HsiKer = 1,
}
impl From<SWPMISEL_A> for bool {
    #[inline(always)]
    fn from(variant: SWPMISEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SWPMISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWPMISEL_A {
        match self.bits {
            false => SWPMISEL_A::Pclk,
            true => SWPMISEL_A::HsiKer,
        }
    }
    #[doc = "Checks if the value of the field is `Pclk`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == SWPMISEL_A::Pclk
    }
    #[doc = "Checks if the value of the field is `HsiKer`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SWPMISEL_A::HsiKer
    }
}
#[doc = "Field `SWPMISEL` writer - SWPMI kernel clock source selection Set and reset by software."]
pub type SWPMISEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDCCIP1R_SPEC, SWPMISEL_A, O>;
impl<'a, const O: u8> SWPMISEL_W<'a, O> {
    #[doc = "pclk selected as peripheral clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(SWPMISEL_A::Pclk)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SWPMISEL_A::HsiKer)
    }
}
impl R {
    #[doc = "Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it isnot be possible to switch to another clock. Refer to for additional information. Note: DFSDM1 clock source selection is done by DFSDM1SEL. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:8 - SAI2 kernel clock source A selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the SPDIFRX (see )."]
    #[inline(always)]
    pub fn sai2asel(&self) -> SAI2ASEL_R {
        SAI2ASEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SAI2 kernel clock B source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the spdifrx (see )."]
    #[inline(always)]
    pub fn sai2bsel(&self) -> SAI2BSEL_R {
        SAI2BSEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
    #[inline(always)]
    pub fn spi123sel(&self) -> SPI123SEL_R {
        SPI123SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi45sel(&self) -> SPI45SEL_R {
        SPI45SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - SPDIFRX kernel clock source selection"]
    #[inline(always)]
    pub fn spdifrxsel(&self) -> SPDIFRXSEL_R {
        SPDIFRXSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - DFSDM1 kernel clock Clk source selection Set and reset by software. Note: the DFSDM1 Aclk clock source selection is done by SAI1SEL (see )."]
    #[inline(always)]
    pub fn dfsdm1sel(&self) -> DFSDM1SEL_R {
        DFSDM1SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - FDCAN kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - SWPMI kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn swpmisel(&self) -> SWPMISEL_R {
        SWPMISEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it isnot be possible to switch to another clock. Refer to for additional information. Note: DFSDM1 clock source selection is done by DFSDM1SEL. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<0> {
        SAI1SEL_W::new(self)
    }
    #[doc = "Bits 6:8 - SAI2 kernel clock source A selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the SPDIFRX (see )."]
    #[inline(always)]
    #[must_use]
    pub fn sai2asel(&mut self) -> SAI2ASEL_W<6> {
        SAI2ASEL_W::new(self)
    }
    #[doc = "Bits 9:11 - SAI2 kernel clock B source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin. spdifrx_symb_ck is the symbol clock generated by the spdifrx (see )."]
    #[inline(always)]
    #[must_use]
    pub fn sai2bsel(&mut self) -> SAI2BSEL_W<9> {
        SAI2BSEL_W::new(self)
    }
    #[doc = "Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection Set and reset by software. If the selected clock is the external clock and this clock is stopped, it is not be possible to switch to another clock. Refer to for additional information. others: reserved, the kernel clock is disabled Note: I2S_CKIN is an external clock taken from a pin."]
    #[inline(always)]
    #[must_use]
    pub fn spi123sel(&mut self) -> SPI123SEL_W<12> {
        SPI123SEL_W::new(self)
    }
    #[doc = "Bits 16:18 - SPI4 and 5 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn spi45sel(&mut self) -> SPI45SEL_W<16> {
        SPI45SEL_W::new(self)
    }
    #[doc = "Bits 20:21 - SPDIFRX kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn spdifrxsel(&mut self) -> SPDIFRXSEL_W<20> {
        SPDIFRXSEL_W::new(self)
    }
    #[doc = "Bit 24 - DFSDM1 kernel clock Clk source selection Set and reset by software. Note: the DFSDM1 Aclk clock source selection is done by SAI1SEL (see )."]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1sel(&mut self) -> DFSDM1SEL_W<24> {
        DFSDM1SEL_W::new(self)
    }
    #[doc = "Bits 28:29 - FDCAN kernel clock source selection Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<28> {
        FDCANSEL_W::new(self)
    }
    #[doc = "Bit 31 - SWPMI kernel clock source selection Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn swpmisel(&mut self) -> SWPMISEL_W<31> {
        SWPMISEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC CPU domain kernel clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdccip1r](index.html) module"]
pub struct CDCCIP1R_SPEC;
impl crate::RegisterSpec for CDCCIP1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdccip1r::R](R) reader structure"]
impl crate::Readable for CDCCIP1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdccip1r::W](W) writer structure"]
impl crate::Writable for CDCCIP1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDCCIP1R to value 0"]
impl crate::Resettable for CDCCIP1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
