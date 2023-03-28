#[doc = "Register `FMC_CSQCR` writer"]
pub struct W(crate::W<FMC_CSQCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQCR_SPEC>;
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
impl From<crate::W<FMC_CSQCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSQSTART` writer - CSQSTART"]
pub type CSQSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - CSQSTART"]
    #[inline(always)]
    #[must_use]
    pub fn csqstart(&mut self) -> CSQSTART_W<0> {
        CSQSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC NAND Command Sequencer Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqcr](index.html) module"]
pub struct FMC_CSQCR_SPEC;
impl crate::RegisterSpec for FMC_CSQCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqcr::W](W) writer structure"]
impl crate::Writable for FMC_CSQCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_CSQCR to value 0"]
impl crate::Resettable for FMC_CSQCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
