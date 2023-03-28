#[doc = "Register `PUPDR` reader"]
pub struct R(crate::R<PUPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUPDR` writer"]
pub struct W(crate::W<PUPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUPDR_SPEC>;
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
impl From<crate::W<PUPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUPDR3` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR3_R = crate::FieldReader<u8, PUPDR3_A>;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPDR3_A {
    #[doc = "0: No pull-up, pull-down"]
    Floating = 0,
    #[doc = "1: Pull-up"]
    PullUp = 1,
    #[doc = "2: Pull-down"]
    PullDown = 2,
}
impl From<PUPDR3_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPDR3_A) -> Self {
        variant as _
    }
}
impl PUPDR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PUPDR3_A> {
        match self.bits {
            0 => Some(PUPDR3_A::Floating),
            1 => Some(PUPDR3_A::PullUp),
            2 => Some(PUPDR3_A::PullDown),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Floating`"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == PUPDR3_A::Floating
    }
    #[doc = "Checks if the value of the field is `PullUp`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PUPDR3_A::PullUp
    }
    #[doc = "Checks if the value of the field is `PullDown`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PUPDR3_A::PullDown
    }
}
#[doc = "Field `PUPDR3` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, PUPDR3_A, 2, O>;
impl<'a, const O: u8> PUPDR3_W<'a, O> {
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR3_A::Floating)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR3_A::PullUp)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR3_A::PullDown)
    }
}
impl R {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr3(&mut self) -> PUPDR3_W<6> {
        PUPDR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pupdr](index.html) module"]
pub struct PUPDR_SPEC;
impl crate::RegisterSpec for PUPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pupdr::R](R) reader structure"]
impl crate::Readable for PUPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pupdr::W](W) writer structure"]
impl crate::Writable for PUPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUPDR to value 0"]
impl crate::Resettable for PUPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
