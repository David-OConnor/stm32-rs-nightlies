#[doc = "Register `APB3LPENR` reader"]
pub struct R(crate::R<APB3LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB3LPENR` writer"]
pub struct W(crate::W<APB3LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3LPENR_SPEC>;
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
impl From<crate::W<APB3LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBSLPEN` reader - SBS clock enable during sleep mode Set and reset by software."]
pub type SBSLPEN_R = crate::BitReader<bool>;
#[doc = "Field `SBSLPEN` writer - SBS clock enable during sleep mode Set and reset by software."]
pub type SBSLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `LPUART1LPEN` reader - LPUART1 clock enable during sleep mode Set and reset by software."]
pub type LPUART1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1LPEN` writer - LPUART1 clock enable during sleep mode Set and reset by software."]
pub type LPUART1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `I3C2LPEN` reader - I3C2 clock enable during sleep mode Set and reset by software."]
pub type I3C2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `I3C2LPEN` writer - I3C2 clock enable during sleep mode Set and reset by software."]
pub type I3C2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `LPTIM1LPEN` reader - LPTIM1 clock enable during sleep mode Set and reset by software."]
pub type LPTIM1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1LPEN` writer - LPTIM1 clock enable during sleep mode Set and reset by software."]
pub type LPTIM1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `VREFLPEN` reader - VREF clock enable during sleep mode Set and reset by software."]
pub type VREFLPEN_R = crate::BitReader<bool>;
#[doc = "Field `VREFLPEN` writer - VREF clock enable during sleep mode Set and reset by software."]
pub type VREFLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
#[doc = "Field `RTCAPBLPEN` reader - RTC APB interface clock enable during sleep mode Set and reset by software."]
pub type RTCAPBLPEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCAPBLPEN` writer - RTC APB interface clock enable during sleep mode Set and reset by software."]
pub type RTCAPBLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3LPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SBS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sbslpen(&self) -> SBSLPEN_R {
        SBSLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - I3C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i3c2lpen(&self) -> I3C2LPEN_R {
        I3C2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim1lpen(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sbslpen(&mut self) -> SBSLPEN_W<1> {
        SBSLPEN_W::new(self)
    }
    #[doc = "Bit 6 - LPUART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<6> {
        LPUART1LPEN_W::new(self)
    }
    #[doc = "Bit 9 - I3C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c2lpen(&mut self) -> I3C2LPEN_W<9> {
        I3C2LPEN_W::new(self)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1lpen(&mut self) -> LPTIM1LPEN_W<11> {
        LPTIM1LPEN_W::new(self)
    }
    #[doc = "Bit 20 - VREF clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<20> {
        VREFLPEN_W::new(self)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<21> {
        RTCAPBLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB3 sleep clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3lpenr](index.html) module"]
pub struct APB3LPENR_SPEC;
impl crate::RegisterSpec for APB3LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb3lpenr::R](R) reader structure"]
impl crate::Readable for APB3LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb3lpenr::W](W) writer structure"]
impl crate::Writable for APB3LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB3LPENR to value 0x0030_fa42"]
impl crate::Resettable for APB3LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0030_fa42;
}
