#[doc = "Register `AES_KEYR4` writer"]
pub struct W(crate::W<AES_KEYR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_KEYR4_SPEC>;
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
impl From<crate::W<AES_KEYR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_KEYR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[159:128\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AES_KEYR4_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[159:128\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
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
#[doc = "AES key register 4\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_keyr4](index.html) module"]
pub struct AES_KEYR4_SPEC;
impl crate::RegisterSpec for AES_KEYR4_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [aes_keyr4::W](W) writer structure"]
impl crate::Writable for AES_KEYR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_KEYR4 to value 0"]
impl crate::Resettable for AES_KEYR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
