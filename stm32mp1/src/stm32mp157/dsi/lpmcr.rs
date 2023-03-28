#[doc = "Register `LPMCR` reader"]
pub struct R(crate::R<LPMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMCR` writer"]
pub struct W(crate::W<LPMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMCR_SPEC>;
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
impl From<crate::W<LPMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLPSIZE` reader - VLPSIZE"]
pub type VLPSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VLPSIZE` writer - VLPSIZE"]
pub type VLPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `LPSIZE` reader - LPSIZE"]
pub type LPSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPSIZE` writer - LPSIZE"]
pub type LPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - VLPSIZE"]
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LPSIZE"]
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VLPSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn vlpsize(&mut self) -> VLPSIZE_W<0> {
        VLPSIZE_W::new(self)
    }
    #[doc = "Bits 16:23 - LPSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn lpsize(&mut self) -> LPSIZE_W<16> {
        LPSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host low-power mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcr](index.html) module"]
pub struct LPMCR_SPEC;
impl crate::RegisterSpec for LPMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmcr::R](R) reader structure"]
impl crate::Readable for LPMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmcr::W](W) writer structure"]
impl crate::Writable for LPMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMCR to value 0"]
impl crate::Resettable for LPMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
