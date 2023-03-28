#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVEN` reader - Low voltage enable"]
pub type LVEN_R = crate::BitReader<LVEN_A>;
#[doc = "Low voltage enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVEN_A {
    #[doc = "0: Flash low voltage disabled"]
    Disabled = 0,
    #[doc = "1: Flash low voltage enabled"]
    Enabled = 1,
}
impl From<LVEN_A> for bool {
    #[inline(always)]
    fn from(variant: LVEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVEN_A {
        match self.bits {
            false => LVEN_A::Disabled,
            true => LVEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LVEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LVEN_A::Enabled
    }
}
#[doc = "Field `LVEN` writer - Low voltage enable"]
pub type LVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, LVEN_A, O>;
impl<'a, const O: u8> LVEN_W<'a, O> {
    #[doc = "Flash low voltage disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LVEN_A::Disabled)
    }
    #[doc = "Flash low voltage enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LVEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Low voltage enable"]
    #[inline(always)]
    pub fn lven(&self) -> LVEN_R {
        LVEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low voltage enable"]
    #[inline(always)]
    #[must_use]
    pub fn lven(&mut self) -> LVEN_W<0> {
        LVEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "flash configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
