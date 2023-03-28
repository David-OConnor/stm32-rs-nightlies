#[doc = "Register `LPUART_RDR` reader"]
pub struct R(crate::R<LPUART_RDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPUART_RDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPUART_RDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPUART_RDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDR` reader - Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see ). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit."]
pub type RDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see ). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit."]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "LPUART receive data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpuart_rdr](index.html) module"]
pub struct LPUART_RDR_SPEC;
impl crate::RegisterSpec for LPUART_RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpuart_rdr::R](R) reader structure"]
impl crate::Readable for LPUART_RDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LPUART_RDR to value 0"]
impl crate::Resettable for LPUART_RDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
