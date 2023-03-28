#[doc = "Register `GTZC1_TZIC_FCR4` writer"]
pub struct W(crate::W<GTZC1_TZIC_FCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZIC_FCR4_SPEC>;
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
impl From<crate::W<GTZC1_TZIC_FCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZIC_FCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CGPDMA1F` writer - clear the illegal access flag for GPDMA1"]
pub type CGPDMA1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CGPDMA2F` writer - clear the illegal access flag for GPDMA2"]
pub type CGPDMA2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CFLASH_REGF` writer - clear the illegal access flag for FLASH registers"]
pub type CFLASH_REGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CFLASHF` writer - clear the illegal access flag for FLASH memory"]
pub type CFLASHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CSBSF` writer - clear the illegal access flag for SBS"]
pub type CSBSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CRTCF` writer - clear the illegal access flag for RTC"]
pub type CRTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CTAMPF` writer - clear the illegal access flag for TAMP"]
pub type CTAMPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CPWRF` writer - clear the illegal access flag for PWR"]
pub type CPWRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CRCCF` writer - clear the illegal access flag for RCC"]
pub type CRCCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CEXTIF` writer - clear the illegal access flag for EXTI"]
pub type CEXTIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CTZSC1F` writer - clear the illegal access flag for GTZC1 TZSC registers"]
pub type CTZSC1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CTZIC1F` writer - clear the illegal access flag for GTZC1 TZIC registers"]
pub type CTZIC1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `COCTOSPI1_MEMF` writer - clear the illegal access flag for MPCWM1 (OCTOSPI1) memory bank"]
pub type COCTOSPI1_MEMF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CFMC_MEMF` writer - clear the illegal access flag for MPCWM2 (FMC_NOR bank), MPCWM3 (FMC_NAND bank and FMC_SDRAM bank 1), and MPCWM4 (FMC_SDRAM�bank 2)"]
pub type CFMC_MEMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CBKPSRAMF` writer - clear the illegal access flag for MPCWM4 (BKPSRAM) memory bank"]
pub type CBKPSRAMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CSRAM1F` writer - clear the illegal access flag for SRAM1"]
pub type CSRAM1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CMPCBB1_REGF` writer - clear the illegal access flag for MPCBB1 registers"]
pub type CMPCBB1_REGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CSRAM2F` writer - clear the illegal access flag for SRAM2"]
pub type CSRAM2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CMPCBB2_REGF` writer - clear the illegal access flag for MPCBB2 registers"]
pub type CMPCBB2_REGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CSRAM3F` writer - clear the illegal access flag for SRAM3"]
pub type CSRAM3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
#[doc = "Field `CMPCBB3_REGF` writer - clear the illegal access flag for MPCBB3 registers"]
pub type CMPCBB3_REGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR4_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - clear the illegal access flag for GPDMA1"]
    #[inline(always)]
    #[must_use]
    pub fn cgpdma1f(&mut self) -> CGPDMA1F_W<0> {
        CGPDMA1F_W::new(self)
    }
    #[doc = "Bit 1 - clear the illegal access flag for GPDMA2"]
    #[inline(always)]
    #[must_use]
    pub fn cgpdma2f(&mut self) -> CGPDMA2F_W<1> {
        CGPDMA2F_W::new(self)
    }
    #[doc = "Bit 2 - clear the illegal access flag for FLASH registers"]
    #[inline(always)]
    #[must_use]
    pub fn cflash_regf(&mut self) -> CFLASH_REGF_W<2> {
        CFLASH_REGF_W::new(self)
    }
    #[doc = "Bit 3 - clear the illegal access flag for FLASH memory"]
    #[inline(always)]
    #[must_use]
    pub fn cflashf(&mut self) -> CFLASHF_W<3> {
        CFLASHF_W::new(self)
    }
    #[doc = "Bit 6 - clear the illegal access flag for SBS"]
    #[inline(always)]
    #[must_use]
    pub fn csbsf(&mut self) -> CSBSF_W<6> {
        CSBSF_W::new(self)
    }
    #[doc = "Bit 7 - clear the illegal access flag for RTC"]
    #[inline(always)]
    #[must_use]
    pub fn crtcf(&mut self) -> CRTCF_W<7> {
        CRTCF_W::new(self)
    }
    #[doc = "Bit 8 - clear the illegal access flag for TAMP"]
    #[inline(always)]
    #[must_use]
    pub fn ctampf(&mut self) -> CTAMPF_W<8> {
        CTAMPF_W::new(self)
    }
    #[doc = "Bit 9 - clear the illegal access flag for PWR"]
    #[inline(always)]
    #[must_use]
    pub fn cpwrf(&mut self) -> CPWRF_W<9> {
        CPWRF_W::new(self)
    }
    #[doc = "Bit 10 - clear the illegal access flag for RCC"]
    #[inline(always)]
    #[must_use]
    pub fn crccf(&mut self) -> CRCCF_W<10> {
        CRCCF_W::new(self)
    }
    #[doc = "Bit 11 - clear the illegal access flag for EXTI"]
    #[inline(always)]
    #[must_use]
    pub fn cextif(&mut self) -> CEXTIF_W<11> {
        CEXTIF_W::new(self)
    }
    #[doc = "Bit 16 - clear the illegal access flag for GTZC1 TZSC registers"]
    #[inline(always)]
    #[must_use]
    pub fn ctzsc1f(&mut self) -> CTZSC1F_W<16> {
        CTZSC1F_W::new(self)
    }
    #[doc = "Bit 17 - clear the illegal access flag for GTZC1 TZIC registers"]
    #[inline(always)]
    #[must_use]
    pub fn ctzic1f(&mut self) -> CTZIC1F_W<17> {
        CTZIC1F_W::new(self)
    }
    #[doc = "Bit 18 - clear the illegal access flag for MPCWM1 (OCTOSPI1) memory bank"]
    #[inline(always)]
    #[must_use]
    pub fn coctospi1_memf(&mut self) -> COCTOSPI1_MEMF_W<18> {
        COCTOSPI1_MEMF_W::new(self)
    }
    #[doc = "Bit 19 - clear the illegal access flag for MPCWM2 (FMC_NOR bank), MPCWM3 (FMC_NAND bank and FMC_SDRAM bank 1), and MPCWM4 (FMC_SDRAM�bank 2)"]
    #[inline(always)]
    #[must_use]
    pub fn cfmc_memf(&mut self) -> CFMC_MEMF_W<19> {
        CFMC_MEMF_W::new(self)
    }
    #[doc = "Bit 20 - clear the illegal access flag for MPCWM4 (BKPSRAM) memory bank"]
    #[inline(always)]
    #[must_use]
    pub fn cbkpsramf(&mut self) -> CBKPSRAMF_W<20> {
        CBKPSRAMF_W::new(self)
    }
    #[doc = "Bit 24 - clear the illegal access flag for SRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn csram1f(&mut self) -> CSRAM1F_W<24> {
        CSRAM1F_W::new(self)
    }
    #[doc = "Bit 25 - clear the illegal access flag for MPCBB1 registers"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcbb1_regf(&mut self) -> CMPCBB1_REGF_W<25> {
        CMPCBB1_REGF_W::new(self)
    }
    #[doc = "Bit 26 - clear the illegal access flag for SRAM2"]
    #[inline(always)]
    #[must_use]
    pub fn csram2f(&mut self) -> CSRAM2F_W<26> {
        CSRAM2F_W::new(self)
    }
    #[doc = "Bit 27 - clear the illegal access flag for MPCBB2 registers"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcbb2_regf(&mut self) -> CMPCBB2_REGF_W<27> {
        CMPCBB2_REGF_W::new(self)
    }
    #[doc = "Bit 28 - clear the illegal access flag for SRAM3"]
    #[inline(always)]
    #[must_use]
    pub fn csram3f(&mut self) -> CSRAM3F_W<28> {
        CSRAM3F_W::new(self)
    }
    #[doc = "Bit 29 - clear the illegal access flag for MPCBB3 registers"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcbb3_regf(&mut self) -> CMPCBB3_REGF_W<29> {
        CMPCBB3_REGF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTZC1 TZIC flag clear register 4\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzic_fcr4](index.html) module"]
pub struct GTZC1_TZIC_FCR4_SPEC;
impl crate::RegisterSpec for GTZC1_TZIC_FCR4_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzic_fcr4::W](W) writer structure"]
impl crate::Writable for GTZC1_TZIC_FCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZIC_FCR4 to value 0"]
impl crate::Resettable for GTZC1_TZIC_FCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
