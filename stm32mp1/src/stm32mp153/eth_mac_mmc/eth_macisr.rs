#[doc = "Register `ETH_MACISR` reader"]
pub struct R(crate::R<ETH_MACISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RGSMIIIS` reader - RGSMIIIS"]
pub type RGSMIIIS_R = crate::BitReader<bool>;
#[doc = "Field `PHYIS` reader - PHYIS"]
pub type PHYIS_R = crate::BitReader<bool>;
#[doc = "Field `PMTIS` reader - PMTIS"]
pub type PMTIS_R = crate::BitReader<bool>;
#[doc = "Field `LPIIS` reader - LPIIS"]
pub type LPIIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCIS` reader - MMCIS"]
pub type MMCIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCRXIS` reader - MMCRXIS"]
pub type MMCRXIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCTXIS` reader - MMCTXIS"]
pub type MMCTXIS_R = crate::BitReader<bool>;
#[doc = "Field `TSIS` reader - TSIS"]
pub type TSIS_R = crate::BitReader<bool>;
#[doc = "Field `TXSTSIS` reader - TXSTSIS"]
pub type TXSTSIS_R = crate::BitReader<bool>;
#[doc = "Field `RXSTSIS` reader - RXSTSIS"]
pub type RXSTSIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RGSMIIIS"]
    #[inline(always)]
    pub fn rgsmiiis(&self) -> RGSMIIIS_R {
        RGSMIIIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - PHYIS"]
    #[inline(always)]
    pub fn phyis(&self) -> PHYIS_R {
        PHYIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMTIS"]
    #[inline(always)]
    pub fn pmtis(&self) -> PMTIS_R {
        PMTIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPIIS"]
    #[inline(always)]
    pub fn lpiis(&self) -> LPIIS_R {
        LPIIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - MMCIS"]
    #[inline(always)]
    pub fn mmcis(&self) -> MMCIS_R {
        MMCIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMCRXIS"]
    #[inline(always)]
    pub fn mmcrxis(&self) -> MMCRXIS_R {
        MMCRXIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMCTXIS"]
    #[inline(always)]
    pub fn mmctxis(&self) -> MMCTXIS_R {
        MMCTXIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - TSIS"]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TXSTSIS"]
    #[inline(always)]
    pub fn txstsis(&self) -> TXSTSIS_R {
        TXSTSIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RXSTSIS"]
    #[inline(always)]
    pub fn rxstsis(&self) -> RXSTSIS_R {
        RXSTSIS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "The Interrupt Status register contains the status of interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macisr](index.html) module"]
pub struct ETH_MACISR_SPEC;
impl crate::RegisterSpec for ETH_MACISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macisr::R](R) reader structure"]
impl crate::Readable for ETH_MACISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACISR to value 0"]
impl crate::Resettable for ETH_MACISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
