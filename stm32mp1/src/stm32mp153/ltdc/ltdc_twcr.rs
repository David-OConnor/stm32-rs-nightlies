#[doc = "Register `LTDC_TWCR` reader"]
pub struct R(crate::R<LTDC_TWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_TWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_TWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_TWCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_TWCR` writer"]
pub struct W(crate::W<LTDC_TWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_TWCR_SPEC>;
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
impl From<crate::W<LTDC_TWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_TWCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOTALH` reader - TOTALH"]
pub type TOTALH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOTALH` writer - TOTALH"]
pub type TOTALH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_TWCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `TOTALW` reader - TOTALW"]
pub type TOTALW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOTALW` writer - TOTALW"]
pub type TOTALW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_TWCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - TOTALH"]
    #[inline(always)]
    pub fn totalh(&self) -> TOTALH_R {
        TOTALH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - TOTALW"]
    #[inline(always)]
    pub fn totalw(&self) -> TOTALW_R {
        TOTALW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TOTALH"]
    #[inline(always)]
    #[must_use]
    pub fn totalh(&mut self) -> TOTALH_W<0> {
        TOTALH_W::new(self)
    }
    #[doc = "Bits 16:27 - TOTALW"]
    #[inline(always)]
    #[must_use]
    pub fn totalw(&mut self) -> TOTALW_W<16> {
        TOTALW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_twcr](index.html) module"]
pub struct LTDC_TWCR_SPEC;
impl crate::RegisterSpec for LTDC_TWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_twcr::R](R) reader structure"]
impl crate::Readable for LTDC_TWCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_twcr::W](W) writer structure"]
impl crate::Writable for LTDC_TWCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTDC_TWCR to value 0"]
impl crate::Resettable for LTDC_TWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
