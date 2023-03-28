#[doc = "Register `ADC_OFR4` reader"]
pub struct R(crate::R<ADC_OFR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR4` writer"]
pub struct W(crate::W<ADC_OFR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR4_SPEC>;
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
impl From<crate::W<ADC_OFR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET4` reader - OFFSET4"]
pub type OFFSET4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OFFSET4` writer - OFFSET4"]
pub type OFFSET4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_OFR4_SPEC, u32, u32, 26, O>;
#[doc = "Field `OFFSET4_CH` reader - OFFSET4_CH"]
pub type OFFSET4_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET4_CH` writer - OFFSET4_CH"]
pub type OFFSET4_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC_OFR4_SPEC, u8, u8, 5, O>;
#[doc = "Field `SSATE` reader - SSATE"]
pub type SSATE_R = crate::BitReader<bool>;
#[doc = "Field `SSATE` writer - SSATE"]
pub type SSATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_OFR4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:25 - OFFSET4"]
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:30 - OFFSET4_CH"]
    #[inline(always)]
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - SSATE"]
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - OFFSET4"]
    #[inline(always)]
    #[must_use]
    pub fn offset4(&mut self) -> OFFSET4_W<0> {
        OFFSET4_W::new(self)
    }
    #[doc = "Bits 26:30 - OFFSET4_CH"]
    #[inline(always)]
    #[must_use]
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W<26> {
        OFFSET4_CH_W::new(self)
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
#[doc = "ADC offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr4](index.html) module"]
pub struct ADC_OFR4_SPEC;
impl crate::RegisterSpec for ADC_OFR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr4::R](R) reader structure"]
impl crate::Readable for ADC_OFR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr4::W](W) writer structure"]
impl crate::Writable for ADC_OFR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR4 to value 0"]
impl crate::Resettable for ADC_OFR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
