#[doc = "Register `FDCAN_TXBAR` reader"]
pub struct R(crate::R<FDCAN_TXBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TXBAR` writer"]
pub struct W(crate::W<FDCAN_TXBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXBAR_SPEC>;
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
impl From<crate::W<FDCAN_TXBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AR` reader - Add Request"]
pub type AR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AR` writer - Add Request"]
pub type AR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TXBAR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Add Request"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Add Request"]
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> AR_W<0> {
        AR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx Buffer Add Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbar](index.html) module"]
pub struct FDCAN_TXBAR_SPEC;
impl crate::RegisterSpec for FDCAN_TXBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbar::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_txbar::W](W) writer structure"]
impl crate::Writable for FDCAN_TXBAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBAR to value 0"]
impl crate::Resettable for FDCAN_TXBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
