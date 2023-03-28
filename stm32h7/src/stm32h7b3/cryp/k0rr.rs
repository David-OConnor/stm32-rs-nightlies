#[doc = "Register `K0RR` writer"]
pub struct W(crate::W<K0RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<K0RR_SPEC>;
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
impl From<crate::W<K0RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<K0RR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `b` writer - b192"]
pub type B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, K0RR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - b192"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<0> {
        B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k0rr](index.html) module"]
pub struct K0RR_SPEC;
impl crate::RegisterSpec for K0RR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [k0rr::W](W) writer structure"]
impl crate::Writable for K0RR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets K0RR to value 0"]
impl crate::Resettable for K0RR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
