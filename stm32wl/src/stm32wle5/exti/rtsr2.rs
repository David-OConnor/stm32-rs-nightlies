#[doc = "Register `RTSR2` reader"]
pub struct R(crate::R<RTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTSR2` writer"]
pub struct W(crate::W<RTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTSR2_SPEC>;
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
impl From<crate::W<RTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RT34` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT34_R = crate::BitReader<RT34_A>;
#[doc = "Rising trigger event configuration bit of Configurable Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT34_A {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<RT34_A> for bool {
    #[inline(always)]
    fn from(variant: RT34_A) -> Self {
        variant as u8 != 0
    }
}
impl RT34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT34_A {
        match self.bits {
            false => RT34_A::Disabled,
            true => RT34_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RT34_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RT34_A::Enabled
    }
}
#[doc = "Field `RT34` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT34_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSR2_SPEC, RT34_A, O>;
impl<'a, const O: u8> RT34_W<'a, O> {
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT34_A::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT34_A::Enabled)
    }
}
#[doc = "Field `RT45` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use RT34_R as RT45_R;
#[doc = "Field `RT45` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use RT34_W as RT45_W;
impl R {
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt34(&self) -> RT34_R {
        RT34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt45(&self) -> RT45_R {
        RT45_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt34(&mut self) -> RT34_W<2> {
        RT34_W::new(self)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt45(&mut self) -> RT45_W<13> {
        RT45_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rising trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr2](index.html) module"]
pub struct RTSR2_SPEC;
impl crate::RegisterSpec for RTSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtsr2::R](R) reader structure"]
impl crate::Readable for RTSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtsr2::W](W) writer structure"]
impl crate::Writable for RTSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTSR2 to value 0"]
impl crate::Resettable for RTSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
