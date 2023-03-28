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
#[doc = "Field `HSICAL` reader - HSI clock calibration"]
pub type HSICAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSITRIM` reader - HSI clock trimming"]
pub type HSITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSITRIM` writer - HSI clock trimming"]
pub type HSITRIM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICSCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `CSICAL` reader - CSI clock calibration"]
pub type CSICAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSITRIM` reader - CSI clock trimming"]
pub type CSITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSITRIM` writer - CSI clock trimming"]
pub type CSITRIM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICSCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:11 - HSI clock calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:17 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:25 - CSI clock calibration"]
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 26:30 - CSI clock trimming"]
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:17 - HSI clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<12> {
        HSITRIM_W::new(self)
    }
    #[doc = "Bits 26:30 - CSI clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn csitrim(&mut self) -> CSITRIM_W<26> {
        CSITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Internal Clock Source Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icscr](index.html) module"]
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
