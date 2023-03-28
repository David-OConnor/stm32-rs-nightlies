#[doc = "Register `ETH_RX_LPI_USEC_CNTR` reader"]
pub struct R(crate::R<ETH_RX_LPI_USEC_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_RX_LPI_USEC_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_RX_LPI_USEC_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_RX_LPI_USEC_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXLPIUSC` reader - Rx LPI Microseconds Counter This field indicates the number of microseconds Rx LPI is asserted. For every Rx LPI Entry and Exit, the Timer value can have an error of +/- 1 microsecond."]
pub type RXLPIUSC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rx LPI Microseconds Counter This field indicates the number of microseconds Rx LPI is asserted. For every Rx LPI Entry and Exit, the Timer value can have an error of +/- 1 microsecond."]
    #[inline(always)]
    pub fn rxlpiusc(&self) -> RXLPIUSC_R {
        RXLPIUSC_R::new(self.bits)
    }
}
#[doc = "Rx LPI microsecond counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_rx_lpi_usec_cntr](index.html) module"]
pub struct ETH_RX_LPI_USEC_CNTR_SPEC;
impl crate::RegisterSpec for ETH_RX_LPI_USEC_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_rx_lpi_usec_cntr::R](R) reader structure"]
impl crate::Readable for ETH_RX_LPI_USEC_CNTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_RX_LPI_USEC_CNTR to value 0"]
impl crate::Resettable for ETH_RX_LPI_USEC_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
