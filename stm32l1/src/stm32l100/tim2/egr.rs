#[doc = "Register `EGR` reader"]
pub struct R(crate::R<EGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EGR` writer"]
pub struct W(crate::W<EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EGR_SPEC>;
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
impl From<crate::W<EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UG` reader - Update generation"]
pub type UG_R = crate::BitReader<UG_A>;
#[doc = "Update generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG_A {
    #[doc = "1: Re-initializes the timer counter and generates an update of the registers."]
    Update = 1,
}
impl From<UG_A> for bool {
    #[inline(always)]
    fn from(variant: UG_A) -> Self {
        variant as u8 != 0
    }
}
impl UG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UG_A> {
        match self.bits {
            true => Some(UG_A::Update),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Update`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == UG_A::Update
    }
}
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, UG_A, O>;
impl<'a, const O: u8> UG_W<'a, O> {
    #[doc = "Re-initializes the timer counter and generates an update of the registers."]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(UG_A::Update)
    }
}
#[doc = "Field `CC1G` reader - Capture/compare 1 generation"]
pub type CC1G_R = crate::BitReader<CC1GW_A>;
#[doc = "Capture/compare 1 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1GW_A {
    #[doc = "1: If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    Trigger = 1,
}
impl From<CC1GW_A> for bool {
    #[inline(always)]
    fn from(variant: CC1GW_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1G_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CC1GW_A> {
        match self.bits {
            true => Some(CC1GW_A::Trigger),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == CC1GW_A::Trigger
    }
}
#[doc = "Field `CC1G` writer - Capture/compare 1 generation"]
pub type CC1G_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, CC1GW_A, O>;
impl<'a, const O: u8> CC1G_W<'a, O> {
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC1GW_A::Trigger)
    }
}
#[doc = "Field `CC2G` reader - Capture/compare 2 generation"]
pub use CC1G_R as CC2G_R;
#[doc = "Field `CC3G` reader - Capture/compare 3 generation"]
pub use CC1G_R as CC3G_R;
#[doc = "Field `CC4G` reader - Capture/compare 4 generation"]
pub use CC1G_R as CC4G_R;
#[doc = "Field `CC2G` writer - Capture/compare 2 generation"]
pub use CC1G_W as CC2G_W;
#[doc = "Field `CC3G` writer - Capture/compare 3 generation"]
pub use CC1G_W as CC3G_W;
#[doc = "Field `CC4G` writer - Capture/compare 4 generation"]
pub use CC1G_W as CC4G_W;
#[doc = "Field `TG` reader - Trigger generation"]
pub type TG_R = crate::BitReader<TGW_A>;
#[doc = "Trigger generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TGW_A {
    #[doc = "1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    Trigger = 1,
}
impl From<TGW_A> for bool {
    #[inline(always)]
    fn from(variant: TGW_A) -> Self {
        variant as u8 != 0
    }
}
impl TG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TGW_A> {
        match self.bits {
            true => Some(TGW_A::Trigger),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TGW_A::Trigger
    }
}
#[doc = "Field `TG` writer - Trigger generation"]
pub type TG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, TGW_A, O>;
impl<'a, const O: u8> TG_W<'a, O> {
    #[doc = "The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TGW_A::Trigger)
    }
}
impl R {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&self) -> UG_R {
        UG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&self) -> CC1G_R {
        CC1G_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    pub fn cc2g(&self) -> CC2G_R {
        CC2G_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline(always)]
    pub fn cc3g(&self) -> CC3G_R {
        CC3G_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline(always)]
    pub fn cc4g(&self) -> CC4G_R {
        CC4G_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn tg(&self) -> TG_R {
        TG_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<1> {
        CC1G_W::new(self)
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc2g(&mut self) -> CC2G_W<2> {
        CC2G_W::new(self)
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc3g(&mut self) -> CC3G_W<3> {
        CC3G_W::new(self)
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc4g(&mut self) -> CC4G_W<4> {
        CC4G_W::new(self)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<6> {
        TG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "event generation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [egr](index.html) module"]
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [egr::R](R) reader structure"]
impl crate::Readable for EGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [egr::W](W) writer structure"]
impl crate::Writable for EGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
