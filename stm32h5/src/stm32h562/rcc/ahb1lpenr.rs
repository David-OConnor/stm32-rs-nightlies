#[doc = "Register `AHB1LPENR` reader"]
pub struct R(crate::R<AHB1LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1LPENR` writer"]
pub struct W(crate::W<AHB1LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1LPENR_SPEC>;
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
impl From<crate::W<AHB1LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPDMA1LPEN` reader - GPDMA1 clock enable during sleep mode Set and reset by software."]
pub type GPDMA1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPDMA1LPEN` writer - GPDMA1 clock enable during sleep mode Set and reset by software."]
pub type GPDMA1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `GPDMA2LPEN` reader - GPDMA2 clock enable during sleep mode Set and reset by software."]
pub type GPDMA2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPDMA2LPEN` writer - GPDMA2 clock enable during sleep mode Set and reset by software."]
pub type GPDMA2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `FLITFLPEN` reader - Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
pub type FLITFLPEN_R = crate::BitReader<bool>;
#[doc = "Field `FLITFLPEN` writer - Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
pub type FLITFLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `CRCLPEN` reader - CRC clock enable during sleep mode Set and reset by software."]
pub type CRCLPEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCLPEN` writer - CRC clock enable during sleep mode Set and reset by software."]
pub type CRCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `CORDICLPEN` reader - CORDIC clock enable during sleep mode Set and reset by software."]
pub type CORDICLPEN_R = crate::BitReader<bool>;
#[doc = "Field `CORDICLPEN` writer - CORDIC clock enable during sleep mode Set and reset by software."]
pub type CORDICLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `FMACLPEN` reader - FMAC clock enable during sleep mode Set and reset by software."]
pub type FMACLPEN_R = crate::BitReader<bool>;
#[doc = "Field `FMACLPEN` writer - FMAC clock enable during sleep mode Set and reset by software."]
pub type FMACLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `RAMCFGLPEN` reader - RAMCFG clock enable during sleep mode Set and reset by software."]
pub type RAMCFGLPEN_R = crate::BitReader<bool>;
#[doc = "Field `RAMCFGLPEN` writer - RAMCFG clock enable during sleep mode Set and reset by software."]
pub type RAMCFGLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `TZSC1LPEN` reader - TZSC1 clock enable during sleep mode Set and reset by software"]
pub type TZSC1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TZSC1LPEN` writer - TZSC1 clock enable during sleep mode Set and reset by software"]
pub type TZSC1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `BKPRAMLPEN` reader - BKPRAM clock enable during sleep mode Set and reset by software"]
pub type BKPRAMLPEN_R = crate::BitReader<bool>;
#[doc = "Field `BKPRAMLPEN` writer - BKPRAM clock enable during sleep mode Set and reset by software"]
pub type BKPRAMLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `ICACHELPEN` reader - ICACHE clock enable during sleep mode Set and reset by software"]
pub type ICACHELPEN_R = crate::BitReader<bool>;
#[doc = "Field `ICACHELPEN` writer - ICACHE clock enable during sleep mode Set and reset by software"]
pub type ICACHELPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `DCACHELPEN` reader - DCACHE clock enable during sleep mode Set and reset by software"]
pub type DCACHELPEN_R = crate::BitReader<bool>;
#[doc = "Field `DCACHELPEN` writer - DCACHE clock enable during sleep mode Set and reset by software"]
pub type DCACHELPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `SRAM1LPEN` reader - SRAM1 clock enable during sleep mode Set and reset by software"]
pub type SRAM1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1LPEN` writer - SRAM1 clock enable during sleep mode Set and reset by software"]
pub type SRAM1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPDMA1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpdma1lpen(&self) -> GPDMA1LPEN_R {
        GPDMA1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPDMA2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpdma2lpen(&self) -> GPDMA2LPEN_R {
        GPDMA2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CORDIC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn cordiclpen(&self) -> CORDICLPEN_R {
        CORDICLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FMAC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn fmaclpen(&self) -> FMACLPEN_R {
        FMACLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMCFG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ramcfglpen(&self) -> RAMCFGLPEN_R {
        RAMCFGLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - TZSC1 clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn tzsc1lpen(&self) -> TZSC1LPEN_R {
        TZSC1LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - BKPRAM clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn bkpramlpen(&self) -> BKPRAMLPEN_R {
        BKPRAMLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ICACHE clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn icachelpen(&self) -> ICACHELPEN_R {
        ICACHELPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DCACHE clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn dcachelpen(&self) -> DCACHELPEN_R {
        DCACHELPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM1 clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma1lpen(&mut self) -> GPDMA1LPEN_W<0> {
        GPDMA1LPEN_W::new(self)
    }
    #[doc = "Bit 1 - GPDMA2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma2lpen(&mut self) -> GPDMA2LPEN_W<1> {
        GPDMA2LPEN_W::new(self)
    }
    #[doc = "Bit 8 - Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W<8> {
        FLITFLPEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crclpen(&mut self) -> CRCLPEN_W<12> {
        CRCLPEN_W::new(self)
    }
    #[doc = "Bit 14 - CORDIC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cordiclpen(&mut self) -> CORDICLPEN_W<14> {
        CORDICLPEN_W::new(self)
    }
    #[doc = "Bit 15 - FMAC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmaclpen(&mut self) -> FMACLPEN_W<15> {
        FMACLPEN_W::new(self)
    }
    #[doc = "Bit 17 - RAMCFG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ramcfglpen(&mut self) -> RAMCFGLPEN_W<17> {
        RAMCFGLPEN_W::new(self)
    }
    #[doc = "Bit 24 - TZSC1 clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn tzsc1lpen(&mut self) -> TZSC1LPEN_W<24> {
        TZSC1LPEN_W::new(self)
    }
    #[doc = "Bit 28 - BKPRAM clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn bkpramlpen(&mut self) -> BKPRAMLPEN_W<28> {
        BKPRAMLPEN_W::new(self)
    }
    #[doc = "Bit 29 - ICACHE clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn icachelpen(&mut self) -> ICACHELPEN_W<29> {
        ICACHELPEN_W::new(self)
    }
    #[doc = "Bit 30 - DCACHE clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn dcachelpen(&mut self) -> DCACHELPEN_W<30> {
        DCACHELPEN_W::new(self)
    }
    #[doc = "Bit 31 - SRAM1 clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<31> {
        SRAM1LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB1 sleep clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1lpenr](index.html) module"]
pub struct AHB1LPENR_SPEC;
impl crate::RegisterSpec for AHB1LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1lpenr::R](R) reader structure"]
impl crate::Readable for AHB1LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1lpenr::W](W) writer structure"]
impl crate::Writable for AHB1LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB1LPENR to value 0xf13a_d103"]
impl crate::Resettable for AHB1LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xf13a_d103;
}
