#[doc = "Register `GTZC1_TZSC_PRIVCFGR3` reader"]
pub struct R(crate::R<GTZC1_TZSC_PRIVCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZSC_PRIVCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZSC_PRIVCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZSC_PRIVCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTZC1_TZSC_PRIVCFGR3` writer"]
pub struct W(crate::W<GTZC1_TZSC_PRIVCFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZSC_PRIVCFGR3_SPEC>;
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
impl From<crate::W<GTZC1_TZSC_PRIVCFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZSC_PRIVCFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM6PRIV` reader - privileged access mode for LPTIM6"]
pub type LPTIM6PRIV_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM6PRIV` writer - privileged access mode for LPTIM6"]
pub type LPTIM6PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `VREFBUFPRIV` reader - privileged access mode for VREFBUF"]
pub type VREFBUFPRIV_R = crate::BitReader<bool>;
#[doc = "Field `VREFBUFPRIV` writer - privileged access mode for VREFBUF"]
pub type VREFBUFPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `CRCPRIV` reader - privileged access mode for CRC"]
pub type CRCPRIV_R = crate::BitReader<bool>;
#[doc = "Field `CRCPRIV` writer - privileged access mode for CRC"]
pub type CRCPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `CORDICPRIV` reader - privileged access mode for CORDIC"]
pub type CORDICPRIV_R = crate::BitReader<bool>;
#[doc = "Field `CORDICPRIV` writer - privileged access mode for CORDIC"]
pub type CORDICPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `FMACPRIV` reader - privileged access mode for FMAC"]
pub type FMACPRIV_R = crate::BitReader<bool>;
#[doc = "Field `FMACPRIV` writer - privileged access mode for FMAC"]
pub type FMACPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `ETHPRIV` reader - privileged access mode for register of ETH"]
pub type ETHPRIV_R = crate::BitReader<bool>;
#[doc = "Field `ETHPRIV` writer - privileged access mode for register of ETH"]
pub type ETHPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `ICACHEPRIV` reader - privileged access mode for ICACHE"]
pub type ICACHEPRIV_R = crate::BitReader<bool>;
#[doc = "Field `ICACHEPRIV` writer - privileged access mode for ICACHE"]
pub type ICACHEPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `DCACHEPRIV` reader - privileged access mode for DCACHE"]
pub type DCACHEPRIV_R = crate::BitReader<bool>;
#[doc = "Field `DCACHEPRIV` writer - privileged access mode for DCACHE"]
pub type DCACHEPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `ADC12PRIV` reader - privileged access mode for ADC1 and ADC2"]
pub type ADC12PRIV_R = crate::BitReader<bool>;
#[doc = "Field `ADC12PRIV` writer - privileged access mode for ADC1 and ADC2"]
pub type ADC12PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `DCMIPRIV` reader - privileged access mode for DCMI"]
pub type DCMIPRIV_R = crate::BitReader<bool>;
#[doc = "Field `DCMIPRIV` writer - privileged access mode for DCMI"]
pub type DCMIPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `HASHPRIV` reader - privileged access mode for HASH"]
pub type HASHPRIV_R = crate::BitReader<bool>;
#[doc = "Field `HASHPRIV` writer - privileged access mode for HASH"]
pub type HASHPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `RNGPRIV` reader - privileged access mode for RNG"]
pub type RNGPRIV_R = crate::BitReader<bool>;
#[doc = "Field `RNGPRIV` writer - privileged access mode for RNG"]
pub type RNGPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `SDMMC2PRIV` reader - privileged access mode for SDMMC2"]
pub type SDMMC2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC2PRIV` writer - privileged access mode for SDMMC2"]
pub type SDMMC2PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `SDMMC1PRIV` reader - privileged access mode for SDMMC1"]
pub type SDMMC1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC1PRIV` writer - privileged access mode for SDMMC1"]
pub type SDMMC1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `FMCPRIV` reader - privileged access mode for FMC"]
pub type FMCPRIV_R = crate::BitReader<bool>;
#[doc = "Field `FMCPRIV` writer - privileged access mode for FMC"]
pub type FMCPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `OCTOSPI1PRIV` reader - privileged access mode for OCTOSPI1"]
pub type OCTOSPI1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `OCTOSPI1PRIV` writer - privileged access mode for OCTOSPI1"]
pub type OCTOSPI1PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `RAMCFGPRIV` reader - privileged access mode for RAMSCFG"]
pub type RAMCFGPRIV_R = crate::BitReader<bool>;
#[doc = "Field `RAMCFGPRIV` writer - privileged access mode for RAMSCFG"]
pub type RAMCFGPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - privileged access mode for LPTIM6"]
    #[inline(always)]
    pub fn lptim6priv(&self) -> LPTIM6PRIV_R {
        LPTIM6PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - privileged access mode for VREFBUF"]
    #[inline(always)]
    pub fn vrefbufpriv(&self) -> VREFBUFPRIV_R {
        VREFBUFPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - privileged access mode for CRC"]
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - privileged access mode for CORDIC"]
    #[inline(always)]
    pub fn cordicpriv(&self) -> CORDICPRIV_R {
        CORDICPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - privileged access mode for FMAC"]
    #[inline(always)]
    pub fn fmacpriv(&self) -> FMACPRIV_R {
        FMACPRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - privileged access mode for register of ETH"]
    #[inline(always)]
    pub fn ethpriv(&self) -> ETHPRIV_R {
        ETHPRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - privileged access mode for ICACHE"]
    #[inline(always)]
    pub fn icachepriv(&self) -> ICACHEPRIV_R {
        ICACHEPRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - privileged access mode for DCACHE"]
    #[inline(always)]
    pub fn dcachepriv(&self) -> DCACHEPRIV_R {
        DCACHEPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - privileged access mode for ADC1 and ADC2"]
    #[inline(always)]
    pub fn adc12priv(&self) -> ADC12PRIV_R {
        ADC12PRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - privileged access mode for DCMI"]
    #[inline(always)]
    pub fn dcmipriv(&self) -> DCMIPRIV_R {
        DCMIPRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - privileged access mode for HASH"]
    #[inline(always)]
    pub fn hashpriv(&self) -> HASHPRIV_R {
        HASHPRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - privileged access mode for RNG"]
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - privileged access mode for SDMMC2"]
    #[inline(always)]
    pub fn sdmmc2priv(&self) -> SDMMC2PRIV_R {
        SDMMC2PRIV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - privileged access mode for SDMMC1"]
    #[inline(always)]
    pub fn sdmmc1priv(&self) -> SDMMC1PRIV_R {
        SDMMC1PRIV_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - privileged access mode for FMC"]
    #[inline(always)]
    pub fn fmcpriv(&self) -> FMCPRIV_R {
        FMCPRIV_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - privileged access mode for OCTOSPI1"]
    #[inline(always)]
    pub fn octospi1priv(&self) -> OCTOSPI1PRIV_R {
        OCTOSPI1PRIV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - privileged access mode for RAMSCFG"]
    #[inline(always)]
    pub fn ramcfgpriv(&self) -> RAMCFGPRIV_R {
        RAMCFGPRIV_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for LPTIM6"]
    #[inline(always)]
    #[must_use]
    pub fn lptim6priv(&mut self) -> LPTIM6PRIV_W<0> {
        LPTIM6PRIV_W::new(self)
    }
    #[doc = "Bit 1 - privileged access mode for VREFBUF"]
    #[inline(always)]
    #[must_use]
    pub fn vrefbufpriv(&mut self) -> VREFBUFPRIV_W<1> {
        VREFBUFPRIV_W::new(self)
    }
    #[doc = "Bit 8 - privileged access mode for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcpriv(&mut self) -> CRCPRIV_W<8> {
        CRCPRIV_W::new(self)
    }
    #[doc = "Bit 9 - privileged access mode for CORDIC"]
    #[inline(always)]
    #[must_use]
    pub fn cordicpriv(&mut self) -> CORDICPRIV_W<9> {
        CORDICPRIV_W::new(self)
    }
    #[doc = "Bit 10 - privileged access mode for FMAC"]
    #[inline(always)]
    #[must_use]
    pub fn fmacpriv(&mut self) -> FMACPRIV_W<10> {
        FMACPRIV_W::new(self)
    }
    #[doc = "Bit 11 - privileged access mode for register of ETH"]
    #[inline(always)]
    #[must_use]
    pub fn ethpriv(&mut self) -> ETHPRIV_W<11> {
        ETHPRIV_W::new(self)
    }
    #[doc = "Bit 12 - privileged access mode for ICACHE"]
    #[inline(always)]
    #[must_use]
    pub fn icachepriv(&mut self) -> ICACHEPRIV_W<12> {
        ICACHEPRIV_W::new(self)
    }
    #[doc = "Bit 13 - privileged access mode for DCACHE"]
    #[inline(always)]
    #[must_use]
    pub fn dcachepriv(&mut self) -> DCACHEPRIV_W<13> {
        DCACHEPRIV_W::new(self)
    }
    #[doc = "Bit 14 - privileged access mode for ADC1 and ADC2"]
    #[inline(always)]
    #[must_use]
    pub fn adc12priv(&mut self) -> ADC12PRIV_W<14> {
        ADC12PRIV_W::new(self)
    }
    #[doc = "Bit 15 - privileged access mode for DCMI"]
    #[inline(always)]
    #[must_use]
    pub fn dcmipriv(&mut self) -> DCMIPRIV_W<15> {
        DCMIPRIV_W::new(self)
    }
    #[doc = "Bit 17 - privileged access mode for HASH"]
    #[inline(always)]
    #[must_use]
    pub fn hashpriv(&mut self) -> HASHPRIV_W<17> {
        HASHPRIV_W::new(self)
    }
    #[doc = "Bit 18 - privileged access mode for RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<18> {
        RNGPRIV_W::new(self)
    }
    #[doc = "Bit 21 - privileged access mode for SDMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2priv(&mut self) -> SDMMC2PRIV_W<21> {
        SDMMC2PRIV_W::new(self)
    }
    #[doc = "Bit 22 - privileged access mode for SDMMC1"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1priv(&mut self) -> SDMMC1PRIV_W<22> {
        SDMMC1PRIV_W::new(self)
    }
    #[doc = "Bit 23 - privileged access mode for FMC"]
    #[inline(always)]
    #[must_use]
    pub fn fmcpriv(&mut self) -> FMCPRIV_W<23> {
        FMCPRIV_W::new(self)
    }
    #[doc = "Bit 24 - privileged access mode for OCTOSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1priv(&mut self) -> OCTOSPI1PRIV_W<24> {
        OCTOSPI1PRIV_W::new(self)
    }
    #[doc = "Bit 26 - privileged access mode for RAMSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfgpriv(&mut self) -> RAMCFGPRIV_W<26> {
        RAMCFGPRIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTZC1 TZSC privilege configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_tzsc_privcfgr3](index.html) module"]
pub struct GTZC1_TZSC_PRIVCFGR3_SPEC;
impl crate::RegisterSpec for GTZC1_TZSC_PRIVCFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtzc1_tzsc_privcfgr3::R](R) reader structure"]
impl crate::Readable for GTZC1_TZSC_PRIVCFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_tzsc_privcfgr3::W](W) writer structure"]
impl crate::Writable for GTZC1_TZSC_PRIVCFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_TZSC_PRIVCFGR3 to value 0"]
impl crate::Resettable for GTZC1_TZSC_PRIVCFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
