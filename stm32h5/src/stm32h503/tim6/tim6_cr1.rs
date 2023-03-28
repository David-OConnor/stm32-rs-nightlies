#[doc = "Register `TIM6_CR1` reader"]
pub struct R(crate::R<TIM6_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM6_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM6_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM6_CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM6_CR1` writer"]
pub struct W(crate::W<TIM6_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM6_CR1_SPEC>;
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
impl From<crate::W<TIM6_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM6_CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEN` reader - Counter enable CEN is cleared automatically in one-pulse mode, when an update event occurs."]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - Counter enable CEN is cleared automatically in one-pulse mode, when an update event occurs."]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM6_CR1_SPEC, bool, O>;
#[doc = "Field `UDIS` reader - Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values."]
pub type UDIS_R = crate::BitReader<bool>;
#[doc = "Field `UDIS` writer - Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values."]
pub type UDIS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM6_CR1_SPEC, bool, O>;
#[doc = "Field `URS` reader - Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller"]
pub type URS_R = crate::BitReader<bool>;
#[doc = "Field `URS` writer - Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller"]
pub type URS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM6_CR1_SPEC, bool, O>;
#[doc = "Field `OPM` reader - One-pulse mode"]
pub type OPM_R = crate::BitReader<bool>;
#[doc = "Field `OPM` writer - One-pulse mode"]
pub type OPM_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM6_CR1_SPEC, bool, O>;
#[doc = "Field `ARPE` reader - Auto-reload preload enable"]
pub type ARPE_R = crate::BitReader<bool>;
#[doc = "Field `ARPE` writer - Auto-reload preload enable"]
pub type ARPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM6_CR1_SPEC, bool, O>;
#[doc = "Field `UIFREMAP` reader - UIF status bit remapping"]
pub type UIFREMAP_R = crate::BitReader<bool>;
#[doc = "Field `UIFREMAP` writer - UIF status bit remapping"]
pub type UIFREMAP_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM6_CR1_SPEC, bool, O>;
#[doc = "Field `DITHEN` reader - Dithering enable Note: The DITHEN bit can only be modified when CEN bit is reset."]
pub type DITHEN_R = crate::BitReader<bool>;
#[doc = "Field `DITHEN` writer - Dithering enable Note: The DITHEN bit can only be modified when CEN bit is reset."]
pub type DITHEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM6_CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Counter enable CEN is cleared automatically in one-pulse mode, when an update event occurs."]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values."]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    pub fn uifremap(&self) -> UIFREMAP_R {
        UIFREMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Dithering enable Note: The DITHEN bit can only be modified when CEN bit is reset."]
    #[inline(always)]
    pub fn dithen(&self) -> DITHEN_R {
        DITHEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable CEN is cleared automatically in one-pulse mode, when an update event occurs."]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    #[doc = "Bit 1 - Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values."]
    #[inline(always)]
    #[must_use]
    pub fn udis(&mut self) -> UDIS_W<1> {
        UDIS_W::new(self)
    }
    #[doc = "Bit 2 - Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller"]
    #[inline(always)]
    #[must_use]
    pub fn urs(&mut self) -> URS_W<2> {
        URS_W::new(self)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    #[must_use]
    pub fn opm(&mut self) -> OPM_W<3> {
        OPM_W::new(self)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpe(&mut self) -> ARPE_W<7> {
        ARPE_W::new(self)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    #[must_use]
    pub fn uifremap(&mut self) -> UIFREMAP_W<11> {
        UIFREMAP_W::new(self)
    }
    #[doc = "Bit 12 - Dithering enable Note: The DITHEN bit can only be modified when CEN bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn dithen(&mut self) -> DITHEN_W<12> {
        DITHEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM6 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_cr1](index.html) module"]
pub struct TIM6_CR1_SPEC;
impl crate::RegisterSpec for TIM6_CR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim6_cr1::R](R) reader structure"]
impl crate::Readable for TIM6_CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim6_cr1::W](W) writer structure"]
impl crate::Writable for TIM6_CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM6_CR1 to value 0"]
impl crate::Resettable for TIM6_CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
