#[doc = "Register `AES_KEYR0` writer"]
pub struct W(crate::W<AES_KEYR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_KEYR0_SPEC>;
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
impl From<crate::W<AES_KEYR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_KEYR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[31:0\\]
This write-only bitfield contains the bits \\[31:0\\]
of the AES encryption or decryption key, depending on the operating mode: - In Mode 1 (encryption), Mode 2 (key derivation): the value to write into the bitfield is the encryption key. - In Mode 3 (decryption): the value to write into the bitfield is the encryption key to be derived before being used for decryption. The AES_KEYRx registers may be written only when KEYSIZE value is correct and when the AES peripheral is disabled (EN bit of the AES_CR register cleared). A special writing sequence is also required, as described in KEYVALID bit of the AES_SR register. Note that, if KMOD\\[1:0\\]
= 10 (shared key), the key is directly loaded from SAES peripheral to AES_KEYRx registers (hence writes to key register is ignored and KEIF is set). Refer to for more details."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AES_KEYR0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[31:0\\]
This write-only bitfield contains the bits \\[31:0\\]
of the AES encryption or decryption key, depending on the operating mode: - In Mode 1 (encryption), Mode 2 (key derivation): the value to write into the bitfield is the encryption key. - In Mode 3 (decryption): the value to write into the bitfield is the encryption key to be derived before being used for decryption. The AES_KEYRx registers may be written only when KEYSIZE value is correct and when the AES peripheral is disabled (EN bit of the AES_CR register cleared). A special writing sequence is also required, as described in KEYVALID bit of the AES_SR register. Note that, if KMOD\\[1:0\\]
= 10 (shared key), the key is directly loaded from SAES peripheral to AES_KEYRx registers (hence writes to key register is ignored and KEIF is set). Refer to for more details."]
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
#[doc = "AES key register 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_keyr0](index.html) module"]
pub struct AES_KEYR0_SPEC;
impl crate::RegisterSpec for AES_KEYR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [aes_keyr0::W](W) writer structure"]
impl crate::Writable for AES_KEYR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_KEYR0 to value 0"]
impl crate::Resettable for AES_KEYR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
