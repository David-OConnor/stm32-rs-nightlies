#[doc = "Register `AHB4RSTR` reader"]
pub struct R(crate::R<AHB4RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB4RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB4RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB4RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB4RSTR` writer"]
pub struct W(crate::W<AHB4RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB4RSTR_SPEC>;
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
impl From<crate::W<AHB4RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB4RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDMMC1RST` reader - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
pub type SDMMC1RST_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC1RST` writer - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
pub type SDMMC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4RSTR_SPEC, bool, O>;
#[doc = "Field `FMCRST` reader - FMC block reset Set and reset by software."]
pub type FMCRST_R = crate::BitReader<bool>;
#[doc = "Field `FMCRST` writer - FMC block reset Set and reset by software."]
pub type FMCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4RSTR_SPEC, bool, O>;
#[doc = "Field `OCTOSPI1RST` reader - OCTOSPI1 block reset Set and reset by software."]
pub type OCTOSPI1RST_R = crate::BitReader<bool>;
#[doc = "Field `OCTOSPI1RST` writer - OCTOSPI1 block reset Set and reset by software."]
pub type OCTOSPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 11 - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - FMC block reset Set and reset by software."]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - OCTOSPI1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn octospi1rst(&self) -> OCTOSPI1RST_R {
        OCTOSPI1RST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<11> {
        SDMMC1RST_W::new(self)
    }
    #[doc = "Bit 16 - FMC block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<16> {
        FMCRST_W::new(self)
    }
    #[doc = "Bit 20 - OCTOSPI1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn octospi1rst(&mut self) -> OCTOSPI1RST_W<20> {
        OCTOSPI1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB4 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb4rstr](index.html) module"]
pub struct AHB4RSTR_SPEC;
impl crate::RegisterSpec for AHB4RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb4rstr::R](R) reader structure"]
impl crate::Readable for AHB4RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb4rstr::W](W) writer structure"]
impl crate::Writable for AHB4RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB4RSTR to value 0"]
impl crate::Resettable for AHB4RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
