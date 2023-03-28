#[doc = "Register `SYSCFG_ITLINE10` reader"]
pub struct R(crate::R<SYSCFG_ITLINE10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA1_CH2` reader - DMA1 channel 2 interrupt request pending"]
pub type DMA1_CH2_R = crate::BitReader<bool>;
#[doc = "Field `DMA1_CH3` reader - DMA1 channel 3 interrupt request pending"]
pub type DMA1_CH3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DMA1 channel 2 interrupt request pending"]
    #[inline(always)]
    pub fn dma1_ch2(&self) -> DMA1_CH2_R {
        DMA1_CH2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1 channel 3 interrupt request pending"]
    #[inline(always)]
    pub fn dma1_ch3(&self) -> DMA1_CH3_R {
        DMA1_CH3_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 10 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_itline10](index.html) module"]
pub struct SYSCFG_ITLINE10_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_itline10::R](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG_ITLINE10 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
