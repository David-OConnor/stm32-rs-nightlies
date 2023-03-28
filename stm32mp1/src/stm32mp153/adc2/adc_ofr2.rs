#[doc = "Register `ADC_OFR2` reader"]
pub struct R(crate::R<ADC_OFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR2` writer"]
pub struct W(crate::W<ADC_OFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR2_SPEC>;
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
impl From<crate::W<ADC_OFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET2` reader - OFFSET2"]
pub type OFFSET2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OFFSET2` writer - OFFSET2"]
pub type OFFSET2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_OFR2_SPEC, u32, u32, 26, O>;
#[doc = "Field `OFFSET2_CH` reader - OFFSET2_CH"]
pub type OFFSET2_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET2_CH` writer - OFFSET2_CH"]
pub type OFFSET2_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_OFR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SSATE` reader - SSATE"]
pub type SSATE_R = crate::BitReader<bool>;
#[doc = "Field `SSATE` writer - SSATE"]
pub type SSATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_OFR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:25 - OFFSET2"]
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:30 - OFFSET2_CH"]
    #[inline(always)]
    pub fn offset2_ch(&self) -> OFFSET2_CH_R {
        OFFSET2_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - SSATE"]
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - OFFSET2"]
    #[inline(always)]
    #[must_use]
    pub fn offset2(&mut self) -> OFFSET2_W<0> {
        OFFSET2_W::new(self)
    }
    #[doc = "Bits 26:30 - OFFSET2_CH"]
    #[inline(always)]
    #[must_use]
    pub fn offset2_ch(&mut self) -> OFFSET2_CH_W<26> {
        OFFSET2_CH_W::new(self)
    }
    #[doc = "Bit 31 - SSATE"]
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
#[doc = "ADC offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr2](index.html) module"]
pub struct ADC_OFR2_SPEC;
impl crate::RegisterSpec for ADC_OFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr2::R](R) reader structure"]
impl crate::Readable for ADC_OFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr2::W](W) writer structure"]
impl crate::Writable for ADC_OFR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR2 to value 0"]
impl crate::Resettable for ADC_OFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
