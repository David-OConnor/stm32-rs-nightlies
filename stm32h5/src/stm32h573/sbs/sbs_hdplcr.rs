#[doc = "Register `SBS_HDPLCR` reader"]
pub struct R(crate::R<SBS_HDPLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_HDPLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_HDPLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_HDPLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBS_HDPLCR` writer"]
pub struct W(crate::W<SBS_HDPLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBS_HDPLCR_SPEC>;
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
impl From<crate::W<SBS_HDPLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBS_HDPLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INCR_HDPL` reader - increment HDPL value Other: all other values allow a HDPL level increment."]
pub type INCR_HDPL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INCR_HDPL` writer - increment HDPL value Other: all other values allow a HDPL level increment."]
pub type INCR_HDPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SBS_HDPLCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - increment HDPL value Other: all other values allow a HDPL level increment."]
    #[inline(always)]
    pub fn incr_hdpl(&self) -> INCR_HDPL_R {
        INCR_HDPL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - increment HDPL value Other: all other values allow a HDPL level increment."]
    #[inline(always)]
    #[must_use]
    pub fn incr_hdpl(&mut self) -> INCR_HDPL_W<0> {
        INCR_HDPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SBS temporal isolation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_hdplcr](index.html) module"]
pub struct SBS_HDPLCR_SPEC;
impl crate::RegisterSpec for SBS_HDPLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_hdplcr::R](R) reader structure"]
impl crate::Readable for SBS_HDPLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbs_hdplcr::W](W) writer structure"]
impl crate::Writable for SBS_HDPLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBS_HDPLCR to value 0xb4"]
impl crate::Resettable for SBS_HDPLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xb4;
}
