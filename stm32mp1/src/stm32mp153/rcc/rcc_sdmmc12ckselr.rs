#[doc = "Register `RCC_SDMMC12CKSELR` reader"]
pub struct R(crate::R<RCC_SDMMC12CKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_SDMMC12CKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_SDMMC12CKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_SDMMC12CKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_SDMMC12CKSELR` writer"]
pub struct W(crate::W<RCC_SDMMC12CKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_SDMMC12CKSELR_SPEC>;
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
impl From<crate::W<RCC_SDMMC12CKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_SDMMC12CKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDMMC12SRC` reader - SDMMC12SRC"]
pub type SDMMC12SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDMMC12SRC` writer - SDMMC12SRC"]
pub type SDMMC12SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_SDMMC12CKSELR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - SDMMC12SRC"]
    #[inline(always)]
    pub fn sdmmc12src(&self) -> SDMMC12SRC_R {
        SDMMC12SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDMMC12SRC"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc12src(&mut self) -> SDMMC12SRC_W<0> {
        SDMMC12SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_sdmmc12ckselr](index.html) module"]
pub struct RCC_SDMMC12CKSELR_SPEC;
impl crate::RegisterSpec for RCC_SDMMC12CKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_sdmmc12ckselr::R](R) reader structure"]
impl crate::Readable for RCC_SDMMC12CKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_sdmmc12ckselr::W](W) writer structure"]
impl crate::Writable for RCC_SDMMC12CKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_SDMMC12CKSELR to value 0x03"]
impl crate::Resettable for RCC_SDMMC12CKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
