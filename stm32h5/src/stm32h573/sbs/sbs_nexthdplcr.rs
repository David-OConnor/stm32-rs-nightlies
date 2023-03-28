#[doc = "Register `SBS_NEXTHDPLCR` reader"]
pub struct R(crate::R<SBS_NEXTHDPLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_NEXTHDPLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_NEXTHDPLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_NEXTHDPLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBS_NEXTHDPLCR` writer"]
pub struct W(crate::W<SBS_NEXTHDPLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBS_NEXTHDPLCR_SPEC>;
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
impl From<crate::W<SBS_NEXTHDPLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBS_NEXTHDPLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NEXTHDPL` reader - index to point to a higher HDPL than the current one Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details."]
pub type NEXTHDPL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEXTHDPL` writer - index to point to a higher HDPL than the current one Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details."]
pub type NEXTHDPL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SBS_NEXTHDPLCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - index to point to a higher HDPL than the current one Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details."]
    #[inline(always)]
    pub fn nexthdpl(&self) -> NEXTHDPL_R {
        NEXTHDPL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - index to point to a higher HDPL than the current one Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details."]
    #[inline(always)]
    #[must_use]
    pub fn nexthdpl(&mut self) -> NEXTHDPL_W<0> {
        NEXTHDPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SBS next HDPL control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_nexthdplcr](index.html) module"]
pub struct SBS_NEXTHDPLCR_SPEC;
impl crate::RegisterSpec for SBS_NEXTHDPLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_nexthdplcr::R](R) reader structure"]
impl crate::Readable for SBS_NEXTHDPLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbs_nexthdplcr::W](W) writer structure"]
impl crate::Writable for SBS_NEXTHDPLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBS_NEXTHDPLCR to value 0"]
impl crate::Resettable for SBS_NEXTHDPLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
