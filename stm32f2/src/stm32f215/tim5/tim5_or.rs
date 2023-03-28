#[doc = "Register `TIM5_OR` reader"]
pub struct R(crate::R<TIM5_OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM5_OR` writer"]
pub struct W(crate::W<TIM5_OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_OR_SPEC>;
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
impl From<crate::W<TIM5_OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IT4_RMP` reader - Timer Input 4 remap"]
pub type IT4_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IT4_RMP` writer - Timer Input 4 remap"]
pub type IT4_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM5_OR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 6:7 - Timer Input 4 remap"]
    #[inline(always)]
    pub fn it4_rmp(&self) -> IT4_RMP_R {
        IT4_RMP_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Timer Input 4 remap"]
    #[inline(always)]
    #[must_use]
    pub fn it4_rmp(&mut self) -> IT4_RMP_W<6> {
        IT4_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM5 option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_or](index.html) module"]
pub struct TIM5_OR_SPEC;
impl crate::RegisterSpec for TIM5_OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim5_or::R](R) reader structure"]
impl crate::Readable for TIM5_OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim5_or::W](W) writer structure"]
impl crate::Writable for TIM5_OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM5_OR to value 0"]
impl crate::Resettable for TIM5_OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
