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
#[doc = "Field `RT32` reader - Rising trigger event configuration bit of line 32"]
pub type RT32_R = crate::BitReader<RT32_A>;
#[doc = "Rising trigger event configuration bit of line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT32_A {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<RT32_A> for bool {
    #[inline(always)]
    fn from(variant: RT32_A) -> Self {
        variant as u8 != 0
    }
}
impl RT32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT32_A {
        match self.bits {
            false => RT32_A::Disabled,
            true => RT32_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RT32_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RT32_A::Enabled
    }
}
#[doc = "Field `RT32` writer - Rising trigger event configuration bit of line 32"]
pub type RT32_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSR2_SPEC, RT32_A, O>;
impl<'a, const O: u8> RT32_W<'a, O> {
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT32_A::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT32_A::Enabled)
    }
}
#[doc = "Field `RT33` reader - Rising trigger event configuration bit of line 32"]
pub use RT32_R as RT33_R;
#[doc = "Field `RT40` reader - Rising trigger event configuration bit of line 40"]
pub use RT32_R as RT40_R;
#[doc = "Field `RT41` reader - Rising trigger event configuration bit of line 41"]
pub use RT32_R as RT41_R;
#[doc = "Field `RT33` writer - Rising trigger event configuration bit of line 32"]
pub use RT32_W as RT33_W;
#[doc = "Field `RT40` writer - Rising trigger event configuration bit of line 40"]
pub use RT32_W as RT40_W;
#[doc = "Field `RT41` writer - Rising trigger event configuration bit of line 41"]
pub use RT32_W as RT41_W;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn rt32(&self) -> RT32_R {
        RT32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn rt33(&self) -> RT33_R {
        RT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of line 40"]
    #[inline(always)]
    pub fn rt40(&self) -> RT40_R {
        RT40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of line 41"]
    #[inline(always)]
    pub fn rt41(&self) -> RT41_R {
        RT41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    #[must_use]
    pub fn rt32(&mut self) -> RT32_W<0> {
        RT32_W::new(self)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    #[must_use]
    pub fn rt33(&mut self) -> RT33_W<1> {
        RT33_W::new(self)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of line 40"]
    #[inline(always)]
    #[must_use]
    pub fn rt40(&mut self) -> RT40_W<8> {
        RT40_W::new(self)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of line 41"]
    #[inline(always)]
    #[must_use]
    pub fn rt41(&mut self) -> RT41_W<9> {
        RT41_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rising Trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr2](index.html) module"]
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
