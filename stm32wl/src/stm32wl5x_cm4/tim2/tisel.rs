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
#[doc = "Field `TI1SEL` reader - TI1SEL"]
pub type TI1SEL_R = crate::FieldReader<u8, TI1SEL_A>;
#[doc = "TI1SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1SEL_A {
    #[doc = "0: TIM1_CHx input selected"]
    Selected = 0,
}
impl From<TI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TI1SEL_A) -> Self {
        variant as _
    }
}
impl TI1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TI1SEL_A> {
        match self.bits {
            0 => Some(TI1SEL_A::Selected),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Selected`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == TI1SEL_A::Selected
    }
}
#[doc = "Field `TI1SEL` writer - TI1SEL"]
pub type TI1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TISEL_SPEC, u8, TI1SEL_A, 4, O>;
impl<'a, const O: u8> TI1SEL_W<'a, O> {
    #[doc = "TIM1_CHx input selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(TI1SEL_A::Selected)
    }
}
#[doc = "Field `TI2SEL` reader - TI2SEL"]
pub use TI1SEL_R as TI2SEL_R;
#[doc = "Field `TI2SEL` writer - TI2SEL"]
pub use TI1SEL_W as TI2SEL_W;
impl R {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TI2SEL"]
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<0> {
        TI1SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - TI2SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ti2sel(&mut self) -> TI2SEL_W<8> {
        TI2SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM2 timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tisel](index.html) module"]
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
