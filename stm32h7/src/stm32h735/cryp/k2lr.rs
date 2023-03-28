#[doc = "Register `K2LR` writer"]
pub struct W(crate::W<K2LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<K2LR_SPEC>;
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
impl From<crate::W<K2LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<K2LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `k` writer - k96"]
pub type K_W<'a, const O: u8> = crate::FieldWriter<'a, u32, K2LR_SPEC, u32, u32, 31, O>;
#[doc = "Field `b121` writer - b121"]
pub type B121_W<'a, const O: u8> = crate::BitWriter<'a, u32, K2LR_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:30 - k96"]
    #[inline(always)]
    #[must_use]
    pub fn k(&mut self) -> K_W<0> {
        K_W::new(self)
    }
    #[doc = "Bit 25 - b121"]
    #[inline(always)]
    #[must_use]
    pub fn b121(&mut self) -> B121_W<25> {
        B121_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k2lr](index.html) module"]
pub struct K2LR_SPEC;
impl crate::RegisterSpec for K2LR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [k2lr::W](W) writer structure"]
impl crate::Writable for K2LR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets K2LR to value 0"]
impl crate::Resettable for K2LR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
