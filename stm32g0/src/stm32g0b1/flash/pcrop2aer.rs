#[doc = "Register `PCROP2AER` reader"]
pub struct R(crate::R<PCROP2AER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP2AER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP2AER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP2AER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCROP2AER` writer"]
pub struct W(crate::W<PCROP2AER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP2AER_SPEC>;
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
impl From<crate::W<PCROP2AER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP2AER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP2A_END` reader - PCROP2A area end offset, bank2"]
pub type PCROP2A_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCROP2A_END` writer - PCROP2A area end offset, bank2"]
pub type PCROP2A_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCROP2AER_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - PCROP2A area end offset, bank2"]
    #[inline(always)]
    pub fn pcrop2a_end(&self) -> PCROP2A_END_R {
        PCROP2A_END_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PCROP2A area end offset, bank2"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop2a_end(&mut self) -> PCROP2A_END_W<0> {
        PCROP2A_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash PCROP2 area A end address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop2aer](index.html) module"]
pub struct PCROP2AER_SPEC;
impl crate::RegisterSpec for PCROP2AER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop2aer::R](R) reader structure"]
impl crate::Readable for PCROP2AER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrop2aer::W](W) writer structure"]
impl crate::Writable for PCROP2AER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCROP2AER to value 0"]
impl crate::Resettable for PCROP2AER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
