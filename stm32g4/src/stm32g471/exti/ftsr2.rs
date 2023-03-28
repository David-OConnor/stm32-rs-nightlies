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
#[doc = "Field `FT32` reader - Falling trigger event configuration of line 32"]
pub type FT32_R = crate::BitReader<FT32_A>;
#[doc = "Falling trigger event configuration of line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT32_A {
    #[doc = "0: Falling edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Falling edge trigger is enabled"]
    Enabled = 1,
}
impl From<FT32_A> for bool {
    #[inline(always)]
    fn from(variant: FT32_A) -> Self {
        variant as u8 != 0
    }
}
impl FT32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FT32_A {
        match self.bits {
            false => FT32_A::Disabled,
            true => FT32_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FT32_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FT32_A::Enabled
    }
}
#[doc = "Field `FT32` writer - Falling trigger event configuration of line 32"]
pub type FT32_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR2_SPEC, FT32_A, O>;
impl<'a, const O: u8> FT32_W<'a, O> {
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT32_A::Disabled)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT32_A::Enabled)
    }
}
#[doc = "Field `FT33` reader - Falling trigger event configuration of line 33"]
pub use FT32_R as FT33_R;
#[doc = "Field `FT40` reader - Falling trigger event configuration of line 40"]
pub use FT32_R as FT40_R;
#[doc = "Field `FT41` reader - Falling trigger event configuration of line 41"]
pub use FT32_R as FT41_R;
#[doc = "Field `FT33` writer - Falling trigger event configuration of line 33"]
pub use FT32_W as FT33_W;
#[doc = "Field `FT40` writer - Falling trigger event configuration of line 40"]
pub use FT32_W as FT40_W;
#[doc = "Field `FT41` writer - Falling trigger event configuration of line 41"]
pub use FT32_W as FT41_W;
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration of line 32"]
    #[inline(always)]
    pub fn ft32(&self) -> FT32_R {
        FT32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 33"]
    #[inline(always)]
    pub fn ft33(&self) -> FT33_R {
        FT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 40"]
    #[inline(always)]
    pub fn ft40(&self) -> FT40_R {
        FT40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 41"]
    #[inline(always)]
    pub fn ft41(&self) -> FT41_R {
        FT41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration of line 32"]
    #[inline(always)]
    #[must_use]
    pub fn ft32(&mut self) -> FT32_W<0> {
        FT32_W::new(self)
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 33"]
    #[inline(always)]
    #[must_use]
    pub fn ft33(&mut self) -> FT33_W<1> {
        FT33_W::new(self)
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 40"]
    #[inline(always)]
    #[must_use]
    pub fn ft40(&mut self) -> FT40_W<8> {
        FT40_W::new(self)
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 41"]
    #[inline(always)]
    #[must_use]
    pub fn ft41(&mut self) -> FT41_W<9> {
        FT41_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Falling Trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr2](index.html) module"]
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
