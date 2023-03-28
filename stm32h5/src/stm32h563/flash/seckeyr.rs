#[doc = "Register `SECKEYR` writer"]
pub struct W(crate::W<SECKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECKEYR_SPEC>;
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
impl From<crate::W<SECKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECKEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECKEY` writer - Non-volatile memory secure configuration access unlock key"]
pub type SECKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECKEYR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Non-volatile memory secure configuration access unlock key"]
    #[inline(always)]
    #[must_use]
    pub fn seckey(&mut self) -> SECKEY_W<0> {
        SECKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH secure key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seckeyr](index.html) module"]
pub struct SECKEYR_SPEC;
impl crate::RegisterSpec for SECKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [seckeyr::W](W) writer structure"]
impl crate::Writable for SECKEYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECKEYR to value 0"]
impl crate::Resettable for SECKEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
