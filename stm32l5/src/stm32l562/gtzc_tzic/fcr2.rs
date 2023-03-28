#[doc = "Register `FCR2` reader"]
pub struct R(crate::R<FCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR2` writer"]
pub struct W(crate::W<FCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR2_SPEC>;
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
impl From<crate::W<FCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM8FC` reader - TIM8FC"]
pub type TIM8FC_R = crate::BitReader<bool>;
#[doc = "Field `TIM8FC` writer - TIM8FC"]
pub type TIM8FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `USART1FC` reader - USART1FC"]
pub type USART1FC_R = crate::BitReader<bool>;
#[doc = "Field `USART1FC` writer - USART1FC"]
pub type USART1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `TIM15FC` reader - TIM15FC"]
pub type TIM15FC_R = crate::BitReader<bool>;
#[doc = "Field `TIM15FC` writer - TIM15FC"]
pub type TIM15FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `TIM16FC` reader - TIM16FC"]
pub type TIM16FC_R = crate::BitReader<bool>;
#[doc = "Field `TIM16FC` writer - TIM16FC"]
pub type TIM16FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `TIM17FC` reader - TIM17FC"]
pub type TIM17FC_R = crate::BitReader<bool>;
#[doc = "Field `TIM17FC` writer - TIM17FC"]
pub type TIM17FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `SAI1FC` reader - SAI1FC"]
pub type SAI1FC_R = crate::BitReader<bool>;
#[doc = "Field `SAI1FC` writer - SAI1FC"]
pub type SAI1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `SAI2FC` reader - SAI2FC"]
pub type SAI2FC_R = crate::BitReader<bool>;
#[doc = "Field `SAI2FC` writer - SAI2FC"]
pub type SAI2FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `DFSDM1FC` reader - DFSDM1FC"]
pub type DFSDM1FC_R = crate::BitReader<bool>;
#[doc = "Field `DFSDM1FC` writer - DFSDM1FC"]
pub type DFSDM1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `CRCFC` reader - CRCFC"]
pub type CRCFC_R = crate::BitReader<bool>;
#[doc = "Field `CRCFC` writer - CRCFC"]
pub type CRCFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `TSCFC` reader - TSCFC"]
pub type TSCFC_R = crate::BitReader<bool>;
#[doc = "Field `TSCFC` writer - TSCFC"]
pub type TSCFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `ICACHEFC` reader - ICACHEFC"]
pub type ICACHEFC_R = crate::BitReader<bool>;
#[doc = "Field `ICACHEFC` writer - ICACHEFC"]
pub type ICACHEFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `ADCFC` reader - ADCFC"]
pub type ADCFC_R = crate::BitReader<bool>;
#[doc = "Field `ADCFC` writer - ADCFC"]
pub type ADCFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `AESFC` reader - AESFC"]
pub type AESFC_R = crate::BitReader<bool>;
#[doc = "Field `AESFC` writer - AESFC"]
pub type AESFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `HASHFC` reader - HASHFC"]
pub type HASHFC_R = crate::BitReader<bool>;
#[doc = "Field `HASHFC` writer - HASHFC"]
pub type HASHFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `RNGFC` reader - RNGFC"]
pub type RNGFC_R = crate::BitReader<bool>;
#[doc = "Field `RNGFC` writer - RNGFC"]
pub type RNGFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `PKAFC` reader - PKAFC"]
pub type PKAFC_R = crate::BitReader<bool>;
#[doc = "Field `PKAFC` writer - PKAFC"]
pub type PKAFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `SDMMC1FC` reader - SDMMC1FC"]
pub type SDMMC1FC_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC1FC` writer - SDMMC1FC"]
pub type SDMMC1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `FMC_REGFC` reader - FMC_REGFC"]
pub type FMC_REGFC_R = crate::BitReader<bool>;
#[doc = "Field `FMC_REGFC` writer - FMC_REGFC"]
pub type FMC_REGFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `OCTOSPI1_REGFC` reader - OCTOSPI1_REGFC"]
pub type OCTOSPI1_REGFC_R = crate::BitReader<bool>;
#[doc = "Field `OCTOSPI1_REGFC` writer - OCTOSPI1_REGFC"]
pub type OCTOSPI1_REGFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `RTCFC` reader - RTCFC"]
pub type RTCFC_R = crate::BitReader<bool>;
#[doc = "Field `RTCFC` writer - RTCFC"]
pub type RTCFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `PWRFC` reader - PWRFC"]
pub type PWRFC_R = crate::BitReader<bool>;
#[doc = "Field `PWRFC` writer - PWRFC"]
pub type PWRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `SYSCFGFC` reader - SYSCFGFC"]
pub type SYSCFGFC_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGFC` writer - SYSCFGFC"]
pub type SYSCFGFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `DMA1FC` reader - DMA1FC"]
pub type DMA1FC_R = crate::BitReader<bool>;
#[doc = "Field `DMA1FC` writer - DMA1FC"]
pub type DMA1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `DMA2FC` reader - DMA2FC"]
pub type DMA2FC_R = crate::BitReader<bool>;
#[doc = "Field `DMA2FC` writer - DMA2FC"]
pub type DMA2FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `DMAMUX1FC` reader - DMAMUX1FC"]
pub type DMAMUX1FC_R = crate::BitReader<bool>;
#[doc = "Field `DMAMUX1FC` writer - DMAMUX1FC"]
pub type DMAMUX1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `RCCFC` reader - RCCFC"]
pub type RCCFC_R = crate::BitReader<bool>;
#[doc = "Field `RCCFC` writer - RCCFC"]
pub type RCCFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `FLASHFC` reader - FLASHFC"]
pub type FLASHFC_R = crate::BitReader<bool>;
#[doc = "Field `FLASHFC` writer - FLASHFC"]
pub type FLASHFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `FLASH_REGFC` reader - FLASH_REGFC"]
pub type FLASH_REGFC_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_REGFC` writer - FLASH_REGFC"]
pub type FLASH_REGFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `EXTIFC` reader - EXTIFC"]
pub type EXTIFC_R = crate::BitReader<bool>;
#[doc = "Field `EXTIFC` writer - EXTIFC"]
pub type EXTIFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
#[doc = "Field `OTFDEC1FC` reader - OTFDEC1FC"]
pub type OTFDEC1FC_R = crate::BitReader<bool>;
#[doc = "Field `OTFDEC1FC` writer - OTFDEC1FC"]
pub type OTFDEC1FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM8FC"]
    #[inline(always)]
    pub fn tim8fc(&self) -> TIM8FC_R {
        TIM8FC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART1FC"]
    #[inline(always)]
    pub fn usart1fc(&self) -> USART1FC_R {
        USART1FC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM15FC"]
    #[inline(always)]
    pub fn tim15fc(&self) -> TIM15FC_R {
        TIM15FC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM16FC"]
    #[inline(always)]
    pub fn tim16fc(&self) -> TIM16FC_R {
        TIM16FC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM17FC"]
    #[inline(always)]
    pub fn tim17fc(&self) -> TIM17FC_R {
        TIM17FC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAI1FC"]
    #[inline(always)]
    pub fn sai1fc(&self) -> SAI1FC_R {
        SAI1FC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SAI2FC"]
    #[inline(always)]
    pub fn sai2fc(&self) -> SAI2FC_R {
        SAI2FC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DFSDM1FC"]
    #[inline(always)]
    pub fn dfsdm1fc(&self) -> DFSDM1FC_R {
        DFSDM1FC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CRCFC"]
    #[inline(always)]
    pub fn crcfc(&self) -> CRCFC_R {
        CRCFC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TSCFC"]
    #[inline(always)]
    pub fn tscfc(&self) -> TSCFC_R {
        TSCFC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ICACHEFC"]
    #[inline(always)]
    pub fn icachefc(&self) -> ICACHEFC_R {
        ICACHEFC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADCFC"]
    #[inline(always)]
    pub fn adcfc(&self) -> ADCFC_R {
        ADCFC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AESFC"]
    #[inline(always)]
    pub fn aesfc(&self) -> AESFC_R {
        AESFC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HASHFC"]
    #[inline(always)]
    pub fn hashfc(&self) -> HASHFC_R {
        HASHFC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RNGFC"]
    #[inline(always)]
    pub fn rngfc(&self) -> RNGFC_R {
        RNGFC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PKAFC"]
    #[inline(always)]
    pub fn pkafc(&self) -> PKAFC_R {
        PKAFC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1FC"]
    #[inline(always)]
    pub fn sdmmc1fc(&self) -> SDMMC1FC_R {
        SDMMC1FC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FMC_REGFC"]
    #[inline(always)]
    pub fn fmc_regfc(&self) -> FMC_REGFC_R {
        FMC_REGFC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGFC"]
    #[inline(always)]
    pub fn octospi1_regfc(&self) -> OCTOSPI1_REGFC_R {
        OCTOSPI1_REGFC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RTCFC"]
    #[inline(always)]
    pub fn rtcfc(&self) -> RTCFC_R {
        RTCFC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PWRFC"]
    #[inline(always)]
    pub fn pwrfc(&self) -> PWRFC_R {
        PWRFC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SYSCFGFC"]
    #[inline(always)]
    pub fn syscfgfc(&self) -> SYSCFGFC_R {
        SYSCFGFC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1FC"]
    #[inline(always)]
    pub fn dma1fc(&self) -> DMA1FC_R {
        DMA1FC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMA2FC"]
    #[inline(always)]
    pub fn dma2fc(&self) -> DMA2FC_R {
        DMA2FC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMAMUX1FC"]
    #[inline(always)]
    pub fn dmamux1fc(&self) -> DMAMUX1FC_R {
        DMAMUX1FC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RCCFC"]
    #[inline(always)]
    pub fn rccfc(&self) -> RCCFC_R {
        RCCFC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FLASHFC"]
    #[inline(always)]
    pub fn flashfc(&self) -> FLASHFC_R {
        FLASHFC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FLASH_REGFC"]
    #[inline(always)]
    pub fn flash_regfc(&self) -> FLASH_REGFC_R {
        FLASH_REGFC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EXTIFC"]
    #[inline(always)]
    pub fn extifc(&self) -> EXTIFC_R {
        EXTIFC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OTFDEC1FC"]
    #[inline(always)]
    pub fn otfdec1fc(&self) -> OTFDEC1FC_R {
        OTFDEC1FC_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim8fc(&mut self) -> TIM8FC_W<0> {
        TIM8FC_W::new(self)
    }
    #[doc = "Bit 1 - USART1FC"]
    #[inline(always)]
    #[must_use]
    pub fn usart1fc(&mut self) -> USART1FC_W<1> {
        USART1FC_W::new(self)
    }
    #[doc = "Bit 2 - TIM15FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim15fc(&mut self) -> TIM15FC_W<2> {
        TIM15FC_W::new(self)
    }
    #[doc = "Bit 3 - TIM16FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim16fc(&mut self) -> TIM16FC_W<3> {
        TIM16FC_W::new(self)
    }
    #[doc = "Bit 4 - TIM17FC"]
    #[inline(always)]
    #[must_use]
    pub fn tim17fc(&mut self) -> TIM17FC_W<4> {
        TIM17FC_W::new(self)
    }
    #[doc = "Bit 5 - SAI1FC"]
    #[inline(always)]
    #[must_use]
    pub fn sai1fc(&mut self) -> SAI1FC_W<5> {
        SAI1FC_W::new(self)
    }
    #[doc = "Bit 6 - SAI2FC"]
    #[inline(always)]
    #[must_use]
    pub fn sai2fc(&mut self) -> SAI2FC_W<6> {
        SAI2FC_W::new(self)
    }
    #[doc = "Bit 7 - DFSDM1FC"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1fc(&mut self) -> DFSDM1FC_W<7> {
        DFSDM1FC_W::new(self)
    }
    #[doc = "Bit 8 - CRCFC"]
    #[inline(always)]
    #[must_use]
    pub fn crcfc(&mut self) -> CRCFC_W<8> {
        CRCFC_W::new(self)
    }
    #[doc = "Bit 9 - TSCFC"]
    #[inline(always)]
    #[must_use]
    pub fn tscfc(&mut self) -> TSCFC_W<9> {
        TSCFC_W::new(self)
    }
    #[doc = "Bit 10 - ICACHEFC"]
    #[inline(always)]
    #[must_use]
    pub fn icachefc(&mut self) -> ICACHEFC_W<10> {
        ICACHEFC_W::new(self)
    }
    #[doc = "Bit 11 - ADCFC"]
    #[inline(always)]
    #[must_use]
    pub fn adcfc(&mut self) -> ADCFC_W<11> {
        ADCFC_W::new(self)
    }
    #[doc = "Bit 12 - AESFC"]
    #[inline(always)]
    #[must_use]
    pub fn aesfc(&mut self) -> AESFC_W<12> {
        AESFC_W::new(self)
    }
    #[doc = "Bit 13 - HASHFC"]
    #[inline(always)]
    #[must_use]
    pub fn hashfc(&mut self) -> HASHFC_W<13> {
        HASHFC_W::new(self)
    }
    #[doc = "Bit 14 - RNGFC"]
    #[inline(always)]
    #[must_use]
    pub fn rngfc(&mut self) -> RNGFC_W<14> {
        RNGFC_W::new(self)
    }
    #[doc = "Bit 15 - PKAFC"]
    #[inline(always)]
    #[must_use]
    pub fn pkafc(&mut self) -> PKAFC_W<15> {
        PKAFC_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC1FC"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1fc(&mut self) -> SDMMC1FC_W<16> {
        SDMMC1FC_W::new(self)
    }
    #[doc = "Bit 17 - FMC_REGFC"]
    #[inline(always)]
    #[must_use]
    pub fn fmc_regfc(&mut self) -> FMC_REGFC_W<17> {
        FMC_REGFC_W::new(self)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGFC"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1_regfc(&mut self) -> OCTOSPI1_REGFC_W<18> {
        OCTOSPI1_REGFC_W::new(self)
    }
    #[doc = "Bit 19 - RTCFC"]
    #[inline(always)]
    #[must_use]
    pub fn rtcfc(&mut self) -> RTCFC_W<19> {
        RTCFC_W::new(self)
    }
    #[doc = "Bit 20 - PWRFC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrfc(&mut self) -> PWRFC_W<20> {
        PWRFC_W::new(self)
    }
    #[doc = "Bit 21 - SYSCFGFC"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgfc(&mut self) -> SYSCFGFC_W<21> {
        SYSCFGFC_W::new(self)
    }
    #[doc = "Bit 22 - DMA1FC"]
    #[inline(always)]
    #[must_use]
    pub fn dma1fc(&mut self) -> DMA1FC_W<22> {
        DMA1FC_W::new(self)
    }
    #[doc = "Bit 23 - DMA2FC"]
    #[inline(always)]
    #[must_use]
    pub fn dma2fc(&mut self) -> DMA2FC_W<23> {
        DMA2FC_W::new(self)
    }
    #[doc = "Bit 24 - DMAMUX1FC"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1fc(&mut self) -> DMAMUX1FC_W<24> {
        DMAMUX1FC_W::new(self)
    }
    #[doc = "Bit 25 - RCCFC"]
    #[inline(always)]
    #[must_use]
    pub fn rccfc(&mut self) -> RCCFC_W<25> {
        RCCFC_W::new(self)
    }
    #[doc = "Bit 26 - FLASHFC"]
    #[inline(always)]
    #[must_use]
    pub fn flashfc(&mut self) -> FLASHFC_W<26> {
        FLASHFC_W::new(self)
    }
    #[doc = "Bit 27 - FLASH_REGFC"]
    #[inline(always)]
    #[must_use]
    pub fn flash_regfc(&mut self) -> FLASH_REGFC_W<27> {
        FLASH_REGFC_W::new(self)
    }
    #[doc = "Bit 28 - EXTIFC"]
    #[inline(always)]
    #[must_use]
    pub fn extifc(&mut self) -> EXTIFC_W<28> {
        EXTIFC_W::new(self)
    }
    #[doc = "Bit 29 - OTFDEC1FC"]
    #[inline(always)]
    #[must_use]
    pub fn otfdec1fc(&mut self) -> OTFDEC1FC_W<29> {
        OTFDEC1FC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt clear register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr2](index.html) module"]
pub struct FCR2_SPEC;
impl crate::RegisterSpec for FCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcr2::R](R) reader structure"]
impl crate::Readable for FCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr2::W](W) writer structure"]
impl crate::Writable for FCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR2 to value 0"]
impl crate::Resettable for FCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
