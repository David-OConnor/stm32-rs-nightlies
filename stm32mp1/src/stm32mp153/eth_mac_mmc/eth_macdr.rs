#[doc = "Register `ETH_MACDR` reader"]
pub struct R(crate::R<ETH_MACDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPESTS` reader - RPESTS"]
pub type RPESTS_R = crate::BitReader<bool>;
#[doc = "Field `RFCFCSTS` reader - RFCFCSTS"]
pub type RFCFCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPESTS` reader - TPESTS"]
pub type TPESTS_R = crate::BitReader<bool>;
#[doc = "Field `TFCSTS` reader - TFCSTS"]
pub type TFCSTS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - RPESTS"]
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - RFCFCSTS"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 16 - TPESTS"]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - TFCSTS"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 3) as u8)
    }
}
#[doc = "The Debug register provides the debug status of various MAC blocks.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macdr](index.html) module"]
pub struct ETH_MACDR_SPEC;
impl crate::RegisterSpec for ETH_MACDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macdr::R](R) reader structure"]
impl crate::Readable for ETH_MACDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACDR to value 0"]
impl crate::Resettable for ETH_MACDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
