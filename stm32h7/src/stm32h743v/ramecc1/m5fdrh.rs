#[doc = "Register `M5FDRH` reader"]
pub struct R(crate::R<M5FDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M5FDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M5FDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M5FDRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M5FDRH` writer"]
pub struct W(crate::W<M5FDRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M5FDRH_SPEC>;
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
impl From<crate::W<M5FDRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M5FDRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDATAH` reader - ECC failing data high"]
pub type FDATAH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC failing data high"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor 5 failing data high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5fdrh](index.html) module"]
pub struct M5FDRH_SPEC;
impl crate::RegisterSpec for M5FDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m5fdrh::R](R) reader structure"]
impl crate::Readable for M5FDRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m5fdrh::W](W) writer structure"]
impl crate::Writable for M5FDRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M5FDRH to value 0"]
impl crate::Resettable for M5FDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
