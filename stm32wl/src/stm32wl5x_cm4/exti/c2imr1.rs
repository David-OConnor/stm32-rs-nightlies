#[doc = "Register `C2IMR1` reader"]
pub struct R(crate::R<C2IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2IMR1` writer"]
pub struct W(crate::W<C2IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR1_SPEC>;
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
impl From<crate::W<C2IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM` reader - wakeup with interrupt Mask on Event input"]
pub type IM_R = crate::FieldReader<u32, IM_A>;
#[doc = "wakeup with interrupt Mask on Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum IM_A {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<IM_A> for u32 {
    #[inline(always)]
    fn from(variant: IM_A) -> Self {
        variant as _
    }
}
impl IM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IM_A> {
        match self.bits {
            0 => Some(IM_A::Masked),
            1 => Some(IM_A::Unmasked),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == IM_A::Masked
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == IM_A::Unmasked
    }
}
#[doc = "Field `IM` writer - wakeup with interrupt Mask on Event input"]
pub type IM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2IMR1_SPEC, u32, IM_A, 32, O>;
impl<'a, const O: u8> IM_W<'a, O> {
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM_A::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM_A::Unmasked)
    }
}
impl R {
    #[doc = "Bits 0:31 - wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn im(&mut self) -> IM_W<0> {
        IM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr1](index.html) module"]
pub struct C2IMR1_SPEC;
impl crate::RegisterSpec for C2IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2imr1::R](R) reader structure"]
impl crate::Readable for C2IMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2imr1::W](W) writer structure"]
impl crate::Writable for C2IMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2IMR1 to value 0"]
impl crate::Resettable for C2IMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
