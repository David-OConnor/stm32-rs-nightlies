#[doc = "Register `TIM15_AF2` reader"]
pub struct R(crate::R<TIM15_AF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM15_AF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM15_AF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM15_AF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM15_AF2` writer"]
pub struct W(crate::W<TIM15_AF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM15_AF2_SPEC>;
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
impl From<crate::W<TIM15_AF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM15_AF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCRSEL` reader - ocref_clr source selection These bits select the ocref_clr input source. Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register)."]
pub type OCRSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OCRSEL` writer - ocref_clr source selection These bits select the ocref_clr input source. Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register)."]
pub type OCRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM15_AF2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    pub fn ocrsel(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ocrsel(&mut self) -> OCRSEL_W<16> {
        OCRSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM15 alternate function register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_af2](index.html) module"]
pub struct TIM15_AF2_SPEC;
impl crate::RegisterSpec for TIM15_AF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim15_af2::R](R) reader structure"]
impl crate::Readable for TIM15_AF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim15_af2::W](W) writer structure"]
impl crate::Writable for TIM15_AF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM15_AF2 to value 0"]
impl crate::Resettable for TIM15_AF2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
