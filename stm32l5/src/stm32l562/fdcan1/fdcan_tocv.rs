#[doc = "Register `FDCAN_TOCV` reader"]
pub struct R(crate::R<FDCAN_TOCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TOCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TOCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TOCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TOCV` writer"]
pub struct W(crate::W<FDCAN_TOCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TOCV_SPEC>;
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
impl From<crate::W<FDCAN_TOCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TOCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOC` reader - Timeout Counter"]
pub type TOC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOC` writer - Timeout Counter"]
pub type TOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TOCV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timeout Counter"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<0> {
        TOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Timeout Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tocv](index.html) module"]
pub struct FDCAN_TOCV_SPEC;
impl crate::RegisterSpec for FDCAN_TOCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_tocv::R](R) reader structure"]
impl crate::Readable for FDCAN_TOCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_tocv::W](W) writer structure"]
impl crate::Writable for FDCAN_TOCV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TOCV to value 0xffff"]
impl crate::Resettable for FDCAN_TOCV_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
