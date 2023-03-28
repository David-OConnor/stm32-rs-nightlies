#[doc = "Register `RTSR3` reader"]
pub struct R(crate::R<RTSR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTSR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTSR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTSR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTSR3` writer"]
pub struct W(crate::W<RTSR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTSR3_SPEC>;
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
impl From<crate::W<RTSR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTSR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TR82` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type TR82_R = crate::BitReader<TR82_A>;
#[doc = "Rising trigger event configuration bit of Configurable Event input x+64\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR82_A {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<TR82_A> for bool {
    #[inline(always)]
    fn from(variant: TR82_A) -> Self {
        variant as u8 != 0
    }
}
impl TR82_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TR82_A {
        match self.bits {
            false => TR82_A::Disabled,
            true => TR82_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TR82_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TR82_A::Enabled
    }
}
#[doc = "Field `TR82` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type TR82_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSR3_SPEC, TR82_A, O>;
impl<'a, const O: u8> TR82_W<'a, O> {
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TR82_A::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TR82_A::Enabled)
    }
}
#[doc = "Field `TR84` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub use TR82_R as TR84_R;
#[doc = "Field `TR85` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub use TR82_R as TR85_R;
#[doc = "Field `TR86` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub use TR82_R as TR86_R;
#[doc = "Field `TR84` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub use TR82_W as TR84_W;
#[doc = "Field `TR85` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub use TR82_W as TR85_W;
#[doc = "Field `TR86` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub use TR82_W as TR86_W;
impl R {
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr82(&self) -> TR82_R {
        TR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr84(&self) -> TR84_R {
        TR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr85(&self) -> TR85_R {
        TR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr86(&self) -> TR86_R {
        TR86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn tr82(&mut self) -> TR82_W<18> {
        TR82_W::new(self)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn tr84(&mut self) -> TR84_W<20> {
        TR84_W::new(self)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn tr85(&mut self) -> TR85_W<21> {
        TR85_W::new(self)
    }
    #[doc = "Bit 22 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn tr86(&mut self) -> TR86_W<22> {
        TR86_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI rising trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr3](index.html) module"]
pub struct RTSR3_SPEC;
impl crate::RegisterSpec for RTSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtsr3::R](R) reader structure"]
impl crate::Readable for RTSR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtsr3::W](W) writer structure"]
impl crate::Writable for RTSR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTSR3 to value 0"]
impl crate::Resettable for RTSR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
