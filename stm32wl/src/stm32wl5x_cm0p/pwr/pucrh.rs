#[doc = "Register `PUCRH` reader"]
pub struct R(crate::R<PUCRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRH` writer"]
pub struct W(crate::W<PUCRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRH_SPEC>;
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
impl From<crate::W<PUCRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU3` reader - pull-up"]
pub type PU3_R = crate::BitReader<PU3_A>;
#[doc = "pull-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU3_A {
    #[doc = "0: Disable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Disabled = 0,
    #[doc = "1: Enable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PH\\[y\\]
bit is also set"]
    Enabled = 1,
}
impl From<PU3_A> for bool {
    #[inline(always)]
    fn from(variant: PU3_A) -> Self {
        variant as u8 != 0
    }
}
impl PU3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PU3_A {
        match self.bits {
            false => PU3_A::Disabled,
            true => PU3_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU3_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU3_A::Enabled
    }
}
#[doc = "Field `PU3` writer - pull-up"]
pub type PU3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRH_SPEC, PU3_A, O>;
impl<'a, const O: u8> PU3_W<'a, O> {
    #[doc = "Disable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU3_A::Disabled)
    }
    #[doc = "Enable pull-up on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PH\\[y\\]
bit is also set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU3_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 3 - pull-up"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - pull-up"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<3> {
        PU3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port H pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrh](index.html) module"]
pub struct PUCRH_SPEC;
impl crate::RegisterSpec for PUCRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucrh::R](R) reader structure"]
impl crate::Readable for PUCRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucrh::W](W) writer structure"]
impl crate::Writable for PUCRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUCRH to value 0"]
impl crate::Resettable for PUCRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
