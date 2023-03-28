#[doc = "Register `ETH_MACPCSR` reader"]
pub struct R(crate::R<ETH_MACPCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPCSR` writer"]
pub struct W(crate::W<ETH_MACPCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPCSR_SPEC>;
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
impl From<crate::W<ETH_MACPCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRDWN` reader - Power Down When this bit is set, the MAC receiver drops all received packets until it receives the expected magic packet or remote wakeup packet. This bit is then self-cleared and the power-down mode is disabled. The software can clear this bit before the expected magic packet or remote wakeup packet is received. The packets received by the MAC after this bit is cleared are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote wakeup Packet Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit."]
pub type PWRDWN_R = crate::BitReader<bool>;
#[doc = "Field `PWRDWN` writer - Power Down When this bit is set, the MAC receiver drops all received packets until it receives the expected magic packet or remote wakeup packet. This bit is then self-cleared and the power-down mode is disabled. The software can clear this bit before the expected magic packet or remote wakeup packet is received. The packets received by the MAC after this bit is cleared are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote wakeup Packet Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit."]
pub type PWRDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
#[doc = "Field `MGKPKTEN` reader - Magic Packet Enable When this bit is set, a power management event is generated when the MAC receives a magic packet."]
pub type MGKPKTEN_R = crate::BitReader<bool>;
#[doc = "Field `MGKPKTEN` writer - Magic Packet Enable When this bit is set, a power management event is generated when the MAC receives a magic packet."]
pub type MGKPKTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
#[doc = "Field `RWKPKTEN` reader - Remote wakeup Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wakeup packet."]
pub type RWKPKTEN_R = crate::BitReader<bool>;
#[doc = "Field `RWKPKTEN` writer - Remote wakeup Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wakeup packet."]
pub type RWKPKTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
#[doc = "Field `MGKPRCVD` reader - Magic Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared when this register is read (or written to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type MGKPRCVD_R = crate::BitReader<bool>;
#[doc = "Field `MGKPRCVD` writer - Magic Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared when this register is read (or written to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
pub type MGKPRCVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
#[doc = "Field `RWKPRCVD` reader - Remote wakeup Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a remote wakeup packet. This bit is cleared when this register is read."]
pub type RWKPRCVD_R = crate::BitReader<bool>;
#[doc = "Field `GLBLUCAST` reader - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wakeup packet."]
pub type GLBLUCAST_R = crate::BitReader<bool>;
#[doc = "Field `GLBLUCAST` writer - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wakeup packet."]
pub type GLBLUCAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
#[doc = "Field `RWKPFE` reader - Remote wakeup Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wakeup frame. All frames after that event including the received wakeup frame are forwarded to application. This bit is then self-cleared on receiving the wakeup packet. The application can also clear this bit before the expected wakeup frame is received. In such cases, the MAC reverts to the default behavior where packets received are forwarded to the application. This bit must only be set when RWKPKTEN is set high and PWRDWN is set low. The setting of this bit has no effect when PWRDWN is set high. Note: If Magic Packet Enable and wakeup Frame Enable are both set along with setting of this bit and Magic Packet is received prior to wakeup frame, this bit is self-cleared on receiving Magic Packet, the received Magic packet is dropped, and all frames after received Magic Packet are forwarded to application."]
pub type RWKPFE_R = crate::BitReader<bool>;
#[doc = "Field `RWKPFE` writer - Remote wakeup Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wakeup frame. All frames after that event including the received wakeup frame are forwarded to application. This bit is then self-cleared on receiving the wakeup packet. The application can also clear this bit before the expected wakeup frame is received. In such cases, the MAC reverts to the default behavior where packets received are forwarded to the application. This bit must only be set when RWKPKTEN is set high and PWRDWN is set low. The setting of this bit has no effect when PWRDWN is set high. Note: If Magic Packet Enable and wakeup Frame Enable are both set along with setting of this bit and Magic Packet is received prior to wakeup frame, this bit is self-cleared on receiving Magic Packet, the received Magic packet is dropped, and all frames after received Magic Packet are forwarded to application."]
pub type RWKPFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
#[doc = "Field `RWKPTR` reader - Remote wakeup FIFO Pointer This field gives the current value (0 to 7) of the Remote wakeup Packet Filter register pointer. When the value of this pointer is equal to 7, the contents of the Remote wakeup Packet Filter Register are transferred to the eth_mii_rx_clk domain when a Write occurs to that register."]
pub type RWKPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWKFILTRST` reader - Remote wakeup Packet Filter Register Pointer Reset When this bit is set, the remote wakeup packet filter register pointer is reset to 0. It is automatically cleared after 1 clock cycle."]
pub type RWKFILTRST_R = crate::BitReader<bool>;
#[doc = "Field `RWKFILTRST` writer - Remote wakeup Packet Filter Register Pointer Reset When this bit is set, the remote wakeup packet filter register pointer is reset to 0. It is automatically cleared after 1 clock cycle."]
pub type RWKFILTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power Down When this bit is set, the MAC receiver drops all received packets until it receives the expected magic packet or remote wakeup packet. This bit is then self-cleared and the power-down mode is disabled. The software can clear this bit before the expected magic packet or remote wakeup packet is received. The packets received by the MAC after this bit is cleared are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote wakeup Packet Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit."]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable When this bit is set, a power management event is generated when the MAC receives a magic packet."]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Remote wakeup Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wakeup packet."]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared when this register is read (or written to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Remote wakeup Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a remote wakeup packet. This bit is cleared when this register is read."]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wakeup packet."]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Remote wakeup Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wakeup frame. All frames after that event including the received wakeup frame are forwarded to application. This bit is then self-cleared on receiving the wakeup packet. The application can also clear this bit before the expected wakeup frame is received. In such cases, the MAC reverts to the default behavior where packets received are forwarded to the application. This bit must only be set when RWKPKTEN is set high and PWRDWN is set low. The setting of this bit has no effect when PWRDWN is set high. Note: If Magic Packet Enable and wakeup Frame Enable are both set along with setting of this bit and Magic Packet is received prior to wakeup frame, this bit is self-cleared on receiving Magic Packet, the received Magic packet is dropped, and all frames after received Magic Packet are forwarded to application."]
    #[inline(always)]
    pub fn rwkpfe(&self) -> RWKPFE_R {
        RWKPFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Remote wakeup FIFO Pointer This field gives the current value (0 to 7) of the Remote wakeup Packet Filter register pointer. When the value of this pointer is equal to 7, the contents of the Remote wakeup Packet Filter Register are transferred to the eth_mii_rx_clk domain when a Write occurs to that register."]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Remote wakeup Packet Filter Register Pointer Reset When this bit is set, the remote wakeup packet filter register pointer is reset to 0. It is automatically cleared after 1 clock cycle."]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Down When this bit is set, the MAC receiver drops all received packets until it receives the expected magic packet or remote wakeup packet. This bit is then self-cleared and the power-down mode is disabled. The software can clear this bit before the expected magic packet or remote wakeup packet is received. The packets received by the MAC after this bit is cleared are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote wakeup Packet Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<0> {
        PWRDWN_W::new(self)
    }
    #[doc = "Bit 1 - Magic Packet Enable When this bit is set, a power management event is generated when the MAC receives a magic packet."]
    #[inline(always)]
    #[must_use]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W<1> {
        MGKPKTEN_W::new(self)
    }
    #[doc = "Bit 2 - Remote wakeup Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wakeup packet."]
    #[inline(always)]
    #[must_use]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W<2> {
        RWKPKTEN_W::new(self)
    }
    #[doc = "Bit 5 - Magic Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared when this register is read (or written to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
    #[inline(always)]
    #[must_use]
    pub fn mgkprcvd(&mut self) -> MGKPRCVD_W<5> {
        MGKPRCVD_W::new(self)
    }
    #[doc = "Bit 9 - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wakeup packet."]
    #[inline(always)]
    #[must_use]
    pub fn glblucast(&mut self) -> GLBLUCAST_W<9> {
        GLBLUCAST_W::new(self)
    }
    #[doc = "Bit 10 - Remote wakeup Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wakeup frame. All frames after that event including the received wakeup frame are forwarded to application. This bit is then self-cleared on receiving the wakeup packet. The application can also clear this bit before the expected wakeup frame is received. In such cases, the MAC reverts to the default behavior where packets received are forwarded to the application. This bit must only be set when RWKPKTEN is set high and PWRDWN is set low. The setting of this bit has no effect when PWRDWN is set high. Note: If Magic Packet Enable and wakeup Frame Enable are both set along with setting of this bit and Magic Packet is received prior to wakeup frame, this bit is self-cleared on receiving Magic Packet, the received Magic packet is dropped, and all frames after received Magic Packet are forwarded to application."]
    #[inline(always)]
    #[must_use]
    pub fn rwkpfe(&mut self) -> RWKPFE_W<10> {
        RWKPFE_W::new(self)
    }
    #[doc = "Bit 31 - Remote wakeup Packet Filter Register Pointer Reset When this bit is set, the remote wakeup packet filter register pointer is reset to 0. It is automatically cleared after 1 clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W<31> {
        RWKFILTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMT control status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macpcsr](index.html) module"]
pub struct ETH_MACPCSR_SPEC;
impl crate::RegisterSpec for ETH_MACPCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macpcsr::R](R) reader structure"]
impl crate::Readable for ETH_MACPCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macpcsr::W](W) writer structure"]
impl crate::Writable for ETH_MACPCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACPCSR to value 0"]
impl crate::Resettable for ETH_MACPCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
