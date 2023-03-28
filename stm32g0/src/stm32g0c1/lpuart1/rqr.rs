#[doc = "Register `RQR` writer"]
pub struct W(crate::W<RQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RQR_SPEC>;
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
impl From<crate::W<RQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RQR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBKRQ` writer - Send break request Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available. Note: If the application needs to send the break character following all previously inserted data, including the ones not yet transmitted, the software should wait for the TXE flag assertion before setting the SBKRQ bit."]
pub type SBKRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, bool, O>;
#[doc = "Field `MMRQ` writer - Mute mode request Writing 1 to this bit puts the LPUART in Mute mode and resets the RWU flag."]
pub type MMRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, bool, O>;
#[doc = "Field `RXFRQ` writer - Receive data flush request Writing 1 to this bit clears the RXNE flag. This enables discarding the received data without reading it, and avoid an overrun condition."]
pub type RXFRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, bool, O>;
#[doc = "Field `TXFRQ` writer - Transmit data flush request This bit is used when FIFO mode is enabled. TXFRQ bit is set to flush the whole FIFO. This sets the flag TXFE (TXFIFO empty, bit 23 in the LPUART_ISR register). Note: In FIFO mode, the TXFNF flag is reset during the flush request until TxFIFO is empty in order to ensure that no data are written in the data register."]
pub type TXFRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 1 - Send break request Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available. Note: If the application needs to send the break character following all previously inserted data, including the ones not yet transmitted, the software should wait for the TXE flag assertion before setting the SBKRQ bit."]
    #[inline(always)]
    #[must_use]
    pub fn sbkrq(&mut self) -> SBKRQ_W<1> {
        SBKRQ_W::new(self)
    }
    #[doc = "Bit 2 - Mute mode request Writing 1 to this bit puts the LPUART in Mute mode and resets the RWU flag."]
    #[inline(always)]
    #[must_use]
    pub fn mmrq(&mut self) -> MMRQ_W<2> {
        MMRQ_W::new(self)
    }
    #[doc = "Bit 3 - Receive data flush request Writing 1 to this bit clears the RXNE flag. This enables discarding the received data without reading it, and avoid an overrun condition."]
    #[inline(always)]
    #[must_use]
    pub fn rxfrq(&mut self) -> RXFRQ_W<3> {
        RXFRQ_W::new(self)
    }
    #[doc = "Bit 4 - Transmit data flush request This bit is used when FIFO mode is enabled. TXFRQ bit is set to flush the whole FIFO. This sets the flag TXFE (TXFIFO empty, bit 23 in the LPUART_ISR register). Note: In FIFO mode, the TXFNF flag is reset during the flush request until TxFIFO is empty in order to ensure that no data are written in the data register."]
    #[inline(always)]
    #[must_use]
    pub fn txfrq(&mut self) -> TXFRQ_W<4> {
        TXFRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART request register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqr](index.html) module"]
pub struct RQR_SPEC;
impl crate::RegisterSpec for RQR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rqr::W](W) writer structure"]
impl crate::Writable for RQR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RQR to value 0"]
impl crate::Resettable for RQR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
