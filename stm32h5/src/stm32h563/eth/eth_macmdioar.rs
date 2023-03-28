#[doc = "Register `ETH_MACMDIOAR` reader"]
pub struct R(crate::R<ETH_MACMDIOAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACMDIOAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACMDIOAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACMDIOAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACMDIOAR` writer"]
pub struct W(crate::W<ETH_MACMDIOAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACMDIOAR_SPEC>;
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
impl From<crate::W<ETH_MACMDIOAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACMDIOAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MB` reader - MII Busy The application sets this bit to instruct the SMA to initiate a Read or Write access to the MDIOS. The MAC clears this bit after the MDIO frame transfer is completed. Hence the software must not write or change any of the fields in MDIO address register (ETH_MACMDIOAR) and MDIO data register (ETH_MACMDIODR) as long as this bit is set. For write transfers, the application must first write 16-bit data in the MD field (and also RA field when C45E is set) in MDIO data register (ETH_MACMDIODR) register before setting this bit. When C45E is set, it should also write into the RA field of MDIO data register (ETH_MACMDIODR) before initiating a read transfer. When a read transfer is completed (MII busy=0), the data read from the PHY register is valid in the MD field of the MDIO data register (ETH_MACMDIODR). Note: Even if the addressed PHY is not present, there is no change in the functionality of this bit."]
pub type MB_R = crate::BitReader<bool>;
#[doc = "Field `MB` writer - MII Busy The application sets this bit to instruct the SMA to initiate a Read or Write access to the MDIOS. The MAC clears this bit after the MDIO frame transfer is completed. Hence the software must not write or change any of the fields in MDIO address register (ETH_MACMDIOAR) and MDIO data register (ETH_MACMDIODR) as long as this bit is set. For write transfers, the application must first write 16-bit data in the MD field (and also RA field when C45E is set) in MDIO data register (ETH_MACMDIODR) register before setting this bit. When C45E is set, it should also write into the RA field of MDIO data register (ETH_MACMDIODR) before initiating a read transfer. When a read transfer is completed (MII busy=0), the data read from the PHY register is valid in the MD field of the MDIO data register (ETH_MACMDIODR). Note: Even if the addressed PHY is not present, there is no change in the functionality of this bit."]
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACMDIOAR_SPEC, bool, O>;
#[doc = "Field `C45E` reader - Clause 45 PHY Enable When this bit is set, Clause 45 capable PHY is connected to MDIO. When this bit is reset, Clause 22 capable PHY is connected to MDIO."]
pub type C45E_R = crate::BitReader<bool>;
#[doc = "Field `C45E` writer - Clause 45 PHY Enable When this bit is set, Clause 45 capable PHY is connected to MDIO. When this bit is reset, Clause 22 capable PHY is connected to MDIO."]
pub type C45E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACMDIOAR_SPEC, bool, O>;
#[doc = "Field `GOC` reader - MII Operation Command This bit indicates the operation command to the PHY. When Clause 22 PHY is enabled, only Write and Read commands are valid."]
pub type GOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GOC` writer - MII Operation Command This bit indicates the operation command to the PHY. When Clause 22 PHY is enabled, only Write and Read commands are valid."]
pub type GOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACMDIOAR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SKAP` reader - Skip Address Packet When this bit is set, the SMA does not send the address packets before read, write, or post-read increment address packets. This bit is valid only when C45E is set."]
pub type SKAP_R = crate::BitReader<bool>;
#[doc = "Field `SKAP` writer - Skip Address Packet When this bit is set, the SMA does not send the address packets before read, write, or post-read increment address packets. This bit is valid only when C45E is set."]
pub type SKAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACMDIOAR_SPEC, bool, O>;
#[doc = "Field `CR` reader - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design: 0110 to 0111: Reserved, must not be used The suggested range of CSR clock frequency applicable for each value (when Bit 11 = 0) ensures that the MDC clock is approximately between 1.0 MHz to 2.5 MHz frequency range. When Bit 11 is set, you can achieve a higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits to 1010, the resultant MDC clock is of 12.5 MHz which is above the range specified in IEEE 802.3. Program the following values only if the interfacing chips support faster MDC clocks:"]
pub type CR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CR` writer - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design: 0110 to 0111: Reserved, must not be used The suggested range of CSR clock frequency applicable for each value (when Bit 11 = 0) ensures that the MDC clock is approximately between 1.0 MHz to 2.5 MHz frequency range. When Bit 11 is set, you can achieve a higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits to 1010, the resultant MDC clock is of 12.5 MHz which is above the range specified in IEEE 802.3. Program the following values only if the interfacing chips support faster MDC clocks:"]
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACMDIOAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `NTC` reader - Number of Training Clocks This field controls the number of trailing clock cycles generated on ETH_MDC after the end of transmission of MDIO frame. The valid values can be from 0 to 7. Programming the value to 011 indicates that there are additional three clock cycles on the MDC line after the end of MDIO frame transfer."]
pub type NTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NTC` writer - Number of Training Clocks This field controls the number of trailing clock cycles generated on ETH_MDC after the end of transmission of MDIO frame. The valid values can be from 0 to 7. Programming the value to 011 indicates that there are additional three clock cycles on the MDC line after the end of MDIO frame transfer."]
pub type NTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACMDIOAR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RDA` reader - Register/Device Address These bits select the PHY register in selected Clause 22 PHY device. These bits select the Device (MMD) in selected Clause 45 capable PHY."]
pub type RDA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDA` writer - Register/Device Address These bits select the PHY register in selected Clause 22 PHY device. These bits select the Device (MMD) in selected Clause 45 capable PHY."]
pub type RDA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACMDIOAR_SPEC, u8, u8, 5, O>;
#[doc = "Field `PA` reader - Physical Layer Address This field indicates which Clause 22 PHY devices (out of 32 devices) the MAC is accessing. This field indicates which Clause 45 capable PHYs (out of 32 PHYs) the MAC is accessing."]
pub type PA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA` writer - Physical Layer Address This field indicates which Clause 22 PHY devices (out of 32 devices) the MAC is accessing. This field indicates which Clause 45 capable PHYs (out of 32 PHYs) the MAC is accessing."]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACMDIOAR_SPEC, u8, u8, 5, O>;
#[doc = "Field `BTB` reader - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC informs the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted). The software can thus initiate the next command which is executed immediately irrespective of the number trailing clocks generated for the previous frame. When this bit is reset, then the read/write command completion (MII busy is cleared) only after the trailing clocks are generated. In this mode, it is ensured that the NTC is always generated after each frame. This bit must not be set when NTC=0."]
pub type BTB_R = crate::BitReader<bool>;
#[doc = "Field `BTB` writer - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC informs the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted). The software can thus initiate the next command which is executed immediately irrespective of the number trailing clocks generated for the previous frame. When this bit is reset, then the read/write command completion (MII busy is cleared) only after the trailing clocks are generated. In this mode, it is ensured that the NTC is always generated after each frame. This bit must not be set when NTC=0."]
pub type BTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACMDIOAR_SPEC, bool, O>;
#[doc = "Field `PSE` reader - Preamble Suppression Enable When this bit is set, the SMA suppresses the 32-bit preamble and transmit MDIO frames with only 1 preamble bit. When this bit is 0, the MDIO frame always has 32 bits of preamble as defined in the IEEE specifications."]
pub type PSE_R = crate::BitReader<bool>;
#[doc = "Field `PSE` writer - Preamble Suppression Enable When this bit is set, the SMA suppresses the 32-bit preamble and transmit MDIO frames with only 1 preamble bit. When this bit is 0, the MDIO frame always has 32 bits of preamble as defined in the IEEE specifications."]
pub type PSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACMDIOAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MII Busy The application sets this bit to instruct the SMA to initiate a Read or Write access to the MDIOS. The MAC clears this bit after the MDIO frame transfer is completed. Hence the software must not write or change any of the fields in MDIO address register (ETH_MACMDIOAR) and MDIO data register (ETH_MACMDIODR) as long as this bit is set. For write transfers, the application must first write 16-bit data in the MD field (and also RA field when C45E is set) in MDIO data register (ETH_MACMDIODR) register before setting this bit. When C45E is set, it should also write into the RA field of MDIO data register (ETH_MACMDIODR) before initiating a read transfer. When a read transfer is completed (MII busy=0), the data read from the PHY register is valid in the MD field of the MDIO data register (ETH_MACMDIODR). Note: Even if the addressed PHY is not present, there is no change in the functionality of this bit."]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clause 45 PHY Enable When this bit is set, Clause 45 capable PHY is connected to MDIO. When this bit is reset, Clause 22 capable PHY is connected to MDIO."]
    #[inline(always)]
    pub fn c45e(&self) -> C45E_R {
        C45E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - MII Operation Command This bit indicates the operation command to the PHY. When Clause 22 PHY is enabled, only Write and Read commands are valid."]
    #[inline(always)]
    pub fn goc(&self) -> GOC_R {
        GOC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Skip Address Packet When this bit is set, the SMA does not send the address packets before read, write, or post-read increment address packets. This bit is valid only when C45E is set."]
    #[inline(always)]
    pub fn skap(&self) -> SKAP_R {
        SKAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design: 0110 to 0111: Reserved, must not be used The suggested range of CSR clock frequency applicable for each value (when Bit 11 = 0) ensures that the MDC clock is approximately between 1.0 MHz to 2.5 MHz frequency range. When Bit 11 is set, you can achieve a higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits to 1010, the resultant MDC clock is of 12.5 MHz which is above the range specified in IEEE 802.3. Program the following values only if the interfacing chips support faster MDC clocks:"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Number of Training Clocks This field controls the number of trailing clock cycles generated on ETH_MDC after the end of transmission of MDIO frame. The valid values can be from 0 to 7. Programming the value to 011 indicates that there are additional three clock cycles on the MDC line after the end of MDIO frame transfer."]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Register/Device Address These bits select the PHY register in selected Clause 22 PHY device. These bits select the Device (MMD) in selected Clause 45 capable PHY."]
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Physical Layer Address This field indicates which Clause 22 PHY devices (out of 32 devices) the MAC is accessing. This field indicates which Clause 45 capable PHYs (out of 32 PHYs) the MAC is accessing."]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC informs the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted). The software can thus initiate the next command which is executed immediately irrespective of the number trailing clocks generated for the previous frame. When this bit is reset, then the read/write command completion (MII busy is cleared) only after the trailing clocks are generated. In this mode, it is ensured that the NTC is always generated after each frame. This bit must not be set when NTC=0."]
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Preamble Suppression Enable When this bit is set, the SMA suppresses the 32-bit preamble and transmit MDIO frames with only 1 preamble bit. When this bit is 0, the MDIO frame always has 32 bits of preamble as defined in the IEEE specifications."]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MII Busy The application sets this bit to instruct the SMA to initiate a Read or Write access to the MDIOS. The MAC clears this bit after the MDIO frame transfer is completed. Hence the software must not write or change any of the fields in MDIO address register (ETH_MACMDIOAR) and MDIO data register (ETH_MACMDIODR) as long as this bit is set. For write transfers, the application must first write 16-bit data in the MD field (and also RA field when C45E is set) in MDIO data register (ETH_MACMDIODR) register before setting this bit. When C45E is set, it should also write into the RA field of MDIO data register (ETH_MACMDIODR) before initiating a read transfer. When a read transfer is completed (MII busy=0), the data read from the PHY register is valid in the MD field of the MDIO data register (ETH_MACMDIODR). Note: Even if the addressed PHY is not present, there is no change in the functionality of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<0> {
        MB_W::new(self)
    }
    #[doc = "Bit 1 - Clause 45 PHY Enable When this bit is set, Clause 45 capable PHY is connected to MDIO. When this bit is reset, Clause 22 capable PHY is connected to MDIO."]
    #[inline(always)]
    #[must_use]
    pub fn c45e(&mut self) -> C45E_W<1> {
        C45E_W::new(self)
    }
    #[doc = "Bits 2:3 - MII Operation Command This bit indicates the operation command to the PHY. When Clause 22 PHY is enabled, only Write and Read commands are valid."]
    #[inline(always)]
    #[must_use]
    pub fn goc(&mut self) -> GOC_W<2> {
        GOC_W::new(self)
    }
    #[doc = "Bit 4 - Skip Address Packet When this bit is set, the SMA does not send the address packets before read, write, or post-read increment address packets. This bit is valid only when C45E is set."]
    #[inline(always)]
    #[must_use]
    pub fn skap(&mut self) -> SKAP_W<4> {
        SKAP_W::new(self)
    }
    #[doc = "Bits 8:11 - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design: 0110 to 0111: Reserved, must not be used The suggested range of CSR clock frequency applicable for each value (when Bit 11 = 0) ensures that the MDC clock is approximately between 1.0 MHz to 2.5 MHz frequency range. When Bit 11 is set, you can achieve a higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits to 1010, the resultant MDC clock is of 12.5 MHz which is above the range specified in IEEE 802.3. Program the following values only if the interfacing chips support faster MDC clocks:"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<8> {
        CR_W::new(self)
    }
    #[doc = "Bits 12:14 - Number of Training Clocks This field controls the number of trailing clock cycles generated on ETH_MDC after the end of transmission of MDIO frame. The valid values can be from 0 to 7. Programming the value to 011 indicates that there are additional three clock cycles on the MDC line after the end of MDIO frame transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ntc(&mut self) -> NTC_W<12> {
        NTC_W::new(self)
    }
    #[doc = "Bits 16:20 - Register/Device Address These bits select the PHY register in selected Clause 22 PHY device. These bits select the Device (MMD) in selected Clause 45 capable PHY."]
    #[inline(always)]
    #[must_use]
    pub fn rda(&mut self) -> RDA_W<16> {
        RDA_W::new(self)
    }
    #[doc = "Bits 21:25 - Physical Layer Address This field indicates which Clause 22 PHY devices (out of 32 devices) the MAC is accessing. This field indicates which Clause 45 capable PHYs (out of 32 PHYs) the MAC is accessing."]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<21> {
        PA_W::new(self)
    }
    #[doc = "Bit 26 - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC informs the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted). The software can thus initiate the next command which is executed immediately irrespective of the number trailing clocks generated for the previous frame. When this bit is reset, then the read/write command completion (MII busy is cleared) only after the trailing clocks are generated. In this mode, it is ensured that the NTC is always generated after each frame. This bit must not be set when NTC=0."]
    #[inline(always)]
    #[must_use]
    pub fn btb(&mut self) -> BTB_W<26> {
        BTB_W::new(self)
    }
    #[doc = "Bit 27 - Preamble Suppression Enable When this bit is set, the SMA suppresses the 32-bit preamble and transmit MDIO frames with only 1 preamble bit. When this bit is 0, the MDIO frame always has 32 bits of preamble as defined in the IEEE specifications."]
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PSE_W<27> {
        PSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIO address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macmdioar](index.html) module"]
pub struct ETH_MACMDIOAR_SPEC;
impl crate::RegisterSpec for ETH_MACMDIOAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macmdioar::R](R) reader structure"]
impl crate::Readable for ETH_MACMDIOAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macmdioar::W](W) writer structure"]
impl crate::Writable for ETH_MACMDIOAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACMDIOAR to value 0"]
impl crate::Resettable for ETH_MACMDIOAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
