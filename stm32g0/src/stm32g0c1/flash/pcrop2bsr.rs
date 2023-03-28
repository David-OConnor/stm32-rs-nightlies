#[doc = "Register `PCROP2BSR` reader"]
pub struct R(crate::R<PCROP2BSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP2BSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP2BSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP2BSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCROP2BSR` writer"]
pub struct W(crate::W<PCROP2BSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP2BSR_SPEC>;
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
impl From<crate::W<PCROP2BSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP2BSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP2B_STRT` reader - PCROP2B area start offset, Bank 2"]
pub type PCROP2B_STRT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCROP2B_STRT` writer - PCROP2B area start offset, Bank 2"]
pub type PCROP2B_STRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCROP2BSR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - PCROP2B area start offset, Bank 2"]
    #[inline(always)]
    pub fn pcrop2b_strt(&self) -> PCROP2B_STRT_R {
        PCROP2B_STRT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PCROP2B area start offset, Bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop2b_strt(&mut self) -> PCROP2B_STRT_W<0> {
        PCROP2B_STRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH PCROP2 area B start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop2bsr](index.html) module"]
pub struct PCROP2BSR_SPEC;
impl crate::RegisterSpec for PCROP2BSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop2bsr::R](R) reader structure"]
impl crate::Readable for PCROP2BSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrop2bsr::W](W) writer structure"]
impl crate::Writable for PCROP2BSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCROP2BSR to value 0xffff_ffff"]
impl crate::Resettable for PCROP2BSR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
