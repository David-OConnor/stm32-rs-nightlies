#[doc = "Register `L1WVPCR` reader"]
pub struct R(crate::R<L1WVPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1WVPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1WVPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1WVPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1WVPCR` writer"]
pub struct W(crate::W<L1WVPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1WVPCR_SPEC>;
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
impl From<crate::W<L1WVPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1WVPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WVSTPOS` reader - Window Vertical Start Position"]
pub type WVSTPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WVSTPOS` writer - Window Vertical Start Position"]
pub type WVSTPOS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, L1WVPCR_SPEC, u16, u16, 11, O>;
#[doc = "Field `WVSPPOS` reader - Window Vertical Stop Position"]
pub type WVSPPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WVSPPOS` writer - Window Vertical Stop Position"]
pub type WVSPPOS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, L1WVPCR_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Window Vertical Start Position"]
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Window Vertical Stop Position"]
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Window Vertical Start Position"]
    #[inline(always)]
    #[must_use]
    pub fn wvstpos(&mut self) -> WVSTPOS_W<0> {
        WVSTPOS_W::new(self)
    }
    #[doc = "Bits 16:26 - Window Vertical Stop Position"]
    #[inline(always)]
    #[must_use]
    pub fn wvsppos(&mut self) -> WVSPPOS_W<16> {
        WVSPPOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC Layer Window Vertical Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1wvpcr](index.html) module"]
pub struct L1WVPCR_SPEC;
impl crate::RegisterSpec for L1WVPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1wvpcr::R](R) reader structure"]
impl crate::Readable for L1WVPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1wvpcr::W](W) writer structure"]
impl crate::Writable for L1WVPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1WVPCR to value 0"]
impl crate::Resettable for L1WVPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
