#[doc = "Register `GTZC1_TZSC_SECCFGR3` reader"]
pub struct R(crate::R<GTZC1_TZSC_SECCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZSC_SECCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZSC_SECCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZSC_SECCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTZC1_TZSC_SECCFGR3` writer"]
pub struct W(crate::W<GTZC1_TZSC_SECCFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZSC_SECCFGR3_SPEC>;
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
impl From<crate::W<GTZC1_TZSC_SECCFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZSC_SECCFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM6SEC` reader - secure access mode for LPTIM6"]
pub type LPTIM6SEC_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM6SEC` writer - secure access mode for LPTIM6"]
pub type LPTIM6SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `VREFBUFSEC` reader - secure access mode for VREFBUF"]
pub type VREFBUFSEC_R = crate::BitReader<bool>;
#[doc = "Field `VREFBUFSEC` writer - secure access mode for VREFBUF"]
pub type VREFBUFSEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `CRCSEC` reader - secure access mode for CRC"]
pub type CRCSEC_R = crate::BitReader<bool>;
#[doc = "Field `CRCSEC` writer - secure access mode for CRC"]
pub type CRCSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `CORDICSEC` reader - secure access mode for CORDIC"]
pub type CORDICSEC_R = crate::BitReader<bool>;
#[doc = "Field `CORDICSEC` writer - secure access mode for CORDIC"]
pub type CORDICSEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `FMACSEC` reader - secure access mode for FMAC"]
pub type FMACSEC_R = crate::BitReader<bool>;
#[doc = "Field `FMACSEC` writer - secure access mode for FMAC"]
pub type FMACSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `ETHSEC` reader - secure access mode for register of ETH"]
pub type ETHSEC_R = crate::BitReader<bool>;
#[doc = "Field `ETHSEC` writer - secure access mode for register of ETH"]
pub type ETHSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `ICACHESEC` reader - secure access mode for ICACHE"]
pub type ICACHESEC_R = crate::BitReader<bool>;
#[doc = "Field `ICACHESEC` writer - secure access mode for ICACHE"]
pub type ICACHESEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `DCACHESEC` reader - secure access mode for DCACHE"]
pub type DCACHESEC_R = crate::BitReader<bool>;
#[doc = "Field `DCACHESEC` writer - secure access mode for DCACHE"]
pub type DCACHESEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `ADC12SEC` reader - secure access mode for ADC1 and ADC2"]
pub type ADC12SEC_R = crate::BitReader<bool>;
#[doc = "Field `ADC12SEC` writer - secure access mode for ADC1 and ADC2"]
pub type ADC12SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `DCMISEC` reader - secure access mode for DCMI"]
pub type DCMISEC_R = crate::BitReader<bool>;
#[doc = "Field `DCMISEC` writer - secure access mode for DCMI"]
pub type DCMISEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `AESSEC` reader - secure access mode for AES"]
pub type AESSEC_R = crate::BitReader<bool>;
#[doc = "Field `AESSEC` writer - secure access mode for AES"]
pub type AESSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `HASHSEC` reader - secure access mode for HASH"]
pub type HASHSEC_R = crate::BitReader<bool>;
#[doc = "Field `HASHSEC` writer - secure access mode for HASH"]
pub type HASHSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `RNGSEC` reader - secure access mode for RNG"]
pub type RNGSEC_R = crate::BitReader<bool>;
#[doc = "Field `RNGSEC` writer - secure access mode for RNG"]
pub type RNGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `SAESSEC` reader - secure access mode for SAES"]
pub type SAESSEC_R = crate::BitReader<bool>;
#[doc = "Field `SAESSEC` writer - secure access mode for SAES"]
pub type SAESSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `PKASEC` reader - secure access mode for PKA"]
pub type PKASEC_R = crate::BitReader<bool>;
#[doc = "Field `PKASEC` writer - secure access mode for PKA"]
pub type PKASEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `SDMMC2SEC` reader - secure access mode for SDMMC2"]
pub type SDMMC2SEC_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC2SEC` writer - secure access mode for SDMMC2"]
pub type SDMMC2SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `SDMMC1SEC` reader - secure access mode for SDMMC1"]
pub type SDMMC1SEC_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC1SEC` writer - secure access mode for SDMMC1"]
pub type SDMMC1SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `FMCSEC` reader - secure access mode for FMC"]
pub type FMCSEC_R = crate::BitReader<bool>;
#[doc = "Field `FMCSEC` writer - secure access mode for FMC"]
pub type FMCSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `OCTOSPI1SEC` reader - secure access mode for OCTOSPI1"]
pub type OCTOSPI1SEC_R = crate::BitReader<bool>;
#[doc = "Field `OCTOSPI1SEC` writer - secure access mode for OCTOSPI1"]
pub type OCTOSPI1SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
#[doc = "Field `RAMCFGSEC` reader - secure access mode for RAMSCFG"]
pub type RAMCFGSEC_R = crate::BitReader<bool>;
#[doc = "Field `RAMCFGSEC` writer - secure access mode for RAMSCFG"]
pub type RAMCFGSEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_SECCFGR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - secure access mode for LPTIM6"]
    #[inline(always)]
    pub fn lptim6sec(&self) -> LPTIM6SEC_R {
        LPTIM6SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - secure access mode for VREFBUF"]
    #[inline(always)]
    pub fn vrefbufsec(&self) -> VREFBUFSEC_R {
        VREFBUFSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - secure access mode for CRC"]
    #[inline(always)]
    pub fn crcsec(&self) -> CRCSEC_R {
        CRCSEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - secure access mode for CORDIC"]
    #[inline(always)]
    pub fn cordicsec(&self) -> CORDICSEC_R {
        CORDICSEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - secure access mode for FMAC"]
    #[inline(always)]
    pub fn fmacsec(&self) -> FMACSEC_R {
        FMACSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - secure access mode for register of ETH"]
    #[inline(always)]
    pub fn ethsec(&self) -> ETHSEC_R {
        ETHSEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - secure access mode for ICACHE"]
    #[inline(always)]
    pub fn icachesec(&self) -> ICACHESEC_R {
        ICACHESEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - secure access mode for DCACHE"]
    #[inline(always)]
    pub fn dcachesec(&self) -> DCACHESEC_R {
        DCACHESEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - secure access mode for ADC1 and ADC2"]
    #[inline(always)]
    pub fn adc12sec(&self) -> ADC12SEC_R {
        ADC12SEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - secure access mode for DCMI"]
    #[inline(always)]
    pub fn dcmisec(&self) -> DCMISEC_R {
        DCMISEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - secure access mode for AES"]
    #[inline(always)]
    pub fn aessec(&self) -> AESSEC_R {
        AESSEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - secure access mode for HASH"]
    #[inline(always)]
    pub fn hashsec(&self) -> HASHSEC_R {
        HASHSEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - secure access mode for RNG"]
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - secure access mode for SAES"]
    #[inline(always)]
    pub fn saessec(&self) -> SAESSEC_R {
        SAESSEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - secure access mode for PKA"]
    #[inline(always)]
    pub fn pkasec(&self) -> PKASEC_R {
        PKASEC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - secure access mode for SDMMC2"]
    #[inline(always)]
    pub fn sdmmc2sec(&self) -> SDMMC2SEC_R {
        SDMMC2SEC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - secure access mode for SDMMC1"]
    #[inline(always)]
    pub fn sdmmc1sec(&self) -> SDMMC1SEC_R {
        SDMMC1SEC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - secure access mode for FMC"]
    #[inline(always)]
    pub fn fmcsec(&self) -> FMCSEC_R {
        FMCSEC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - secure access mode for OCTOSPI1"]
    #[inline(always)]
    pub fn octospi1sec(&self) -> OCTOSPI1SEC_R {
        OCTOSPI1SEC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - secure access mode for RAMSCFG"]
    #[inline(always)]
    pub fn ramcfgsec(&self) -> RAMCFGSEC_R {
        RAMCFGSEC_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - secure access mode for LPTIM6"]
    #[inline(always)]
    #[must_use]
    pub fn lptim6sec(&mut self) -> LPTIM6SEC_W<0> {
        LPTIM6SEC_W::new(self)
    }
    #[doc = "Bit 1 - secure access mode for VREFBUF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbufsec(&mut self) -> VREFBUFSEC_W<1> {
        VREFBUFSEC_W::new(self)
    }
    #[doc = "Bit 8 - secure access mode for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcsec(&mut self) -> CRCSEC_W<8> {
        CRCSEC_W::new(self)
    }
    #[doc = "Bit 9 - secure access mode for CORDIC"]
    #[inline(always)]
    #[must_use]
    pub fn cordicsec(&mut self) -> CORDICSEC_W<9> {
        CORDICSEC_W::new(self)
    }
    #[doc = "Bit 10 - secure access mode for FMAC"]
    #[inline(always)]
    #[must_use]
    pub fn fmacsec(&mut self) -> FMACSEC_W<10> {
        FMACSEC_W::new(self)
    }
    #[doc = "Bit 11 - secure access mode for register of ETH"]
    #[inline(always)]
    #[must_use]
    pub fn ethsec(&mut self) -> ETHSEC_W<11> {
        ETHSEC_W::new(self)
    }
    #[doc = "Bit 12 - secure access mode for ICACHE"]
    #[inline(always)]
    #[must_use]
    pub fn icachesec(&mut self) -> ICACHESEC_W<12> {
        ICACHESEC_W::new(self)
    }
    #[doc = "Bit 13 - secure access mode for DCACHE"]
    #[inline(always)]
    #[must_use]
    pub fn dcachesec(&mut self) -> DCACHESEC_W<13> {
        DCACHESEC_W::new(self)
    }
    #[doc = "Bit 14 - secure access mode for ADC1 and ADC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc12sec(&mut self) -> ADC12SEC_W<14> {
        ADC12SEC_W::new(self)
    }
    #[doc = "Bit 15 - secure access mode for DCMI"]
    #[inline(always)]
    #[must_use]
    pub fn dcmisec(&mut self) -> DCMISEC_W<15> {
        DCMISEC_W::new(self)
    }
    #[doc = "Bit 16 - secure access mode for AES"]
    #[inline(always)]
    #[must_use]
    pub fn aessec(&mut self) -> AESSEC_W<16> {
        AESSEC_W::new(self)
    }
    #[doc = "Bit 17 - secure access mode for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn hashsec(&mut self) -> HASHSEC_W<17> {
        HASHSEC_W::new(self)
    }
    #[doc = "Bit 18 - secure access mode for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rngsec(&mut self) -> RNGSEC_W<18> {
        RNGSEC_W::new(self)
    }
    #[doc = "Bit 19 - secure access mode for SAES"]
    #[inline(always)]
    #[must_use]
    pub fn saessec(&mut self) -> SAESSEC_W<19> {
        SAESSEC_W::new(self)
    }
    #[doc = "Bit 20 - secure access mode for PKA"]
    #[inline(always)]
    #[must_use]
    pub fn pkasec(&mut self) -> PKASEC_W<20> {
        PKASEC_W::new(self)
    }
    #[doc = "Bit 21 - secure access mode for SDMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2sec(&mut self) -> SDMMC2SEC_W<21> {
        SDMMC2SEC_W::new(self)
    }
    #[doc = "Bit 22 - secure access mode for SDMMC1"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1sec(&mut self) -> SDMMC1SEC_W<22> {
        SDMMC1SEC_W::new(self)
    }
    #[doc = "Bit 23 - secure access mode for FMC"]
    #[inline(always)]
    #[must_use]
    pub fn fmcsec(&mut self) -> FMCSEC_W<23> {
        FMCSEC_W::new(self)
    }
    #[doc = "Bit 24 - secure access mode for OCTOSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1sec(&mut self) -> OCTOSPI1SEC_W<24> {
        OCTOSPI1SEC_W::new(self)
    }
    #[doc = "Bit 26 - secure access mode for RAMSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgsec(&mut self) -> RAMCFGSEC_W<26> {
        RAMCFGSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTZC1 TZSC secure configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzsc_seccfgr3](index.html) module"]
pub struct GTZC1_TZSC_SECCFGR3_SPEC;
impl crate::RegisterSpec for GTZC1_TZSC_SECCFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtzc1_tzsc_seccfgr3::R](R) reader structure"]
impl crate::Readable for GTZC1_TZSC_SECCFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzsc_seccfgr3::W](W) writer structure"]
impl crate::Writable for GTZC1_TZSC_SECCFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZSC_SECCFGR3 to value 0"]
impl crate::Resettable for GTZC1_TZSC_SECCFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
