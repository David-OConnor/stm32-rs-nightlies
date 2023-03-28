#[doc = "Register `GTZC1_TZIC_FCR3` writer"]
pub struct W(crate::W<GTZC1_TZIC_FCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZIC_FCR3_SPEC>;
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
impl From<crate::W<GTZC1_TZIC_FCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZIC_FCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLPTIM6F` writer - clear illegal access flag for LPTIM6"]
pub type CLPTIM6F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CVREFBUFF` writer - clear illegal access flag for VREFBUF"]
pub type CVREFBUFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CCRCF` writer - clear illegal access flag for CRC"]
pub type CCRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CCORDICF` writer - clear illegal access flag for CORDIC"]
pub type CCORDICF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CFMACF` writer - clear illegal access flag for FMAC"]
pub type CFMACF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CETHF` writer - clear illegal access flag for register of ETH"]
pub type CETHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CICACHEF` writer - clear illegal access flag for ICACHE"]
pub type CICACHEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CDCACHEF` writer - clear illegal access flag for DCACHE"]
pub type CDCACHEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CADC12F` writer - clear illegal access flag for ADC1 and ADC2"]
pub type CADC12F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CDCMIF` writer - clear illegal access flag for DCMI"]
pub type CDCMIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CAESF` writer - clear illegal access flag for AES"]
pub type CAESF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CHASHF` writer - clear illegal access flag for HASH"]
pub type CHASHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CRNGF` writer - clear illegal access flag for RNG"]
pub type CRNGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CSAESF` writer - clear illegal access flag for SAES"]
pub type CSAESF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CPKAF` writer - clear illegal access flag for PKA"]
pub type CPKAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CSDMMC2F` writer - clear illegal access flag for SDMMC2"]
pub type CSDMMC2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CSDMMC1F` writer - clear illegal access flag for SDMMC1"]
pub type CSDMMC1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CFMCF` writer - clear illegal access flag for FMC"]
pub type CFMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `COCTOSPI1F` writer - clear illegal access flag for OCTOSPI1"]
pub type COCTOSPI1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
#[doc = "Field `CRAMCFGF` writer - clear illegal access flag for RAMSCFG"]
pub type CRAMCFGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZIC_FCR3_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - clear illegal access flag for LPTIM6"]
    #[inline(always)]
    #[must_use]
    pub fn clptim6f(&mut self) -> CLPTIM6F_W<0> {
        CLPTIM6F_W::new(self)
    }
    #[doc = "Bit 1 - clear illegal access flag for VREFBUF"]
    #[inline(always)]
    #[must_use]
    pub fn cvrefbuff(&mut self) -> CVREFBUFF_W<1> {
        CVREFBUFF_W::new(self)
    }
    #[doc = "Bit 8 - clear illegal access flag for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn ccrcf(&mut self) -> CCRCF_W<8> {
        CCRCF_W::new(self)
    }
    #[doc = "Bit 9 - clear illegal access flag for CORDIC"]
    #[inline(always)]
    #[must_use]
    pub fn ccordicf(&mut self) -> CCORDICF_W<9> {
        CCORDICF_W::new(self)
    }
    #[doc = "Bit 10 - clear illegal access flag for FMAC"]
    #[inline(always)]
    #[must_use]
    pub fn cfmacf(&mut self) -> CFMACF_W<10> {
        CFMACF_W::new(self)
    }
    #[doc = "Bit 11 - clear illegal access flag for register of ETH"]
    #[inline(always)]
    #[must_use]
    pub fn cethf(&mut self) -> CETHF_W<11> {
        CETHF_W::new(self)
    }
    #[doc = "Bit 12 - clear illegal access flag for ICACHE"]
    #[inline(always)]
    #[must_use]
    pub fn cicachef(&mut self) -> CICACHEF_W<12> {
        CICACHEF_W::new(self)
    }
    #[doc = "Bit 13 - clear illegal access flag for DCACHE"]
    #[inline(always)]
    #[must_use]
    pub fn cdcachef(&mut self) -> CDCACHEF_W<13> {
        CDCACHEF_W::new(self)
    }
    #[doc = "Bit 14 - clear illegal access flag for ADC1 and ADC2"]
    #[inline(always)]
    #[must_use]
    pub fn cadc12f(&mut self) -> CADC12F_W<14> {
        CADC12F_W::new(self)
    }
    #[doc = "Bit 15 - clear illegal access flag for DCMI"]
    #[inline(always)]
    #[must_use]
    pub fn cdcmif(&mut self) -> CDCMIF_W<15> {
        CDCMIF_W::new(self)
    }
    #[doc = "Bit 16 - clear illegal access flag for AES"]
    #[inline(always)]
    #[must_use]
    pub fn caesf(&mut self) -> CAESF_W<16> {
        CAESF_W::new(self)
    }
    #[doc = "Bit 17 - clear illegal access flag for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn chashf(&mut self) -> CHASHF_W<17> {
        CHASHF_W::new(self)
    }
    #[doc = "Bit 18 - clear illegal access flag for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn crngf(&mut self) -> CRNGF_W<18> {
        CRNGF_W::new(self)
    }
    #[doc = "Bit 19 - clear illegal access flag for SAES"]
    #[inline(always)]
    #[must_use]
    pub fn csaesf(&mut self) -> CSAESF_W<19> {
        CSAESF_W::new(self)
    }
    #[doc = "Bit 20 - clear illegal access flag for PKA"]
    #[inline(always)]
    #[must_use]
    pub fn cpkaf(&mut self) -> CPKAF_W<20> {
        CPKAF_W::new(self)
    }
    #[doc = "Bit 21 - clear illegal access flag for SDMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn csdmmc2f(&mut self) -> CSDMMC2F_W<21> {
        CSDMMC2F_W::new(self)
    }
    #[doc = "Bit 22 - clear illegal access flag for SDMMC1"]
    #[inline(always)]
    #[must_use]
    pub fn csdmmc1f(&mut self) -> CSDMMC1F_W<22> {
        CSDMMC1F_W::new(self)
    }
    #[doc = "Bit 23 - clear illegal access flag for FMC"]
    #[inline(always)]
    #[must_use]
    pub fn cfmcf(&mut self) -> CFMCF_W<23> {
        CFMCF_W::new(self)
    }
    #[doc = "Bit 24 - clear illegal access flag for OCTOSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn coctospi1f(&mut self) -> COCTOSPI1F_W<24> {
        COCTOSPI1F_W::new(self)
    }
    #[doc = "Bit 26 - clear illegal access flag for RAMSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn cramcfgf(&mut self) -> CRAMCFGF_W<26> {
        CRAMCFGF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC flag clear register 3\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzic_fcr3](index.html) module"]
pub struct GTZC1_TZIC_FCR3_SPEC;
impl crate::RegisterSpec for GTZC1_TZIC_FCR3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzic_fcr3::W](W) writer structure"]
impl crate::Writable for GTZC1_TZIC_FCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZIC_FCR3 to value 0"]
impl crate::Resettable for GTZC1_TZIC_FCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
