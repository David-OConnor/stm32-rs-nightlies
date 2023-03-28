#[doc = "Register `OFR2` reader"]
pub struct R(crate::R<OFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFR2` writer"]
pub struct W(crate::W<OFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR2_SPEC>;
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
impl From<crate::W<OFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET2` reader - ADC offset number 1 offset level"]
pub type OFFSET2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OFFSET2` writer - ADC offset number 1 offset level"]
pub type OFFSET2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR2_SPEC, u32, u32, 26, O>;
#[doc = "Field `OFFSET2_CH` reader - ADC offset number 1 channel selection"]
pub type OFFSET2_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET2_CH` writer - ADC offset number 1 channel selection"]
pub type OFFSET2_CH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SSATE` reader - Signed saturation enable"]
pub type SSATE_R = crate::BitReader<SSATE_A>;
#[doc = "Signed saturation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSATE_A {
    #[doc = "0: Offset is subtracted maintaining data integrity and extending result size (9-bit and 17-bit signed format)"]
    Disabled = 0,
    #[doc = "1: Offset is subtracted and result is saturated to maintain result size"]
    Enabled = 1,
}
impl From<SSATE_A> for bool {
    #[inline(always)]
    fn from(variant: SSATE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSATE_A {
        match self.bits {
            false => SSATE_A::Disabled,
            true => SSATE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSATE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSATE_A::Enabled
    }
}
#[doc = "Field `SSATE` writer - Signed saturation enable"]
pub type SSATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR2_SPEC, SSATE_A, O>;
impl<'a, const O: u8> SSATE_W<'a, O> {
    #[doc = "Offset is subtracted maintaining data integrity and extending result size (9-bit and 17-bit signed format)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSATE_A::Disabled)
    }
    #[doc = "Offset is subtracted and result is saturated to maintain result size"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSATE_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:25 - ADC offset number 1 offset level"]
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:30 - ADC offset number 1 channel selection"]
    #[inline(always)]
    pub fn offset2_ch(&self) -> OFFSET2_CH_R {
        OFFSET2_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Signed saturation enable"]
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - ADC offset number 1 offset level"]
    #[inline(always)]
    #[must_use]
    pub fn offset2(&mut self) -> OFFSET2_W<0> {
        OFFSET2_W::new(self)
    }
    #[doc = "Bits 26:30 - ADC offset number 1 channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn offset2_ch(&mut self) -> OFFSET2_CH_W<26> {
        OFFSET2_CH_W::new(self)
    }
    #[doc = "Bit 31 - Signed saturation enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssate(&mut self) -> SSATE_W<31> {
        SSATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC offset number 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr2](index.html) module"]
pub struct OFR2_SPEC;
impl crate::RegisterSpec for OFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofr2::R](R) reader structure"]
impl crate::Readable for OFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofr2::W](W) writer structure"]
impl crate::Writable for OFR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFR2 to value 0"]
impl crate::Resettable for OFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
