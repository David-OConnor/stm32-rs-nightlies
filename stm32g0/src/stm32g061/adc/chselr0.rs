#[doc = "Register `CHSELR0` reader"]
pub struct R(crate::R<CHSELR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSELR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSELR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSELR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHSELR0` writer"]
pub struct W(crate::W<CHSELR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSELR0_SPEC>;
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
impl From<crate::W<CHSELR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSELR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSEL0` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL0_R = crate::BitReader<CHSEL0_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL0_A {
    #[doc = "0: Input Channel is not selected for conversion"]
    NotSelected = 0,
    #[doc = "1: Input Channel is selected for conversion"]
    Selected = 1,
}
impl From<CHSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSEL0_A {
        match self.bits {
            false => CHSEL0_A::NotSelected,
            true => CHSEL0_A::Selected,
        }
    }
    #[doc = "Checks if the value of the field is `NotSelected`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == CHSEL0_A::NotSelected
    }
    #[doc = "Checks if the value of the field is `Selected`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == CHSEL0_A::Selected
    }
}
#[doc = "Field `CHSEL0` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHSELR0_SPEC, CHSEL0_A, O>;
impl<'a, const O: u8> CHSEL0_W<'a, O> {
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(CHSEL0_A::NotSelected)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(CHSEL0_A::Selected)
    }
}
#[doc = "Field `CHSEL1` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL1_R;
#[doc = "Field `CHSEL2` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL2_R;
#[doc = "Field `CHSEL3` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL3_R;
#[doc = "Field `CHSEL4` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL4_R;
#[doc = "Field `CHSEL5` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL5_R;
#[doc = "Field `CHSEL6` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL6_R;
#[doc = "Field `CHSEL7` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL7_R;
#[doc = "Field `CHSEL8` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL8_R;
#[doc = "Field `CHSEL9` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL9_R;
#[doc = "Field `CHSEL10` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL10_R;
#[doc = "Field `CHSEL11` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL11_R;
#[doc = "Field `CHSEL12` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL12_R;
#[doc = "Field `CHSEL13` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL13_R;
#[doc = "Field `CHSEL14` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL14_R;
#[doc = "Field `CHSEL15` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL15_R;
#[doc = "Field `CHSEL16` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL16_R;
#[doc = "Field `CHSEL17` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL17_R;
#[doc = "Field `CHSEL18` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_R as CHSEL18_R;
#[doc = "Field `CHSEL1` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL1_W;
#[doc = "Field `CHSEL2` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL2_W;
#[doc = "Field `CHSEL3` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL3_W;
#[doc = "Field `CHSEL4` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL4_W;
#[doc = "Field `CHSEL5` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL5_W;
#[doc = "Field `CHSEL6` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL6_W;
#[doc = "Field `CHSEL7` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL7_W;
#[doc = "Field `CHSEL8` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL8_W;
#[doc = "Field `CHSEL9` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL9_W;
#[doc = "Field `CHSEL10` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL10_W;
#[doc = "Field `CHSEL11` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL11_W;
#[doc = "Field `CHSEL12` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL12_W;
#[doc = "Field `CHSEL13` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL13_W;
#[doc = "Field `CHSEL14` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL14_W;
#[doc = "Field `CHSEL15` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL15_W;
#[doc = "Field `CHSEL16` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL16_W;
#[doc = "Field `CHSEL17` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL17_W;
#[doc = "Field `CHSEL18` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub use CHSEL0_W as CHSEL18_W;
impl R {
    #[doc = "Bit 0 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL8_R {
        CHSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL9_R {
        CHSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel10(&self) -> CHSEL10_R {
        CHSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel11(&self) -> CHSEL11_R {
        CHSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel12(&self) -> CHSEL12_R {
        CHSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel13(&self) -> CHSEL13_R {
        CHSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel14(&self) -> CHSEL14_R {
        CHSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel15(&self) -> CHSEL15_R {
        CHSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel16(&self) -> CHSEL16_R {
        CHSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel17(&self) -> CHSEL17_R {
        CHSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel18(&self) -> CHSEL18_R {
        CHSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel0(&mut self) -> CHSEL0_W<0> {
        CHSEL0_W::new(self)
    }
    #[doc = "Bit 1 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel1(&mut self) -> CHSEL1_W<1> {
        CHSEL1_W::new(self)
    }
    #[doc = "Bit 2 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel2(&mut self) -> CHSEL2_W<2> {
        CHSEL2_W::new(self)
    }
    #[doc = "Bit 3 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel3(&mut self) -> CHSEL3_W<3> {
        CHSEL3_W::new(self)
    }
    #[doc = "Bit 4 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel4(&mut self) -> CHSEL4_W<4> {
        CHSEL4_W::new(self)
    }
    #[doc = "Bit 5 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel5(&mut self) -> CHSEL5_W<5> {
        CHSEL5_W::new(self)
    }
    #[doc = "Bit 6 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel6(&mut self) -> CHSEL6_W<6> {
        CHSEL6_W::new(self)
    }
    #[doc = "Bit 7 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel7(&mut self) -> CHSEL7_W<7> {
        CHSEL7_W::new(self)
    }
    #[doc = "Bit 8 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel8(&mut self) -> CHSEL8_W<8> {
        CHSEL8_W::new(self)
    }
    #[doc = "Bit 9 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel9(&mut self) -> CHSEL9_W<9> {
        CHSEL9_W::new(self)
    }
    #[doc = "Bit 10 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel10(&mut self) -> CHSEL10_W<10> {
        CHSEL10_W::new(self)
    }
    #[doc = "Bit 11 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel11(&mut self) -> CHSEL11_W<11> {
        CHSEL11_W::new(self)
    }
    #[doc = "Bit 12 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel12(&mut self) -> CHSEL12_W<12> {
        CHSEL12_W::new(self)
    }
    #[doc = "Bit 13 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel13(&mut self) -> CHSEL13_W<13> {
        CHSEL13_W::new(self)
    }
    #[doc = "Bit 14 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel14(&mut self) -> CHSEL14_W<14> {
        CHSEL14_W::new(self)
    }
    #[doc = "Bit 15 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel15(&mut self) -> CHSEL15_W<15> {
        CHSEL15_W::new(self)
    }
    #[doc = "Bit 16 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel16(&mut self) -> CHSEL16_W<16> {
        CHSEL16_W::new(self)
    }
    #[doc = "Bit 17 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel17(&mut self) -> CHSEL17_W<17> {
        CHSEL17_W::new(self)
    }
    #[doc = "Bit 18 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel18(&mut self) -> CHSEL18_W<18> {
        CHSEL18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chselr0](index.html) module"]
pub struct CHSELR0_SPEC;
impl crate::RegisterSpec for CHSELR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chselr0::R](R) reader structure"]
impl crate::Readable for CHSELR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chselr0::W](W) writer structure"]
impl crate::Writable for CHSELR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHSELR0 to value 0"]
impl crate::Resettable for CHSELR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
