#[doc = "Register `DDRPERFM_IER` reader"]
pub struct R(crate::R<DDRPERFM_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPERFM_IER` writer"]
pub struct W(crate::W<DDRPERFM_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPERFM_IER_SPEC>;
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
impl From<crate::W<DDRPERFM_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPERFM_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVFIE` reader - OVFIE"]
pub type OVFIE_R = crate::BitReader<bool>;
#[doc = "Field `OVFIE` writer - OVFIE"]
pub type OVFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPERFM_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OVFIE"]
    #[inline(always)]
    pub fn ovfie(&self) -> OVFIE_R {
        OVFIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OVFIE"]
    #[inline(always)]
    #[must_use]
    pub fn ovfie(&mut self) -> OVFIE_W<0> {
        OVFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPERFM interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_ier](index.html) module"]
pub struct DDRPERFM_IER_SPEC;
impl crate::RegisterSpec for DDRPERFM_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_ier::R](R) reader structure"]
impl crate::Readable for DDRPERFM_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrperfm_ier::W](W) writer structure"]
impl crate::Writable for DDRPERFM_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPERFM_IER to value 0"]
impl crate::Resettable for DDRPERFM_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
