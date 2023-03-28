#[doc = "Register `L2PFCR` reader"]
pub struct R(crate::R<L2PFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2PFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2PFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2PFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L2PFCR` writer"]
pub struct W(crate::W<L2PFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2PFCR_SPEC>;
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
impl From<crate::W<L2PFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2PFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PF` reader - Pixel Format"]
pub type PF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PF` writer - Pixel Format"]
pub type PF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, L2PFCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Pixel Format"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pixel Format"]
    #[inline(always)]
    #[must_use]
    pub fn pf(&mut self) -> PF_W<0> {
        PF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC Layer Pixel Format Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2pfcr](index.html) module"]
pub struct L2PFCR_SPEC;
impl crate::RegisterSpec for L2PFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2pfcr::R](R) reader structure"]
impl crate::Readable for L2PFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l2pfcr::W](W) writer structure"]
impl crate::Writable for L2PFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2PFCR to value 0"]
impl crate::Resettable for L2PFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}