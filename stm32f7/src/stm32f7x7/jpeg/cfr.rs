#[doc = "Register `CFR` writer"]
pub struct W(crate::W<CFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFR_SPEC>;
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
impl From<crate::W<CFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEOCF` writer - Clear End of Conversion Flag"]
pub type CEOCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CHPDF` writer - Clear Header Parsing Done Flag"]
pub type CHPDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 5 - Clear End of Conversion Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ceocf(&mut self) -> CEOCF_W<5> {
        CEOCF_W::new(self)
    }
    #[doc = "Bit 6 - Clear Header Parsing Done Flag"]
    #[inline(always)]
    #[must_use]
    pub fn chpdf(&mut self) -> CHPDF_W<6> {
        CHPDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr](index.html) module"]
pub struct CFR_SPEC;
impl crate::RegisterSpec for CFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cfr::W](W) writer structure"]
impl crate::Writable for CFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFR to value 0"]
impl crate::Resettable for CFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
