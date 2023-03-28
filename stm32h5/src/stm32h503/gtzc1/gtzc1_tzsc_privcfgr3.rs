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
#[doc = "Field `I3C2PRIV` reader - privileged access mode for I3C2"]
pub type I3C2PRIV_R = crate::BitReader<bool>;
#[doc = "Field `I3C2PRIV` writer - privileged access mode for I3C2"]
pub type I3C2PRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `CRCPRIV` reader - privileged access mode for CRC"]
pub type CRCPRIV_R = crate::BitReader<bool>;
#[doc = "Field `CRCPRIV` writer - privileged access mode for CRC"]
pub type CRCPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `ICACHEPRIV` reader - privileged access mode for ICACHE"]
pub type ICACHEPRIV_R = crate::BitReader<bool>;
#[doc = "Field `ICACHEPRIV` writer - privileged access mode for ICACHE"]
pub type ICACHEPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
#[doc = "Field `ADC1PRIV` reader - privileged access mode for ADC1"]
pub type ADC1PRIV_R = crate::BitReader<bool>;
#[doc = "Field `ADC1PRIV` writer - privileged access mode for ADC1"]
pub type ADC1PRIV_W<'a, const O: u8> =
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
#[doc = "Field `RAMCFGPRIV` reader - privileged access mode for RAMSCFG"]
pub type RAMCFGPRIV_R = crate::BitReader<bool>;
#[doc = "Field `RAMCFGPRIV` writer - privileged access mode for RAMSCFG"]
pub type RAMCFGPRIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GTZC1_TZSC_PRIVCFGR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - privileged access mode for I3C2"]
    #[inline(always)]
    pub fn i3c2priv(&self) -> I3C2PRIV_R {
        I3C2PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - privileged access mode for CRC"]
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - privileged access mode for ICACHE"]
    #[inline(always)]
    pub fn icachepriv(&self) -> ICACHEPRIV_R {
        ICACHEPRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - privileged access mode for ADC1"]
    #[inline(always)]
    pub fn adc1priv(&self) -> ADC1PRIV_R {
        ADC1PRIV_R::new(((self.bits >> 14) & 1) != 0)
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
    #[doc = "Bit 26 - privileged access mode for RAMSCFG"]
    #[inline(always)]
    pub fn ramcfgpriv(&self) -> RAMCFGPRIV_R {
        RAMCFGPRIV_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - privileged access mode for I3C2"]
    #[inline(always)]
    #[must_use]
    pub fn i3c2priv(&mut self) -> I3C2PRIV_W<2> {
        I3C2PRIV_W::new(self)
    }
    #[doc = "Bit 8 - privileged access mode for CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcpriv(&mut self) -> CRCPRIV_W<8> {
        CRCPRIV_W::new(self)
    }
    #[doc = "Bit 12 - privileged access mode for ICACHE"]
    #[inline(always)]
    #[must_use]
    pub fn icachepriv(&mut self) -> ICACHEPRIV_W<12> {
        ICACHEPRIV_W::new(self)
    }
    #[doc = "Bit 14 - privileged access mode for ADC1"]
    #[inline(always)]
    #[must_use]
    pub fn adc1priv(&mut self) -> ADC1PRIV_W<14> {
        ADC1PRIV_W::new(self)
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
