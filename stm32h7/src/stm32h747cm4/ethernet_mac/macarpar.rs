#[doc = "Register `MACARPAR` reader"]
pub struct R(crate::R<MACARPAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACARPAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACARPAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACARPAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACARPAR` writer"]
pub struct W(crate::W<MACARPAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACARPAR_SPEC>;
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
impl From<crate::W<MACARPAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACARPAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARPPA` reader - ARP Protocol Address"]
pub type ARPPA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ARPPA` writer - ARP Protocol Address"]
pub type ARPPA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACARPAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ARP Protocol Address"]
    #[inline(always)]
    pub fn arppa(&self) -> ARPPA_R {
        ARPPA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ARP Protocol Address"]
    #[inline(always)]
    #[must_use]
    pub fn arppa(&mut self) -> ARPPA_W<0> {
        ARPPA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ARP address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macarpar](index.html) module"]
pub struct MACARPAR_SPEC;
impl crate::RegisterSpec for MACARPAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macarpar::R](R) reader structure"]
impl crate::Readable for MACARPAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macarpar::W](W) writer structure"]
impl crate::Writable for MACARPAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACARPAR to value 0"]
impl crate::Resettable for MACARPAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
