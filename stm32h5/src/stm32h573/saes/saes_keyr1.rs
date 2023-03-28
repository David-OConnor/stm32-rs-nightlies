#[doc = "Register `SAES_KEYR1` writer"]
pub struct W(crate::W<SAES_KEYR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAES_KEYR1_SPEC>;
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
impl From<crate::W<SAES_KEYR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAES_KEYR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[63:32\\]
Refer to the SAES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAES_KEYR1_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[63:32\\]
Refer to the SAES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAES key register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saes_keyr1](index.html) module"]
pub struct SAES_KEYR1_SPEC;
impl crate::RegisterSpec for SAES_KEYR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [saes_keyr1::W](W) writer structure"]
impl crate::Writable for SAES_KEYR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAES_KEYR1 to value 0"]
impl crate::Resettable for SAES_KEYR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
