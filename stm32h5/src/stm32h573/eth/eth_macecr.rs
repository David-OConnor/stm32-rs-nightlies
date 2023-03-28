#[doc = "Register `ETH_MACECR` reader"]
pub struct R(crate::R<ETH_MACECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACECR` writer"]
pub struct W(crate::W<ETH_MACECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACECR_SPEC>;
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
impl From<crate::W<ETH_MACECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPSL` reader - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet. The value programmed in this field must be greater than or equal to 1,518 bytes. Any other programmed value is considered as 1,518 bytes. For VLAN tagged packets, the MAC adds 4 bytes to the programmed value. For double VLAN tagged packets, the MAC adds 8 bytes to the programmed value. The value in this field is applicable when the GPSLCE bit is set in ETH_MACCR register."]
pub type GPSL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GPSL` writer - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet. The value programmed in this field must be greater than or equal to 1,518 bytes. Any other programmed value is considered as 1,518 bytes. For VLAN tagged packets, the MAC adds 4 bytes to the programmed value. For double VLAN tagged packets, the MAC adds 8 bytes to the programmed value. The value in this field is applicable when the GPSLCE bit is set in ETH_MACCR register."]
pub type GPSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACECR_SPEC, u16, u16, 14, O>;
#[doc = "Field `DCRCC` reader - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets. When this bit is reset, the MAC receiver always checks the CRC field in the received packets."]
pub type DCRCC_R = crate::BitReader<bool>;
#[doc = "Field `DCRCC` writer - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets. When this bit is reset, the MAC receiver always checks the CRC field in the received packets."]
pub type DCRCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACECR_SPEC, bool, O>;
#[doc = "Field `SPEN` reader - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status. The MAC discards the Slow Protocol packets with invalid subtypes. When this bit is reset, the MAC forwards all error-free Slow Protocol packets to the application. The MAC considers such packets as normal Type packets."]
pub type SPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPEN` writer - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status. The MAC discards the Slow Protocol packets with invalid subtypes. When this bit is reset, the MAC forwards all error-free Slow Protocol packets to the application. The MAC considers such packets as normal Type packets."]
pub type SPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACECR_SPEC, bool, O>;
#[doc = "Field `USP` reader - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address 0 high register (ETH_MACA0HR) and MAC Address 0 low register MAC Address x low register (ETH_MACAxLR). The MAC also detects the Slow Protocol packets with the Slow Protocols multicast address (01-80-C2-00-00-02). When this bit is reset, the MAC detects only Slow Protocol packets with the Slow Protocol multicast address specified in the IEEE 802.3-2008, Section 5."]
pub type USP_R = crate::BitReader<bool>;
#[doc = "Field `USP` writer - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address 0 high register (ETH_MACA0HR) and MAC Address 0 low register MAC Address x low register (ETH_MACAxLR). The MAC also detects the Slow Protocol packets with the Slow Protocols multicast address (01-80-C2-00-00-02). When this bit is reset, the MAC detects only Slow Protocol packets with the Slow Protocol multicast address specified in the IEEE 802.3-2008, Section 5."]
pub type USP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACECR_SPEC, bool, O>;
#[doc = "Field `EIPGEN` reader - Extended Inter-Packet Gap Enable When this bit is set, the MAC interprets EIPG field and IPG field in Operating mode configuration register (ETH_MACCR) together as minimum IPG greater than 96 bit times in steps of 8 bit times. When this bit is reset, the MAC ignores EIPG field and interprets IPG field in Operating mode configuration register (ETH_MACCR) as minimum IPG less than or equal to 96 bit times in steps of 8 bit times. Note: The extended Inter-Packet Gap feature must be enabled when operating in Full-duplex mode only. There may be undesirable effects on back-pressure function and frame transmission if it is enabled in Half-duplex mode."]
pub type EIPGEN_R = crate::BitReader<bool>;
#[doc = "Field `EIPGEN` writer - Extended Inter-Packet Gap Enable When this bit is set, the MAC interprets EIPG field and IPG field in Operating mode configuration register (ETH_MACCR) together as minimum IPG greater than 96 bit times in steps of 8 bit times. When this bit is reset, the MAC ignores EIPG field and interprets IPG field in Operating mode configuration register (ETH_MACCR) as minimum IPG less than or equal to 96 bit times in steps of 8 bit times. Note: The extended Inter-Packet Gap feature must be enabled when operating in Full-duplex mode only. There may be undesirable effects on back-pressure function and frame transmission if it is enabled in Half-duplex mode."]
pub type EIPGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACECR_SPEC, bool, O>;
#[doc = "Field `EIPG` reader - Extended Inter-Packet Gap The value in this field is applicable when the EIPGEN bit is set. This field (as Most Significant bits) along with IPG field in Operating mode configuration register (ETH_MACCR), gives the minimum IPG greater than 96 bit times in steps of 8 bit times. For example: EIPG�=� 0 and IPG�=�0 give 104 bit times EIPG�=� 0 and IPG�=�1 give 112 bit times EIPG�=� 0 and IPG�=�2 give 120 bit times .. EIPG�=�7 and IPG�=�31 give 2144 bit times"]
pub type EIPG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EIPG` writer - Extended Inter-Packet Gap The value in this field is applicable when the EIPGEN bit is set. This field (as Most Significant bits) along with IPG field in Operating mode configuration register (ETH_MACCR), gives the minimum IPG greater than 96 bit times in steps of 8 bit times. For example: EIPG�=� 0 and IPG�=�0 give 104 bit times EIPG�=� 0 and IPG�=�1 give 112 bit times EIPG�=� 0 and IPG�=�2 give 120 bit times .. EIPG�=�7 and IPG�=�31 give 2144 bit times"]
pub type EIPG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACECR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:13 - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet. The value programmed in this field must be greater than or equal to 1,518 bytes. Any other programmed value is considered as 1,518 bytes. For VLAN tagged packets, the MAC adds 4 bytes to the programmed value. For double VLAN tagged packets, the MAC adds 8 bytes to the programmed value. The value in this field is applicable when the GPSLCE bit is set in ETH_MACCR register."]
    #[inline(always)]
    pub fn gpsl(&self) -> GPSL_R {
        GPSL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets. When this bit is reset, the MAC receiver always checks the CRC field in the received packets."]
    #[inline(always)]
    pub fn dcrcc(&self) -> DCRCC_R {
        DCRCC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status. The MAC discards the Slow Protocol packets with invalid subtypes. When this bit is reset, the MAC forwards all error-free Slow Protocol packets to the application. The MAC considers such packets as normal Type packets."]
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address 0 high register (ETH_MACA0HR) and MAC Address 0 low register MAC Address x low register (ETH_MACAxLR). The MAC also detects the Slow Protocol packets with the Slow Protocols multicast address (01-80-C2-00-00-02). When this bit is reset, the MAC detects only Slow Protocol packets with the Slow Protocol multicast address specified in the IEEE 802.3-2008, Section 5."]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Extended Inter-Packet Gap Enable When this bit is set, the MAC interprets EIPG field and IPG field in Operating mode configuration register (ETH_MACCR) together as minimum IPG greater than 96 bit times in steps of 8 bit times. When this bit is reset, the MAC ignores EIPG field and interprets IPG field in Operating mode configuration register (ETH_MACCR) as minimum IPG less than or equal to 96 bit times in steps of 8 bit times. Note: The extended Inter-Packet Gap feature must be enabled when operating in Full-duplex mode only. There may be undesirable effects on back-pressure function and frame transmission if it is enabled in Half-duplex mode."]
    #[inline(always)]
    pub fn eipgen(&self) -> EIPGEN_R {
        EIPGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Extended Inter-Packet Gap The value in this field is applicable when the EIPGEN bit is set. This field (as Most Significant bits) along with IPG field in Operating mode configuration register (ETH_MACCR), gives the minimum IPG greater than 96 bit times in steps of 8 bit times. For example: EIPG�=� 0 and IPG�=�0 give 104 bit times EIPG�=� 0 and IPG�=�1 give 112 bit times EIPG�=� 0 and IPG�=�2 give 120 bit times .. EIPG�=�7 and IPG�=�31 give 2144 bit times"]
    #[inline(always)]
    pub fn eipg(&self) -> EIPG_R {
        EIPG_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet. The value programmed in this field must be greater than or equal to 1,518 bytes. Any other programmed value is considered as 1,518 bytes. For VLAN tagged packets, the MAC adds 4 bytes to the programmed value. For double VLAN tagged packets, the MAC adds 8 bytes to the programmed value. The value in this field is applicable when the GPSLCE bit is set in ETH_MACCR register."]
    #[inline(always)]
    #[must_use]
    pub fn gpsl(&mut self) -> GPSL_W<0> {
        GPSL_W::new(self)
    }
    #[doc = "Bit 16 - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets. When this bit is reset, the MAC receiver always checks the CRC field in the received packets."]
    #[inline(always)]
    #[must_use]
    pub fn dcrcc(&mut self) -> DCRCC_W<16> {
        DCRCC_W::new(self)
    }
    #[doc = "Bit 17 - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status. The MAC discards the Slow Protocol packets with invalid subtypes. When this bit is reset, the MAC forwards all error-free Slow Protocol packets to the application. The MAC considers such packets as normal Type packets."]
    #[inline(always)]
    #[must_use]
    pub fn spen(&mut self) -> SPEN_W<17> {
        SPEN_W::new(self)
    }
    #[doc = "Bit 18 - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address 0 high register (ETH_MACA0HR) and MAC Address 0 low register MAC Address x low register (ETH_MACAxLR). The MAC also detects the Slow Protocol packets with the Slow Protocols multicast address (01-80-C2-00-00-02). When this bit is reset, the MAC detects only Slow Protocol packets with the Slow Protocol multicast address specified in the IEEE 802.3-2008, Section 5."]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<18> {
        USP_W::new(self)
    }
    #[doc = "Bit 24 - Extended Inter-Packet Gap Enable When this bit is set, the MAC interprets EIPG field and IPG field in Operating mode configuration register (ETH_MACCR) together as minimum IPG greater than 96 bit times in steps of 8 bit times. When this bit is reset, the MAC ignores EIPG field and interprets IPG field in Operating mode configuration register (ETH_MACCR) as minimum IPG less than or equal to 96 bit times in steps of 8 bit times. Note: The extended Inter-Packet Gap feature must be enabled when operating in Full-duplex mode only. There may be undesirable effects on back-pressure function and frame transmission if it is enabled in Half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn eipgen(&mut self) -> EIPGEN_W<24> {
        EIPGEN_W::new(self)
    }
    #[doc = "Bits 25:29 - Extended Inter-Packet Gap The value in this field is applicable when the EIPGEN bit is set. This field (as Most Significant bits) along with IPG field in Operating mode configuration register (ETH_MACCR), gives the minimum IPG greater than 96 bit times in steps of 8 bit times. For example: EIPG�=� 0 and IPG�=�0 give 104 bit times EIPG�=� 0 and IPG�=�1 give 112 bit times EIPG�=� 0 and IPG�=�2 give 120 bit times .. EIPG�=�7 and IPG�=�31 give 2144 bit times"]
    #[inline(always)]
    #[must_use]
    pub fn eipg(&mut self) -> EIPG_W<25> {
        EIPG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended operating mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macecr](index.html) module"]
pub struct ETH_MACECR_SPEC;
impl crate::RegisterSpec for ETH_MACECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macecr::R](R) reader structure"]
impl crate::Readable for ETH_MACECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macecr::W](W) writer structure"]
impl crate::Writable for ETH_MACECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACECR to value 0"]
impl crate::Resettable for ETH_MACECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
