#[doc = "Register `FDCAN_TSCV` reader"]
pub struct R(crate::R<FDCAN_TSCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TSCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TSCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TSCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TSCV` writer"]
pub struct W(crate::W<FDCAN_TSCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TSCV_SPEC>;
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
impl From<crate::W<FDCAN_TSCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TSCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSC` reader - TSC"]
pub type TSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSC` writer - TSC"]
pub type TSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TSCV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - TSC"]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TSC"]
    #[inline(always)]
    #[must_use]
    pub fn tsc(&mut self) -> TSC_W<0> {
        TSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN timestamp counter value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tscv](index.html) module"]
pub struct FDCAN_TSCV_SPEC;
impl crate::RegisterSpec for FDCAN_TSCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_tscv::R](R) reader structure"]
impl crate::Readable for FDCAN_TSCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_tscv::W](W) writer structure"]
impl crate::Writable for FDCAN_TSCV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TSCV to value 0"]
impl crate::Resettable for FDCAN_TSCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
