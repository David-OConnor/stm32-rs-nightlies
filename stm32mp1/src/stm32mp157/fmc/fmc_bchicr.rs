#[doc = "Register `FMC_BCHICR` writer"]
pub struct W(crate::W<FMC_BCHICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_BCHICR_SPEC>;
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
impl From<crate::W<FMC_BCHICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_BCHICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDUEF` writer - CDUEF"]
pub type CDUEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_BCHICR_SPEC, bool, O>;
#[doc = "Field `CDERF` writer - CDERF"]
pub type CDERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_BCHICR_SPEC, bool, O>;
#[doc = "Field `CDEFF` writer - CDEFF"]
pub type CDEFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_BCHICR_SPEC, bool, O>;
#[doc = "Field `CDSRF` writer - CDSRF"]
pub type CDSRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_BCHICR_SPEC, bool, O>;
#[doc = "Field `CEPBRF` writer - CEPBRF"]
pub type CEPBRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_BCHICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - CDUEF"]
    #[inline(always)]
    #[must_use]
    pub fn cduef(&mut self) -> CDUEF_W<0> {
        CDUEF_W::new(self)
    }
    #[doc = "Bit 1 - CDERF"]
    #[inline(always)]
    #[must_use]
    pub fn cderf(&mut self) -> CDERF_W<1> {
        CDERF_W::new(self)
    }
    #[doc = "Bit 2 - CDEFF"]
    #[inline(always)]
    #[must_use]
    pub fn cdeff(&mut self) -> CDEFF_W<2> {
        CDEFF_W::new(self)
    }
    #[doc = "Bit 3 - CDSRF"]
    #[inline(always)]
    #[must_use]
    pub fn cdsrf(&mut self) -> CDSRF_W<3> {
        CDSRF_W::new(self)
    }
    #[doc = "Bit 4 - CEPBRF"]
    #[inline(always)]
    #[must_use]
    pub fn cepbrf(&mut self) -> CEPBRF_W<4> {
        CEPBRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC BCH Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchicr](index.html) module"]
pub struct FMC_BCHICR_SPEC;
impl crate::RegisterSpec for FMC_BCHICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fmc_bchicr::W](W) writer structure"]
impl crate::Writable for FMC_BCHICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_BCHICR to value 0"]
impl crate::Resettable for FMC_BCHICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
