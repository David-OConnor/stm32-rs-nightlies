#[doc = "Register `CRYP_DIN` reader"]
pub struct R(crate::R<CRYP_DIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_DIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_DIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_DIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYP_DIN` writer"]
pub struct W(crate::W<CRYP_DIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_DIN_SPEC>;
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
impl From<crate::W<CRYP_DIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_DIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAIN` reader - DATAIN"]
pub type DATAIN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATAIN` writer - DATAIN"]
pub type DATAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRYP_DIN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DATAIN"]
    #[inline(always)]
    pub fn datain(&self) -> DATAIN_R {
        DATAIN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DATAIN"]
    #[inline(always)]
    #[must_use]
    pub fn datain(&mut self) -> DATAIN_W<0> {
        DATAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_din](index.html) module"]
pub struct CRYP_DIN_SPEC;
impl crate::RegisterSpec for CRYP_DIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_din::R](R) reader structure"]
impl crate::Readable for CRYP_DIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cryp_din::W](W) writer structure"]
impl crate::Writable for CRYP_DIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYP_DIN to value 0"]
impl crate::Resettable for CRYP_DIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
