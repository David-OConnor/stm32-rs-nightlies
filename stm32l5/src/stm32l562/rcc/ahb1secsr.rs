#[doc = "Register `AHB1SECSR` reader"]
pub struct R(crate::R<AHB1SECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1SECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1SECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1SECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA1SECF` reader - DMA1SECF"]
pub type DMA1SECF_R = crate::BitReader<bool>;
#[doc = "Field `DMA2SECF` reader - DMA2SECF"]
pub type DMA2SECF_R = crate::BitReader<bool>;
#[doc = "Field `DMAMUX1SECF` reader - DMAMUX1SECF"]
pub type DMAMUX1SECF_R = crate::BitReader<bool>;
#[doc = "Field `FLASHSECF` reader - FLASHSECF"]
pub type FLASHSECF_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1SECF` reader - SRAM1SECF"]
pub type SRAM1SECF_R = crate::BitReader<bool>;
#[doc = "Field `CRCSECF` reader - CRCSECF"]
pub type CRCSECF_R = crate::BitReader<bool>;
#[doc = "Field `TSCSECF` reader - TSCSECF"]
pub type TSCSECF_R = crate::BitReader<bool>;
#[doc = "Field `GTZCSECF` reader - GTZCSECF"]
pub type GTZCSECF_R = crate::BitReader<bool>;
#[doc = "Field `ICACHESECF` reader - ICACHESECF"]
pub type ICACHESECF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DMA1SECF"]
    #[inline(always)]
    pub fn dma1secf(&self) -> DMA1SECF_R {
        DMA1SECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2SECF"]
    #[inline(always)]
    pub fn dma2secf(&self) -> DMA2SECF_R {
        DMA2SECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX1SECF"]
    #[inline(always)]
    pub fn dmamux1secf(&self) -> DMAMUX1SECF_R {
        DMAMUX1SECF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - FLASHSECF"]
    #[inline(always)]
    pub fn flashsecf(&self) -> FLASHSECF_R {
        FLASHSECF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM1SECF"]
    #[inline(always)]
    pub fn sram1secf(&self) -> SRAM1SECF_R {
        SRAM1SECF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRCSECF"]
    #[inline(always)]
    pub fn crcsecf(&self) -> CRCSECF_R {
        CRCSECF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - TSCSECF"]
    #[inline(always)]
    pub fn tscsecf(&self) -> TSCSECF_R {
        TSCSECF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 22 - GTZCSECF"]
    #[inline(always)]
    pub fn gtzcsecf(&self) -> GTZCSECF_R {
        GTZCSECF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ICACHESECF"]
    #[inline(always)]
    pub fn icachesecf(&self) -> ICACHESECF_R {
        ICACHESECF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "RCC AHB1 security status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1secsr](index.html) module"]
pub struct AHB1SECSR_SPEC;
impl crate::RegisterSpec for AHB1SECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1secsr::R](R) reader structure"]
impl crate::Readable for AHB1SECSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AHB1SECSR to value 0x0040_0300"]
impl crate::Resettable for AHB1SECSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0300;
}
