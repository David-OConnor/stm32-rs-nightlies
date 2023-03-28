#[doc = "Register `ETH_MACLMIR` reader"]
pub struct R(crate::R<ETH_MACLMIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACLMIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACLMIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACLMIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACLMIR` writer"]
pub struct W(crate::W<ETH_MACLMIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACLMIR_SPEC>;
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
impl From<crate::W<ETH_MACLMIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACLMIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSI` reader - Log Sync Interval This field indicates the periodicity of the automatically generated SYNC message when the PTP node is Master. Allowed values are -15 to 15. Negative value must be represented in 2's-complement form. For example, if the required value is -1, the value programmed must be 0xFF."]
pub type LSI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSI` writer - Log Sync Interval This field indicates the periodicity of the automatically generated SYNC message when the PTP node is Master. Allowed values are -15 to 15. Negative value must be represented in 2's-complement form. For example, if the required value is -1, the value programmed must be 0xFF."]
pub type LSI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACLMIR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DRSYNCR` reader - Delay_Req to SYNC Ratio In Slave mode, it is used for controlling frequency of Delay_Req messages transmitted. Others: Reserved, must not be used The master sends this information (logMinDelayReqInterval) in the DelayResp PTP messages to the slave. The reception processes this value from the received DelayResp messages and updates this field accordingly. In the Slave mode, the host must not write/update this register unless it has to override the received value. In Master mode, the sum of this field and logSyncInterval (LSI) field is provided in the logMinDelayReqInterval field of the generated multicast Delay_Resp PTP message."]
pub type DRSYNCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRSYNCR` writer - Delay_Req to SYNC Ratio In Slave mode, it is used for controlling frequency of Delay_Req messages transmitted. Others: Reserved, must not be used The master sends this information (logMinDelayReqInterval) in the DelayResp PTP messages to the slave. The reception processes this value from the received DelayResp messages and updates this field accordingly. In the Slave mode, the host must not write/update this register unless it has to override the received value. In Master mode, the sum of this field and logSyncInterval (LSI) field is provided in the logMinDelayReqInterval field of the generated multicast Delay_Resp PTP message."]
pub type DRSYNCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACLMIR_SPEC, u8, u8, 3, O>;
#[doc = "Field `LMPDRI` reader - Log Min Pdelay_Req Interval This field indicates logMinPdelayReqInterval of PTP node. This is used to schedule the periodic Pdelay request packet transmission. Allowed values are -15 to 15.Negative value must be represented in 2's-complement form. For example, if the required value is -1, the value programmed must be 0xFF."]
pub type LMPDRI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LMPDRI` writer - Log Min Pdelay_Req Interval This field indicates logMinPdelayReqInterval of PTP node. This is used to schedule the periodic Pdelay request packet transmission. Allowed values are -15 to 15.Negative value must be represented in 2's-complement form. For example, if the required value is -1, the value programmed must be 0xFF."]
pub type LMPDRI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACLMIR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Log Sync Interval This field indicates the periodicity of the automatically generated SYNC message when the PTP node is Master. Allowed values are -15 to 15. Negative value must be represented in 2's-complement form. For example, if the required value is -1, the value programmed must be 0xFF."]
    #[inline(always)]
    pub fn lsi(&self) -> LSI_R {
        LSI_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Delay_Req to SYNC Ratio In Slave mode, it is used for controlling frequency of Delay_Req messages transmitted. Others: Reserved, must not be used The master sends this information (logMinDelayReqInterval) in the DelayResp PTP messages to the slave. The reception processes this value from the received DelayResp messages and updates this field accordingly. In the Slave mode, the host must not write/update this register unless it has to override the received value. In Master mode, the sum of this field and logSyncInterval (LSI) field is provided in the logMinDelayReqInterval field of the generated multicast Delay_Resp PTP message."]
    #[inline(always)]
    pub fn drsyncr(&self) -> DRSYNCR_R {
        DRSYNCR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 24:31 - Log Min Pdelay_Req Interval This field indicates logMinPdelayReqInterval of PTP node. This is used to schedule the periodic Pdelay request packet transmission. Allowed values are -15 to 15.Negative value must be represented in 2's-complement form. For example, if the required value is -1, the value programmed must be 0xFF."]
    #[inline(always)]
    pub fn lmpdri(&self) -> LMPDRI_R {
        LMPDRI_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Log Sync Interval This field indicates the periodicity of the automatically generated SYNC message when the PTP node is Master. Allowed values are -15 to 15. Negative value must be represented in 2's-complement form. For example, if the required value is -1, the value programmed must be 0xFF."]
    #[inline(always)]
    #[must_use]
    pub fn lsi(&mut self) -> LSI_W<0> {
        LSI_W::new(self)
    }
    #[doc = "Bits 8:10 - Delay_Req to SYNC Ratio In Slave mode, it is used for controlling frequency of Delay_Req messages transmitted. Others: Reserved, must not be used The master sends this information (logMinDelayReqInterval) in the DelayResp PTP messages to the slave. The reception processes this value from the received DelayResp messages and updates this field accordingly. In the Slave mode, the host must not write/update this register unless it has to override the received value. In Master mode, the sum of this field and logSyncInterval (LSI) field is provided in the logMinDelayReqInterval field of the generated multicast Delay_Resp PTP message."]
    #[inline(always)]
    #[must_use]
    pub fn drsyncr(&mut self) -> DRSYNCR_W<8> {
        DRSYNCR_W::new(self)
    }
    #[doc = "Bits 24:31 - Log Min Pdelay_Req Interval This field indicates logMinPdelayReqInterval of PTP node. This is used to schedule the periodic Pdelay request packet transmission. Allowed values are -15 to 15.Negative value must be represented in 2's-complement form. For example, if the required value is -1, the value programmed must be 0xFF."]
    #[inline(always)]
    #[must_use]
    pub fn lmpdri(&mut self) -> LMPDRI_W<24> {
        LMPDRI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Log message interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maclmir](index.html) module"]
pub struct ETH_MACLMIR_SPEC;
impl crate::RegisterSpec for ETH_MACLMIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_maclmir::R](R) reader structure"]
impl crate::Readable for ETH_MACLMIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_maclmir::W](W) writer structure"]
impl crate::Writable for ETH_MACLMIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACLMIR to value 0"]
impl crate::Resettable for ETH_MACLMIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
