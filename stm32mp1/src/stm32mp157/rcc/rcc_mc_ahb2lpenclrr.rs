#[doc = "Register `RCC_MC_AHB2LPENCLRR` reader"]
pub struct R(crate::R<RCC_MC_AHB2LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AHB2LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AHB2LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AHB2LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_AHB2LPENCLRR` writer"]
pub struct W(crate::W<RCC_MC_AHB2LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AHB2LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MC_AHB2LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AHB2LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1LPEN` reader - DMA1LPEN"]
pub type DMA1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `DMA1LPEN` writer - DMA1LPEN"]
pub type DMA1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `DMA2LPEN` reader - DMA2LPEN"]
pub type DMA2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `DMA2LPEN` writer - DMA2LPEN"]
pub type DMA2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `DMAMUXLPEN` reader - DMAMUXLPEN"]
pub type DMAMUXLPEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAMUXLPEN` writer - DMAMUXLPEN"]
pub type DMAMUXLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_AHB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `ADC12LPEN` reader - ADC12LPEN"]
pub type ADC12LPEN_R = crate::BitReader<bool>;
#[doc = "Field `ADC12LPEN` writer - ADC12LPEN"]
pub type ADC12LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_AHB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `USBOLPEN` reader - USBOLPEN"]
pub type USBOLPEN_R = crate::BitReader<bool>;
#[doc = "Field `USBOLPEN` writer - USBOLPEN"]
pub type USBOLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB2LPENCLRR_SPEC, bool, O>;
#[doc = "Field `SDMMC3LPEN` reader - SDMMC3LPEN"]
pub type SDMMC3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC3LPEN` writer - SDMMC3LPEN"]
pub type SDMMC3LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_AHB2LPENCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DMA1LPEN"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2LPEN"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUXLPEN"]
    #[inline(always)]
    pub fn dmamuxlpen(&self) -> DMAMUXLPEN_R {
        DMAMUXLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC12LPEN"]
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USBOLPEN"]
    #[inline(always)]
    pub fn usbolpen(&self) -> USBOLPEN_R {
        USBOLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC3LPEN"]
    #[inline(always)]
    pub fn sdmmc3lpen(&self) -> SDMMC3LPEN_R {
        SDMMC3LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<0> {
        DMA1LPEN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<1> {
        DMA2LPEN_W::new(self)
    }
    #[doc = "Bit 2 - DMAMUXLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmamuxlpen(&mut self) -> DMAMUXLPEN_W<2> {
        DMAMUXLPEN_W::new(self)
    }
    #[doc = "Bit 5 - ADC12LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W<5> {
        ADC12LPEN_W::new(self)
    }
    #[doc = "Bit 8 - USBOLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn usbolpen(&mut self) -> USBOLPEN_W<8> {
        USBOLPEN_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC3LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc3lpen(&mut self) -> SDMMC3LPEN_W<16> {
        SDMMC3LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb2lpenclrr](index.html) module"]
pub struct RCC_MC_AHB2LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MC_AHB2LPENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_ahb2lpenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MC_AHB2LPENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb2lpenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MC_AHB2LPENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MC_AHB2LPENCLRR to value 0x0001_0127"]
impl crate::Resettable for RCC_MC_AHB2LPENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0127;
}
