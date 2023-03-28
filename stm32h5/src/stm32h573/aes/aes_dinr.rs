#[doc = "Register `AES_DINR` writer"]
pub struct W(crate::W<AES_DINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_DINR_SPEC>;
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
impl From<crate::W<AES_DINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_DINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIN` writer - Input data word A four-fold sequential write to this bitfield during the input phase results in writing a complete 128-bit block of input data to the AES peripheral. From the first to the fourth write, the corresponding data weights are \\[127:96\\], \\[95:64\\], \\[63:32\\], and \\[31:0\\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\\[1:0\\]
bitfield, then written into the AES core 128-bit input buffer. The data signification of the input data block depends on the AES operating mode: - Mode 1 (encryption): plaintext - Mode 2 (key derivation): the bitfield is not used (AES_KEYRx registers used for input) - Mode 3 (decryption): ciphertext The data swap operation is described in page 1149."]
pub type DIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AES_DINR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Input data word A four-fold sequential write to this bitfield during the input phase results in writing a complete 128-bit block of input data to the AES peripheral. From the first to the fourth write, the corresponding data weights are \\[127:96\\], \\[95:64\\], \\[63:32\\], and \\[31:0\\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\\[1:0\\]
bitfield, then written into the AES core 128-bit input buffer. The data signification of the input data block depends on the AES operating mode: - Mode 1 (encryption): plaintext - Mode 2 (key derivation): the bitfield is not used (AES_KEYRx registers used for input) - Mode 3 (decryption): ciphertext The data swap operation is described in page 1149."]
    #[inline(always)]
    #[must_use]
    pub fn din(&mut self) -> DIN_W<0> {
        DIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES data input register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_dinr](index.html) module"]
pub struct AES_DINR_SPEC;
impl crate::RegisterSpec for AES_DINR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [aes_dinr::W](W) writer structure"]
impl crate::Writable for AES_DINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_DINR to value 0"]
impl crate::Resettable for AES_DINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
