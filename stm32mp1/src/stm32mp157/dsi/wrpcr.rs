#[doc = "Register `WRPCR` reader"]
pub struct R(crate::R<WRPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRPCR` writer"]
pub struct W(crate::W<WRPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRPCR_SPEC>;
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
impl From<crate::W<WRPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLEN` reader - PLLEN"]
pub type PLLEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLEN` writer - PLLEN"]
pub type PLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRPCR_SPEC, bool, O>;
#[doc = "Field `NDIV` reader - NDIV"]
pub type NDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NDIV` writer - NDIV"]
pub type NDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRPCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `IDF` reader - IDF"]
pub type IDF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDF` writer - IDF"]
pub type IDF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRPCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ODF` reader - ODF"]
pub type ODF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ODF` writer - ODF"]
pub type ODF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRPCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `REGEN` reader - REGEN"]
pub type REGEN_R = crate::BitReader<bool>;
#[doc = "Field `REGEN` writer - REGEN"]
pub type REGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRPCR_SPEC, bool, O>;
#[doc = "Field `BGREN` reader - BGREN"]
pub type BGREN_R = crate::BitReader<bool>;
#[doc = "Field `BGREN` writer - BGREN"]
pub type BGREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:8 - NDIV"]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 11:14 - IDF"]
    #[inline(always)]
    pub fn idf(&self) -> IDF_R {
        IDF_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - ODF"]
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - REGEN"]
    #[inline(always)]
    pub fn regen(&self) -> REGEN_R {
        REGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - BGREN"]
    #[inline(always)]
    pub fn bgren(&self) -> BGREN_R {
        BGREN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLLEN"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<0> {
        PLLEN_W::new(self)
    }
    #[doc = "Bits 2:8 - NDIV"]
    #[inline(always)]
    #[must_use]
    pub fn ndiv(&mut self) -> NDIV_W<2> {
        NDIV_W::new(self)
    }
    #[doc = "Bits 11:14 - IDF"]
    #[inline(always)]
    #[must_use]
    pub fn idf(&mut self) -> IDF_W<11> {
        IDF_W::new(self)
    }
    #[doc = "Bits 16:17 - ODF"]
    #[inline(always)]
    #[must_use]
    pub fn odf(&mut self) -> ODF_W<16> {
        ODF_W::new(self)
    }
    #[doc = "Bit 24 - REGEN"]
    #[inline(always)]
    #[must_use]
    pub fn regen(&mut self) -> REGEN_W<24> {
        REGEN_W::new(self)
    }
    #[doc = "Bit 28 - BGREN"]
    #[inline(always)]
    #[must_use]
    pub fn bgren(&mut self) -> BGREN_W<28> {
        BGREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI wrapper regulator and PLL control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrpcr](index.html) module"]
pub struct WRPCR_SPEC;
impl crate::RegisterSpec for WRPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrpcr::R](R) reader structure"]
impl crate::Readable for WRPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrpcr::W](W) writer structure"]
impl crate::Writable for WRPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRPCR to value 0"]
impl crate::Resettable for WRPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
