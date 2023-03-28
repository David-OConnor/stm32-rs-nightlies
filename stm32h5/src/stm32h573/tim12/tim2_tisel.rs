#[doc = "Register `TIM2_TISEL` reader"]
pub struct R(crate::R<TIM2_TISEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM2_TISEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM2_TISEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM2_TISEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM2_TISEL` writer"]
pub struct W(crate::W<TIM2_TISEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM2_TISEL_SPEC>;
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
impl From<crate::W<TIM2_TISEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM2_TISEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI1SEL` reader - Selects tim_ti1\\[0..15\\]
input ... Refer to for product specific implementation."]
pub type TI1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI1SEL` writer - Selects tim_ti1\\[0..15\\]
input ... Refer to for product specific implementation."]
pub type TI1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM2_TISEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `TI2SEL` reader - Selects tim_ti2\\[0..15\\]
input ... Refer to for product specific implementation."]
pub type TI2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI2SEL` writer - Selects tim_ti2\\[0..15\\]
input ... Refer to for product specific implementation."]
pub type TI2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM2_TISEL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Selects tim_ti1\\[0..15\\]
input ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Selects tim_ti2\\[0..15\\]
input ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects tim_ti1\\[0..15\\]
input ... Refer to for product specific implementation."]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<0> {
        TI1SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - Selects tim_ti2\\[0..15\\]
input ... Refer to for product specific implementation."]
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
#[doc = "TIM2 timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_tisel](index.html) module"]
pub struct TIM2_TISEL_SPEC;
impl crate::RegisterSpec for TIM2_TISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim2_tisel::R](R) reader structure"]
impl crate::Readable for TIM2_TISEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim2_tisel::W](W) writer structure"]
impl crate::Writable for TIM2_TISEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM2_TISEL to value 0"]
impl crate::Resettable for TIM2_TISEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
