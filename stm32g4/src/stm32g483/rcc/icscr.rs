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
#[doc = "Field `HSICAL0` reader - Internal High Speed clock Calibration"]
pub type HSICAL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSITRIM` reader - Internal High Speed clock trimming"]
pub type HSITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSITRIM` writer - Internal High Speed clock trimming"]
pub type HSITRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSCR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 16:23 - Internal High Speed clock Calibration"]
    #[inline(always)]
    pub fn hsical0(&self) -> HSICAL0_R {
        HSICAL0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Internal High Speed clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30 - Internal High Speed clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<24> {
        HSITRIM_W::new(self)
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
#[doc = "`reset()` method sets ICSCR to value 0x4000_0000"]
impl crate::Resettable for ICSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
