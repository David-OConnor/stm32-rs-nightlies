#[doc = "Register `ICSCR` reader"]
pub struct R(crate::R<ICSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSCR` writer"]
pub struct W(crate::W<ICSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSCR_SPEC>;
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
impl From<crate::W<ICSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSICAL` reader - nternal high speed clock calibration"]
pub type HSICAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSITRIM` reader - High speed internal clock trimming"]
pub type HSITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSITRIM` writer - High speed internal clock trimming"]
pub type HSITRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `MSIRANGE` reader - MSI clock ranges"]
pub type MSIRANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSIRANGE` writer - MSI clock ranges"]
pub type MSIRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MSICAL` reader - MSI clock calibration"]
pub type MSICAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSITRIM` reader - MSI clock trimming"]
pub type MSITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSITRIM` writer - MSI clock trimming"]
pub type MSITRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - nternal high speed clock calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:23 - MSI clock calibration"]
    #[inline(always)]
    pub fn msical(&self) -> MSICAL_R {
        MSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - MSI clock trimming"]
    #[inline(always)]
    pub fn msitrim(&self) -> MSITRIM_R {
        MSITRIM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - High speed internal clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<8> {
        HSITRIM_W::new(self)
    }
    #[doc = "Bits 13:15 - MSI clock ranges"]
    #[inline(always)]
    #[must_use]
    pub fn msirange(&mut self) -> MSIRANGE_W<13> {
        MSIRANGE_W::new(self)
    }
    #[doc = "Bits 24:31 - MSI clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn msitrim(&mut self) -> MSITRIM_W<24> {
        MSITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal clock sources calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icscr](index.html) module"]
pub struct ICSCR_SPEC;
impl crate::RegisterSpec for ICSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icscr::R](R) reader structure"]
impl crate::Readable for ICSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icscr::W](W) writer structure"]
impl crate::Writable for ICSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICSCR to value 0xb000"]
impl crate::Resettable for ICSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xb000;
}
