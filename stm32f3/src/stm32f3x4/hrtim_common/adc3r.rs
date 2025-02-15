#[doc = "Register `ADC3R` reader"]
pub struct R(crate::R<ADC3R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC3R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC3R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC3R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC3R` writer"]
pub struct W(crate::W<ADC3R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC3R_SPEC>;
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
impl From<crate::W<ADC3R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC3R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1MC1` reader - AD1MC1"]
pub type AD1MC1_R = crate::BitReader<AD1MC1_A>;
#[doc = "AD1MC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1MC1_A {
    #[doc = "0: No generation of ADC trigger on master compare event"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on master compare event"]
    Enabled = 1,
}
impl From<AD1MC1_A> for bool {
    #[inline(always)]
    fn from(variant: AD1MC1_A) -> Self {
        variant as u8 != 0
    }
}
impl AD1MC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD1MC1_A {
        match self.bits {
            false => AD1MC1_A::Disabled,
            true => AD1MC1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1MC1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1MC1_A::Enabled
    }
}
#[doc = "Field `AD1MC1` writer - AD1MC1"]
pub type AD1MC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC3R_SPEC, AD1MC1_A, O>;
impl<'a, const O: u8> AD1MC1_W<'a, O> {
    #[doc = "No generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1MC1_A::Disabled)
    }
    #[doc = "Generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1MC1_A::Enabled)
    }
}
#[doc = "Field `AD1MC2` reader - AD1MC2"]
pub use AD1MC1_R as AD1MC2_R;
#[doc = "Field `AD1MC3` reader - AD1MC3"]
pub use AD1MC1_R as AD1MC3_R;
#[doc = "Field `AD1MC4` reader - AD1MC4"]
pub use AD1MC1_R as AD1MC4_R;
#[doc = "Field `AD1MC2` writer - AD1MC2"]
pub use AD1MC1_W as AD1MC2_W;
#[doc = "Field `AD1MC3` writer - AD1MC3"]
pub use AD1MC1_W as AD1MC3_W;
#[doc = "Field `AD1MC4` writer - AD1MC4"]
pub use AD1MC1_W as AD1MC4_W;
#[doc = "Field `AD1MPER` reader - AD1MPER"]
pub type AD1MPER_R = crate::BitReader<AD1MPER_A>;
#[doc = "AD1MPER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1MPER_A {
    #[doc = "0: No generation of ADC trigger on timer period event"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on timer period event"]
    Enabled = 1,
}
impl From<AD1MPER_A> for bool {
    #[inline(always)]
    fn from(variant: AD1MPER_A) -> Self {
        variant as u8 != 0
    }
}
impl AD1MPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD1MPER_A {
        match self.bits {
            false => AD1MPER_A::Disabled,
            true => AD1MPER_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1MPER_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1MPER_A::Enabled
    }
}
#[doc = "Field `AD1MPER` writer - AD1MPER"]
pub type AD1MPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC3R_SPEC, AD1MPER_A, O>;
impl<'a, const O: u8> AD1MPER_W<'a, O> {
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1MPER_A::Disabled)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1MPER_A::Enabled)
    }
}
#[doc = "Field `AD1EEV1` reader - AD1EEV1"]
pub type AD1EEV1_R = crate::BitReader<AD1EEV1_A>;
#[doc = "AD1EEV1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1EEV1_A {
    #[doc = "0: No generation of ADC trigger on external event"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on external event"]
    Enabled = 1,
}
impl From<AD1EEV1_A> for bool {
    #[inline(always)]
    fn from(variant: AD1EEV1_A) -> Self {
        variant as u8 != 0
    }
}
impl AD1EEV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD1EEV1_A {
        match self.bits {
            false => AD1EEV1_A::Disabled,
            true => AD1EEV1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1EEV1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1EEV1_A::Enabled
    }
}
#[doc = "Field `AD1EEV1` writer - AD1EEV1"]
pub type AD1EEV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC3R_SPEC, AD1EEV1_A, O>;
impl<'a, const O: u8> AD1EEV1_W<'a, O> {
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1EEV1_A::Disabled)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1EEV1_A::Enabled)
    }
}
#[doc = "Field `AD1EEV2` reader - AD1EEV2"]
pub use AD1EEV1_R as AD1EEV2_R;
#[doc = "Field `AD1EEV3` reader - AD1EEV3"]
pub use AD1EEV1_R as AD1EEV3_R;
#[doc = "Field `AD1EEV4` reader - AD1EEV4"]
pub use AD1EEV1_R as AD1EEV4_R;
#[doc = "Field `AD1EEV5` reader - AD1EEV5"]
pub use AD1EEV1_R as AD1EEV5_R;
#[doc = "Field `AD1EEV2` writer - AD1EEV2"]
pub use AD1EEV1_W as AD1EEV2_W;
#[doc = "Field `AD1EEV3` writer - AD1EEV3"]
pub use AD1EEV1_W as AD1EEV3_W;
#[doc = "Field `AD1EEV4` writer - AD1EEV4"]
pub use AD1EEV1_W as AD1EEV4_W;
#[doc = "Field `AD1EEV5` writer - AD1EEV5"]
pub use AD1EEV1_W as AD1EEV5_W;
#[doc = "Field `AD1TAC2` reader - AD1TAC2"]
pub type AD1TAC2_R = crate::BitReader<AD1TAC2_A>;
#[doc = "AD1TAC2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1TAC2_A {
    #[doc = "0: No generation of ADC trigger on timer compare event"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on timer compare event"]
    Enabled = 1,
}
impl From<AD1TAC2_A> for bool {
    #[inline(always)]
    fn from(variant: AD1TAC2_A) -> Self {
        variant as u8 != 0
    }
}
impl AD1TAC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD1TAC2_A {
        match self.bits {
            false => AD1TAC2_A::Disabled,
            true => AD1TAC2_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1TAC2_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1TAC2_A::Enabled
    }
}
#[doc = "Field `AD1TAC2` writer - AD1TAC2"]
pub type AD1TAC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC3R_SPEC, AD1TAC2_A, O>;
impl<'a, const O: u8> AD1TAC2_W<'a, O> {
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TAC2_A::Disabled)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TAC2_A::Enabled)
    }
}
#[doc = "Field `AD1TAPER` reader - AD1TAPER"]
pub use AD1MPER_R as AD1TAPER_R;
#[doc = "Field `AD1TAPER` writer - AD1TAPER"]
pub use AD1MPER_W as AD1TAPER_W;
#[doc = "Field `AD1TAC3` reader - AD1TAC3"]
pub use AD1TAC2_R as AD1TAC3_R;
#[doc = "Field `AD1TAC4` reader - AD1TAC4"]
pub use AD1TAC2_R as AD1TAC4_R;
#[doc = "Field `AD1TAC3` writer - AD1TAC3"]
pub use AD1TAC2_W as AD1TAC3_W;
#[doc = "Field `AD1TAC4` writer - AD1TAC4"]
pub use AD1TAC2_W as AD1TAC4_W;
#[doc = "Field `AD1TARST` reader - AD1TARST"]
pub type AD1TARST_R = crate::BitReader<AD1TARST_A>;
#[doc = "AD1TARST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1TARST_A {
    #[doc = "0: No generation of ADC trigger on timer reset and roll-over"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on timer reset and roll-over"]
    Enabled = 1,
}
impl From<AD1TARST_A> for bool {
    #[inline(always)]
    fn from(variant: AD1TARST_A) -> Self {
        variant as u8 != 0
    }
}
impl AD1TARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD1TARST_A {
        match self.bits {
            false => AD1TARST_A::Disabled,
            true => AD1TARST_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1TARST_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1TARST_A::Enabled
    }
}
#[doc = "Field `AD1TARST` writer - AD1TARST"]
pub type AD1TARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC3R_SPEC, AD1TARST_A, O>;
impl<'a, const O: u8> AD1TARST_W<'a, O> {
    #[doc = "No generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TARST_A::Disabled)
    }
    #[doc = "Generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TARST_A::Enabled)
    }
}
#[doc = "Field `AD1TBPER` reader - AD1TBPER"]
pub use AD1MPER_R as AD1TBPER_R;
#[doc = "Field `AD1TCPER` reader - AD1TCPER"]
pub use AD1MPER_R as AD1TCPER_R;
#[doc = "Field `AD1TDPER` reader - AD1TDPER"]
pub use AD1MPER_R as AD1TDPER_R;
#[doc = "Field `AD1TEPER` reader - AD1TEPER"]
pub use AD1MPER_R as AD1TEPER_R;
#[doc = "Field `AD1TBPER` writer - AD1TBPER"]
pub use AD1MPER_W as AD1TBPER_W;
#[doc = "Field `AD1TCPER` writer - AD1TCPER"]
pub use AD1MPER_W as AD1TCPER_W;
#[doc = "Field `AD1TDPER` writer - AD1TDPER"]
pub use AD1MPER_W as AD1TDPER_W;
#[doc = "Field `AD1TEPER` writer - AD1TEPER"]
pub use AD1MPER_W as AD1TEPER_W;
#[doc = "Field `AD1TBC2` reader - AD1TBC2"]
pub use AD1TAC2_R as AD1TBC2_R;
#[doc = "Field `AD1TBC3` reader - AD1TBC3"]
pub use AD1TAC2_R as AD1TBC3_R;
#[doc = "Field `AD1TBC4` reader - AD1TBC4"]
pub use AD1TAC2_R as AD1TBC4_R;
#[doc = "Field `AD1TCC2` reader - AD1TCC2"]
pub use AD1TAC2_R as AD1TCC2_R;
#[doc = "Field `AD1TCC3` reader - AD1TCC3"]
pub use AD1TAC2_R as AD1TCC3_R;
#[doc = "Field `AD1TCC4` reader - AD1TCC4"]
pub use AD1TAC2_R as AD1TCC4_R;
#[doc = "Field `AD1TDC2` reader - AD1TDC2"]
pub use AD1TAC2_R as AD1TDC2_R;
#[doc = "Field `AD1TDC3` reader - AD1TDC3"]
pub use AD1TAC2_R as AD1TDC3_R;
#[doc = "Field `AD1TDC4` reader - AD1TDC4"]
pub use AD1TAC2_R as AD1TDC4_R;
#[doc = "Field `AD1TEC2` reader - AD1TEC2"]
pub use AD1TAC2_R as AD1TEC2_R;
#[doc = "Field `AD1TEC3` reader - AD1TEC3"]
pub use AD1TAC2_R as AD1TEC3_R;
#[doc = "Field `AD1TEC4` reader - AD1TEC4"]
pub use AD1TAC2_R as AD1TEC4_R;
#[doc = "Field `AD1TBC2` writer - AD1TBC2"]
pub use AD1TAC2_W as AD1TBC2_W;
#[doc = "Field `AD1TBC3` writer - AD1TBC3"]
pub use AD1TAC2_W as AD1TBC3_W;
#[doc = "Field `AD1TBC4` writer - AD1TBC4"]
pub use AD1TAC2_W as AD1TBC4_W;
#[doc = "Field `AD1TCC2` writer - AD1TCC2"]
pub use AD1TAC2_W as AD1TCC2_W;
#[doc = "Field `AD1TCC3` writer - AD1TCC3"]
pub use AD1TAC2_W as AD1TCC3_W;
#[doc = "Field `AD1TCC4` writer - AD1TCC4"]
pub use AD1TAC2_W as AD1TCC4_W;
#[doc = "Field `AD1TDC2` writer - AD1TDC2"]
pub use AD1TAC2_W as AD1TDC2_W;
#[doc = "Field `AD1TDC3` writer - AD1TDC3"]
pub use AD1TAC2_W as AD1TDC3_W;
#[doc = "Field `AD1TDC4` writer - AD1TDC4"]
pub use AD1TAC2_W as AD1TDC4_W;
#[doc = "Field `AD1TEC2` writer - AD1TEC2"]
pub use AD1TAC2_W as AD1TEC2_W;
#[doc = "Field `AD1TEC3` writer - AD1TEC3"]
pub use AD1TAC2_W as AD1TEC3_W;
#[doc = "Field `AD1TEC4` writer - AD1TEC4"]
pub use AD1TAC2_W as AD1TEC4_W;
#[doc = "Field `AD1TBRST` reader - AD1TBRST"]
pub use AD1TARST_R as AD1TBRST_R;
#[doc = "Field `AD1TBRST` writer - AD1TBRST"]
pub use AD1TARST_W as AD1TBRST_W;
impl R {
    #[doc = "Bit 0 - AD1MC1"]
    #[inline(always)]
    pub fn ad1mc1(&self) -> AD1MC1_R {
        AD1MC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD1MC2"]
    #[inline(always)]
    pub fn ad1mc2(&self) -> AD1MC2_R {
        AD1MC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD1MC3"]
    #[inline(always)]
    pub fn ad1mc3(&self) -> AD1MC3_R {
        AD1MC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD1MC4"]
    #[inline(always)]
    pub fn ad1mc4(&self) -> AD1MC4_R {
        AD1MC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD1MPER"]
    #[inline(always)]
    pub fn ad1mper(&self) -> AD1MPER_R {
        AD1MPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AD1EEV1"]
    #[inline(always)]
    pub fn ad1eev1(&self) -> AD1EEV1_R {
        AD1EEV1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AD1EEV2"]
    #[inline(always)]
    pub fn ad1eev2(&self) -> AD1EEV2_R {
        AD1EEV2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AD1EEV3"]
    #[inline(always)]
    pub fn ad1eev3(&self) -> AD1EEV3_R {
        AD1EEV3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AD1EEV4"]
    #[inline(always)]
    pub fn ad1eev4(&self) -> AD1EEV4_R {
        AD1EEV4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD1EEV5"]
    #[inline(always)]
    pub fn ad1eev5(&self) -> AD1EEV5_R {
        AD1EEV5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD1TAC2"]
    #[inline(always)]
    pub fn ad1tac2(&self) -> AD1TAC2_R {
        AD1TAC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AD1TAC3"]
    #[inline(always)]
    pub fn ad1tac3(&self) -> AD1TAC3_R {
        AD1TAC3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AD1TAC4"]
    #[inline(always)]
    pub fn ad1tac4(&self) -> AD1TAC4_R {
        AD1TAC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AD1TAPER"]
    #[inline(always)]
    pub fn ad1taper(&self) -> AD1TAPER_R {
        AD1TAPER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AD1TARST"]
    #[inline(always)]
    pub fn ad1tarst(&self) -> AD1TARST_R {
        AD1TARST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AD1TBC2"]
    #[inline(always)]
    pub fn ad1tbc2(&self) -> AD1TBC2_R {
        AD1TBC2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AD1TBC3"]
    #[inline(always)]
    pub fn ad1tbc3(&self) -> AD1TBC3_R {
        AD1TBC3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AD1TBC4"]
    #[inline(always)]
    pub fn ad1tbc4(&self) -> AD1TBC4_R {
        AD1TBC4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - AD1TBPER"]
    #[inline(always)]
    pub fn ad1tbper(&self) -> AD1TBPER_R {
        AD1TBPER_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - AD1TBRST"]
    #[inline(always)]
    pub fn ad1tbrst(&self) -> AD1TBRST_R {
        AD1TBRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AD1TCC2"]
    #[inline(always)]
    pub fn ad1tcc2(&self) -> AD1TCC2_R {
        AD1TCC2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - AD1TCC3"]
    #[inline(always)]
    pub fn ad1tcc3(&self) -> AD1TCC3_R {
        AD1TCC3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - AD1TCC4"]
    #[inline(always)]
    pub fn ad1tcc4(&self) -> AD1TCC4_R {
        AD1TCC4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AD1TCPER"]
    #[inline(always)]
    pub fn ad1tcper(&self) -> AD1TCPER_R {
        AD1TCPER_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - AD1TDC2"]
    #[inline(always)]
    pub fn ad1tdc2(&self) -> AD1TDC2_R {
        AD1TDC2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AD1TDC3"]
    #[inline(always)]
    pub fn ad1tdc3(&self) -> AD1TDC3_R {
        AD1TDC3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - AD1TDC4"]
    #[inline(always)]
    pub fn ad1tdc4(&self) -> AD1TDC4_R {
        AD1TDC4_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - AD1TDPER"]
    #[inline(always)]
    pub fn ad1tdper(&self) -> AD1TDPER_R {
        AD1TDPER_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - AD1TEC2"]
    #[inline(always)]
    pub fn ad1tec2(&self) -> AD1TEC2_R {
        AD1TEC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - AD1TEC3"]
    #[inline(always)]
    pub fn ad1tec3(&self) -> AD1TEC3_R {
        AD1TEC3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AD1TEC4"]
    #[inline(always)]
    pub fn ad1tec4(&self) -> AD1TEC4_R {
        AD1TEC4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AD1TEPER"]
    #[inline(always)]
    pub fn ad1teper(&self) -> AD1TEPER_R {
        AD1TEPER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD1MC1"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mc1(&mut self) -> AD1MC1_W<0> {
        AD1MC1_W::new(self)
    }
    #[doc = "Bit 1 - AD1MC2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mc2(&mut self) -> AD1MC2_W<1> {
        AD1MC2_W::new(self)
    }
    #[doc = "Bit 2 - AD1MC3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mc3(&mut self) -> AD1MC3_W<2> {
        AD1MC3_W::new(self)
    }
    #[doc = "Bit 3 - AD1MC4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mc4(&mut self) -> AD1MC4_W<3> {
        AD1MC4_W::new(self)
    }
    #[doc = "Bit 4 - AD1MPER"]
    #[inline(always)]
    #[must_use]
    pub fn ad1mper(&mut self) -> AD1MPER_W<4> {
        AD1MPER_W::new(self)
    }
    #[doc = "Bit 5 - AD1EEV1"]
    #[inline(always)]
    #[must_use]
    pub fn ad1eev1(&mut self) -> AD1EEV1_W<5> {
        AD1EEV1_W::new(self)
    }
    #[doc = "Bit 6 - AD1EEV2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1eev2(&mut self) -> AD1EEV2_W<6> {
        AD1EEV2_W::new(self)
    }
    #[doc = "Bit 7 - AD1EEV3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1eev3(&mut self) -> AD1EEV3_W<7> {
        AD1EEV3_W::new(self)
    }
    #[doc = "Bit 8 - AD1EEV4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1eev4(&mut self) -> AD1EEV4_W<8> {
        AD1EEV4_W::new(self)
    }
    #[doc = "Bit 9 - AD1EEV5"]
    #[inline(always)]
    #[must_use]
    pub fn ad1eev5(&mut self) -> AD1EEV5_W<9> {
        AD1EEV5_W::new(self)
    }
    #[doc = "Bit 10 - AD1TAC2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tac2(&mut self) -> AD1TAC2_W<10> {
        AD1TAC2_W::new(self)
    }
    #[doc = "Bit 11 - AD1TAC3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tac3(&mut self) -> AD1TAC3_W<11> {
        AD1TAC3_W::new(self)
    }
    #[doc = "Bit 12 - AD1TAC4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tac4(&mut self) -> AD1TAC4_W<12> {
        AD1TAC4_W::new(self)
    }
    #[doc = "Bit 13 - AD1TAPER"]
    #[inline(always)]
    #[must_use]
    pub fn ad1taper(&mut self) -> AD1TAPER_W<13> {
        AD1TAPER_W::new(self)
    }
    #[doc = "Bit 14 - AD1TARST"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tarst(&mut self) -> AD1TARST_W<14> {
        AD1TARST_W::new(self)
    }
    #[doc = "Bit 15 - AD1TBC2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tbc2(&mut self) -> AD1TBC2_W<15> {
        AD1TBC2_W::new(self)
    }
    #[doc = "Bit 16 - AD1TBC3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tbc3(&mut self) -> AD1TBC3_W<16> {
        AD1TBC3_W::new(self)
    }
    #[doc = "Bit 17 - AD1TBC4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tbc4(&mut self) -> AD1TBC4_W<17> {
        AD1TBC4_W::new(self)
    }
    #[doc = "Bit 18 - AD1TBPER"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tbper(&mut self) -> AD1TBPER_W<18> {
        AD1TBPER_W::new(self)
    }
    #[doc = "Bit 19 - AD1TBRST"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tbrst(&mut self) -> AD1TBRST_W<19> {
        AD1TBRST_W::new(self)
    }
    #[doc = "Bit 20 - AD1TCC2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tcc2(&mut self) -> AD1TCC2_W<20> {
        AD1TCC2_W::new(self)
    }
    #[doc = "Bit 21 - AD1TCC3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tcc3(&mut self) -> AD1TCC3_W<21> {
        AD1TCC3_W::new(self)
    }
    #[doc = "Bit 22 - AD1TCC4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tcc4(&mut self) -> AD1TCC4_W<22> {
        AD1TCC4_W::new(self)
    }
    #[doc = "Bit 23 - AD1TCPER"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tcper(&mut self) -> AD1TCPER_W<23> {
        AD1TCPER_W::new(self)
    }
    #[doc = "Bit 24 - AD1TDC2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tdc2(&mut self) -> AD1TDC2_W<24> {
        AD1TDC2_W::new(self)
    }
    #[doc = "Bit 25 - AD1TDC3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tdc3(&mut self) -> AD1TDC3_W<25> {
        AD1TDC3_W::new(self)
    }
    #[doc = "Bit 26 - AD1TDC4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tdc4(&mut self) -> AD1TDC4_W<26> {
        AD1TDC4_W::new(self)
    }
    #[doc = "Bit 27 - AD1TDPER"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tdper(&mut self) -> AD1TDPER_W<27> {
        AD1TDPER_W::new(self)
    }
    #[doc = "Bit 28 - AD1TEC2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tec2(&mut self) -> AD1TEC2_W<28> {
        AD1TEC2_W::new(self)
    }
    #[doc = "Bit 29 - AD1TEC3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tec3(&mut self) -> AD1TEC3_W<29> {
        AD1TEC3_W::new(self)
    }
    #[doc = "Bit 30 - AD1TEC4"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tec4(&mut self) -> AD1TEC4_W<30> {
        AD1TEC4_W::new(self)
    }
    #[doc = "Bit 31 - AD1TEPER"]
    #[inline(always)]
    #[must_use]
    pub fn ad1teper(&mut self) -> AD1TEPER_W<31> {
        AD1TEPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Trigger 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc3r](index.html) module"]
pub struct ADC3R_SPEC;
impl crate::RegisterSpec for ADC3R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc3r::R](R) reader structure"]
impl crate::Readable for ADC3R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc3r::W](W) writer structure"]
impl crate::Writable for ADC3R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC3R to value 0"]
impl crate::Resettable for ADC3R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
