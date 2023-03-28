#[doc = "Register `K1LR` writer"]
pub struct W(crate::W<K1LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<K1LR_SPEC>;
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
impl From<crate::W<K1LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<K1LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `b1` writer - b160"]
pub type B1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, K1LR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - b160"]
    #[inline(always)]
    #[must_use]
    pub fn b1(&mut self) -> B1_W<0> {
        B1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k1lr](index.html) module"]
pub struct K1LR_SPEC;
impl crate::RegisterSpec for K1LR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [k1lr::W](W) writer structure"]
impl crate::Writable for K1LR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets K1LR to value 0"]
impl crate::Resettable for K1LR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
