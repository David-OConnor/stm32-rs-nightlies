#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "compare match Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPMCFW_AW {
    #[doc = "1: Compare match Clear Flag"]
    Clear = 1,
}
impl From<CMPMCFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPMCFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPMCF` writer - compare match Clear Flag"]
pub type CMPMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, CMPMCFW_AW, O>;
impl<'a, const O: u8> CMPMCF_W<'a, O> {
    #[doc = "Compare match Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMPMCFW_AW::Clear)
    }
}
#[doc = "Autoreload match Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARRMCFW_AW {
    #[doc = "1: Autoreload match Clear Flag"]
    Clear = 1,
}
impl From<ARRMCFW_AW> for bool {
    #[inline(always)]
    fn from(variant: ARRMCFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARRMCF` writer - Autoreload match Clear Flag"]
pub type ARRMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ARRMCFW_AW, O>;
impl<'a, const O: u8> ARRMCF_W<'a, O> {
    #[doc = "Autoreload match Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARRMCFW_AW::Clear)
    }
}
#[doc = "External trigger valid edge Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIGCFW_AW {
    #[doc = "1: External trigger valid edge Clear Flag"]
    Clear = 1,
}
impl From<EXTTRIGCFW_AW> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGCFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge Clear Flag"]
pub type EXTTRIGCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, EXTTRIGCFW_AW, O>;
impl<'a, const O: u8> EXTTRIGCF_W<'a, O> {
    #[doc = "External trigger valid edge Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EXTTRIGCFW_AW::Clear)
    }
}
#[doc = "Compare register update OK Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOKCFW_AW {
    #[doc = "1: Compare register update OK Clear Flag"]
    Clear = 1,
}
impl From<CMPOKCFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPOKCFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPOKCF` writer - Compare register update OK Clear Flag"]
pub type CMPOKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, CMPOKCFW_AW, O>;
impl<'a, const O: u8> CMPOKCF_W<'a, O> {
    #[doc = "Compare register update OK Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMPOKCFW_AW::Clear)
    }
}
#[doc = "Autoreload register update OK Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARROKCFW_AW {
    #[doc = "1: Autoreload register update OK Clear Flag"]
    Clear = 1,
}
impl From<ARROKCFW_AW> for bool {
    #[inline(always)]
    fn from(variant: ARROKCFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARROKCF` writer - Autoreload register update OK Clear Flag"]
pub type ARROKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ARROKCFW_AW, O>;
impl<'a, const O: u8> ARROKCF_W<'a, O> {
    #[doc = "Autoreload register update OK Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARROKCFW_AW::Clear)
    }
}
#[doc = "Direction change to UP Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPCFW_AW {
    #[doc = "1: Direction change to up Clear Flag"]
    Clear = 1,
}
impl From<UPCFW_AW> for bool {
    #[inline(always)]
    fn from(variant: UPCFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPCF` writer - Direction change to UP Clear Flag"]
pub type UPCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, UPCFW_AW, O>;
impl<'a, const O: u8> UPCF_W<'a, O> {
    #[doc = "Direction change to up Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UPCFW_AW::Clear)
    }
}
#[doc = "Direction change to down Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOWNCFW_AW {
    #[doc = "1: Direction change to down Clear Flag"]
    Clear = 1,
}
impl From<DOWNCFW_AW> for bool {
    #[inline(always)]
    fn from(variant: DOWNCFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWNCF` writer - Direction change to down Clear Flag"]
pub type DOWNCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, DOWNCFW_AW, O>;
impl<'a, const O: u8> DOWNCF_W<'a, O> {
    #[doc = "Direction change to down Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DOWNCFW_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - compare match Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmcf(&mut self) -> CMPMCF_W<0> {
        CMPMCF_W::new(self)
    }
    #[doc = "Bit 1 - Autoreload match Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn arrmcf(&mut self) -> ARRMCF_W<1> {
        ARRMCF_W::new(self)
    }
    #[doc = "Bit 2 - External trigger valid edge Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<2> {
        EXTTRIGCF_W::new(self)
    }
    #[doc = "Bit 3 - Compare register update OK Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpokcf(&mut self) -> CMPOKCF_W<3> {
        CMPOKCF_W::new(self)
    }
    #[doc = "Bit 4 - Autoreload register update OK Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn arrokcf(&mut self) -> ARROKCF_W<4> {
        ARROKCF_W::new(self)
    }
    #[doc = "Bit 5 - Direction change to UP Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn upcf(&mut self) -> UPCF_W<5> {
        UPCF_W::new(self)
    }
    #[doc = "Bit 6 - Direction change to down Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn downcf(&mut self) -> DOWNCF_W<6> {
        DOWNCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
