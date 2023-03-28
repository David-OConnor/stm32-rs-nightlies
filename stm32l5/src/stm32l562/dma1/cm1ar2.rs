#[doc = "Register `CM1AR2` reader"]
pub struct R(crate::R<CM1AR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM1AR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM1AR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM1AR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM1AR2` writer"]
pub struct W(crate::W<CM1AR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM1AR2_SPEC>;
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
impl From<crate::W<CM1AR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM1AR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub type NDT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub type NDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM1AR2_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Number of data to transfer"]
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
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1ar2](index.html) module"]
pub struct CM1AR2_SPEC;
impl crate::RegisterSpec for CM1AR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm1ar2::R](R) reader structure"]
impl crate::Readable for CM1AR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm1ar2::W](W) writer structure"]
impl crate::Writable for CM1AR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM1AR2 to value 0"]
impl crate::Resettable for CM1AR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
