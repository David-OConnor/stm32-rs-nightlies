#[doc = "Register `TISEL` reader"]
pub struct R(crate::R<TISEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TISEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TISEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TISEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TISEL` writer"]
pub struct W(crate::W<TISEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TISEL_SPEC>;
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
impl From<crate::W<TISEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TISEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TISEL` reader - TISEL"]
pub type TISEL_R = crate::FieldReader<u8, TISEL_A>;
#[doc = "TISEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TISEL_A {
    #[doc = "0: TIM1_CH1 input selected"]
    Selected = 0,
}
impl From<TISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TISEL_A) -> Self {
        variant as _
    }
}
impl TISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TISEL_A> {
        match self.bits {
            0 => Some(TISEL_A::Selected),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Selected`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == TISEL_A::Selected
    }
}
#[doc = "Field `TISEL` writer - TISEL"]
pub type TISEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TISEL_SPEC, u8, TISEL_A, 4, O>;
impl<'a, const O: u8> TISEL_W<'a, O> {
    #[doc = "TIM1_CH1 input selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(TISEL_A::Selected)
    }
}
impl R {
    #[doc = "Bits 0:3 - TISEL"]
    #[inline(always)]
    pub fn tisel(&self) -> TISEL_R {
        TISEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TISEL"]
    #[inline(always)]
    #[must_use]
    pub fn tisel(&mut self) -> TISEL_W<0> {
        TISEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM17 input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tisel](index.html) module"]
pub struct TISEL_SPEC;
impl crate::RegisterSpec for TISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tisel::R](R) reader structure"]
impl crate::Readable for TISEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tisel::W](W) writer structure"]
impl crate::Writable for TISEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TISEL to value 0"]
impl crate::Resettable for TISEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
