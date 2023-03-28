#[doc = "Register `ETH_MMC_RX_INTERRUPT` reader"]
pub struct R(crate::R<ETH_MMC_RX_INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MMC_RX_INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MMC_RX_INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MMC_RX_INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MMC_RX_INTERRUPT` writer"]
pub struct W(crate::W<ETH_MMC_RX_INTERRUPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MMC_RX_INTERRUPT_SPEC>;
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
impl From<crate::W<ETH_MMC_RX_INTERRUPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MMC_RX_INTERRUPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXCRCERPIS` reader - MMC Receive CRC Error Packet Counter Interrupt Status This bit is set when the Rx CRC error packets register (ETH_RX_CRC_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type RXCRCERPIS_R = crate::BitReader<bool>;
#[doc = "Field `RXCRCERPIS` writer - MMC Receive CRC Error Packet Counter Interrupt Status This bit is set when the Rx CRC error packets register (ETH_RX_CRC_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value."]
pub type RXCRCERPIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_MMC_RX_INTERRUPT_SPEC, bool, O>;
#[doc = "Field `RXALGNERPIS` reader - MMC Receive Alignment Error Packet Counter Interrupt Status This bit is set when the Rx alignment error packets register (ETH_RX_ALIGNMENT_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type RXALGNERPIS_R = crate::BitReader<bool>;
#[doc = "Field `RXALGNERPIS` writer - MMC Receive Alignment Error Packet Counter Interrupt Status This bit is set when the Rx alignment error packets register (ETH_RX_ALIGNMENT_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value."]
pub type RXALGNERPIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_MMC_RX_INTERRUPT_SPEC, bool, O>;
#[doc = "Field `RXUCGPIS` reader - MMC Receive Unicast Good Packet Counter Interrupt Status This bit is set when the Rx unicast packets good register (ETH_RX_UNICAST_PACKETS_GOOD) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type RXUCGPIS_R = crate::BitReader<bool>;
#[doc = "Field `RXUCGPIS` writer - MMC Receive Unicast Good Packet Counter Interrupt Status This bit is set when the Rx unicast packets good register (ETH_RX_UNICAST_PACKETS_GOOD) counter reaches half of the maximum value or the maximum value."]
pub type RXUCGPIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_MMC_RX_INTERRUPT_SPEC, bool, O>;
#[doc = "Field `RXLPIUSCIS` reader - MMC Receive LPI microsecond counter interrupt status This bit is set when the Rx LPI microsecond counter register (ETH_RX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type RXLPIUSCIS_R = crate::BitReader<bool>;
#[doc = "Field `RXLPIUSCIS` writer - MMC Receive LPI microsecond counter interrupt status This bit is set when the Rx LPI microsecond counter register (ETH_RX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value."]
pub type RXLPIUSCIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_MMC_RX_INTERRUPT_SPEC, bool, O>;
#[doc = "Field `RXLPITRCIS` reader - MMC Receive LPI transition counter interrupt status This bit is set when the Rx LPI transition counter register (ETH_RX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type RXLPITRCIS_R = crate::BitReader<bool>;
#[doc = "Field `RXLPITRCIS` writer - MMC Receive LPI transition counter interrupt status This bit is set when the Rx LPI transition counter register (ETH_RX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value."]
pub type RXLPITRCIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_MMC_RX_INTERRUPT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Status This bit is set when the Rx CRC error packets register (ETH_RX_CRC_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxcrcerpis(&self) -> RXCRCERPIS_R {
        RXCRCERPIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Status This bit is set when the Rx alignment error packets register (ETH_RX_ALIGNMENT_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxalgnerpis(&self) -> RXALGNERPIS_R {
        RXALGNERPIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Status This bit is set when the Rx unicast packets good register (ETH_RX_UNICAST_PACKETS_GOOD) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxucgpis(&self) -> RXUCGPIS_R {
        RXUCGPIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Receive LPI microsecond counter interrupt status This bit is set when the Rx LPI microsecond counter register (ETH_RX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxlpiuscis(&self) -> RXLPIUSCIS_R {
        RXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Receive LPI transition counter interrupt status This bit is set when the Rx LPI transition counter register (ETH_RX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxlpitrcis(&self) -> RXLPITRCIS_R {
        RXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Status This bit is set when the Rx CRC error packets register (ETH_RX_CRC_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn rxcrcerpis(&mut self) -> RXCRCERPIS_W<5> {
        RXCRCERPIS_W::new(self)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Status This bit is set when the Rx alignment error packets register (ETH_RX_ALIGNMENT_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn rxalgnerpis(&mut self) -> RXALGNERPIS_W<6> {
        RXALGNERPIS_W::new(self)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Status This bit is set when the Rx unicast packets good register (ETH_RX_UNICAST_PACKETS_GOOD) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn rxucgpis(&mut self) -> RXUCGPIS_W<17> {
        RXUCGPIS_W::new(self)
    }
    #[doc = "Bit 26 - MMC Receive LPI microsecond counter interrupt status This bit is set when the Rx LPI microsecond counter register (ETH_RX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn rxlpiuscis(&mut self) -> RXLPIUSCIS_W<26> {
        RXLPIUSCIS_W::new(self)
    }
    #[doc = "Bit 27 - MMC Receive LPI transition counter interrupt status This bit is set when the Rx LPI transition counter register (ETH_RX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn rxlpitrcis(&mut self) -> RXLPITRCIS_W<27> {
        RXLPITRCIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Rx interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mmc_rx_interrupt](index.html) module"]
pub struct ETH_MMC_RX_INTERRUPT_SPEC;
impl crate::RegisterSpec for ETH_MMC_RX_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mmc_rx_interrupt::R](R) reader structure"]
impl crate::Readable for ETH_MMC_RX_INTERRUPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mmc_rx_interrupt::W](W) writer structure"]
impl crate::Writable for ETH_MMC_RX_INTERRUPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MMC_RX_INTERRUPT to value 0"]
impl crate::Resettable for ETH_MMC_RX_INTERRUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
