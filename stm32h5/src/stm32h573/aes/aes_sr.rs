#[doc = "Register `AES_SR` reader"]
pub struct R(crate::R<AES_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CCF` reader - Computation completed flag This bit mirrors the CCF bit of the AES_ISR register."]
pub type CCF_R = crate::BitReader<bool>;
#[doc = "Field `RDERR` reader - Read error flag This flag indicates the detection of an unexpected read operation from the AES_DOUTR register (during computation or data input phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the AES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the AES_ICR register. The flag setting has no impact on the AES operation. Unexpected read returns zero."]
pub type RDERR_R = crate::BitReader<bool>;
#[doc = "Field `WRERR` reader - Write error This flag indicates the detection of an unexpected write operation to the AES_DINR register (during computation or data output phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the AES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the AES_ICR register. The flag setting has no impact on the AES operation. Unexpected write is ignored."]
pub type WRERR_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - Busy This flag indicates whether AES is idle or busy during GCM payload encryption phase: When the flag indicates “idle”, the current GCM encryption processing may be suspended to process a higher-priority message. In other chaining modes, or in GCM phases other than payload encryption, the flag must be ignored for the suspend process. The flag is set when transferring a shared key from SAES peripheral."]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `KEYVALID` reader - Key Valid flag This bit is set by hardware when the amount of key information defined by KEYSIZE in AES_CR has been loaded in AES_KEYx key registers. In normal mode when KEYSEL equals to zero, the application must write the key registers in the correct sequence, otherwise the KEIF flag of the AES_ISR register is set and KEYVALID stays at zero. When KEYSEL is different from zero the BUSY flag is automatically set by AES. When key is loaded successfully, the BUSY flag is cleared and KEYVALID set. Upon an error, the KEIF flag of the AES_ISR register is set, the BUSY flag cleared and KEYVALID kept at zero. When the KEIF flag is set, the application must clear it through the AES_ICR register, otherwise KEYVALID cannot be set. See the KEIF bit description for more details. For more information on key loading please refer to ."]
pub type KEYVALID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Computation completed flag This bit mirrors the CCF bit of the AES_ISR register."]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read error flag This flag indicates the detection of an unexpected read operation from the AES_DOUTR register (during computation or data input phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the AES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the AES_ICR register. The flag setting has no impact on the AES operation. Unexpected read returns zero."]
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write error This flag indicates the detection of an unexpected write operation to the AES_DINR register (during computation or data output phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the AES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the AES_ICR register. The flag setting has no impact on the AES operation. Unexpected write is ignored."]
    #[inline(always)]
    pub fn wrerr(&self) -> WRERR_R {
        WRERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy This flag indicates whether AES is idle or busy during GCM payload encryption phase: When the flag indicates “idle”, the current GCM encryption processing may be suspended to process a higher-priority message. In other chaining modes, or in GCM phases other than payload encryption, the flag must be ignored for the suspend process. The flag is set when transferring a shared key from SAES peripheral."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Key Valid flag This bit is set by hardware when the amount of key information defined by KEYSIZE in AES_CR has been loaded in AES_KEYx key registers. In normal mode when KEYSEL equals to zero, the application must write the key registers in the correct sequence, otherwise the KEIF flag of the AES_ISR register is set and KEYVALID stays at zero. When KEYSEL is different from zero the BUSY flag is automatically set by AES. When key is loaded successfully, the BUSY flag is cleared and KEYVALID set. Upon an error, the KEIF flag of the AES_ISR register is set, the BUSY flag cleared and KEYVALID kept at zero. When the KEIF flag is set, the application must clear it through the AES_ICR register, otherwise KEYVALID cannot be set. See the KEIF bit description for more details. For more information on key loading please refer to ."]
    #[inline(always)]
    pub fn keyvalid(&self) -> KEYVALID_R {
        KEYVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "AES status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_sr](index.html) module"]
pub struct AES_SR_SPEC;
impl crate::RegisterSpec for AES_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_sr::R](R) reader structure"]
impl crate::Readable for AES_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AES_SR to value 0"]
impl crate::Resettable for AES_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
