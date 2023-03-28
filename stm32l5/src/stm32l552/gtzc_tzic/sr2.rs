#[doc = "Register `SR2` reader"]
pub struct R(crate::R<SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR2` writer"]
pub struct W(crate::W<SR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR2_SPEC>;
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
impl From<crate::W<SR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM8F` reader - TIM8F"]
pub type TIM8F_R = crate::BitReader<bool>;
#[doc = "Field `TIM8F` writer - TIM8F"]
pub type TIM8F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `USART1F` reader - USART1F"]
pub type USART1F_R = crate::BitReader<bool>;
#[doc = "Field `USART1F` writer - USART1F"]
pub type USART1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `TIM15F` reader - TIM15F"]
pub type TIM15F_R = crate::BitReader<bool>;
#[doc = "Field `TIM15F` writer - TIM15F"]
pub type TIM15F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `TIM16F` reader - TIM16F"]
pub type TIM16F_R = crate::BitReader<bool>;
#[doc = "Field `TIM16F` writer - TIM16F"]
pub type TIM16F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `TIM17F` reader - TIM17F"]
pub type TIM17F_R = crate::BitReader<bool>;
#[doc = "Field `TIM17F` writer - TIM17F"]
pub type TIM17F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `SAI1F` reader - SAI1F"]
pub type SAI1F_R = crate::BitReader<bool>;
#[doc = "Field `SAI1F` writer - SAI1F"]
pub type SAI1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `SAI2F` reader - SAI2F"]
pub type SAI2F_R = crate::BitReader<bool>;
#[doc = "Field `SAI2F` writer - SAI2F"]
pub type SAI2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `DFSDM1F` reader - DFSDM1F"]
pub type DFSDM1F_R = crate::BitReader<bool>;
#[doc = "Field `DFSDM1F` writer - DFSDM1F"]
pub type DFSDM1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `CRCF` reader - CRCF"]
pub type CRCF_R = crate::BitReader<bool>;
#[doc = "Field `CRCF` writer - CRCF"]
pub type CRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `TSCF` reader - TSCF"]
pub type TSCF_R = crate::BitReader<bool>;
#[doc = "Field `TSCF` writer - TSCF"]
pub type TSCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `ICACHEF` reader - ICACHEF"]
pub type ICACHEF_R = crate::BitReader<bool>;
#[doc = "Field `ICACHEF` writer - ICACHEF"]
pub type ICACHEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `ADCF` reader - ADCF"]
pub type ADCF_R = crate::BitReader<bool>;
#[doc = "Field `ADCF` writer - ADCF"]
pub type ADCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `AESF` reader - AESF"]
pub type AESF_R = crate::BitReader<bool>;
#[doc = "Field `AESF` writer - AESF"]
pub type AESF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `HASHF` reader - HASHF"]
pub type HASHF_R = crate::BitReader<bool>;
#[doc = "Field `HASHF` writer - HASHF"]
pub type HASHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `RNGF` reader - RNGF"]
pub type RNGF_R = crate::BitReader<bool>;
#[doc = "Field `RNGF` writer - RNGF"]
pub type RNGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `PKAF` reader - PKAF"]
pub type PKAF_R = crate::BitReader<bool>;
#[doc = "Field `PKAF` writer - PKAF"]
pub type PKAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `SDMMC1F` reader - SDMMC1F"]
pub type SDMMC1F_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC1F` writer - SDMMC1F"]
pub type SDMMC1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `FMC_REGF` reader - FMC_REGF"]
pub type FMC_REGF_R = crate::BitReader<bool>;
#[doc = "Field `FMC_REGF` writer - FMC_REGF"]
pub type FMC_REGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `OCTOSPI1_REGF` reader - OCTOSPI1_REGF"]
pub type OCTOSPI1_REGF_R = crate::BitReader<bool>;
#[doc = "Field `OCTOSPI1_REGF` writer - OCTOSPI1_REGF"]
pub type OCTOSPI1_REGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `RTCF` reader - RTCF"]
pub type RTCF_R = crate::BitReader<bool>;
#[doc = "Field `RTCF` writer - RTCF"]
pub type RTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `PWRF` reader - PWRF"]
pub type PWRF_R = crate::BitReader<bool>;
#[doc = "Field `PWRF` writer - PWRF"]
pub type PWRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `SYSCFGF` reader - SYSCFGF"]
pub type SYSCFGF_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGF` writer - SYSCFGF"]
pub type SYSCFGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `DMA1F` reader - DMA1F"]
pub type DMA1F_R = crate::BitReader<bool>;
#[doc = "Field `DMA1F` writer - DMA1F"]
pub type DMA1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `DMA2F` reader - DMA2F"]
pub type DMA2F_R = crate::BitReader<bool>;
#[doc = "Field `DMA2F` writer - DMA2F"]
pub type DMA2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `DMAMUX1F` reader - DMAMUX1F"]
pub type DMAMUX1F_R = crate::BitReader<bool>;
#[doc = "Field `DMAMUX1F` writer - DMAMUX1F"]
pub type DMAMUX1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `RCCF` reader - RCCF"]
pub type RCCF_R = crate::BitReader<bool>;
#[doc = "Field `RCCF` writer - RCCF"]
pub type RCCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `FLASHF` reader - FLASHF"]
pub type FLASHF_R = crate::BitReader<bool>;
#[doc = "Field `FLASHF` writer - FLASHF"]
pub type FLASHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `FLASH_REGF` reader - FLASH_REGF"]
pub type FLASH_REGF_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_REGF` writer - FLASH_REGF"]
pub type FLASH_REGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `EXTIF` reader - EXTIF"]
pub type EXTIF_R = crate::BitReader<bool>;
#[doc = "Field `EXTIF` writer - EXTIF"]
pub type EXTIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
#[doc = "Field `OTFDEC1F` reader - OTFDEC1F"]
pub type OTFDEC1F_R = crate::BitReader<bool>;
#[doc = "Field `OTFDEC1F` writer - OTFDEC1F"]
pub type OTFDEC1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM8F"]
    #[inline(always)]
    pub fn tim8f(&self) -> TIM8F_R {
        TIM8F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART1F"]
    #[inline(always)]
    pub fn usart1f(&self) -> USART1F_R {
        USART1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM15F"]
    #[inline(always)]
    pub fn tim15f(&self) -> TIM15F_R {
        TIM15F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM16F"]
    #[inline(always)]
    pub fn tim16f(&self) -> TIM16F_R {
        TIM16F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM17F"]
    #[inline(always)]
    pub fn tim17f(&self) -> TIM17F_R {
        TIM17F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAI1F"]
    #[inline(always)]
    pub fn sai1f(&self) -> SAI1F_R {
        SAI1F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SAI2F"]
    #[inline(always)]
    pub fn sai2f(&self) -> SAI2F_R {
        SAI2F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DFSDM1F"]
    #[inline(always)]
    pub fn dfsdm1f(&self) -> DFSDM1F_R {
        DFSDM1F_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CRCF"]
    #[inline(always)]
    pub fn crcf(&self) -> CRCF_R {
        CRCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TSCF"]
    #[inline(always)]
    pub fn tscf(&self) -> TSCF_R {
        TSCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ICACHEF"]
    #[inline(always)]
    pub fn icachef(&self) -> ICACHEF_R {
        ICACHEF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADCF"]
    #[inline(always)]
    pub fn adcf(&self) -> ADCF_R {
        ADCF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AESF"]
    #[inline(always)]
    pub fn aesf(&self) -> AESF_R {
        AESF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HASHF"]
    #[inline(always)]
    pub fn hashf(&self) -> HASHF_R {
        HASHF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RNGF"]
    #[inline(always)]
    pub fn rngf(&self) -> RNGF_R {
        RNGF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PKAF"]
    #[inline(always)]
    pub fn pkaf(&self) -> PKAF_R {
        PKAF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1F"]
    #[inline(always)]
    pub fn sdmmc1f(&self) -> SDMMC1F_R {
        SDMMC1F_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FMC_REGF"]
    #[inline(always)]
    pub fn fmc_regf(&self) -> FMC_REGF_R {
        FMC_REGF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGF"]
    #[inline(always)]
    pub fn octospi1_regf(&self) -> OCTOSPI1_REGF_R {
        OCTOSPI1_REGF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RTCF"]
    #[inline(always)]
    pub fn rtcf(&self) -> RTCF_R {
        RTCF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PWRF"]
    #[inline(always)]
    pub fn pwrf(&self) -> PWRF_R {
        PWRF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SYSCFGF"]
    #[inline(always)]
    pub fn syscfgf(&self) -> SYSCFGF_R {
        SYSCFGF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1F"]
    #[inline(always)]
    pub fn dma1f(&self) -> DMA1F_R {
        DMA1F_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMA2F"]
    #[inline(always)]
    pub fn dma2f(&self) -> DMA2F_R {
        DMA2F_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMAMUX1F"]
    #[inline(always)]
    pub fn dmamux1f(&self) -> DMAMUX1F_R {
        DMAMUX1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RCCF"]
    #[inline(always)]
    pub fn rccf(&self) -> RCCF_R {
        RCCF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FLASHF"]
    #[inline(always)]
    pub fn flashf(&self) -> FLASHF_R {
        FLASHF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FLASH_REGF"]
    #[inline(always)]
    pub fn flash_regf(&self) -> FLASH_REGF_R {
        FLASH_REGF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EXTIF"]
    #[inline(always)]
    pub fn extif(&self) -> EXTIF_R {
        EXTIF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OTFDEC1F"]
    #[inline(always)]
    pub fn otfdec1f(&self) -> OTFDEC1F_R {
        OTFDEC1F_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8F"]
    #[inline(always)]
    #[must_use]
    pub fn tim8f(&mut self) -> TIM8F_W<0> {
        TIM8F_W::new(self)
    }
    #[doc = "Bit 1 - USART1F"]
    #[inline(always)]
    #[must_use]
    pub fn usart1f(&mut self) -> USART1F_W<1> {
        USART1F_W::new(self)
    }
    #[doc = "Bit 2 - TIM15F"]
    #[inline(always)]
    #[must_use]
    pub fn tim15f(&mut self) -> TIM15F_W<2> {
        TIM15F_W::new(self)
    }
    #[doc = "Bit 3 - TIM16F"]
    #[inline(always)]
    #[must_use]
    pub fn tim16f(&mut self) -> TIM16F_W<3> {
        TIM16F_W::new(self)
    }
    #[doc = "Bit 4 - TIM17F"]
    #[inline(always)]
    #[must_use]
    pub fn tim17f(&mut self) -> TIM17F_W<4> {
        TIM17F_W::new(self)
    }
    #[doc = "Bit 5 - SAI1F"]
    #[inline(always)]
    #[must_use]
    pub fn sai1f(&mut self) -> SAI1F_W<5> {
        SAI1F_W::new(self)
    }
    #[doc = "Bit 6 - SAI2F"]
    #[inline(always)]
    #[must_use]
    pub fn sai2f(&mut self) -> SAI2F_W<6> {
        SAI2F_W::new(self)
    }
    #[doc = "Bit 7 - DFSDM1F"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1f(&mut self) -> DFSDM1F_W<7> {
        DFSDM1F_W::new(self)
    }
    #[doc = "Bit 8 - CRCF"]
    #[inline(always)]
    #[must_use]
    pub fn crcf(&mut self) -> CRCF_W<8> {
        CRCF_W::new(self)
    }
    #[doc = "Bit 9 - TSCF"]
    #[inline(always)]
    #[must_use]
    pub fn tscf(&mut self) -> TSCF_W<9> {
        TSCF_W::new(self)
    }
    #[doc = "Bit 10 - ICACHEF"]
    #[inline(always)]
    #[must_use]
    pub fn icachef(&mut self) -> ICACHEF_W<10> {
        ICACHEF_W::new(self)
    }
    #[doc = "Bit 11 - ADCF"]
    #[inline(always)]
    #[must_use]
    pub fn adcf(&mut self) -> ADCF_W<11> {
        ADCF_W::new(self)
    }
    #[doc = "Bit 12 - AESF"]
    #[inline(always)]
    #[must_use]
    pub fn aesf(&mut self) -> AESF_W<12> {
        AESF_W::new(self)
    }
    #[doc = "Bit 13 - HASHF"]
    #[inline(always)]
    #[must_use]
    pub fn hashf(&mut self) -> HASHF_W<13> {
        HASHF_W::new(self)
    }
    #[doc = "Bit 14 - RNGF"]
    #[inline(always)]
    #[must_use]
    pub fn rngf(&mut self) -> RNGF_W<14> {
        RNGF_W::new(self)
    }
    #[doc = "Bit 15 - PKAF"]
    #[inline(always)]
    #[must_use]
    pub fn pkaf(&mut self) -> PKAF_W<15> {
        PKAF_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC1F"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1f(&mut self) -> SDMMC1F_W<16> {
        SDMMC1F_W::new(self)
    }
    #[doc = "Bit 17 - FMC_REGF"]
    #[inline(always)]
    #[must_use]
    pub fn fmc_regf(&mut self) -> FMC_REGF_W<17> {
        FMC_REGF_W::new(self)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGF"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1_regf(&mut self) -> OCTOSPI1_REGF_W<18> {
        OCTOSPI1_REGF_W::new(self)
    }
    #[doc = "Bit 19 - RTCF"]
    #[inline(always)]
    #[must_use]
    pub fn rtcf(&mut self) -> RTCF_W<19> {
        RTCF_W::new(self)
    }
    #[doc = "Bit 20 - PWRF"]
    #[inline(always)]
    #[must_use]
    pub fn pwrf(&mut self) -> PWRF_W<20> {
        PWRF_W::new(self)
    }
    #[doc = "Bit 21 - SYSCFGF"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgf(&mut self) -> SYSCFGF_W<21> {
        SYSCFGF_W::new(self)
    }
    #[doc = "Bit 22 - DMA1F"]
    #[inline(always)]
    #[must_use]
    pub fn dma1f(&mut self) -> DMA1F_W<22> {
        DMA1F_W::new(self)
    }
    #[doc = "Bit 23 - DMA2F"]
    #[inline(always)]
    #[must_use]
    pub fn dma2f(&mut self) -> DMA2F_W<23> {
        DMA2F_W::new(self)
    }
    #[doc = "Bit 24 - DMAMUX1F"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1f(&mut self) -> DMAMUX1F_W<24> {
        DMAMUX1F_W::new(self)
    }
    #[doc = "Bit 25 - RCCF"]
    #[inline(always)]
    #[must_use]
    pub fn rccf(&mut self) -> RCCF_W<25> {
        RCCF_W::new(self)
    }
    #[doc = "Bit 26 - FLASHF"]
    #[inline(always)]
    #[must_use]
    pub fn flashf(&mut self) -> FLASHF_W<26> {
        FLASHF_W::new(self)
    }
    #[doc = "Bit 27 - FLASH_REGF"]
    #[inline(always)]
    #[must_use]
    pub fn flash_regf(&mut self) -> FLASH_REGF_W<27> {
        FLASH_REGF_W::new(self)
    }
    #[doc = "Bit 28 - EXTIF"]
    #[inline(always)]
    #[must_use]
    pub fn extif(&mut self) -> EXTIF_W<28> {
        EXTIF_W::new(self)
    }
    #[doc = "Bit 29 - OTFDEC1F"]
    #[inline(always)]
    #[must_use]
    pub fn otfdec1f(&mut self) -> OTFDEC1F_W<29> {
        OTFDEC1F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt status register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr2](index.html) module"]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr2::R](R) reader structure"]
impl crate::Readable for SR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr2::W](W) writer structure"]
impl crate::Writable for SR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
