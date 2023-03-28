#[doc = "Register `DVR` reader"]
pub struct R(crate::R<DVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DVR` writer"]
pub struct W(crate::W<DVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVR_SPEC>;
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
impl From<crate::W<DVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DV` reader - Default value"]
pub type DV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DV` writer - Default value"]
pub type DV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DVR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Default value"]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Default value"]
    #[inline(always)]
    #[must_use]
    pub fn dv(&mut self) -> DV_W<0> {
        DV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphic MMU default value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvr](index.html) module"]
pub struct DVR_SPEC;
impl crate::RegisterSpec for DVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvr::R](R) reader structure"]
impl crate::Readable for DVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvr::W](W) writer structure"]
impl crate::Writable for DVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DVR to value 0"]
impl crate::Resettable for DVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
