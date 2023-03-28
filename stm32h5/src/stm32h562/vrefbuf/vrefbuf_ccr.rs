#[doc = "Register `VREFBUF_CCR` reader"]
pub struct R(crate::R<VREFBUF_CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREFBUF_CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREFBUF_CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREFBUF_CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREFBUF_CCR` writer"]
pub struct W(crate::W<VREFBUF_CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREFBUF_CCR_SPEC>;
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
impl From<crate::W<VREFBUF_CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREFBUF_CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\\[5:0\\]
is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\\[5:0\\]
is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\\[5:0\\]: User can modify the TRIM\\[5:0\\]
with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order."]
pub type TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM` writer - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\\[5:0\\]
is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\\[5:0\\]
is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\\[5:0\\]: User can modify the TRIM\\[5:0\\]
with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order."]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREFBUF_CCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\\[5:0\\]
is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\\[5:0\\]
is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\\[5:0\\]: User can modify the TRIM\\[5:0\\]
with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\\[5:0\\]
is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\\[5:0\\]
is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\\[5:0\\]: User can modify the TRIM\\[5:0\\]
with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<0> {
        TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREFBUF calibration control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrefbuf_ccr](index.html) module"]
pub struct VREFBUF_CCR_SPEC;
impl crate::RegisterSpec for VREFBUF_CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vrefbuf_ccr::R](R) reader structure"]
impl crate::Readable for VREFBUF_CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vrefbuf_ccr::W](W) writer structure"]
impl crate::Writable for VREFBUF_CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREFBUF_CCR to value 0"]
impl crate::Resettable for VREFBUF_CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
