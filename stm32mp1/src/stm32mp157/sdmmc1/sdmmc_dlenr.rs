#[doc = "Register `SDMMC_DLENR` reader"]
pub struct R(crate::R<SDMMC_DLENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_DLENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_DLENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_DLENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_DLENR` writer"]
pub struct W(crate::W<SDMMC_DLENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_DLENR_SPEC>;
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
impl From<crate::W<SDMMC_DLENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_DLENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATALENGTH` reader - DATALENGTH"]
pub type DATALENGTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATALENGTH` writer - DATALENGTH"]
pub type DATALENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDMMC_DLENR_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bits 0:24 - DATALENGTH"]
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - DATALENGTH"]
    #[inline(always)]
    #[must_use]
    pub fn datalength(&mut self) -> DATALENGTH_W<0> {
        DATALENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_dlenr](index.html) module"]
pub struct SDMMC_DLENR_SPEC;
impl crate::RegisterSpec for SDMMC_DLENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_dlenr::R](R) reader structure"]
impl crate::Readable for SDMMC_DLENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_dlenr::W](W) writer structure"]
impl crate::Writable for SDMMC_DLENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMMC_DLENR to value 0"]
impl crate::Resettable for SDMMC_DLENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
