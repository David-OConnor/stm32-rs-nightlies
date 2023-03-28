#[doc = "Register `EMR2` reader"]
pub struct R(crate::R<EMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR2` writer"]
pub struct W(crate::W<EMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR2_SPEC>;
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
impl From<crate::W<EMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM` reader - CPU(m) Wakeup with event generation Mask on Event input"]
pub type EM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EM` writer - CPU(m) Wakeup with event generation Mask on Event input"]
pub type EM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 8:9 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    #[must_use]
    pub fn em(&mut self) -> EM_W<8> {
        EM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPUm wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr2](index.html) module"]
pub struct EMR2_SPEC;
impl crate::RegisterSpec for EMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr2::R](R) reader structure"]
impl crate::Readable for EMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr2::W](W) writer structure"]
impl crate::Writable for EMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for EMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
