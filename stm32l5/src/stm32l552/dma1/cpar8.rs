#[doc = "Register `CPAR8` reader"]
pub struct R(crate::R<CPAR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPAR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPAR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPAR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPAR8` writer"]
pub struct W(crate::W<CPAR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPAR8_SPEC>;
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
impl From<crate::W<CPAR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPAR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDT` reader - number of data to transfer"]
pub type NDT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NDT` writer - number of data to transfer"]
pub type NDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPAR8_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - number of data to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<0> {
        NDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar8](index.html) module"]
pub struct CPAR8_SPEC;
impl crate::RegisterSpec for CPAR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpar8::R](R) reader structure"]
impl crate::Readable for CPAR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpar8::W](W) writer structure"]
impl crate::Writable for CPAR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPAR8 to value 0"]
impl crate::Resettable for CPAR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
