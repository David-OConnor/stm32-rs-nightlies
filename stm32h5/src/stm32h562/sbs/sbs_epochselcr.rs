#[doc = "Register `SBS_EPOCHSELCR` reader"]
pub struct R(crate::R<SBS_EPOCHSELCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_EPOCHSELCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_EPOCHSELCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_EPOCHSELCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBS_EPOCHSELCR` writer"]
pub struct W(crate::W<SBS_EPOCHSELCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBS_EPOCHSELCR_SPEC>;
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
impl From<crate::W<SBS_EPOCHSELCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBS_EPOCHSELCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPOCH_SEL` reader - select EPOCH value to be sent to the SAES 1x: EPOCH forced to zero (value used to retrieve PUF reference value at boot time)"]
pub type EPOCH_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPOCH_SEL` writer - select EPOCH value to be sent to the SAES 1x: EPOCH forced to zero (value used to retrieve PUF reference value at boot time)"]
pub type EPOCH_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SBS_EPOCHSELCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - select EPOCH value to be sent to the SAES 1x: EPOCH forced to zero (value used to retrieve PUF reference value at boot time)"]
    #[inline(always)]
    pub fn epoch_sel(&self) -> EPOCH_SEL_R {
        EPOCH_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - select EPOCH value to be sent to the SAES 1x: EPOCH forced to zero (value used to retrieve PUF reference value at boot time)"]
    #[inline(always)]
    #[must_use]
    pub fn epoch_sel(&mut self) -> EPOCH_SEL_W<0> {
        EPOCH_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SBS EPOCH selection control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_epochselcr](index.html) module"]
pub struct SBS_EPOCHSELCR_SPEC;
impl crate::RegisterSpec for SBS_EPOCHSELCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_epochselcr::R](R) reader structure"]
impl crate::Readable for SBS_EPOCHSELCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbs_epochselcr::W](W) writer structure"]
impl crate::Writable for SBS_EPOCHSELCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBS_EPOCHSELCR to value 0"]
impl crate::Resettable for SBS_EPOCHSELCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
