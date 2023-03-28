#[doc = "Register `LTDC_AWCR` reader"]
pub struct R(crate::R<LTDC_AWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_AWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_AWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_AWCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_AWCR` writer"]
pub struct W(crate::W<LTDC_AWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_AWCR_SPEC>;
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
impl From<crate::W<LTDC_AWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_AWCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AAH` reader - AAH"]
pub type AAH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AAH` writer - AAH"]
pub type AAH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_AWCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `AAW` reader - AAW"]
pub type AAW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AAW` writer - AAW"]
pub type AAW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_AWCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - AAH"]
    #[inline(always)]
    pub fn aah(&self) -> AAH_R {
        AAH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - AAW"]
    #[inline(always)]
    pub fn aaw(&self) -> AAW_R {
        AAW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - AAH"]
    #[inline(always)]
    #[must_use]
    pub fn aah(&mut self) -> AAH_W<0> {
        AAH_W::new(self)
    }
    #[doc = "Bits 16:27 - AAW"]
    #[inline(always)]
    #[must_use]
    pub fn aaw(&mut self) -> AAW_W<16> {
        AAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_awcr](index.html) module"]
pub struct LTDC_AWCR_SPEC;
impl crate::RegisterSpec for LTDC_AWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_awcr::R](R) reader structure"]
impl crate::Readable for LTDC_AWCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_awcr::W](W) writer structure"]
impl crate::Writable for LTDC_AWCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_AWCR to value 0"]
impl crate::Resettable for LTDC_AWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
