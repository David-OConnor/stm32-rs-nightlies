#[doc = "Register `ETH_DMAISR` reader"]
pub struct R(crate::R<ETH_DMAISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC0IS` reader - DMA Channel Interrupt Status"]
pub type DC0IS_R = crate::BitReader<bool>;
#[doc = "Field `DC1IS` reader - DC1IS"]
pub type DC1IS_R = crate::BitReader<bool>;
#[doc = "Field `MTLIS` reader - MTL Interrupt Status"]
pub type MTLIS_R = crate::BitReader<bool>;
#[doc = "Field `MACIS` reader - MAC Interrupt Status"]
pub type MACIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DMA Channel Interrupt Status"]
    #[inline(always)]
    pub fn dc0is(&self) -> DC0IS_R {
        DC0IS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DC1IS"]
    #[inline(always)]
    pub fn dc1is(&self) -> DC1IS_R {
        DC1IS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Status"]
    #[inline(always)]
    pub fn mtlis(&self) -> MTLIS_R {
        MTLIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MAC Interrupt Status"]
    #[inline(always)]
    pub fn macis(&self) -> MACIS_R {
        MACIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaisr](index.html) module"]
pub struct ETH_DMAISR_SPEC;
impl crate::RegisterSpec for ETH_DMAISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmaisr::R](R) reader structure"]
impl crate::Readable for ETH_DMAISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_DMAISR to value 0x8000"]
impl crate::Resettable for ETH_DMAISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
