#[doc = "Register `AHB4ENR` reader"]
pub struct R(crate::R<AHB4ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB4ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB4ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB4ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB4ENR` writer"]
pub struct W(crate::W<AHB4ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB4ENR_SPEC>;
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
impl From<crate::W<AHB4ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB4ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDMMC1EN` reader - SDMMC1 and SDMMC1 delay peripheral clock enable reset"]
pub type SDMMC1EN_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC1EN` writer - SDMMC1 and SDMMC1 delay peripheral clock enable reset"]
pub type SDMMC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4ENR_SPEC, bool, O>;
#[doc = "Field `SDMMC2EN` reader - SDMMC2 and SDMMC2 delay peripheral clock enabled Set and reset by software."]
pub type SDMMC2EN_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC2EN` writer - SDMMC2 and SDMMC2 delay peripheral clock enabled Set and reset by software."]
pub type SDMMC2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4ENR_SPEC, bool, O>;
#[doc = "Field `FMCEN` reader - FMC clock enable Set and reset by software."]
pub type FMCEN_R = crate::BitReader<bool>;
#[doc = "Field `FMCEN` writer - FMC clock enable Set and reset by software."]
pub type FMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4ENR_SPEC, bool, O>;
#[doc = "Field `OCTOSPI1EN` reader - OCTOSPI1 clock enable Set and reset by software."]
pub type OCTOSPI1EN_R = crate::BitReader<bool>;
#[doc = "Field `OCTOSPI1EN` writer - OCTOSPI1 clock enable Set and reset by software."]
pub type OCTOSPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 11 - SDMMC1 and SDMMC1 delay peripheral clock enable reset"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SDMMC2 and SDMMC2 delay peripheral clock enabled Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - FMC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - OCTOSPI1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn octospi1en(&self) -> OCTOSPI1EN_R {
        OCTOSPI1EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - SDMMC1 and SDMMC1 delay peripheral clock enable reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<11> {
        SDMMC1EN_W::new(self)
    }
    #[doc = "Bit 12 - SDMMC2 and SDMMC2 delay peripheral clock enabled Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<12> {
        SDMMC2EN_W::new(self)
    }
    #[doc = "Bit 16 - FMC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<16> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 20 - OCTOSPI1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn octospi1en(&mut self) -> OCTOSPI1EN_W<20> {
        OCTOSPI1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB4 peripheral clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb4enr](index.html) module"]
pub struct AHB4ENR_SPEC;
impl crate::RegisterSpec for AHB4ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb4enr::R](R) reader structure"]
impl crate::Readable for AHB4ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb4enr::W](W) writer structure"]
impl crate::Writable for AHB4ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB4ENR to value 0"]
impl crate::Resettable for AHB4ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
