#[doc = "Register `GTZC1_TZIC_IER3` reader"]
pub struct R(crate::R<GTZC1_TZIC_IER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZIC_IER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZIC_IER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZIC_IER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTZC1_TZIC_IER3` writer"]
pub struct W(crate::W<GTZC1_TZIC_IER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZIC_IER3_SPEC>;
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
impl From<crate::W<GTZC1_TZIC_IER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZIC_IER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM6IE` reader - illegal access interrupt enable for LPTIM6"]
pub type LPTIM6IE_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM6IE` writer - illegal access interrupt enable for LPTIM6"]
pub type LPTIM6IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `VREFBUFIE` reader - illegal access interrupt enable for VREFBUF"]
pub type VREFBUFIE_R = crate::BitReader<bool>;
#[doc = "Field `VREFBUFIE` writer - illegal access interrupt enable for VREFBUF"]
pub type VREFBUFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `CRCIE` reader - illegal access interrupt enable for CRC"]
pub type CRCIE_R = crate::BitReader<bool>;
#[doc = "Field `CRCIE` writer - illegal access interrupt enable for CRC"]
pub type CRCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `CORDICIE` reader - illegal access interrupt enable for CORDIC"]
pub type CORDICIE_R = crate::BitReader<bool>;
#[doc = "Field `CORDICIE` writer - illegal access interrupt enable for CORDIC"]
pub type CORDICIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `FMACIE` reader - illegal access interrupt enable for FMAC"]
pub type FMACIE_R = crate::BitReader<bool>;
#[doc = "Field `FMACIE` writer - illegal access interrupt enable for FMAC"]
pub type FMACIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `ETHIE` reader - illegal access interrupt enable for register of ETH"]
pub type ETHIE_R = crate::BitReader<bool>;
#[doc = "Field `ETHIE` writer - illegal access interrupt enable for register of ETH"]
pub type ETHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `ICACHEIE` reader - illegal access interrupt enable for ICACHE"]
pub type ICACHEIE_R = crate::BitReader<bool>;
#[doc = "Field `ICACHEIE` writer - illegal access interrupt enable for ICACHE"]
pub type ICACHEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `DCACHEIE` reader - illegal access interrupt enable for DCACHE"]
pub type DCACHEIE_R = crate::BitReader<bool>;
#[doc = "Field `DCACHEIE` writer - illegal access interrupt enable for DCACHE"]
pub type DCACHEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `ADC12IE` reader - illegal access interrupt enable for ADC1 and ADC2"]
pub type ADC12IE_R = crate::BitReader<bool>;
#[doc = "Field `ADC12IE` writer - illegal access interrupt enable for ADC1 and ADC2"]
pub type ADC12IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `DCMIIE` reader - illegal access interrupt enable for DCMI"]
pub type DCMIIE_R = crate::BitReader<bool>;
#[doc = "Field `DCMIIE` writer - illegal access interrupt enable for DCMI"]
pub type DCMIIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `AESIE` reader - illegal access interrupt enable for AES"]
pub type AESIE_R = crate::BitReader<bool>;
#[doc = "Field `AESIE` writer - illegal access interrupt enable for AES"]
pub type AESIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `HASHIE` reader - illegal access interrupt enable for HASH"]
pub type HASHIE_R = crate::BitReader<bool>;
#[doc = "Field `HASHIE` writer - illegal access interrupt enable for HASH"]
pub type HASHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `RNGIE` reader - illegal access interrupt enable for RNG"]
pub type RNGIE_R = crate::BitReader<bool>;
#[doc = "Field `RNGIE` writer - illegal access interrupt enable for RNG"]
pub type RNGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `SAESIE` reader - illegal access interrupt enable for SAES"]
pub type SAESIE_R = crate::BitReader<bool>;
#[doc = "Field `SAESIE` writer - illegal access interrupt enable for SAES"]
pub type SAESIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `PKAIE` reader - illegal access interrupt enable for PKA"]
pub type PKAIE_R = crate::BitReader<bool>;
#[doc = "Field `PKAIE` writer - illegal access interrupt enable for PKA"]
pub type PKAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `SDMMC2IE` reader - illegal access interrupt enable for SDMMC2"]
pub type SDMMC2IE_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC2IE` writer - illegal access interrupt enable for SDMMC2"]
pub type SDMMC2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `SDMMC1IE` reader - illegal access interrupt enable for SDMMC1"]
pub type SDMMC1IE_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC1IE` writer - illegal access interrupt enable for SDMMC1"]
pub type SDMMC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `FMCIE` reader - illegal access interrupt enable for FMC"]
pub type FMCIE_R = crate::BitReader<bool>;
#[doc = "Field `FMCIE` writer - illegal access interrupt enable for FMC"]
pub type FMCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `OCTOSPI1IE` reader - illegal access interrupt enable for OCTOSPI1"]
pub type OCTOSPI1IE_R = crate::BitReader<bool>;
#[doc = "Field `OCTOSPI1IE` writer - illegal access interrupt enable for OCTOSPI1"]
pub type OCTOSPI1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
#[doc = "Field `RAMCFGIE` reader - illegal access interrupt enable for RAMSCFG"]
pub type RAMCFGIE_R = crate::BitReader<bool>;
#[doc = "Field `RAMCFGIE` writer - illegal access interrupt enable for RAMSCFG"]
pub type RAMCFGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_IER3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - illegal access interrupt enable for LPTIM6"]
    #[inline(always)]
    pub fn lptim6ie(&self) -> LPTIM6IE_R {
        LPTIM6IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for VREFBUF"]
    #[inline(always)]
    pub fn vrefbufie(&self) -> VREFBUFIE_R {
        VREFBUFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for CRC"]
    #[inline(always)]
    pub fn crcie(&self) -> CRCIE_R {
        CRCIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for CORDIC"]
    #[inline(always)]
    pub fn cordicie(&self) -> CORDICIE_R {
        CORDICIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - illegal access interrupt enable for FMAC"]
    #[inline(always)]
    pub fn fmacie(&self) -> FMACIE_R {
        FMACIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for register of ETH"]
    #[inline(always)]
    pub fn ethie(&self) -> ETHIE_R {
        ETHIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - illegal access interrupt enable for ICACHE"]
    #[inline(always)]
    pub fn icacheie(&self) -> ICACHEIE_R {
        ICACHEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - illegal access interrupt enable for DCACHE"]
    #[inline(always)]
    pub fn dcacheie(&self) -> DCACHEIE_R {
        DCACHEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - illegal access interrupt enable for ADC1 and ADC2"]
    #[inline(always)]
    pub fn adc12ie(&self) -> ADC12IE_R {
        ADC12IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - illegal access interrupt enable for DCMI"]
    #[inline(always)]
    pub fn dcmiie(&self) -> DCMIIE_R {
        DCMIIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - illegal access interrupt enable for AES"]
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - illegal access interrupt enable for HASH"]
    #[inline(always)]
    pub fn hashie(&self) -> HASHIE_R {
        HASHIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - illegal access interrupt enable for RNG"]
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - illegal access interrupt enable for SAES"]
    #[inline(always)]
    pub fn saesie(&self) -> SAESIE_R {
        SAESIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - illegal access interrupt enable for PKA"]
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - illegal access interrupt enable for SDMMC2"]
    #[inline(always)]
    pub fn sdmmc2ie(&self) -> SDMMC2IE_R {
        SDMMC2IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - illegal access interrupt enable for SDMMC1"]
    #[inline(always)]
    pub fn sdmmc1ie(&self) -> SDMMC1IE_R {
        SDMMC1IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - illegal access interrupt enable for FMC"]
    #[inline(always)]
    pub fn fmcie(&self) -> FMCIE_R {
        FMCIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - illegal access interrupt enable for OCTOSPI1"]
    #[inline(always)]
    pub fn octospi1ie(&self) -> OCTOSPI1IE_R {
        OCTOSPI1IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - illegal access interrupt enable for RAMSCFG"]
    #[inline(always)]
    pub fn ramcfgie(&self) -> RAMCFGIE_R {
        RAMCFGIE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - illegal access interrupt enable for LPTIM6"]
    #[inline(always)]
    #[must_use]
    pub fn lptim6ie(&mut self) -> LPTIM6IE_W<0> {
        LPTIM6IE_W::new(self)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for VREFBUF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbufie(&mut self) -> VREFBUFIE_W<1> {
        VREFBUFIE_W::new(self)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcie(&mut self) -> CRCIE_W<8> {
        CRCIE_W::new(self)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for CORDIC"]
    #[inline(always)]
    #[must_use]
    pub fn cordicie(&mut self) -> CORDICIE_W<9> {
        CORDICIE_W::new(self)
    }
    #[doc = "Bit 10 - illegal access interrupt enable for FMAC"]
    #[inline(always)]
    #[must_use]
    pub fn fmacie(&mut self) -> FMACIE_W<10> {
        FMACIE_W::new(self)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for register of ETH"]
    #[inline(always)]
    #[must_use]
    pub fn ethie(&mut self) -> ETHIE_W<11> {
        ETHIE_W::new(self)
    }
    #[doc = "Bit 12 - illegal access interrupt enable for ICACHE"]
    #[inline(always)]
    #[must_use]
    pub fn icacheie(&mut self) -> ICACHEIE_W<12> {
        ICACHEIE_W::new(self)
    }
    #[doc = "Bit 13 - illegal access interrupt enable for DCACHE"]
    #[inline(always)]
    #[must_use]
    pub fn dcacheie(&mut self) -> DCACHEIE_W<13> {
        DCACHEIE_W::new(self)
    }
    #[doc = "Bit 14 - illegal access interrupt enable for ADC1 and ADC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc12ie(&mut self) -> ADC12IE_W<14> {
        ADC12IE_W::new(self)
    }
    #[doc = "Bit 15 - illegal access interrupt enable for DCMI"]
    #[inline(always)]
    #[must_use]
    pub fn dcmiie(&mut self) -> DCMIIE_W<15> {
        DCMIIE_W::new(self)
    }
    #[doc = "Bit 16 - illegal access interrupt enable for AES"]
    #[inline(always)]
    #[must_use]
    pub fn aesie(&mut self) -> AESIE_W<16> {
        AESIE_W::new(self)
    }
    #[doc = "Bit 17 - illegal access interrupt enable for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn hashie(&mut self) -> HASHIE_W<17> {
        HASHIE_W::new(self)
    }
    #[doc = "Bit 18 - illegal access interrupt enable for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rngie(&mut self) -> RNGIE_W<18> {
        RNGIE_W::new(self)
    }
    #[doc = "Bit 19 - illegal access interrupt enable for SAES"]
    #[inline(always)]
    #[must_use]
    pub fn saesie(&mut self) -> SAESIE_W<19> {
        SAESIE_W::new(self)
    }
    #[doc = "Bit 20 - illegal access interrupt enable for PKA"]
    #[inline(always)]
    #[must_use]
    pub fn pkaie(&mut self) -> PKAIE_W<20> {
        PKAIE_W::new(self)
    }
    #[doc = "Bit 21 - illegal access interrupt enable for SDMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2ie(&mut self) -> SDMMC2IE_W<21> {
        SDMMC2IE_W::new(self)
    }
    #[doc = "Bit 22 - illegal access interrupt enable for SDMMC1"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1ie(&mut self) -> SDMMC1IE_W<22> {
        SDMMC1IE_W::new(self)
    }
    #[doc = "Bit 23 - illegal access interrupt enable for FMC"]
    #[inline(always)]
    #[must_use]
    pub fn fmcie(&mut self) -> FMCIE_W<23> {
        FMCIE_W::new(self)
    }
    #[doc = "Bit 24 - illegal access interrupt enable for OCTOSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1ie(&mut self) -> OCTOSPI1IE_W<24> {
        OCTOSPI1IE_W::new(self)
    }
    #[doc = "Bit 26 - illegal access interrupt enable for RAMSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgie(&mut self) -> RAMCFGIE_W<26> {
        RAMCFGIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTZC1 TZIC interrupt enable register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzic_ier3](index.html) module"]
pub struct GTZC1_TZIC_IER3_SPEC;
impl crate::RegisterSpec for GTZC1_TZIC_IER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtzc1_tzic_ier3::R](R) reader structure"]
impl crate::Readable for GTZC1_TZIC_IER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzic_ier3::W](W) writer structure"]
impl crate::Writable for GTZC1_TZIC_IER3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZIC_IER3 to value 0"]
impl crate::Resettable for GTZC1_TZIC_IER3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
