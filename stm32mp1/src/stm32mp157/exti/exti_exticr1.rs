#[doc = "Register `EXTI_EXTICR1` reader"]
pub struct R(crate::R<EXTI_EXTICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_EXTICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_EXTICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_EXTICR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_EXTICR1` writer"]
pub struct W(crate::W<EXTI_EXTICR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_EXTICR1_SPEC>;
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
impl From<crate::W<EXTI_EXTICR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_EXTICR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0` reader - EXTI0"]
pub type EXTI0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI0` writer - EXTI0"]
pub type EXTI0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTI_EXTICR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXTI1` reader - EXTI1"]
pub type EXTI1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI1` writer - EXTI1"]
pub type EXTI1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTI_EXTICR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXTI2` reader - EXTI2"]
pub type EXTI2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI2` writer - EXTI2"]
pub type EXTI2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTI_EXTICR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXTI3` reader - EXTI3"]
pub type EXTI3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI3` writer - EXTI3"]
pub type EXTI3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTI_EXTICR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - EXTI0"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI1"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI2"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI3"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI0"]
    #[inline(always)]
    #[must_use]
    pub fn exti0(&mut self) -> EXTI0_W<0> {
        EXTI0_W::new(self)
    }
    #[doc = "Bits 8:15 - EXTI1"]
    #[inline(always)]
    #[must_use]
    pub fn exti1(&mut self) -> EXTI1_W<8> {
        EXTI1_W::new(self)
    }
    #[doc = "Bits 16:23 - EXTI2"]
    #[inline(always)]
    #[must_use]
    pub fn exti2(&mut self) -> EXTI2_W<16> {
        EXTI2_W::new(self)
    }
    #[doc = "Bits 24:31 - EXTI3"]
    #[inline(always)]
    #[must_use]
    pub fn exti3(&mut self) -> EXTI3_W<24> {
        EXTI3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_exticr1](index.html) module"]
pub struct EXTI_EXTICR1_SPEC;
impl crate::RegisterSpec for EXTI_EXTICR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_exticr1::R](R) reader structure"]
impl crate::Readable for EXTI_EXTICR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_exticr1::W](W) writer structure"]
impl crate::Writable for EXTI_EXTICR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_EXTICR1 to value 0"]
impl crate::Resettable for EXTI_EXTICR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
