#[doc = "Register `WPCR1` reader"]
pub struct R(crate::R<WPCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR1` writer"]
pub struct W(crate::W<WPCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR1_SPEC>;
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
impl From<crate::W<WPCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SKEWCL` reader - SKEWCL"]
pub type SKEWCL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SKEWCL` writer - SKEWCL"]
pub type SKEWCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SKEWDL` reader - SKEWDL"]
pub type SKEWDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SKEWDL` writer - SKEWDL"]
pub type SKEWDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPTXSRCL` reader - LPTXSRCL"]
pub type LPTXSRCL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTXSRCL` writer - LPTXSRCL"]
pub type LPTXSRCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPTXSRDL` reader - LPTXSRDL"]
pub type LPTXSRDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTXSRDL` writer - LPTXSRDL"]
pub type LPTXSRDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SDDCCL` reader - SDDCCL"]
pub type SDDCCL_R = crate::BitReader<bool>;
#[doc = "Field `SDDCCL` writer - SDDCCL"]
pub type SDDCCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR1_SPEC, bool, O>;
#[doc = "Field `SDDCDL` reader - SDDCDL"]
pub type SDDCDL_R = crate::BitReader<bool>;
#[doc = "Field `SDDCDL` writer - SDDCDL"]
pub type SDDCDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR1_SPEC, bool, O>;
#[doc = "Field `HSTXSRUCL` reader - HSTXSRUCL"]
pub type HSTXSRUCL_R = crate::BitReader<bool>;
#[doc = "Field `HSTXSRUCL` writer - HSTXSRUCL"]
pub type HSTXSRUCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR1_SPEC, bool, O>;
#[doc = "Field `HSTXSRDCL` reader - HSTXSRDCL"]
pub type HSTXSRDCL_R = crate::BitReader<bool>;
#[doc = "Field `HSTXSRDCL` writer - HSTXSRDCL"]
pub type HSTXSRDCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR1_SPEC, bool, O>;
#[doc = "Field `HSTXSRUDL` reader - HSTXSRUDL"]
pub type HSTXSRUDL_R = crate::BitReader<bool>;
#[doc = "Field `HSTXSRUDL` writer - HSTXSRUDL"]
pub type HSTXSRUDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR1_SPEC, bool, O>;
#[doc = "Field `HSTXSRDDL` reader - HSTXSRDDL"]
pub type HSTXSRDDL_R = crate::BitReader<bool>;
#[doc = "Field `HSTXSRDDL` writer - HSTXSRDDL"]
pub type HSTXSRDDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - SKEWCL"]
    #[inline(always)]
    pub fn skewcl(&self) -> SKEWCL_R {
        SKEWCL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SKEWDL"]
    #[inline(always)]
    pub fn skewdl(&self) -> SKEWDL_R {
        SKEWDL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - LPTXSRCL"]
    #[inline(always)]
    pub fn lptxsrcl(&self) -> LPTXSRCL_R {
        LPTXSRCL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - LPTXSRDL"]
    #[inline(always)]
    pub fn lptxsrdl(&self) -> LPTXSRDL_R {
        LPTXSRDL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - SDDCCL"]
    #[inline(always)]
    pub fn sddccl(&self) -> SDDCCL_R {
        SDDCCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SDDCDL"]
    #[inline(always)]
    pub fn sddcdl(&self) -> SDDCDL_R {
        SDDCDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - HSTXSRUCL"]
    #[inline(always)]
    pub fn hstxsrucl(&self) -> HSTXSRUCL_R {
        HSTXSRUCL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSTXSRDCL"]
    #[inline(always)]
    pub fn hstxsrdcl(&self) -> HSTXSRDCL_R {
        HSTXSRDCL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSTXSRUDL"]
    #[inline(always)]
    pub fn hstxsrudl(&self) -> HSTXSRUDL_R {
        HSTXSRUDL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSTXSRDDL"]
    #[inline(always)]
    pub fn hstxsrddl(&self) -> HSTXSRDDL_R {
        HSTXSRDDL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SKEWCL"]
    #[inline(always)]
    #[must_use]
    pub fn skewcl(&mut self) -> SKEWCL_W<0> {
        SKEWCL_W::new(self)
    }
    #[doc = "Bits 2:3 - SKEWDL"]
    #[inline(always)]
    #[must_use]
    pub fn skewdl(&mut self) -> SKEWDL_W<2> {
        SKEWDL_W::new(self)
    }
    #[doc = "Bits 6:7 - LPTXSRCL"]
    #[inline(always)]
    #[must_use]
    pub fn lptxsrcl(&mut self) -> LPTXSRCL_W<6> {
        LPTXSRCL_W::new(self)
    }
    #[doc = "Bits 8:9 - LPTXSRDL"]
    #[inline(always)]
    #[must_use]
    pub fn lptxsrdl(&mut self) -> LPTXSRDL_W<8> {
        LPTXSRDL_W::new(self)
    }
    #[doc = "Bit 12 - SDDCCL"]
    #[inline(always)]
    #[must_use]
    pub fn sddccl(&mut self) -> SDDCCL_W<12> {
        SDDCCL_W::new(self)
    }
    #[doc = "Bit 13 - SDDCDL"]
    #[inline(always)]
    #[must_use]
    pub fn sddcdl(&mut self) -> SDDCDL_W<13> {
        SDDCDL_W::new(self)
    }
    #[doc = "Bit 16 - HSTXSRUCL"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrucl(&mut self) -> HSTXSRUCL_W<16> {
        HSTXSRUCL_W::new(self)
    }
    #[doc = "Bit 17 - HSTXSRDCL"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrdcl(&mut self) -> HSTXSRDCL_W<17> {
        HSTXSRDCL_W::new(self)
    }
    #[doc = "Bit 18 - HSTXSRUDL"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrudl(&mut self) -> HSTXSRUDL_W<18> {
        HSTXSRUDL_W::new(self)
    }
    #[doc = "Bit 19 - HSTXSRDDL"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrddl(&mut self) -> HSTXSRDDL_W<19> {
        HSTXSRDDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr1](index.html) module"]
pub struct WPCR1_SPEC;
impl crate::RegisterSpec for WPCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr1::R](R) reader structure"]
impl crate::Readable for WPCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr1::W](W) writer structure"]
impl crate::Writable for WPCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPCR1 to value 0"]
impl crate::Resettable for WPCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
