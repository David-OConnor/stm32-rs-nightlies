#[doc = "Register `TZC_ACTION` reader"]
pub struct R(crate::R<TZC_ACTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_ACTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_ACTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_ACTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZC_ACTION` writer"]
pub struct W(crate::W<TZC_ACTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_ACTION_SPEC>;
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
impl From<crate::W<TZC_ACTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_ACTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REACTION_VALUE` reader - REACTION_VALUE"]
pub type REACTION_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REACTION_VALUE` writer - REACTION_VALUE"]
pub type REACTION_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_ACTION_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - REACTION_VALUE"]
    #[inline(always)]
    pub fn reaction_value(&self) -> REACTION_VALUE_R {
        REACTION_VALUE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - REACTION_VALUE"]
    #[inline(always)]
    #[must_use]
    pub fn reaction_value(&mut self) -> REACTION_VALUE_W<0> {
        REACTION_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls interrupt and bus error response behavior when regions permission failures occur.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_action](index.html) module"]
pub struct TZC_ACTION_SPEC;
impl crate::RegisterSpec for TZC_ACTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_action::R](R) reader structure"]
impl crate::Readable for TZC_ACTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_action::W](W) writer structure"]
impl crate::Writable for TZC_ACTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TZC_ACTION to value 0"]
impl crate::Resettable for TZC_ACTION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
