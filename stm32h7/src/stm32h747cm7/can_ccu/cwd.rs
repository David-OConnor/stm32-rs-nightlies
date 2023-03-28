#[doc = "Register `CWD` reader"]
pub struct R(crate::R<CWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWD` writer"]
pub struct W(crate::W<CWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWD_SPEC>;
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
impl From<crate::W<CWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDC` reader - WDC"]
pub type WDC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDC` writer - WDC"]
pub type WDC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWD_SPEC, u16, u16, 16, O>;
#[doc = "Field `WDV` reader - WDV"]
pub type WDV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDV` writer - WDV"]
pub type WDV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - WDC"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - WDV"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDC"]
    #[inline(always)]
    #[must_use]
    pub fn wdc(&mut self) -> WDC_W<0> {
        WDC_W::new(self)
    }
    #[doc = "Bits 16:31 - WDV"]
    #[inline(always)]
    #[must_use]
    pub fn wdv(&mut self) -> WDV_W<16> {
        WDV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Watchdog Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwd](index.html) module"]
pub struct CWD_SPEC;
impl crate::RegisterSpec for CWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwd::R](R) reader structure"]
impl crate::Readable for CWD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwd::W](W) writer structure"]
impl crate::Writable for CWD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWD to value 0"]
impl crate::Resettable for CWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
