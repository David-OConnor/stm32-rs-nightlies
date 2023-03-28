#[doc = "Register `AHB4LPENR` reader"]
pub struct R(crate::R<AHB4LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB4LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB4LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB4LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB4LPENR` writer"]
pub struct W(crate::W<AHB4LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB4LPENR_SPEC>;
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
impl From<crate::W<AHB4LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB4LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software"]
pub type SDMMC1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software"]
pub type SDMMC1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4LPENR_SPEC, bool, O>;
#[doc = "Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 delay peripheral clock enable during sleep mode Set and reset by software."]
pub type SDMMC2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 delay peripheral clock enable during sleep mode Set and reset by software."]
pub type SDMMC2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4LPENR_SPEC, bool, O>;
#[doc = "Field `FMCLPEN` reader - FMC clock enable during sleep mode Set and reset by software."]
pub type FMCLPEN_R = crate::BitReader<bool>;
#[doc = "Field `FMCLPEN` writer - FMC clock enable during sleep mode Set and reset by software."]
pub type FMCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4LPENR_SPEC, bool, O>;
#[doc = "Field `OCTOSPI1LPEN` reader - OCTOSPI1 clock enable during sleep mode Set and reset by software."]
pub type OCTOSPI1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `OCTOSPI1LPEN` writer - OCTOSPI1 clock enable during sleep mode Set and reset by software."]
pub type OCTOSPI1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4LPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 11 - SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SDMMC2 and SDMMC2 delay peripheral clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - FMC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - OCTOSPI1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn octospi1lpen(&self) -> OCTOSPI1LPEN_R {
        OCTOSPI1LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<11> {
        SDMMC1LPEN_W::new(self)
    }
    #[doc = "Bit 12 - SDMMC2 and SDMMC2 delay peripheral clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<12> {
        SDMMC2LPEN_W::new(self)
    }
    #[doc = "Bit 16 - FMC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<16> {
        FMCLPEN_W::new(self)
    }
    #[doc = "Bit 20 - OCTOSPI1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn octospi1lpen(&mut self) -> OCTOSPI1LPEN_W<20> {
        OCTOSPI1LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB4 sleep clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb4lpenr](index.html) module"]
pub struct AHB4LPENR_SPEC;
impl crate::RegisterSpec for AHB4LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb4lpenr::R](R) reader structure"]
impl crate::Readable for AHB4LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb4lpenr::W](W) writer structure"]
impl crate::Writable for AHB4LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB4LPENR to value 0x0011_1880"]
impl crate::Resettable for AHB4LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0011_1880;
}
