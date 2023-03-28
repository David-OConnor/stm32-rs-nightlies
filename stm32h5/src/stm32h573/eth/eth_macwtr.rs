#[doc = "Register `ETH_MACWTR` reader"]
pub struct R(crate::R<ETH_MACWTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACWTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACWTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACWTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACWTR` writer"]
pub struct W(crate::W<ETH_MACWTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACWTR_SPEC>;
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
impl From<crate::W<ETH_MACWTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACWTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WTO` reader - Watchdog Timeout When the PWE bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, this field is used as watchdog timeout for a received packet. If the length of a received packet exceeds the value of this field, such packet is terminated and declared as an error packet. Encoding is as follows: .. Note: When the PWE bit is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE 802.3-specified valid tagged packets are declared as error packets and then dropped."]
pub type WTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WTO` writer - Watchdog Timeout When the PWE bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, this field is used as watchdog timeout for a received packet. If the length of a received packet exceeds the value of this field, such packet is terminated and declared as an error packet. Encoding is as follows: .. Note: When the PWE bit is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE 802.3-specified valid tagged packets are declared as error packets and then dropped."]
pub type WTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACWTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PWE` reader - Programmable Watchdog Enable When this bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, the WTO field is used as watchdog timeout for a received packet. When this bit is cleared, the watchdog timeout for a received packet is controlled by setting of WD and JE bits in Operating mode configuration register (ETH_MACCR) register."]
pub type PWE_R = crate::BitReader<bool>;
#[doc = "Field `PWE` writer - Programmable Watchdog Enable When this bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, the WTO field is used as watchdog timeout for a received packet. When this bit is cleared, the watchdog timeout for a received packet is controlled by setting of WD and JE bits in Operating mode configuration register (ETH_MACCR) register."]
pub type PWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACWTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Watchdog Timeout When the PWE bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, this field is used as watchdog timeout for a received packet. If the length of a received packet exceeds the value of this field, such packet is terminated and declared as an error packet. Encoding is as follows: .. Note: When the PWE bit is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE 802.3-specified valid tagged packets are declared as error packets and then dropped."]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Programmable Watchdog Enable When this bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, the WTO field is used as watchdog timeout for a received packet. When this bit is cleared, the watchdog timeout for a received packet is controlled by setting of WD and JE bits in Operating mode configuration register (ETH_MACCR) register."]
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Watchdog Timeout When the PWE bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, this field is used as watchdog timeout for a received packet. If the length of a received packet exceeds the value of this field, such packet is terminated and declared as an error packet. Encoding is as follows: .. Note: When the PWE bit is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE 802.3-specified valid tagged packets are declared as error packets and then dropped."]
    #[inline(always)]
    #[must_use]
    pub fn wto(&mut self) -> WTO_W<0> {
        WTO_W::new(self)
    }
    #[doc = "Bit 8 - Programmable Watchdog Enable When this bit is set and the WD bit of the Operating mode configuration register (ETH_MACCR) register is reset, the WTO field is used as watchdog timeout for a received packet. When this bit is cleared, the watchdog timeout for a received packet is controlled by setting of WD and JE bits in Operating mode configuration register (ETH_MACCR) register."]
    #[inline(always)]
    #[must_use]
    pub fn pwe(&mut self) -> PWE_W<8> {
        PWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macwtr](index.html) module"]
pub struct ETH_MACWTR_SPEC;
impl crate::RegisterSpec for ETH_MACWTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macwtr::R](R) reader structure"]
impl crate::Readable for ETH_MACWTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macwtr::W](W) writer structure"]
impl crate::Writable for ETH_MACWTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACWTR to value 0"]
impl crate::Resettable for ETH_MACWTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
