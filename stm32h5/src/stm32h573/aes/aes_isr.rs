#[doc = "Register `AES_ISR` reader"]
pub struct R(crate::R<AES_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CCF` reader - Computation complete flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCF bit of the AES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the AES_IER register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1."]
pub type CCF_R = crate::BitReader<bool>;
#[doc = "Field `RWEIF` reader - Read or write error interrupt flag This read-only bit is set by hardware when a RDERR or a WRERR error flag is set in the AES_SR register. RWEIF bit is cleared when application sets the corresponding bit of AES_ICR register. An interrupt is generated if the RWEIE bit has been previously set in the AES_IER register. This flags has no meaning when key derivation mode is selected."]
pub type RWEIF_R = crate::BitReader<bool>;
#[doc = "Field `KEIF` reader - Key error interrupt flag This read-only bit is set by hardware when key information failed to load into key registers. Setting the corresponding bit of the AES_ICR register clears the KEIF and generates interrupt if the KEIE bit of the AES_IER register is set. KEIF is triggered upon any of the following errors: AES_KEYRx register write does not respect the correct order. (For KEYSIZE = 0, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 register, or reverse. For KEYSIZE = 1, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 then AES_KEYR4 then AES_KEYR5 then AES_KEYR6 then AES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set."]
pub type KEIF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Computation complete flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCF bit of the AES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the AES_IER register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1."]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read or write error interrupt flag This read-only bit is set by hardware when a RDERR or a WRERR error flag is set in the AES_SR register. RWEIF bit is cleared when application sets the corresponding bit of AES_ICR register. An interrupt is generated if the RWEIE bit has been previously set in the AES_IER register. This flags has no meaning when key derivation mode is selected."]
    #[inline(always)]
    pub fn rweif(&self) -> RWEIF_R {
        RWEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key error interrupt flag This read-only bit is set by hardware when key information failed to load into key registers. Setting the corresponding bit of the AES_ICR register clears the KEIF and generates interrupt if the KEIE bit of the AES_IER register is set. KEIF is triggered upon any of the following errors: AES_KEYRx register write does not respect the correct order. (For KEYSIZE = 0, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 register, or reverse. For KEYSIZE = 1, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 then AES_KEYR4 then AES_KEYR5 then AES_KEYR6 then AES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set."]
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "AES interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_isr](index.html) module"]
pub struct AES_ISR_SPEC;
impl crate::RegisterSpec for AES_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_isr::R](R) reader structure"]
impl crate::Readable for AES_ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AES_ISR to value 0"]
impl crate::Resettable for AES_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
