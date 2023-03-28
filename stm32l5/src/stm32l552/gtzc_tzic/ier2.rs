#[doc = "Register `IER2` reader"]
pub struct R(crate::R<IER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER2` writer"]
pub struct W(crate::W<IER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM8IE` reader - TIM8IE"]
pub type TIM8IE_R = crate::BitReader<bool>;
#[doc = "Field `TIM8IE` writer - TIM8IE"]
pub type TIM8IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `USART1IE` reader - USART1IE"]
pub type USART1IE_R = crate::BitReader<bool>;
#[doc = "Field `USART1IE` writer - USART1IE"]
pub type USART1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `TIM15IE` reader - TIM15IE"]
pub type TIM15IE_R = crate::BitReader<bool>;
#[doc = "Field `TIM15IE` writer - TIM15IE"]
pub type TIM15IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `TIM16IE` reader - TIM16IE"]
pub type TIM16IE_R = crate::BitReader<bool>;
#[doc = "Field `TIM16IE` writer - TIM16IE"]
pub type TIM16IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `TIM17IE` reader - TIM17IE"]
pub type TIM17IE_R = crate::BitReader<bool>;
#[doc = "Field `TIM17IE` writer - TIM17IE"]
pub type TIM17IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `SAI1IE` reader - SAI1IE"]
pub type SAI1IE_R = crate::BitReader<bool>;
#[doc = "Field `SAI1IE` writer - SAI1IE"]
pub type SAI1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `SAI2IE` reader - SAI2IE"]
pub type SAI2IE_R = crate::BitReader<bool>;
#[doc = "Field `SAI2IE` writer - SAI2IE"]
pub type SAI2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `DFSDM1IE` reader - DFSDM1IE"]
pub type DFSDM1IE_R = crate::BitReader<bool>;
#[doc = "Field `DFSDM1IE` writer - DFSDM1IE"]
pub type DFSDM1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `CRCIE` reader - CRCIE"]
pub type CRCIE_R = crate::BitReader<bool>;
#[doc = "Field `CRCIE` writer - CRCIE"]
pub type CRCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `TSCIE` reader - TSCIE"]
pub type TSCIE_R = crate::BitReader<bool>;
#[doc = "Field `TSCIE` writer - TSCIE"]
pub type TSCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `ICACHEIE` reader - ICACHEIE"]
pub type ICACHEIE_R = crate::BitReader<bool>;
#[doc = "Field `ICACHEIE` writer - ICACHEIE"]
pub type ICACHEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `ADCIE` reader - ADCIE"]
pub type ADCIE_R = crate::BitReader<bool>;
#[doc = "Field `ADCIE` writer - ADCIE"]
pub type ADCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `AESIE` reader - AESIE"]
pub type AESIE_R = crate::BitReader<bool>;
#[doc = "Field `AESIE` writer - AESIE"]
pub type AESIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `HASHIE` reader - HASHIE"]
pub type HASHIE_R = crate::BitReader<bool>;
#[doc = "Field `HASHIE` writer - HASHIE"]
pub type HASHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `RNGIE` reader - RNGIE"]
pub type RNGIE_R = crate::BitReader<bool>;
#[doc = "Field `RNGIE` writer - RNGIE"]
pub type RNGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `PKAIE` reader - PKAIE"]
pub type PKAIE_R = crate::BitReader<bool>;
#[doc = "Field `PKAIE` writer - PKAIE"]
pub type PKAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `SDMMC1IE` reader - SDMMC1IE"]
pub type SDMMC1IE_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC1IE` writer - SDMMC1IE"]
pub type SDMMC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `FMC_REGIE` reader - FMC_REGIE"]
pub type FMC_REGIE_R = crate::BitReader<bool>;
#[doc = "Field `FMC_REGIE` writer - FMC_REGIE"]
pub type FMC_REGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `OCTOSPI1_REGIE` reader - OCTOSPI1_REGIE"]
pub type OCTOSPI1_REGIE_R = crate::BitReader<bool>;
#[doc = "Field `OCTOSPI1_REGIE` writer - OCTOSPI1_REGIE"]
pub type OCTOSPI1_REGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `RTCIE` reader - RTCIE"]
pub type RTCIE_R = crate::BitReader<bool>;
#[doc = "Field `RTCIE` writer - RTCIE"]
pub type RTCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `PWRIE` reader - PWRIE"]
pub type PWRIE_R = crate::BitReader<bool>;
#[doc = "Field `PWRIE` writer - PWRIE"]
pub type PWRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `SYSCFGIE` reader - SYSCFGIE"]
pub type SYSCFGIE_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGIE` writer - SYSCFGIE"]
pub type SYSCFGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `DMA1IE` reader - DMA1IE"]
pub type DMA1IE_R = crate::BitReader<bool>;
#[doc = "Field `DMA1IE` writer - DMA1IE"]
pub type DMA1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `DMA2IE` reader - DMA2IE"]
pub type DMA2IE_R = crate::BitReader<bool>;
#[doc = "Field `DMA2IE` writer - DMA2IE"]
pub type DMA2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `DMAMUX1IE` reader - DMAMUX1IE"]
pub type DMAMUX1IE_R = crate::BitReader<bool>;
#[doc = "Field `DMAMUX1IE` writer - DMAMUX1IE"]
pub type DMAMUX1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `RCCIE` reader - RCCIE"]
pub type RCCIE_R = crate::BitReader<bool>;
#[doc = "Field `RCCIE` writer - RCCIE"]
pub type RCCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `FLASHIE` reader - FLASHIE"]
pub type FLASHIE_R = crate::BitReader<bool>;
#[doc = "Field `FLASHIE` writer - FLASHIE"]
pub type FLASHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `FLASH_REGIE` reader - FLASH_REGIE"]
pub type FLASH_REGIE_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_REGIE` writer - FLASH_REGIE"]
pub type FLASH_REGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `EXTIIE` reader - EXTIIE"]
pub type EXTIIE_R = crate::BitReader<bool>;
#[doc = "Field `EXTIIE` writer - EXTIIE"]
pub type EXTIIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
#[doc = "Field `OTFDEC1IE` reader - OTFDEC1IE"]
pub type OTFDEC1IE_R = crate::BitReader<bool>;
#[doc = "Field `OTFDEC1IE` writer - OTFDEC1IE"]
pub type OTFDEC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM8IE"]
    #[inline(always)]
    pub fn tim8ie(&self) -> TIM8IE_R {
        TIM8IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART1IE"]
    #[inline(always)]
    pub fn usart1ie(&self) -> USART1IE_R {
        USART1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM15IE"]
    #[inline(always)]
    pub fn tim15ie(&self) -> TIM15IE_R {
        TIM15IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM16IE"]
    #[inline(always)]
    pub fn tim16ie(&self) -> TIM16IE_R {
        TIM16IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM17IE"]
    #[inline(always)]
    pub fn tim17ie(&self) -> TIM17IE_R {
        TIM17IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAI1IE"]
    #[inline(always)]
    pub fn sai1ie(&self) -> SAI1IE_R {
        SAI1IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SAI2IE"]
    #[inline(always)]
    pub fn sai2ie(&self) -> SAI2IE_R {
        SAI2IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DFSDM1IE"]
    #[inline(always)]
    pub fn dfsdm1ie(&self) -> DFSDM1IE_R {
        DFSDM1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CRCIE"]
    #[inline(always)]
    pub fn crcie(&self) -> CRCIE_R {
        CRCIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TSCIE"]
    #[inline(always)]
    pub fn tscie(&self) -> TSCIE_R {
        TSCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ICACHEIE"]
    #[inline(always)]
    pub fn icacheie(&self) -> ICACHEIE_R {
        ICACHEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADCIE"]
    #[inline(always)]
    pub fn adcie(&self) -> ADCIE_R {
        ADCIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AESIE"]
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HASHIE"]
    #[inline(always)]
    pub fn hashie(&self) -> HASHIE_R {
        HASHIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RNGIE"]
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PKAIE"]
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1IE"]
    #[inline(always)]
    pub fn sdmmc1ie(&self) -> SDMMC1IE_R {
        SDMMC1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FMC_REGIE"]
    #[inline(always)]
    pub fn fmc_regie(&self) -> FMC_REGIE_R {
        FMC_REGIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGIE"]
    #[inline(always)]
    pub fn octospi1_regie(&self) -> OCTOSPI1_REGIE_R {
        OCTOSPI1_REGIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RTCIE"]
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PWRIE"]
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SYSCFGIE"]
    #[inline(always)]
    pub fn syscfgie(&self) -> SYSCFGIE_R {
        SYSCFGIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1IE"]
    #[inline(always)]
    pub fn dma1ie(&self) -> DMA1IE_R {
        DMA1IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMA2IE"]
    #[inline(always)]
    pub fn dma2ie(&self) -> DMA2IE_R {
        DMA2IE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMAMUX1IE"]
    #[inline(always)]
    pub fn dmamux1ie(&self) -> DMAMUX1IE_R {
        DMAMUX1IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RCCIE"]
    #[inline(always)]
    pub fn rccie(&self) -> RCCIE_R {
        RCCIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FLASHIE"]
    #[inline(always)]
    pub fn flashie(&self) -> FLASHIE_R {
        FLASHIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FLASH_REGIE"]
    #[inline(always)]
    pub fn flash_regie(&self) -> FLASH_REGIE_R {
        FLASH_REGIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EXTIIE"]
    #[inline(always)]
    pub fn extiie(&self) -> EXTIIE_R {
        EXTIIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OTFDEC1IE"]
    #[inline(always)]
    pub fn otfdec1ie(&self) -> OTFDEC1IE_R {
        OTFDEC1IE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim8ie(&mut self) -> TIM8IE_W<0> {
        TIM8IE_W::new(self)
    }
    #[doc = "Bit 1 - USART1IE"]
    #[inline(always)]
    #[must_use]
    pub fn usart1ie(&mut self) -> USART1IE_W<1> {
        USART1IE_W::new(self)
    }
    #[doc = "Bit 2 - TIM15IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim15ie(&mut self) -> TIM15IE_W<2> {
        TIM15IE_W::new(self)
    }
    #[doc = "Bit 3 - TIM16IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim16ie(&mut self) -> TIM16IE_W<3> {
        TIM16IE_W::new(self)
    }
    #[doc = "Bit 4 - TIM17IE"]
    #[inline(always)]
    #[must_use]
    pub fn tim17ie(&mut self) -> TIM17IE_W<4> {
        TIM17IE_W::new(self)
    }
    #[doc = "Bit 5 - SAI1IE"]
    #[inline(always)]
    #[must_use]
    pub fn sai1ie(&mut self) -> SAI1IE_W<5> {
        SAI1IE_W::new(self)
    }
    #[doc = "Bit 6 - SAI2IE"]
    #[inline(always)]
    #[must_use]
    pub fn sai2ie(&mut self) -> SAI2IE_W<6> {
        SAI2IE_W::new(self)
    }
    #[doc = "Bit 7 - DFSDM1IE"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1ie(&mut self) -> DFSDM1IE_W<7> {
        DFSDM1IE_W::new(self)
    }
    #[doc = "Bit 8 - CRCIE"]
    #[inline(always)]
    #[must_use]
    pub fn crcie(&mut self) -> CRCIE_W<8> {
        CRCIE_W::new(self)
    }
    #[doc = "Bit 9 - TSCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tscie(&mut self) -> TSCIE_W<9> {
        TSCIE_W::new(self)
    }
    #[doc = "Bit 10 - ICACHEIE"]
    #[inline(always)]
    #[must_use]
    pub fn icacheie(&mut self) -> ICACHEIE_W<10> {
        ICACHEIE_W::new(self)
    }
    #[doc = "Bit 11 - ADCIE"]
    #[inline(always)]
    #[must_use]
    pub fn adcie(&mut self) -> ADCIE_W<11> {
        ADCIE_W::new(self)
    }
    #[doc = "Bit 12 - AESIE"]
    #[inline(always)]
    #[must_use]
    pub fn aesie(&mut self) -> AESIE_W<12> {
        AESIE_W::new(self)
    }
    #[doc = "Bit 13 - HASHIE"]
    #[inline(always)]
    #[must_use]
    pub fn hashie(&mut self) -> HASHIE_W<13> {
        HASHIE_W::new(self)
    }
    #[doc = "Bit 14 - RNGIE"]
    #[inline(always)]
    #[must_use]
    pub fn rngie(&mut self) -> RNGIE_W<14> {
        RNGIE_W::new(self)
    }
    #[doc = "Bit 15 - PKAIE"]
    #[inline(always)]
    #[must_use]
    pub fn pkaie(&mut self) -> PKAIE_W<15> {
        PKAIE_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC1IE"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1ie(&mut self) -> SDMMC1IE_W<16> {
        SDMMC1IE_W::new(self)
    }
    #[doc = "Bit 17 - FMC_REGIE"]
    #[inline(always)]
    #[must_use]
    pub fn fmc_regie(&mut self) -> FMC_REGIE_W<17> {
        FMC_REGIE_W::new(self)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGIE"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1_regie(&mut self) -> OCTOSPI1_REGIE_W<18> {
        OCTOSPI1_REGIE_W::new(self)
    }
    #[doc = "Bit 19 - RTCIE"]
    #[inline(always)]
    #[must_use]
    pub fn rtcie(&mut self) -> RTCIE_W<19> {
        RTCIE_W::new(self)
    }
    #[doc = "Bit 20 - PWRIE"]
    #[inline(always)]
    #[must_use]
    pub fn pwrie(&mut self) -> PWRIE_W<20> {
        PWRIE_W::new(self)
    }
    #[doc = "Bit 21 - SYSCFGIE"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgie(&mut self) -> SYSCFGIE_W<21> {
        SYSCFGIE_W::new(self)
    }
    #[doc = "Bit 22 - DMA1IE"]
    #[inline(always)]
    #[must_use]
    pub fn dma1ie(&mut self) -> DMA1IE_W<22> {
        DMA1IE_W::new(self)
    }
    #[doc = "Bit 23 - DMA2IE"]
    #[inline(always)]
    #[must_use]
    pub fn dma2ie(&mut self) -> DMA2IE_W<23> {
        DMA2IE_W::new(self)
    }
    #[doc = "Bit 24 - DMAMUX1IE"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1ie(&mut self) -> DMAMUX1IE_W<24> {
        DMAMUX1IE_W::new(self)
    }
    #[doc = "Bit 25 - RCCIE"]
    #[inline(always)]
    #[must_use]
    pub fn rccie(&mut self) -> RCCIE_W<25> {
        RCCIE_W::new(self)
    }
    #[doc = "Bit 26 - FLASHIE"]
    #[inline(always)]
    #[must_use]
    pub fn flashie(&mut self) -> FLASHIE_W<26> {
        FLASHIE_W::new(self)
    }
    #[doc = "Bit 27 - FLASH_REGIE"]
    #[inline(always)]
    #[must_use]
    pub fn flash_regie(&mut self) -> FLASH_REGIE_W<27> {
        FLASH_REGIE_W::new(self)
    }
    #[doc = "Bit 28 - EXTIIE"]
    #[inline(always)]
    #[must_use]
    pub fn extiie(&mut self) -> EXTIIE_W<28> {
        EXTIIE_W::new(self)
    }
    #[doc = "Bit 29 - OTFDEC1IE"]
    #[inline(always)]
    #[must_use]
    pub fn otfdec1ie(&mut self) -> OTFDEC1IE_W<29> {
        OTFDEC1IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier2](index.html) module"]
pub struct IER2_SPEC;
impl crate::RegisterSpec for IER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier2::R](R) reader structure"]
impl crate::Readable for IER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier2::W](W) writer structure"]
impl crate::Writable for IER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER2 to value 0"]
impl crate::Resettable for IER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
