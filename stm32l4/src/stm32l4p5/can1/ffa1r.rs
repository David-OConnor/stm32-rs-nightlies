#[doc = "Register `FFA1R` reader"]
pub struct R(crate::R<FFA1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFA1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFA1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFA1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFA1R` writer"]
pub struct W(crate::W<FFA1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFA1R_SPEC>;
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
impl From<crate::W<FFA1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFA1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFA` reader - Filter FIFO assignment for filter 0"]
pub type FFA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FFA` writer - Filter FIFO assignment for filter 0"]
pub type FFA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFA1R_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:27 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa(&self) -> FFA_R {
        FFA_R::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    #[must_use]
    pub fn ffa(&mut self) -> FFA_W<0> {
        FFA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "filter FIFO assignment register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffa1r](index.html) module"]
pub struct FFA1R_SPEC;
impl crate::RegisterSpec for FFA1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffa1r::R](R) reader structure"]
impl crate::Readable for FFA1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffa1r::W](W) writer structure"]
impl crate::Writable for FFA1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FFA1R to value 0"]
impl crate::Resettable for FFA1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
