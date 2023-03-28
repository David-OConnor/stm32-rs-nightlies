#[doc = "Register `ICFR` writer"]
pub struct W(crate::W<ICFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICFR_SPEC>;
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
impl From<crate::W<ICFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1IF` writer - Clear COMP channel 1 Interrupt Flag"]
pub type CC1IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICFR_SPEC, bool, O>;
#[doc = "Field `CC2IF` writer - Clear COMP channel 2 Interrupt Flag"]
pub type CC2IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICFR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 16 - Clear COMP channel 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<16> {
        CC1IF_W::new(self)
    }
    #[doc = "Bit 17 - Clear COMP channel 2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CC2IF_W<17> {
        CC2IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator interrupt clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfr](index.html) module"]
pub struct ICFR_SPEC;
impl crate::RegisterSpec for ICFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icfr::W](W) writer structure"]
impl crate::Writable for ICFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICFR to value 0"]
impl crate::Resettable for ICFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
