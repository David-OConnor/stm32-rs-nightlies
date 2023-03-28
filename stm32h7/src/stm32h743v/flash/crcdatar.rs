#[doc = "Register `CRCDATAR` reader"]
pub struct R(crate::R<CRCDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCDATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCDATAR` writer"]
pub struct W(crate::W<CRCDATAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCDATAR_SPEC>;
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
impl From<crate::W<CRCDATAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCDATAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_DATA` reader - CRC result"]
pub type CRC_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRC_DATA` writer - CRC result"]
pub type CRC_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRCDATAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CRC result"]
    #[inline(always)]
    pub fn crc_data(&self) -> CRC_DATA_R {
        CRC_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC result"]
    #[inline(always)]
    #[must_use]
    pub fn crc_data(&mut self) -> CRC_DATA_W<0> {
        CRC_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH CRC data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdatar](index.html) module"]
pub struct CRCDATAR_SPEC;
impl crate::RegisterSpec for CRCDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcdatar::R](R) reader structure"]
impl crate::Readable for CRCDATAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcdatar::W](W) writer structure"]
impl crate::Writable for CRCDATAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDATAR to value 0"]
impl crate::Resettable for CRCDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
