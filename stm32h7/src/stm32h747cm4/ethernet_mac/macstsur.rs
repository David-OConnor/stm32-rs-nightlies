#[doc = "Register `MACSTSUR` reader"]
pub struct R(crate::R<MACSTSUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSTSUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSTSUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSTSUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACSTSUR` writer"]
pub struct W(crate::W<MACSTSUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACSTSUR_SPEC>;
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
impl From<crate::W<MACSTSUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACSTSUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSS` reader - Timestamp Seconds"]
pub type TSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSS` writer - Timestamp Seconds"]
pub type TSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACSTSUR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Seconds"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Seconds"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<0> {
        TSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System time seconds update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macstsur](index.html) module"]
pub struct MACSTSUR_SPEC;
impl crate::RegisterSpec for MACSTSUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macstsur::R](R) reader structure"]
impl crate::Readable for MACSTSUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macstsur::W](W) writer structure"]
impl crate::Writable for MACSTSUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACSTSUR to value 0"]
impl crate::Resettable for MACSTSUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
