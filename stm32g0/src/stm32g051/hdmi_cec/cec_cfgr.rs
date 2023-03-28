#[doc = "Register `CEC_CFGR` reader"]
pub struct R(crate::R<CEC_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEC_CFGR` writer"]
pub struct W(crate::W<CEC_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_CFGR_SPEC>;
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
impl From<crate::W<CEC_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFT` reader - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)"]
pub type SFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFT` writer - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)"]
pub type SFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEC_CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RXTOL` reader - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 µs rise, +/- 200 µs fall Data-bit: +/- 200 µs rise. +/- 350 µs fall Start-bit: +/- 400 µs rise, +/- 400 µs fall Data-bit: +/-300 µs rise, +/- 500 µs fall"]
pub type RXTOL_R = crate::BitReader<bool>;
#[doc = "Field `RXTOL` writer - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 µs rise, +/- 200 µs fall Data-bit: +/- 200 µs rise. +/- 350 µs fall Start-bit: +/- 400 µs rise, +/- 400 µs fall Data-bit: +/-300 µs rise, +/- 500 µs fall"]
pub type RXTOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `BRESTP` reader - Rx-stop on bit rising error The BRESTP bit is set and cleared by software."]
pub type BRESTP_R = crate::BitReader<bool>;
#[doc = "Field `BRESTP` writer - Rx-stop on bit rising error The BRESTP bit is set and cleared by software."]
pub type BRESTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `BREGEN` reader - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0."]
pub type BREGEN_R = crate::BitReader<bool>;
#[doc = "Field `BREGEN` writer - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0."]
pub type BREGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `LBPEGEN` reader - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0."]
pub type LBPEGEN_R = crate::BitReader<bool>;
#[doc = "Field `LBPEGEN` writer - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0."]
pub type LBPEGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `BRDNOGEN` reader - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line."]
pub type BRDNOGEN_R = crate::BitReader<bool>;
#[doc = "Field `BRDNOGEN` writer - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line."]
pub type BRDNOGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `SFTOP` reader - SFT option bit The SFTOPT bit is set and cleared by software."]
pub type SFTOP_R = crate::BitReader<bool>;
#[doc = "Field `SFTOP` writer - SFT option bit The SFTOPT bit is set and cleared by software."]
pub type SFTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `OAR` reader - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received."]
pub type OAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OAR` writer - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received."]
pub type OAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEC_CFGR_SPEC, u16, u16, 15, O>;
#[doc = "Field `LSTN` reader - Listen mode LSTN bit is set and cleared by software."]
pub type LSTN_R = crate::BitReader<bool>;
#[doc = "Field `LSTN` writer - Listen mode LSTN bit is set and cleared by software."]
pub type LSTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 µs rise, +/- 200 µs fall Data-bit: +/- 200 µs rise. +/- 350 µs fall Start-bit: +/- 400 µs rise, +/- 400 µs fall Data-bit: +/-300 µs rise, +/- 500 µs fall"]
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx-stop on bit rising error The BRESTP bit is set and cleared by software."]
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0."]
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0."]
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line."]
    #[inline(always)]
    pub fn brdnogen(&self) -> BRDNOGEN_R {
        BRDNOGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SFT option bit The SFTOPT bit is set and cleared by software."]
    #[inline(always)]
    pub fn sftop(&self) -> SFTOP_R {
        SFTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:30 - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received."]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Listen mode LSTN bit is set and cleared by software."]
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn sft(&mut self) -> SFT_W<0> {
        SFT_W::new(self)
    }
    #[doc = "Bit 3 - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 µs rise, +/- 200 µs fall Data-bit: +/- 200 µs rise. +/- 350 µs fall Start-bit: +/- 400 µs rise, +/- 400 µs fall Data-bit: +/-300 µs rise, +/- 500 µs fall"]
    #[inline(always)]
    #[must_use]
    pub fn rxtol(&mut self) -> RXTOL_W<3> {
        RXTOL_W::new(self)
    }
    #[doc = "Bit 4 - Rx-stop on bit rising error The BRESTP bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn brestp(&mut self) -> BRESTP_W<4> {
        BRESTP_W::new(self)
    }
    #[doc = "Bit 5 - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0."]
    #[inline(always)]
    #[must_use]
    pub fn bregen(&mut self) -> BREGEN_W<5> {
        BREGEN_W::new(self)
    }
    #[doc = "Bit 6 - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0."]
    #[inline(always)]
    #[must_use]
    pub fn lbpegen(&mut self) -> LBPEGEN_W<6> {
        LBPEGEN_W::new(self)
    }
    #[doc = "Bit 7 - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line."]
    #[inline(always)]
    #[must_use]
    pub fn brdnogen(&mut self) -> BRDNOGEN_W<7> {
        BRDNOGEN_W::new(self)
    }
    #[doc = "Bit 8 - SFT option bit The SFTOPT bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sftop(&mut self) -> SFTOP_W<8> {
        SFTOP_W::new(self)
    }
    #[doc = "Bits 16:30 - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received."]
    #[inline(always)]
    #[must_use]
    pub fn oar(&mut self) -> OAR_W<16> {
        OAR_W::new(self)
    }
    #[doc = "Bit 31 - Listen mode LSTN bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lstn(&mut self) -> LSTN_W<31> {
        LSTN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the HDMI-CEC controller. It is mandatory to write CEC_CFGR only when CECEN=0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_cfgr](index.html) module"]
pub struct CEC_CFGR_SPEC;
impl crate::RegisterSpec for CEC_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cec_cfgr::R](R) reader structure"]
impl crate::Readable for CEC_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cec_cfgr::W](W) writer structure"]
impl crate::Writable for CEC_CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CEC_CFGR to value 0"]
impl crate::Resettable for CEC_CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
