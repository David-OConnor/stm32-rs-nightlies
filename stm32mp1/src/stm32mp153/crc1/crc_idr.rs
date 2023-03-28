#[doc = "Register `CRC_IDR` reader"]
pub struct R(crate::R<CRC_IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_IDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_IDR` writer"]
pub struct W(crate::W<CRC_IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_IDR_SPEC>;
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
impl From<crate::W<CRC_IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDR` reader - IDR"]
pub type IDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IDR` writer - IDR"]
pub type IDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRC_IDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IDR"]
    #[inline(always)]
    pub fn idr(&self) -> IDR_R {
        IDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IDR"]
    #[inline(always)]
    #[must_use]
    pub fn idr(&mut self) -> IDR_W<0> {
        IDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC independent data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_idr](index.html) module"]
pub struct CRC_IDR_SPEC;
impl crate::RegisterSpec for CRC_IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_idr::R](R) reader structure"]
impl crate::Readable for CRC_IDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_idr::W](W) writer structure"]
impl crate::Writable for CRC_IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_IDR to value 0"]
impl crate::Resettable for CRC_IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
