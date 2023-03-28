#[doc = "Register `SAI_GCR` reader"]
pub struct R(crate::R<SAI_GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_GCR` writer"]
pub struct W(crate::W<SAI_GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_GCR_SPEC>;
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
impl From<crate::W<SAI_GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCIN` reader - SYNCIN"]
pub type SYNCIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCIN` writer - SYNCIN"]
pub type SYNCIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_GCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SYNCOUT` reader - SYNCOUT"]
pub type SYNCOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCOUT` writer - SYNCOUT"]
pub type SYNCOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_GCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - SYNCIN"]
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - SYNCOUT"]
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SYNCIN"]
    #[inline(always)]
    #[must_use]
    pub fn syncin(&mut self) -> SYNCIN_W<0> {
        SYNCIN_W::new(self)
    }
    #[doc = "Bits 4:5 - SYNCOUT"]
    #[inline(always)]
    #[must_use]
    pub fn syncout(&mut self) -> SYNCOUT_W<4> {
        SYNCOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_gcr](index.html) module"]
pub struct SAI_GCR_SPEC;
impl crate::RegisterSpec for SAI_GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_gcr::R](R) reader structure"]
impl crate::Readable for SAI_GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_gcr::W](W) writer structure"]
impl crate::Writable for SAI_GCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_GCR to value 0"]
impl crate::Resettable for SAI_GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
