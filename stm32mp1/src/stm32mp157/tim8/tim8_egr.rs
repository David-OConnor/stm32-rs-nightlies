#[doc = "Register `TIM8_EGR` writer"]
pub struct W(crate::W<TIM8_EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM8_EGR_SPEC>;
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
impl From<crate::W<TIM8_EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM8_EGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UG` writer - UG"]
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM8_EGR_SPEC, bool, O>;
#[doc = "Field `CC1G` writer - CC1G"]
pub type CC1G_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM8_EGR_SPEC, bool, O>;
#[doc = "Field `CC2G` writer - CC2G"]
pub type CC2G_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM8_EGR_SPEC, bool, O>;
#[doc = "Field `CC3G` writer - CC3G"]
pub type CC3G_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM8_EGR_SPEC, bool, O>;
#[doc = "Field `CC4G` writer - CC4G"]
pub type CC4G_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM8_EGR_SPEC, bool, O>;
#[doc = "Field `COMG` writer - COMG"]
pub type COMG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM8_EGR_SPEC, bool, O>;
#[doc = "Field `TG` writer - TG"]
pub type TG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM8_EGR_SPEC, bool, O>;
#[doc = "Field `BG` writer - BG"]
pub type BG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM8_EGR_SPEC, bool, O>;
#[doc = "Field `B2G` writer - B2G"]
pub type B2G_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM8_EGR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - UG"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    #[doc = "Bit 1 - CC1G"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<1> {
        CC1G_W::new(self)
    }
    #[doc = "Bit 2 - CC2G"]
    #[inline(always)]
    #[must_use]
    pub fn cc2g(&mut self) -> CC2G_W<2> {
        CC2G_W::new(self)
    }
    #[doc = "Bit 3 - CC3G"]
    #[inline(always)]
    #[must_use]
    pub fn cc3g(&mut self) -> CC3G_W<3> {
        CC3G_W::new(self)
    }
    #[doc = "Bit 4 - CC4G"]
    #[inline(always)]
    #[must_use]
    pub fn cc4g(&mut self) -> CC4G_W<4> {
        CC4G_W::new(self)
    }
    #[doc = "Bit 5 - COMG"]
    #[inline(always)]
    #[must_use]
    pub fn comg(&mut self) -> COMG_W<5> {
        COMG_W::new(self)
    }
    #[doc = "Bit 6 - TG"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<6> {
        TG_W::new(self)
    }
    #[doc = "Bit 7 - BG"]
    #[inline(always)]
    #[must_use]
    pub fn bg(&mut self) -> BG_W<7> {
        BG_W::new(self)
    }
    #[doc = "Bit 8 - B2G"]
    #[inline(always)]
    #[must_use]
    pub fn b2g(&mut self) -> B2G_W<8> {
        B2G_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM8 event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_egr](index.html) module"]
pub struct TIM8_EGR_SPEC;
impl crate::RegisterSpec for TIM8_EGR_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [tim8_egr::W](W) writer structure"]
impl crate::Writable for TIM8_EGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM8_EGR to value 0"]
impl crate::Resettable for TIM8_EGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
