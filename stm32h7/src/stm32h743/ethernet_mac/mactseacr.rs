#[doc = "Register `MACTSEACR` reader"]
pub struct R(crate::R<MACTSEACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTSEACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTSEACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTSEACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACTSEACR` writer"]
pub struct W(crate::W<MACTSEACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACTSEACR_SPEC>;
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
impl From<crate::W<MACTSEACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACTSEACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSTEAC` reader - One-Step Timestamp Egress Asymmetry Correction"]
pub type OSTEAC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OSTEAC` writer - One-Step Timestamp Egress Asymmetry Correction"]
pub type OSTEAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACTSEACR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - One-Step Timestamp Egress Asymmetry Correction"]
    #[inline(always)]
    pub fn osteac(&self) -> OSTEAC_R {
        OSTEAC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - One-Step Timestamp Egress Asymmetry Correction"]
    #[inline(always)]
    #[must_use]
    pub fn osteac(&mut self) -> OSTEAC_W<0> {
        OSTEAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Egress asymmetric correction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactseacr](index.html) module"]
pub struct MACTSEACR_SPEC;
impl crate::RegisterSpec for MACTSEACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mactseacr::R](R) reader structure"]
impl crate::Readable for MACTSEACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mactseacr::W](W) writer structure"]
impl crate::Writable for MACTSEACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACTSEACR to value 0"]
impl crate::Resettable for MACTSEACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
