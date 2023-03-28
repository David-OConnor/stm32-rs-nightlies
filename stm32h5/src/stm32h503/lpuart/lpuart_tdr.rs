#[doc = "Register `LPUART_TDR` reader"]
pub struct R(crate::R<LPUART_TDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPUART_TDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPUART_TDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPUART_TDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPUART_TDR` writer"]
pub struct W(crate::W<LPUART_TDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPUART_TDR_SPEC>;
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
impl From<crate::W<LPUART_TDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPUART_TDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDR` reader - Transmit data value Contains the data character to be transmitted. The TDR register provides the parallel interface between the internal bus and the output shift register (see ). When transmitting with the parity enabled (PCE bit set to 1 in the LPUART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF=1."]
pub type TDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TDR` writer - Transmit data value Contains the data character to be transmitted. The TDR register provides the parallel interface between the internal bus and the output shift register (see ). When transmitting with the parity enabled (PCE bit set to 1 in the LPUART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF=1."]
pub type TDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPUART_TDR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Transmit data value Contains the data character to be transmitted. The TDR register provides the parallel interface between the internal bus and the output shift register (see ). When transmitting with the parity enabled (PCE bit set to 1 in the LPUART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF=1."]
    #[inline(always)]
    pub fn tdr(&self) -> TDR_R {
        TDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit data value Contains the data character to be transmitted. The TDR register provides the parallel interface between the internal bus and the output shift register (see ). When transmitting with the parity enabled (PCE bit set to 1 in the LPUART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF=1."]
    #[inline(always)]
    #[must_use]
    pub fn tdr(&mut self) -> TDR_W<0> {
        TDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART transmit data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpuart_tdr](index.html) module"]
pub struct LPUART_TDR_SPEC;
impl crate::RegisterSpec for LPUART_TDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpuart_tdr::R](R) reader structure"]
impl crate::Readable for LPUART_TDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpuart_tdr::W](W) writer structure"]
impl crate::Writable for LPUART_TDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPUART_TDR to value 0"]
impl crate::Resettable for LPUART_TDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
