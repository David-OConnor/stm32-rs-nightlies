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
#[doc = "Update generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG_AW {
    #[doc = "1: Re-initializes the timer counter and generates an update of the registers."]
    Update = 1,
}
impl From<UG_AW> for bool {
    #[inline(always)]
    fn from(variant: UG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, UG_AW, O>;
impl<'a, const O: u8> UG_W<'a, O> {
    #[doc = "Re-initializes the timer counter and generates an update of the registers."]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(UG_AW::Update)
    }
}
#[doc = "Capture/Compare 1 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1GW_AW {
    #[doc = "1: If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register"]
    Trigger = 1,
}
impl From<CC1GW_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1GW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1G` writer - Capture/Compare 1 generation"]
pub type CC1G_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, CC1GW_AW, O>;
impl<'a, const O: u8> CC1G_W<'a, O> {
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC1GW_AW::Trigger)
    }
}
#[doc = "Capture/Compare control update generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMGW_AW {
    #[doc = "1: When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated"]
    Trigger = 1,
}
impl From<COMGW_AW> for bool {
    #[inline(always)]
    fn from(variant: COMGW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMG` writer - Capture/Compare control update generation"]
pub type COMG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, COMGW_AW, O>;
impl<'a, const O: u8> COMG_W<'a, O> {
    #[doc = "When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(COMGW_AW::Trigger)
    }
}
#[doc = "Break generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGW_AW {
    #[doc = "1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled"]
    Trigger = 1,
}
impl From<BGW_AW> for bool {
    #[inline(always)]
    fn from(variant: BGW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BG` writer - Break generation"]
pub type BG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, BGW_AW, O>;
impl<'a, const O: u8> BG_W<'a, O> {
    #[doc = "A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(BGW_AW::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    #[doc = "Bit 1 - Capture/Compare 1 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<1> {
        CC1G_W::new(self)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation"]
    #[inline(always)]
    #[must_use]
    pub fn comg(&mut self) -> COMG_W<5> {
        COMG_W::new(self)
    }
    #[doc = "Bit 7 - Break generation"]
    #[inline(always)]
    #[must_use]
    pub fn bg(&mut self) -> BG_W<7> {
        BG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16/TIM17 event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [egr](index.html) module"]
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u32;
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
