#[doc = "Register `ETH_TX_LPI_TRAN_CNTR` reader"]
pub struct R(crate::R<ETH_TX_LPI_TRAN_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_TX_LPI_TRAN_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_TX_LPI_TRAN_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_TX_LPI_TRAN_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXLPITRC` reader - Tx LPI Transition counter This field indicates the number of times Tx LPI Entry has occurred. Even if Tx LPI Entry occurs in Automate mode (because of LPITXA bit set in the LPI control and status register (ETH_MACLCSR)), the counter is incremented."]
pub type TXLPITRC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tx LPI Transition counter This field indicates the number of times Tx LPI Entry has occurred. Even if Tx LPI Entry occurs in Automate mode (because of LPITXA bit set in the LPI control and status register (ETH_MACLCSR)), the counter is incremented."]
    #[inline(always)]
    pub fn txlpitrc(&self) -> TXLPITRC_R {
        TXLPITRC_R::new(self.bits)
    }
}
#[doc = "Tx LPI transition counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_tx_lpi_tran_cntr](index.html) module"]
pub struct ETH_TX_LPI_TRAN_CNTR_SPEC;
impl crate::RegisterSpec for ETH_TX_LPI_TRAN_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_tx_lpi_tran_cntr::R](R) reader structure"]
impl crate::Readable for ETH_TX_LPI_TRAN_CNTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_TX_LPI_TRAN_CNTR to value 0"]
impl crate::Resettable for ETH_TX_LPI_TRAN_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
