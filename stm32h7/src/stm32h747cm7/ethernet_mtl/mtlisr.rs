#[doc = "Register `MTLISR` reader"]
pub struct R(crate::R<MTLISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLISR` writer"]
pub struct W(crate::W<MTLISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLISR_SPEC>;
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
impl From<crate::W<MTLISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Q0IS` reader - Queue interrupt status"]
pub type Q0IS_R = crate::BitReader<bool>;
#[doc = "Field `Q0IS` writer - Queue interrupt status"]
pub type Q0IS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Queue interrupt status"]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Queue interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn q0is(&mut self) -> Q0IS_W<0> {
        Q0IS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlisr](index.html) module"]
pub struct MTLISR_SPEC;
impl crate::RegisterSpec for MTLISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlisr::R](R) reader structure"]
impl crate::Readable for MTLISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtlisr::W](W) writer structure"]
impl crate::Writable for MTLISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTLISR to value 0"]
impl crate::Resettable for MTLISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
