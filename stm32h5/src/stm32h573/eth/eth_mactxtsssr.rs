#[doc = "Register `ETH_MACTXTSSSR` reader"]
pub struct R(crate::R<ETH_MACTXTSSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACTXTSSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACTXTSSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACTXTSSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXTSSHI` reader - Transmit Timestamp Status High This field contains the lower 32 bits of the Seconds field of Transmit packet's captured timestamp."]
pub type TXTSSHI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Timestamp Status High This field contains the lower 32 bits of the Seconds field of Transmit packet's captured timestamp."]
    #[inline(always)]
    pub fn txtsshi(&self) -> TXTSSHI_R {
        TXTSSHI_R::new(self.bits)
    }
}
#[doc = "Tx timestamp status seconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactxtsssr](index.html) module"]
pub struct ETH_MACTXTSSSR_SPEC;
impl crate::RegisterSpec for ETH_MACTXTSSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mactxtsssr::R](R) reader structure"]
impl crate::Readable for ETH_MACTXTSSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACTXTSSSR to value 0"]
impl crate::Resettable for ETH_MACTXTSSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
