#[doc = "Register `MTLRxQDR` reader"]
pub struct R(crate::R<MTLRX_QDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLRX_QDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLRX_QDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLRX_QDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RWCSTS` reader - MTL Rx Queue Write Controller Active Status"]
pub type RWCSTS_R = crate::BitReader<bool>;
#[doc = "Field `RRCSTS` reader - MTL Rx Queue Read Controller State"]
pub type RRCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXQSTS` reader - MTL Rx Queue Fill-Level Status"]
pub type RXQSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRXQ` reader - Number of Packets in Receive Queue"]
pub type PRXQ_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status"]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue"]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "Rx queue debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlrx_qdr](index.html) module"]
pub struct MTLRX_QDR_SPEC;
impl crate::RegisterSpec for MTLRX_QDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlrx_qdr::R](R) reader structure"]
impl crate::Readable for MTLRX_QDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MTLRxQDR to value 0"]
impl crate::Resettable for MTLRX_QDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
