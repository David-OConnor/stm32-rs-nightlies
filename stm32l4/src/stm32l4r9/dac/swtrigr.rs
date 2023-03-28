#[doc = "Register `SWTRIGR` writer"]
pub struct W(crate::W<SWTRIGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRIGR_SPEC>;
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
impl From<crate::W<SWTRIGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTRIGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWTRIG1` writer - DAC channel1 software trigger"]
pub type SWTRIG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGR_SPEC, bool, O>;
#[doc = "Field `SWTRIG2` writer - DAC channel2 software trigger"]
pub type SWTRIG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DAC channel1 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<0> {
        SWTRIG1_W::new(self)
    }
    #[doc = "Bit 1 - DAC channel2 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<1> {
        SWTRIG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "software trigger register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrigr](index.html) module"]
pub struct SWTRIGR_SPEC;
impl crate::RegisterSpec for SWTRIGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swtrigr::W](W) writer structure"]
impl crate::Writable for SWTRIGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWTRIGR to value 0"]
impl crate::Resettable for SWTRIGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
