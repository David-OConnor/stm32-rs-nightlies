#[doc = "Register `MACDR` reader"]
pub struct R(crate::R<MACDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPESTS` reader - MAC MII Receive Protocol Engine Status"]
pub type RPESTS_R = crate::BitReader<bool>;
#[doc = "Field `RFCFCSTS` reader - MAC Receive Packet Controller FIFO Status"]
pub type RFCFCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPESTS` reader - MAC MII Transmit Protocol Engine Status"]
pub type TPESTS_R = crate::BitReader<bool>;
#[doc = "Field `TFCSTS` reader - MAC Transmit Packet Controller Status"]
pub type TFCSTS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Packet Controller FIFO Status"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Packet Controller Status"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 3) as u8)
    }
}
#[doc = "Debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macdr](index.html) module"]
pub struct MACDR_SPEC;
impl crate::RegisterSpec for MACDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macdr::R](R) reader structure"]
impl crate::Readable for MACDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACDR to value 0"]
impl crate::Resettable for MACDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
