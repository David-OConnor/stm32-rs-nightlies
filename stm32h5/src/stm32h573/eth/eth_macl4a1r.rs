#[doc = "Register `ETH_MACL4A1R` reader"]
pub struct R(crate::R<ETH_MACL4A1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACL4A1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACL4A1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACL4A1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACL4A1R` writer"]
pub struct W(crate::W<ETH_MACL4A1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACL4A1R_SPEC>;
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
impl From<crate::W<ETH_MACL4A1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACL4A1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L4SP1` reader - Layer 4 Source Port Number Field When the L4PEN1 bit is reset and the L4DPM1 bit is set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets. When the L4PEN1 and L4DPM1 bits are set in L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 packets."]
pub type L4SP1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `L4SP1` writer - Layer 4 Source Port Number Field When the L4PEN1 bit is reset and the L4DPM1 bit is set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets. When the L4PEN1 and L4DPM1 bits are set in L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 packets."]
pub type L4SP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACL4A1R_SPEC, u16, u16, 16, O>;
#[doc = "Field `L4DP1` reader - Layer 4 Destination Port Number Field When the L4PEN1 bit is reset and the L4DPM1 bit is set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets. When the L4PEN1 and L4DPM1 bits are set in L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 packets."]
pub type L4DP1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `L4DP1` writer - Layer 4 Destination Port Number Field When the L4PEN1 bit is reset and the L4DPM1 bit is set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets. When the L4PEN1 and L4DPM1 bits are set in L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 packets."]
pub type L4DP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACL4A1R_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Layer 4 Source Port Number Field When the L4PEN1 bit is reset and the L4DPM1 bit is set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets. When the L4PEN1 and L4DPM1 bits are set in L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 packets."]
    #[inline(always)]
    pub fn l4sp1(&self) -> L4SP1_R {
        L4SP1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Layer 4 Destination Port Number Field When the L4PEN1 bit is reset and the L4DPM1 bit is set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets. When the L4PEN1 and L4DPM1 bits are set in L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 packets."]
    #[inline(always)]
    pub fn l4dp1(&self) -> L4DP1_R {
        L4DP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Layer 4 Source Port Number Field When the L4PEN1 bit is reset and the L4DPM1 bit is set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets. When the L4PEN1 and L4DPM1 bits are set in L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 packets."]
    #[inline(always)]
    #[must_use]
    pub fn l4sp1(&mut self) -> L4SP1_W<0> {
        L4SP1_W::new(self)
    }
    #[doc = "Bits 16:31 - Layer 4 Destination Port Number Field When the L4PEN1 bit is reset and the L4DPM1 bit is set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets. When the L4PEN1 and L4DPM1 bits are set in L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 packets."]
    #[inline(always)]
    #[must_use]
    pub fn l4dp1(&mut self) -> L4DP1_W<16> {
        L4DP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 4 address filter 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl4a1r](index.html) module"]
pub struct ETH_MACL4A1R_SPEC;
impl crate::RegisterSpec for ETH_MACL4A1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macl4a1r::R](R) reader structure"]
impl crate::Readable for ETH_MACL4A1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macl4a1r::W](W) writer structure"]
impl crate::Writable for ETH_MACL4A1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACL4A1R to value 0"]
impl crate::Resettable for ETH_MACL4A1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
