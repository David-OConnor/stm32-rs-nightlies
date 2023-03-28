#[doc = "Register `FTSR2` reader"]
pub struct R(crate::R<FTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTSR2` writer"]
pub struct W(crate::W<FTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR2_SPEC>;
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
impl From<crate::W<FTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FT34` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT34_R = crate::BitReader<FT34_A>;
#[doc = "Falling trigger event configuration bit of Configurable Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT34_A {
    #[doc = "0: Falling edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Falling edge trigger is enabled"]
    Enabled = 1,
}
impl From<FT34_A> for bool {
    #[inline(always)]
    fn from(variant: FT34_A) -> Self {
        variant as u8 != 0
    }
}
impl FT34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FT34_A {
        match self.bits {
            false => FT34_A::Disabled,
            true => FT34_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FT34_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FT34_A::Enabled
    }
}
#[doc = "Field `FT34` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT34_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR2_SPEC, FT34_A, O>;
impl<'a, const O: u8> FT34_W<'a, O> {
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT34_A::Disabled)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT34_A::Enabled)
    }
}
#[doc = "Field `FT45` reader - Falling trigger event configuration bit of Configurable Event input"]
pub use FT34_R as FT45_R;
#[doc = "Field `FT45` writer - Falling trigger event configuration bit of Configurable Event input"]
pub use FT34_W as FT45_W;
impl R {
    #[doc = "Bit 2 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft34(&self) -> FT34_R {
        FT34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft45(&self) -> FT45_R {
        FT45_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn ft34(&mut self) -> FT34_W<2> {
        FT34_W::new(self)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn ft45(&mut self) -> FT45_W<13> {
        FT45_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr2](index.html) module"]
pub struct FTSR2_SPEC;
impl crate::RegisterSpec for FTSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftsr2::R](R) reader structure"]
impl crate::Readable for FTSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftsr2::W](W) writer structure"]
impl crate::Writable for FTSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTSR2 to value 0"]
impl crate::Resettable for FTSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
