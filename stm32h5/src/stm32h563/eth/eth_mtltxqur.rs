#[doc = "Register `ETH_MTLTXQUR` reader"]
pub struct R(crate::R<ETH_MTLTXQUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTXQUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTXQUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTXQUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLTXQUR` writer"]
pub struct W(crate::W<ETH_MTLTXQUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLTXQUR_SPEC>;
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
impl From<crate::W<ETH_MTLTXQUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLTXQUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UFFRMCNT` reader - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx queue Underflow. This counter is incremented each time the MAC aborts outgoing packet because of underflow. The counter is cleared when this register is read.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type UFFRMCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UFFRMCNT` writer - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx queue Underflow. This counter is incremented each time the MAC aborts outgoing packet because of underflow. The counter is cleared when this register is read."]
pub type UFFRMCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MTLTXQUR_SPEC, u16, u16, 11, O>;
#[doc = "Field `UFCNTOVF` reader - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count. In such a scenario, the overflow packet counter is reset to all-zeros and this bit indicates that the rollover happened.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type UFCNTOVF_R = crate::BitReader<bool>;
#[doc = "Field `UFCNTOVF` writer - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count. In such a scenario, the overflow packet counter is reset to all-zeros and this bit indicates that the rollover happened."]
pub type UFCNTOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLTXQUR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10 - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx queue Underflow. This counter is incremented each time the MAC aborts outgoing packet because of underflow. The counter is cleared when this register is read."]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count. In such a scenario, the overflow packet counter is reset to all-zeros and this bit indicates that the rollover happened."]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx queue Underflow. This counter is incremented each time the MAC aborts outgoing packet because of underflow. The counter is cleared when this register is read."]
    #[inline(always)]
    #[must_use]
    pub fn uffrmcnt(&mut self) -> UFFRMCNT_W<0> {
        UFFRMCNT_W::new(self)
    }
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count. In such a scenario, the overflow packet counter is reset to all-zeros and this bit indicates that the rollover happened."]
    #[inline(always)]
    #[must_use]
    pub fn ufcntovf(&mut self) -> UFCNTOVF_W<11> {
        UFCNTOVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx queue underflow register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltxqur](index.html) module"]
pub struct ETH_MTLTXQUR_SPEC;
impl crate::RegisterSpec for ETH_MTLTXQUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltxqur::R](R) reader structure"]
impl crate::Readable for ETH_MTLTXQUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtltxqur::W](W) writer structure"]
impl crate::Writable for ETH_MTLTXQUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MTLTXQUR to value 0"]
impl crate::Resettable for ETH_MTLTXQUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
