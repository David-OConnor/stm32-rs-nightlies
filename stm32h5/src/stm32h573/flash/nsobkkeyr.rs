#[doc = "Register `NSOBKKEYR` writer"]
pub struct W(crate::W<NSOBKKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSOBKKEYR_SPEC>;
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
impl From<crate::W<NSOBKKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSOBKKEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSOBKKEY` writer - FLASH non-secure option bytes keys control access unlock key"]
pub type NSOBKKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NSOBKKEYR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - FLASH non-secure option bytes keys control access unlock key"]
    #[inline(always)]
    #[must_use]
    pub fn nsobkkey(&mut self) -> NSOBKKEY_W<0> {
        NSOBKKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH non-secure OBK key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsobkkeyr](index.html) module"]
pub struct NSOBKKEYR_SPEC;
impl crate::RegisterSpec for NSOBKKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [nsobkkeyr::W](W) writer structure"]
impl crate::Writable for NSOBKKEYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSOBKKEYR to value 0"]
impl crate::Resettable for NSOBKKEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
