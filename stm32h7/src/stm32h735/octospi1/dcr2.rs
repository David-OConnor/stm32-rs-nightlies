#[doc = "Register `DCR2` reader"]
pub struct R(crate::R<DCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR2` writer"]
pub struct W(crate::W<DCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR2_SPEC>;
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
impl From<crate::W<DCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER` reader - Clock prescaler"]
pub type PRESCALER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESCALER` writer - Clock prescaler"]
pub type PRESCALER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `WRAPSIZE` reader - Wrap size"]
pub type WRAPSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRAPSIZE` writer - Wrap size"]
pub type WRAPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:7 - Clock prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Wrap size"]
    #[inline(always)]
    pub fn wrapsize(&self) -> WRAPSIZE_R {
        WRAPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<0> {
        PRESCALER_W::new(self)
    }
    #[doc = "Bits 16:18 - Wrap size"]
    #[inline(always)]
    #[must_use]
    pub fn wrapsize(&mut self) -> WRAPSIZE_W<16> {
        WRAPSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr2](index.html) module"]
pub struct DCR2_SPEC;
impl crate::RegisterSpec for DCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr2::R](R) reader structure"]
impl crate::Readable for DCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr2::W](W) writer structure"]
impl crate::Writable for DCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCR2 to value 0"]
impl crate::Resettable for DCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
