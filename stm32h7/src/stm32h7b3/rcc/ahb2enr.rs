#[doc = "Register `AHB2ENR` reader"]
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2ENR` writer"]
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCMI_PSSIEN` reader - digital camera interface peripheral clock enable (DCMI or PSSI depending which IP is active) Set and reset by software."]
pub type DCMI_PSSIEN_R = crate::BitReader<DCMI_PSSIEN_A>;
#[doc = "digital camera interface peripheral clock enable (DCMI or PSSI depending which IP is active) Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMI_PSSIEN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<DCMI_PSSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMI_PSSIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMI_PSSIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMI_PSSIEN_A {
        match self.bits {
            false => DCMI_PSSIEN_A::Disabled,
            true => DCMI_PSSIEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMI_PSSIEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMI_PSSIEN_A::Enabled
    }
}
#[doc = "Field `DCMI_PSSIEN` writer - digital camera interface peripheral clock enable (DCMI or PSSI depending which IP is active) Set and reset by software."]
pub type DCMI_PSSIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, DCMI_PSSIEN_A, O>;
impl<'a, const O: u8> DCMI_PSSIEN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCMI_PSSIEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCMI_PSSIEN_A::Enabled)
    }
}
#[doc = "Field `HSEMEN` reader - HSEM peripheral clock enable Set and reset by software."]
pub use DCMI_PSSIEN_R as HSEMEN_R;
#[doc = "Field `CRYPTEN` reader - CRYPT peripheral clock enable Set and reset by software."]
pub use DCMI_PSSIEN_R as CRYPTEN_R;
#[doc = "Field `HASHEN` reader - HASH peripheral clock enable Set and reset by software."]
pub use DCMI_PSSIEN_R as HASHEN_R;
#[doc = "Field `RNGEN` reader - RNG peripheral clocks enable Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
pub use DCMI_PSSIEN_R as RNGEN_R;
#[doc = "Field `SDMMC2EN` reader - SDMMC2 and SDMMC2 delay clock enable Set and reset by software."]
pub use DCMI_PSSIEN_R as SDMMC2EN_R;
#[doc = "Field `BDMA1EN` reader - DMA clock enable (DFSDM dedicated DMA) Set and reset by software."]
pub use DCMI_PSSIEN_R as BDMA1EN_R;
#[doc = "Field `AHBSRAM1EN` reader - AHBSRAM1 block enable Set and reset by software. When set, this bit indicates that the SRAM1 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun."]
pub use DCMI_PSSIEN_R as AHBSRAM1EN_R;
#[doc = "Field `AHBSRAM2EN` reader - AHBSRAM2 block enable Set and reset by software. When set, this bit indicates that the SRAM2 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun."]
pub use DCMI_PSSIEN_R as AHBSRAM2EN_R;
#[doc = "Field `HSEMEN` writer - HSEM peripheral clock enable Set and reset by software."]
pub use DCMI_PSSIEN_W as HSEMEN_W;
#[doc = "Field `CRYPTEN` writer - CRYPT peripheral clock enable Set and reset by software."]
pub use DCMI_PSSIEN_W as CRYPTEN_W;
#[doc = "Field `HASHEN` writer - HASH peripheral clock enable Set and reset by software."]
pub use DCMI_PSSIEN_W as HASHEN_W;
#[doc = "Field `RNGEN` writer - RNG peripheral clocks enable Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
pub use DCMI_PSSIEN_W as RNGEN_W;
#[doc = "Field `SDMMC2EN` writer - SDMMC2 and SDMMC2 delay clock enable Set and reset by software."]
pub use DCMI_PSSIEN_W as SDMMC2EN_W;
#[doc = "Field `BDMA1EN` writer - DMA clock enable (DFSDM dedicated DMA) Set and reset by software."]
pub use DCMI_PSSIEN_W as BDMA1EN_W;
#[doc = "Field `AHBSRAM1EN` writer - AHBSRAM1 block enable Set and reset by software. When set, this bit indicates that the SRAM1 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun."]
pub use DCMI_PSSIEN_W as AHBSRAM1EN_W;
#[doc = "Field `AHBSRAM2EN` writer - AHBSRAM2 block enable Set and reset by software. When set, this bit indicates that the SRAM2 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun."]
pub use DCMI_PSSIEN_W as AHBSRAM2EN_W;
impl R {
    #[doc = "Bit 0 - digital camera interface peripheral clock enable (DCMI or PSSI depending which IP is active) Set and reset by software."]
    #[inline(always)]
    pub fn dcmi_pssien(&self) -> DCMI_PSSIEN_R {
        DCMI_PSSIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - HSEM peripheral clock enable Set and reset by software."]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable Set and reset by software."]
    #[inline(always)]
    pub fn crypten(&self) -> CRYPTEN_R {
        CRYPTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable Set and reset by software."]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clocks enable Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA clock enable (DFSDM dedicated DMA) Set and reset by software."]
    #[inline(always)]
    pub fn bdma1en(&self) -> BDMA1EN_R {
        BDMA1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 29 - AHBSRAM1 block enable Set and reset by software. When set, this bit indicates that the SRAM1 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun."]
    #[inline(always)]
    pub fn ahbsram1en(&self) -> AHBSRAM1EN_R {
        AHBSRAM1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AHBSRAM2 block enable Set and reset by software. When set, this bit indicates that the SRAM2 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun."]
    #[inline(always)]
    pub fn ahbsram2en(&self) -> AHBSRAM2EN_R {
        AHBSRAM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - digital camera interface peripheral clock enable (DCMI or PSSI depending which IP is active) Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssien(&mut self) -> DCMI_PSSIEN_W<0> {
        DCMI_PSSIEN_W::new(self)
    }
    #[doc = "Bit 2 - HSEM peripheral clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hsemen(&mut self) -> HSEMEN_W<2> {
        HSEMEN_W::new(self)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crypten(&mut self) -> CRYPTEN_W<4> {
        CRYPTEN_W::new(self)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<5> {
        HASHEN_W::new(self)
    }
    #[doc = "Bit 6 - RNG peripheral clocks enable Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<6> {
        RNGEN_W::new(self)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<9> {
        SDMMC2EN_W::new(self)
    }
    #[doc = "Bit 11 - DMA clock enable (DFSDM dedicated DMA) Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn bdma1en(&mut self) -> BDMA1EN_W<11> {
        BDMA1EN_W::new(self)
    }
    #[doc = "Bit 29 - AHBSRAM1 block enable Set and reset by software. When set, this bit indicates that the SRAM1 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun."]
    #[inline(always)]
    #[must_use]
    pub fn ahbsram1en(&mut self) -> AHBSRAM1EN_W<29> {
        AHBSRAM1EN_W::new(self)
    }
    #[doc = "Bit 30 - AHBSRAM2 block enable Set and reset by software. When set, this bit indicates that the SRAM2 is allocated by the CPU. It causes the CPU domain to take into account also the CPU operation modes, keeping the CPU domain in DRun when the CPU is in CRun."]
    #[inline(always)]
    #[must_use]
    pub fn ahbsram2en(&mut self) -> AHBSRAM2EN_W<30> {
        AHBSRAM2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](index.html) module"]
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2enr::R](R) reader structure"]
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](W) writer structure"]
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
