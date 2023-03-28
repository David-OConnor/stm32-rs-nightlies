#[doc = "Register `TZC_REGION_TOP_LOW5` reader"]
pub struct R(crate::R<TZC_REGION_TOP_LOW5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_REGION_TOP_LOW5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_REGION_TOP_LOW5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_REGION_TOP_LOW5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZC_REGION_TOP_LOW5` writer"]
pub struct W(crate::W<TZC_REGION_TOP_LOW5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_REGION_TOP_LOW5_SPEC>;
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
impl From<crate::W<TZC_REGION_TOP_LOW5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_REGION_TOP_LOW5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOP_ADDRESS_LOW` reader - TOP_ADDRESS_LOW"]
pub type TOP_ADDRESS_LOW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TOP_ADDRESS_LOW` writer - TOP_ADDRESS_LOW"]
pub type TOP_ADDRESS_LOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_REGION_TOP_LOW5_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 12:31 - TOP_ADDRESS_LOW"]
    #[inline(always)]
    pub fn top_address_low(&self) -> TOP_ADDRESS_LOW_R {
        TOP_ADDRESS_LOW_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - TOP_ADDRESS_LOW"]
    #[inline(always)]
    #[must_use]
    pub fn top_address_low(&mut self) -> TOP_ADDRESS_LOW_W<12> {
        TOP_ADDRESS_LOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Top address bits \\[31:12\\]
for region x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_low5](index.html) module"]
pub struct TZC_REGION_TOP_LOW5_SPEC;
impl crate::RegisterSpec for TZC_REGION_TOP_LOW5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_region_top_low5::R](R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_region_top_low5::W](W) writer structure"]
impl crate::Writable for TZC_REGION_TOP_LOW5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TZC_REGION_TOP_LOW5 to value 0x0fff"]
impl crate::Resettable for TZC_REGION_TOP_LOW5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}