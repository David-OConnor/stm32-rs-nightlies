#[doc = "Register `CONFCHR2` reader"]
pub struct R(crate::R<CONFCHR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFCHR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFCHR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFCHR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFCHR2` writer"]
pub struct W(crate::W<CONFCHR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFCHR2_SPEC>;
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
impl From<crate::W<CONFCHR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFCHR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONFCH8` reader - Channel 8 configuration"]
pub type CONFCH8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONFCH8` writer - Channel 8 configuration"]
pub type CONFCH8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFCHR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Channel 8 configuration"]
    #[inline(always)]
    pub fn confch8(&self) -> CONFCH8_R {
        CONFCH8_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 8 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn confch8(&mut self) -> CONFCH8_W<0> {
        CONFCH8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confchr2](index.html) module"]
pub struct CONFCHR2_SPEC;
impl crate::RegisterSpec for CONFCHR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [confchr2::R](R) reader structure"]
impl crate::Readable for CONFCHR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [confchr2::W](W) writer structure"]
impl crate::Writable for CONFCHR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFCHR2 to value 0"]
impl crate::Resettable for CONFCHR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
