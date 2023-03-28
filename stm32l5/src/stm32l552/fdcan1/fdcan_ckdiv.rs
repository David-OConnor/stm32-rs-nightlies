#[doc = "Register `FDCAN_CKDIV` reader"]
pub struct R(crate::R<FDCAN_CKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_CKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_CKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_CKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_CKDIV` writer"]
pub struct W(crate::W<FDCAN_CKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_CKDIV_SPEC>;
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
impl From<crate::W<FDCAN_CKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_CKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDIV` reader - PDIV"]
pub type PDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDIV` writer - PDIV"]
pub type PDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_CKDIV_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PDIV"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDIV"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<0> {
        PDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Trigger Memory Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ckdiv](index.html) module"]
pub struct FDCAN_CKDIV_SPEC;
impl crate::RegisterSpec for FDCAN_CKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ckdiv::R](R) reader structure"]
impl crate::Readable for FDCAN_CKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ckdiv::W](W) writer structure"]
impl crate::Writable for FDCAN_CKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_CKDIV to value 0"]
impl crate::Resettable for FDCAN_CKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
