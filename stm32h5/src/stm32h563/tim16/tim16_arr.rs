#[doc = "Register `TIM16_ARR` reader"]
pub struct R(crate::R<TIM16_ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM16_ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM16_ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM16_ARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM16_ARR` writer"]
pub struct W(crate::W<TIM16_ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM16_ARR_SPEC>;
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
impl From<crate::W<TIM16_ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM16_ARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARR` reader - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value in ARR\\[15:0\\]. The ARR\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\]
bitfield contains the dithered part."]
pub type ARR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ARR` writer - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value in ARR\\[15:0\\]. The ARR\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\]
bitfield contains the dithered part."]
pub type ARR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM16_ARR_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value in ARR\\[15:0\\]. The ARR\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\]
bitfield contains the dithered part."]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value in ARR\\[15:0\\]. The ARR\\[19:16\\]
bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\]
bitfield contains the dithered part."]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<0> {
        ARR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim16_arr](index.html) module"]
pub struct TIM16_ARR_SPEC;
impl crate::RegisterSpec for TIM16_ARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim16_arr::R](R) reader structure"]
impl crate::Readable for TIM16_ARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim16_arr::W](W) writer structure"]
impl crate::Writable for TIM16_ARR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM16_ARR to value 0xffff"]
impl crate::Resettable for TIM16_ARR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
