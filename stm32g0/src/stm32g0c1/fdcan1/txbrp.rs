#[doc = "Register `TXBRP` reader"]
pub struct R(crate::R<TXBRP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBRP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBRP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBRP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRP` reader - Transmission request pending Each Tx Buffer has its own transmission request pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been canceled via register TXBCR. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx Buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically canceled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions."]
pub type TRP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Transmission request pending Each Tx Buffer has its own transmission request pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been canceled via register TXBCR. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx Buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically canceled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions."]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 7) as u8)
    }
}
#[doc = "FDCAN Tx buffer request pending register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbrp](index.html) module"]
pub struct TXBRP_SPEC;
impl crate::RegisterSpec for TXBRP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbrp::R](R) reader structure"]
impl crate::Readable for TXBRP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXBRP to value 0"]
impl crate::Resettable for TXBRP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
