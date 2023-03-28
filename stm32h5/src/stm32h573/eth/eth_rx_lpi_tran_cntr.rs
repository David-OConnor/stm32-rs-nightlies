#[doc = "Register `ETH_RX_LPI_TRAN_CNTR` reader"]
pub struct R(crate::R<ETH_RX_LPI_TRAN_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_RX_LPI_TRAN_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_RX_LPI_TRAN_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_RX_LPI_TRAN_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXLPITRC` reader - Rx LPI Transition counter This field indicates the number of times Rx LPI Entry has occurred."]
pub type RXLPITRC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rx LPI Transition counter This field indicates the number of times Rx LPI Entry has occurred."]
    #[inline(always)]
    pub fn rxlpitrc(&self) -> RXLPITRC_R {
        RXLPITRC_R::new(self.bits)
    }
}
#[doc = "Rx LPI transition counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_rx_lpi_tran_cntr](index.html) module"]
pub struct ETH_RX_LPI_TRAN_CNTR_SPEC;
impl crate::RegisterSpec for ETH_RX_LPI_TRAN_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_rx_lpi_tran_cntr::R](R) reader structure"]
impl crate::Readable for ETH_RX_LPI_TRAN_CNTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_RX_LPI_TRAN_CNTR to value 0"]
impl crate::Resettable for ETH_RX_LPI_TRAN_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
