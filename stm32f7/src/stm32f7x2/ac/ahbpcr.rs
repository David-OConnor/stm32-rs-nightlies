#[doc = "Register `AHBPCR` reader"]
pub struct R(crate::R<AHBPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBPCR` writer"]
pub struct W(crate::W<AHBPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBPCR_SPEC>;
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
impl From<crate::W<AHBPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBPCR_SPEC, bool, O>;
#[doc = "Field `SZ` reader - SZ"]
pub type SZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SZ` writer - SZ"]
pub type SZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBPCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SZ"]
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:3 - SZ"]
    #[inline(always)]
    #[must_use]
    pub fn sz(&mut self) -> SZ_W<1> {
        SZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHBP Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbpcr](index.html) module"]
pub struct AHBPCR_SPEC;
impl crate::RegisterSpec for AHBPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbpcr::R](R) reader structure"]
impl crate::Readable for AHBPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbpcr::W](W) writer structure"]
impl crate::Writable for AHBPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBPCR to value 0"]
impl crate::Resettable for AHBPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
