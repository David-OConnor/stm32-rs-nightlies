#[doc = "Register `AES_DOUTR` reader"]
pub struct R(crate::R<AES_DOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_DOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_DOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_DOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOUT` reader - Output data word This read-only bitfield fetches a 32-bit output buffer. A four-fold sequential read of this bitfield, upon the computation completion (CCF set), virtually reads a complete 128-bit block of output data from the AES peripheral. Before reaching the output buffer, the data produced by the AES core are handled by the data swap block according to the DATATYPE\\[1:0\\]
bitfield. Data weights from the first to the fourth read operation are: \\[127:96\\], \\[95:64\\], \\[63:32\\], and \\[31:0\\]. The data signification of the output data block depends on the AES operating mode: - Mode 1 (encryption): ciphertext - Mode 2 (key derivation): the bitfield is not used - Mode 3 (decryption): plaintext The data swap operation is described in page 1149."]
pub type DOUT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output data word This read-only bitfield fetches a 32-bit output buffer. A four-fold sequential read of this bitfield, upon the computation completion (CCF set), virtually reads a complete 128-bit block of output data from the AES peripheral. Before reaching the output buffer, the data produced by the AES core are handled by the data swap block according to the DATATYPE\\[1:0\\]
bitfield. Data weights from the first to the fourth read operation are: \\[127:96\\], \\[95:64\\], \\[63:32\\], and \\[31:0\\]. The data signification of the output data block depends on the AES operating mode: - Mode 1 (encryption): ciphertext - Mode 2 (key derivation): the bitfield is not used - Mode 3 (decryption): plaintext The data swap operation is described in page 1149."]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new(self.bits)
    }
}
#[doc = "AES data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_doutr](index.html) module"]
pub struct AES_DOUTR_SPEC;
impl crate::RegisterSpec for AES_DOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_doutr::R](R) reader structure"]
impl crate::Readable for AES_DOUTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AES_DOUTR to value 0"]
impl crate::Resettable for AES_DOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
