#[doc = "Register `LUT990H` reader"]
pub struct R(crate::R<LUT990H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT990H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT990H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT990H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT990H` writer"]
pub struct W(crate::W<LUT990H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT990H_SPEC>;
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
impl From<crate::W<LUT990H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT990H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LO` reader - Line offset"]
pub type LO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LO` writer - Line offset"]
pub type LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT990H_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 4:21 - Line offset"]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits >> 4) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 4:21 - Line offset"]
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<4> {
        LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphic MMU LUT entry 990 high\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut990h](index.html) module"]
pub struct LUT990H_SPEC;
impl crate::RegisterSpec for LUT990H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lut990h::R](R) reader structure"]
impl crate::Readable for LUT990H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut990h::W](W) writer structure"]
impl crate::Writable for LUT990H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUT990H to value 0"]
impl crate::Resettable for LUT990H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
