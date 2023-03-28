#[doc = "Register `EXTICR4` reader"]
pub struct R(crate::R<EXTICR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR4` writer"]
pub struct W(crate::W<EXTICR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR4_SPEC>;
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
impl From<crate::W<EXTICR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0_7` reader - EXTIm GPIO port selection"]
pub type EXTI0_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI0_7` writer - EXTIm GPIO port selection"]
pub type EXTI0_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXTI8_15` reader - EXTIm+1 GPIO port selection"]
pub type EXTI8_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI8_15` writer - EXTIm+1 GPIO port selection"]
pub type EXTI8_15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXTI16_23` reader - EXTIm+2 GPIO port selection"]
pub type EXTI16_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI16_23` writer - EXTIm+2 GPIO port selection"]
pub type EXTI16_23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXTI24_31` reader - EXTIm+3 GPIO port selection"]
pub type EXTI24_31_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI24_31` writer - EXTIm+3 GPIO port selection"]
pub type EXTI24_31_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - EXTIm GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&self) -> EXTI0_7_R {
        EXTI0_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTIm+1 GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&self) -> EXTI8_15_R {
        EXTI8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTIm+2 GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&self) -> EXTI16_23_R {
        EXTI16_23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTIm+3 GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&self) -> EXTI24_31_R {
        EXTI24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTIm GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti0_7(&mut self) -> EXTI0_7_W<0> {
        EXTI0_7_W::new(self)
    }
    #[doc = "Bits 8:15 - EXTIm+1 GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti8_15(&mut self) -> EXTI8_15_W<8> {
        EXTI8_15_W::new(self)
    }
    #[doc = "Bits 16:23 - EXTIm+2 GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti16_23(&mut self) -> EXTI16_23_W<16> {
        EXTI16_23_W::new(self)
    }
    #[doc = "Bits 24:31 - EXTIm+3 GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti24_31(&mut self) -> EXTI24_31_W<24> {
        EXTI24_31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](index.html) module"]
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr4::R](R) reader structure"]
impl crate::Readable for EXTICR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr4::W](W) writer structure"]
impl crate::Writable for EXTICR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTICR4 to value 0"]
impl crate::Resettable for EXTICR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
