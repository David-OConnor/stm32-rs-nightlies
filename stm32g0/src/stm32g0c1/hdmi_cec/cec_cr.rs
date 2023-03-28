#[doc = "Register `CEC_CR` reader"]
pub struct R(crate::R<CEC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEC_CR` writer"]
pub struct W(crate::W<CEC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_CR_SPEC>;
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
impl From<crate::W<CEC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CECEN` reader - CEC enable The CECEN bit is set and cleared by software. CECEN = 1 starts message reception and enables the TXSOM control. CECEN = 0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission."]
pub type CECEN_R = crate::BitReader<bool>;
#[doc = "Field `CECEN` writer - CEC enable The CECEN bit is set and cleared by software. CECEN = 1 starts message reception and enables the TXSOM control. CECEN = 0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission."]
pub type CECEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CR_SPEC, bool, O>;
#[doc = "Field `TXSOM` reader - Tx start of message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission starts after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND = 1), in case of transmission underrun (TXUDR = 1), negative acknowledge (TXACKE = 1), and transmission error (TXERR = 1). It is also cleared by CECEN = 0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST = 1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN = 1. TXSOM must be set when transmission data is available into TXDR. HEADER first four bits containing own peripheral address are taken from TXDR\\[7:4\\], not from CEC_CFGR.OAR that is used only for reception."]
pub type TXSOM_R = crate::BitReader<bool>;
#[doc = "Field `TXSOM` writer - Tx start of message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission starts after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND = 1), in case of transmission underrun (TXUDR = 1), negative acknowledge (TXACKE = 1), and transmission error (TXERR = 1). It is also cleared by CECEN = 0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST = 1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN = 1. TXSOM must be set when transmission data is available into TXDR. HEADER first four bits containing own peripheral address are taken from TXDR\\[7:4\\], not from CEC_CFGR.OAR that is used only for reception."]
pub type TXSOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CR_SPEC, bool, O>;
#[doc = "Field `TXEOM` reader - Tx end of message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN = 1. TXEOM must be set before writing transmission data to TXDR. If TXEOM is set when TXSOM = 0, transmitted message consists of 1 byte (HEADER) only (PING message)."]
pub type TXEOM_R = crate::BitReader<bool>;
#[doc = "Field `TXEOM` writer - Tx end of message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN = 1. TXEOM must be set before writing transmission data to TXDR. If TXEOM is set when TXSOM = 0, transmitted message consists of 1 byte (HEADER) only (PING message)."]
pub type TXEOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CEC enable The CECEN bit is set and cleared by software. CECEN = 1 starts message reception and enables the TXSOM control. CECEN = 0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission."]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx start of message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission starts after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND = 1), in case of transmission underrun (TXUDR = 1), negative acknowledge (TXACKE = 1), and transmission error (TXERR = 1). It is also cleared by CECEN = 0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST = 1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN = 1. TXSOM must be set when transmission data is available into TXDR. HEADER first four bits containing own peripheral address are taken from TXDR\\[7:4\\], not from CEC_CFGR.OAR that is used only for reception."]
    #[inline(always)]
    pub fn txsom(&self) -> TXSOM_R {
        TXSOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tx end of message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN = 1. TXEOM must be set before writing transmission data to TXDR. If TXEOM is set when TXSOM = 0, transmitted message consists of 1 byte (HEADER) only (PING message)."]
    #[inline(always)]
    pub fn txeom(&self) -> TXEOM_R {
        TXEOM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CEC enable The CECEN bit is set and cleared by software. CECEN = 1 starts message reception and enables the TXSOM control. CECEN = 0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission."]
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<0> {
        CECEN_W::new(self)
    }
    #[doc = "Bit 1 - Tx start of message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission starts after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND = 1), in case of transmission underrun (TXUDR = 1), negative acknowledge (TXACKE = 1), and transmission error (TXERR = 1). It is also cleared by CECEN = 0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST = 1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN = 1. TXSOM must be set when transmission data is available into TXDR. HEADER first four bits containing own peripheral address are taken from TXDR\\[7:4\\], not from CEC_CFGR.OAR that is used only for reception."]
    #[inline(always)]
    #[must_use]
    pub fn txsom(&mut self) -> TXSOM_W<1> {
        TXSOM_W::new(self)
    }
    #[doc = "Bit 2 - Tx end of message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN = 1. TXEOM must be set before writing transmission data to TXDR. If TXEOM is set when TXSOM = 0, transmitted message consists of 1 byte (HEADER) only (PING message)."]
    #[inline(always)]
    #[must_use]
    pub fn txeom(&mut self) -> TXEOM_W<2> {
        TXEOM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_cr](index.html) module"]
pub struct CEC_CR_SPEC;
impl crate::RegisterSpec for CEC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cec_cr::R](R) reader structure"]
impl crate::Readable for CEC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cec_cr::W](W) writer structure"]
impl crate::Writable for CEC_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CEC_CR to value 0"]
impl crate::Resettable for CEC_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
