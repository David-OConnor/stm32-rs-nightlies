#[doc = "Register `OTFDEC_CR` reader"]
pub struct R(crate::R<OTFDEC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTFDEC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTFDEC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTFDEC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTFDEC_CR` writer"]
pub struct W(crate::W<OTFDEC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTFDEC_CR_SPEC>;
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
impl From<crate::W<OTFDEC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTFDEC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENC` reader - Encryption mode bit When this bit is set, OTFDEC is used in encryption mode, during which application can write clear text data then read back encrypted data. When this bit is cleared (default), OTFDEC is used in decryption mode, during which application only read back decrypted data. For both modes, cryptographic context (keys, nonces, firmware versions) must be properly initialized. When this bit is set, only data accesses are allowed (zeros are returned otherwise, and XONEIF is set). When MODE = 11, enhanced encryption mode is automatically selected. Note: When ENC bit is set, no access to OCTOSPI must be done (registers and Memory‑mapped region)."]
pub type ENC_R = crate::BitReader<bool>;
#[doc = "Field `ENC` writer - Encryption mode bit When this bit is set, OTFDEC is used in encryption mode, during which application can write clear text data then read back encrypted data. When this bit is cleared (default), OTFDEC is used in decryption mode, during which application only read back decrypted data. For both modes, cryptographic context (keys, nonces, firmware versions) must be properly initialized. When this bit is set, only data accesses are allowed (zeros are returned otherwise, and XONEIF is set). When MODE = 11, enhanced encryption mode is automatically selected. Note: When ENC bit is set, no access to OCTOSPI must be done (registers and Memory‑mapped region)."]
pub type ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTFDEC_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Encryption mode bit When this bit is set, OTFDEC is used in encryption mode, during which application can write clear text data then read back encrypted data. When this bit is cleared (default), OTFDEC is used in decryption mode, during which application only read back decrypted data. For both modes, cryptographic context (keys, nonces, firmware versions) must be properly initialized. When this bit is set, only data accesses are allowed (zeros are returned otherwise, and XONEIF is set). When MODE = 11, enhanced encryption mode is automatically selected. Note: When ENC bit is set, no access to OCTOSPI must be done (registers and Memory‑mapped region)."]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Encryption mode bit When this bit is set, OTFDEC is used in encryption mode, during which application can write clear text data then read back encrypted data. When this bit is cleared (default), OTFDEC is used in decryption mode, during which application only read back decrypted data. For both modes, cryptographic context (keys, nonces, firmware versions) must be properly initialized. When this bit is set, only data accesses are allowed (zeros are returned otherwise, and XONEIF is set). When MODE = 11, enhanced encryption mode is automatically selected. Note: When ENC bit is set, no access to OCTOSPI must be done (registers and Memory‑mapped region)."]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<0> {
        ENC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otfdec_cr](index.html) module"]
pub struct OTFDEC_CR_SPEC;
impl crate::RegisterSpec for OTFDEC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otfdec_cr::R](R) reader structure"]
impl crate::Readable for OTFDEC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otfdec_cr::W](W) writer structure"]
impl crate::Writable for OTFDEC_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTFDEC_CR to value 0"]
impl crate::Resettable for OTFDEC_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
