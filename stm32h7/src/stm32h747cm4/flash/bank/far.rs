#[doc = "Register `FAR` reader"]
pub struct R(crate::R<FAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAR` writer"]
pub struct W(crate::W<FAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAR_SPEC>;
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
impl From<crate::W<FAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAIL_ECC_ADDR` reader - Bank 1 ECC error address"]
pub type FAIL_ECC_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FAIL_ECC_ADDR` writer - Bank 1 ECC error address"]
pub type FAIL_ECC_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FAR_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Bank 1 ECC error address"]
    #[inline(always)]
    pub fn fail_ecc_addr(&self) -> FAIL_ECC_ADDR_R {
        FAIL_ECC_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Bank 1 ECC error address"]
    #[inline(always)]
    #[must_use]
    pub fn fail_ecc_addr(&mut self) -> FAIL_ECC_ADDR_W<0> {
        FAIL_ECC_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH ECC fail address for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [far](index.html) module"]
pub struct FAR_SPEC;
impl crate::RegisterSpec for FAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [far::R](R) reader structure"]
impl crate::Readable for FAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [far::W](W) writer structure"]
impl crate::Writable for FAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FAR to value 0"]
impl crate::Resettable for FAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
