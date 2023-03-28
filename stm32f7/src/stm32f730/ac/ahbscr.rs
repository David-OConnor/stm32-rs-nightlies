#[doc = "Register `AHBSCR` reader"]
pub struct R(crate::R<AHBSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBSCR` writer"]
pub struct W(crate::W<AHBSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBSCR_SPEC>;
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
impl From<crate::W<AHBSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTL` reader - CTL"]
pub type CTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTL` writer - CTL"]
pub type CTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBSCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TPRI` reader - TPRI"]
pub type TPRI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TPRI` writer - TPRI"]
pub type TPRI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBSCR_SPEC, u16, u16, 9, O>;
#[doc = "Field `INITCOUNT` reader - INITCOUNT"]
pub type INITCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INITCOUNT` writer - INITCOUNT"]
pub type INITCOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AHBSCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:1 - CTL"]
    #[inline(always)]
    pub fn ctl(&self) -> CTL_R {
        CTL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:10 - TPRI"]
    #[inline(always)]
    pub fn tpri(&self) -> TPRI_R {
        TPRI_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:15 - INITCOUNT"]
    #[inline(always)]
    pub fn initcount(&self) -> INITCOUNT_R {
        INITCOUNT_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CTL"]
    #[inline(always)]
    #[must_use]
    pub fn ctl(&mut self) -> CTL_W<0> {
        CTL_W::new(self)
    }
    #[doc = "Bits 2:10 - TPRI"]
    #[inline(always)]
    #[must_use]
    pub fn tpri(&mut self) -> TPRI_W<2> {
        TPRI_W::new(self)
    }
    #[doc = "Bits 11:15 - INITCOUNT"]
    #[inline(always)]
    #[must_use]
    pub fn initcount(&mut self) -> INITCOUNT_W<11> {
        INITCOUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Slave Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbscr](index.html) module"]
pub struct AHBSCR_SPEC;
impl crate::RegisterSpec for AHBSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbscr::R](R) reader structure"]
impl crate::Readable for AHBSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbscr::W](W) writer structure"]
impl crate::Writable for AHBSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBSCR to value 0"]
impl crate::Resettable for AHBSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
