#[doc = "Register `FMC_CSQICR` writer"]
pub struct W(crate::W<FMC_CSQICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQICR_SPEC>;
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
impl From<crate::W<FMC_CSQICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTCF` writer - CTCF"]
pub type CTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQICR_SPEC, bool, O>;
#[doc = "Field `CSCF` writer - CSCF"]
pub type CSCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQICR_SPEC, bool, O>;
#[doc = "Field `CSEF` writer - CSEF"]
pub type CSEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQICR_SPEC, bool, O>;
#[doc = "Field `CSUEF` writer - CSUEF"]
pub type CSUEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQICR_SPEC, bool, O>;
#[doc = "Field `CCMDTCF` writer - CCMDTCF"]
pub type CCMDTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - CTCF"]
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<0> {
        CTCF_W::new(self)
    }
    #[doc = "Bit 1 - CSCF"]
    #[inline(always)]
    #[must_use]
    pub fn cscf(&mut self) -> CSCF_W<1> {
        CSCF_W::new(self)
    }
    #[doc = "Bit 2 - CSEF"]
    #[inline(always)]
    #[must_use]
    pub fn csef(&mut self) -> CSEF_W<2> {
        CSEF_W::new(self)
    }
    #[doc = "Bit 3 - CSUEF"]
    #[inline(always)]
    #[must_use]
    pub fn csuef(&mut self) -> CSUEF_W<3> {
        CSUEF_W::new(self)
    }
    #[doc = "Bit 4 - CCMDTCF"]
    #[inline(always)]
    #[must_use]
    pub fn ccmdtcf(&mut self) -> CCMDTCF_W<4> {
        CCMDTCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC NAND Command Sequencer Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqicr](index.html) module"]
pub struct FMC_CSQICR_SPEC;
impl crate::RegisterSpec for FMC_CSQICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqicr::W](W) writer structure"]
impl crate::Writable for FMC_CSQICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_CSQICR to value 0"]
impl crate::Resettable for FMC_CSQICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
