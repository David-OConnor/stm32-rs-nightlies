#[doc = "Register `SYSCFG_ITLINE9` reader"]
pub struct R(crate::R<SYSCFG_ITLINE9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_ITLINE9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_ITLINE9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_ITLINE9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA1_CH1` reader - DMA1 channel 1interrupt request pending"]
pub type DMA1_CH1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DMA1 channel 1interrupt request pending"]
    #[inline(always)]
    pub fn dma1_ch1(&self) -> DMA1_CH1_R {
        DMA1_CH1_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 9 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_itline9](index.html) module"]
pub struct SYSCFG_ITLINE9_SPEC;
impl crate::RegisterSpec for SYSCFG_ITLINE9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_itline9::R](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE9_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCFG_ITLINE9 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
