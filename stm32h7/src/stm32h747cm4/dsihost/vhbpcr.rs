#[doc = "Register `VHBPCR` reader"]
pub struct R(crate::R<VHBPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VHBPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VHBPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VHBPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VHBPCR` writer"]
pub struct W(crate::W<VHBPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VHBPCR_SPEC>;
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
impl From<crate::W<VHBPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VHBPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HBP` reader - Horizontal back-porch duration"]
pub type HBP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HBP` writer - Horizontal back-porch duration"]
pub type HBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VHBPCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Horizontal back-porch duration"]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal back-porch duration"]
    #[inline(always)]
    #[must_use]
    pub fn hbp(&mut self) -> HBP_W<0> {
        HBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host video HBP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vhbpcr](index.html) module"]
pub struct VHBPCR_SPEC;
impl crate::RegisterSpec for VHBPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vhbpcr::R](R) reader structure"]
impl crate::Readable for VHBPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vhbpcr::W](W) writer structure"]
impl crate::Writable for VHBPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VHBPCR to value 0"]
impl crate::Resettable for VHBPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
