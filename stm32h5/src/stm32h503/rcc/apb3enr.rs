#[doc = "Register `APB3ENR` reader"]
pub struct R(crate::R<APB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB3ENR` writer"]
pub struct W(crate::W<APB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3ENR_SPEC>;
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
impl From<crate::W<APB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBSEN` reader - SBS clock enable Set and reset by software."]
pub type SBSEN_R = crate::BitReader<bool>;
#[doc = "Field `SBSEN` writer - SBS clock enable Set and reset by software."]
pub type SBSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `LPUART1EN` reader - LPUART1 clock enable Set and reset by software."]
pub type LPUART1EN_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1EN` writer - LPUART1 clock enable Set and reset by software."]
pub type LPUART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `I3C2EN` reader - I3C2EN clock enable Set and reset by software."]
pub type I3C2EN_R = crate::BitReader<bool>;
#[doc = "Field `I3C2EN` writer - I3C2EN clock enable Set and reset by software."]
pub type I3C2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `LPTIM1EN` reader - LPTIM1 clock enable Set and reset by software."]
pub type LPTIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1EN` writer - LPTIM1 clock enable Set and reset by software."]
pub type LPTIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `VREFEN` reader - VREF clock enable Set and reset by software."]
pub type VREFEN_R = crate::BitReader<bool>;
#[doc = "Field `VREFEN` writer - VREF clock enable Set and reset by software."]
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
#[doc = "Field `RTCAPBEN` reader - RTC APB interface clock enable Set and reset by software."]
pub type RTCAPBEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCAPBEN` writer - RTC APB interface clock enable Set and reset by software."]
pub type RTCAPBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SBS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn sbsen(&self) -> SBSEN_R {
        SBSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - I3C2EN clock enable Set and reset by software."]
    #[inline(always)]
    pub fn i3c2en(&self) -> I3C2EN_R {
        I3C2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF clock enable Set and reset by software."]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable Set and reset by software."]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sbsen(&mut self) -> SBSEN_W<1> {
        SBSEN_W::new(self)
    }
    #[doc = "Bit 6 - LPUART1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<6> {
        LPUART1EN_W::new(self)
    }
    #[doc = "Bit 9 - I3C2EN clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c2en(&mut self) -> I3C2EN_W<9> {
        I3C2EN_W::new(self)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<11> {
        LPTIM1EN_W::new(self)
    }
    #[doc = "Bit 20 - VREF clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<20> {
        VREFEN_W::new(self)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<21> {
        RTCAPBEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB3 peripheral clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3enr](index.html) module"]
pub struct APB3ENR_SPEC;
impl crate::RegisterSpec for APB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb3enr::R](R) reader structure"]
impl crate::Readable for APB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb3enr::W](W) writer structure"]
impl crate::Writable for APB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB3ENR to value 0"]
impl crate::Resettable for APB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
