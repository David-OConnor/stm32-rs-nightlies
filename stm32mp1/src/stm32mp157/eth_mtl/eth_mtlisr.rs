#[doc = "Register `ETH_MTLISR` reader"]
pub struct R(crate::R<ETH_MTLISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Q0IS` reader - Q0IS"]
pub type Q0IS_R = crate::BitReader<bool>;
#[doc = "Field `Q1IS` reader - Q1IS"]
pub type Q1IS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Q0IS"]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Q1IS"]
    #[inline(always)]
    pub fn q1is(&self) -> Q1IS_R {
        Q1IS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlisr](index.html) module"]
pub struct ETH_MTLISR_SPEC;
impl crate::RegisterSpec for ETH_MTLISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlisr::R](R) reader structure"]
impl crate::Readable for ETH_MTLISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MTLISR to value 0"]
impl crate::Resettable for ETH_MTLISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
